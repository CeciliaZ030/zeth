// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use core::fmt::Debug;
use anyhow::{anyhow, bail, Context};
use anyhow::Result;
use revm::EVM;
use revm::primitives::ResultAndState;
use revm::{Database, DatabaseCommit, primitives::{TxEnv, TransactTo, Account}};
use zeth_primitives::transactions::optimism::TxEssenceOptimismDeposited;
use zeth_primitives::{transactions::{TxEssence, ethereum::{EthereumTxEssence, TransactionKind}}, Address, U256};

use super::BlockBuilder;

pub(super) mod ethereum;
pub(super) mod optimism;
pub(super) mod taiko;

pub trait TxExecStrategy<E: TxEssence> {
    fn execute_transactions<D>(block_builder: BlockBuilder<D, E>) -> Result<BlockBuilder<D, E>>
    where
        D: Database + DatabaseCommit,
        <D as Database>::Error: Debug;
}


pub fn fill_eth_tx_env(tx_env: &mut TxEnv, essence: &EthereumTxEssence, caller: Address) {
    match essence {
        EthereumTxEssence::Legacy(tx) => {
            tx_env.caller = caller;
            tx_env.gas_limit = tx.gas_limit.try_into().unwrap();
            tx_env.gas_price = tx.gas_price;
            tx_env.gas_priority_fee = None;
            tx_env.transact_to = if let TransactionKind::Call(to_addr) = tx.to {
                TransactTo::Call(to_addr)
            } else {
                TransactTo::create()
            };
            tx_env.value = tx.value;
            tx_env.data = tx.data.clone();
            tx_env.chain_id = tx.chain_id;
            tx_env.nonce = Some(tx.nonce);
            tx_env.access_list.clear();
        }
        EthereumTxEssence::Eip2930(tx) => {
            tx_env.caller = caller;
            tx_env.gas_limit = tx.gas_limit.try_into().unwrap();
            tx_env.gas_price = tx.gas_price;
            tx_env.gas_priority_fee = None;
            tx_env.transact_to = if let TransactionKind::Call(to_addr) = tx.to {
                TransactTo::Call(to_addr)
            } else {
                TransactTo::create()
            };
            tx_env.value = tx.value;
            tx_env.data = tx.data.clone();
            tx_env.chain_id = Some(tx.chain_id);
            tx_env.nonce = Some(tx.nonce);
            tx_env.access_list = tx.access_list.clone().into();
        }
        EthereumTxEssence::Eip1559(tx) => {
            tx_env.caller = caller;
            tx_env.gas_limit = tx.gas_limit.try_into().unwrap();
            tx_env.gas_price = tx.max_fee_per_gas;
            tx_env.gas_priority_fee = Some(tx.max_priority_fee_per_gas);
            tx_env.transact_to = if let TransactionKind::Call(to_addr) = tx.to {
                TransactTo::Call(to_addr)
            } else {
                TransactTo::create()
            };
            tx_env.value = tx.value;
            tx_env.data = tx.data.clone();
            tx_env.chain_id = Some(tx.chain_id);
            tx_env.nonce = Some(tx.nonce);
            tx_env.access_list = tx.access_list.clone().into();
        }
    };
}

pub fn increase_account_balance<D>(
    db: &mut D,
    address: Address,
    amount_wei: U256,
) -> anyhow::Result<()>
where
    D: Database + DatabaseCommit,
    <D as Database>::Error: Debug,
{
    // Read account from database
    let mut account: Account = db
        .basic(address)
        .map_err(|db_err| {
            anyhow!(
                "Error increasing account balance for {}: {:?}",
                address,
                db_err
            )
        })?
        .unwrap_or_default()
        .into();
    // Credit withdrawal amount
    account.info.balance = account.info.balance.checked_add(amount_wei).unwrap();
    account.mark_touch();
    // Commit changes to database
    db.commit([(address, account)].into());

    Ok(())
}


fn read_uint<D>(
    evm: &mut EVM<D>,
    abi_call: Vec<u8>,
    chain_id: Option<zeth_primitives::ChainId>,
    gas_limit: U256,
    address: Address,
) -> Result<U256>
where
    D: Database + DatabaseCommit,
    <D as Database>::Error: Debug,
{
    let op_l1_tx =
        EthereumTxEssence::Legacy(zeth_primitives::transactions::ethereum::TxEssenceLegacy {
            chain_id,
            nonce: 0,
            gas_price: U256::ZERO,
            gas_limit,
            to: TransactionKind::Call(address),
            value: U256::ZERO,
            data: abi_call.into(),
        });

    // disable base fees
    evm.env.cfg.disable_base_fee = true;
    evm.env.cfg.disable_balance_check = true;
    fill_eth_tx_env(&mut evm.env.tx, &op_l1_tx, Default::default());

    let Ok(ResultAndState {
        result: execution_result,
        ..
    }) = evm.transact()
    else {
        bail!("Error during execution");
    };

    let revm::primitives::ExecutionResult::Success { output, .. } = execution_result else {
        bail!("Result unsuccessful");
    };

    let revm::primitives::Output::Call(result_encoded) = output else {
        bail!("Unsupported result");
    };

    let ethers_core::abi::Token::Uint(uint_result) =
        ethers_core::abi::decode(&[ethers_core::abi::ParamType::Uint(256)], &result_encoded)?
            .pop()
            .unwrap()
    else {
        bail!("Could not decode result");
    };

    Ok(U256::from_limbs(uint_result.0))
}

fn fill_deposit_tx_env(
    tx_env: &mut TxEnv,
    tx: &TxEssenceOptimismDeposited,
    caller: Address,
    deposit_nonce: Option<u64>,
) {
    tx_env.caller = caller; // previously overridden to tx.from
    tx_env.gas_limit = tx.gas_limit.try_into().unwrap();
    tx_env.gas_price = U256::ZERO;
    tx_env.gas_priority_fee = None;
    tx_env.transact_to = if let TransactionKind::Call(to_addr) = tx.to {
        TransactTo::Call(to_addr)
    } else {
        TransactTo::create()
    };
    tx_env.value = tx.value;
    tx_env.data = tx.data.clone();
    tx_env.chain_id = None;
    tx_env.nonce = deposit_nonce;
    tx_env.access_list.clear();
}

pub fn decrease_account_balance<D>(
    db: &mut D,
    address: Address,
    amount_wei: U256,
) -> anyhow::Result<()>
where
    D: Database + DatabaseCommit,
    <D as Database>::Error: Debug,
{
    // Read account from database
    let mut account: Account = db
        .basic(address)
        .map_err(|db_err| {
            anyhow!(
                "Error decreasing account balance for {}: {:?}",
                address,
                db_err
            )
        })?
        .unwrap_or_default()
        .into();
    // Credit withdrawal amount
    account.info.balance = account.info.balance.checked_sub(amount_wei).unwrap();
    account.mark_touch();
    // Commit changes to database
    db.commit([(address, account)].into());

    Ok(())
}
