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

use std::path::PathBuf;

use anyhow::Result;
use ethers_core::types::{
    Block, Bytes, EIP1186ProofResponse, Transaction, TransactionReceipt, H256, U256,
};

use super::{
    file_provider::FileProvider, rpc_provider::RpcProvider, AccountQuery, BlockQuery, MutProvider,
    ProofQuery, Provider, StorageQuery,
};

#[cfg(feature = "taiko")]
use crate::host::provider::{LogsQuery, TxQuery};
#[cfg(feature = "taiko")]
use ethers_core::types::Log;


pub struct CachedRpcProvider {
    cache: FileProvider,
    rpc: RpcProvider,
}

impl CachedRpcProvider {
    pub fn new(cache_path: PathBuf, rpc_url: String) -> Result<Self> {
        let cache = match FileProvider::from_file(&cache_path) {
            Ok(provider) => provider,
            Err(_) => FileProvider::empty(cache_path),
        };
        let rpc = RpcProvider::new(rpc_url)?;

        Ok(CachedRpcProvider { cache, rpc })
    }
}

impl Provider for CachedRpcProvider {
    fn save(&self) -> Result<()> {
        self.cache.save()
    }

    fn get_full_block(&mut self, query: &BlockQuery) -> Result<Block<Transaction>> {
        let cache_out = self.cache.get_full_block(query);
        if cache_out.is_ok() {
            return cache_out;
        }

        let out = self.rpc.get_full_block(query)?;
        self.cache.insert_full_block(query.clone(), out.clone());

        Ok(out)
    }

    fn get_partial_block(&mut self, query: &BlockQuery) -> Result<Block<H256>> {
        let cache_out = self.cache.get_partial_block(query);
        if cache_out.is_ok() {
            return cache_out;
        }

        let out = self.rpc.get_partial_block(query)?;
        self.cache.insert_partial_block(query.clone(), out.clone());

        Ok(out)
    }

    fn get_block_receipts(&mut self, query: &BlockQuery) -> Result<Vec<TransactionReceipt>> {
        let cache_out = self.cache.get_block_receipts(query);
        if cache_out.is_ok() {
            return cache_out;
        }

        let out = self.rpc.get_block_receipts(query)?;
        self.cache.insert_block_receipts(query.clone(), out.clone());

        Ok(out)
    }

    fn get_proof(&mut self, query: &ProofQuery) -> Result<EIP1186ProofResponse> {
        let cache_out = self.cache.get_proof(query);
        if cache_out.is_ok() {
            return cache_out;
        }

        let out = self.rpc.get_proof(query)?;
        self.cache.insert_proof(query.clone(), out.clone());

        Ok(out)
    }

    fn get_transaction_count(&mut self, query: &AccountQuery) -> Result<U256> {
        let cache_out = self.cache.get_transaction_count(query);
        if cache_out.is_ok() {
            return cache_out;
        }

        let out = self.rpc.get_transaction_count(query)?;
        self.cache.insert_transaction_count(query.clone(), out);

        Ok(out)
    }

    fn get_balance(&mut self, query: &AccountQuery) -> Result<U256> {
        let cache_out = self.cache.get_balance(query);
        if cache_out.is_ok() {
            return cache_out;
        }

        let out = self.rpc.get_balance(query)?;
        self.cache.insert_balance(query.clone(), out);

        Ok(out)
    }

    fn get_code(&mut self, query: &AccountQuery) -> Result<Bytes> {
        let cache_out = self.cache.get_code(query);
        if cache_out.is_ok() {
            return cache_out;
        }

        let out = self.rpc.get_code(query)?;
        self.cache.insert_code(query.clone(), out.clone());

        Ok(out)
    }

    fn get_storage(&mut self, query: &StorageQuery) -> Result<H256> {
        let cache_out = self.cache.get_storage(query);
        if cache_out.is_ok() {
            return cache_out;
        }

        let out = self.rpc.get_storage(query)?;
        self.cache.insert_storage(query.clone(), out);

        Ok(out)
    }

    #[cfg(feature = "taiko")]
    fn get_logs(&mut self, query: &LogsQuery) -> Result<Vec<Log>> {
        let cache_out = self.cache.get_logs(query);
        if cache_out.is_ok() {
            return cache_out;
        }

        let out = self.rpc.get_logs(query)?;
        self.cache.insert_logs(query.clone(), out);

        Ok(out)
    }

    #[cfg(feature = "taiko")]
    fn get_transaction(&mut self, query: &super::TxQuery) -> Result<Transaction> {
        let mut cache_out = self.cache.get_transaction(query);
        if cache_out.is_ok() {
            return cache_out;
        }

        // Search cached block for target Tx
        if let Some(block_no) = query.block_no {
            let cache_block_out  = self.cache
                .get_full_block(&BlockQuery {block_no: block_no})
                .map(|b| b.transactions.iter().filter(|tx| tx.hash == query.tx_hash).collect::<Vec<_>>())
                .map(|txs| txs.pop());
            if let Ok(tx_op) = cache_block_out {
                if let Some(tx) = tx_op {
                    return Ok(tx.clone())
                }
            }   
        }

        let out = self.rpc.get_transaction(query)?;
        self.cache.insert_transaction(query.clone(), out);

        Ok(out)
    }
}
