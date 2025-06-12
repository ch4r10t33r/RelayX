use jsonrpc_core::{IoHandler, Result};
use jsonrpc_derive::rpc;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub hash: String,
    pub nonce: u64,
    pub block_hash: Option<String>,
    pub block_number: Option<u64>,
    pub transaction_index: Option<u64>,
    pub from: String,
    pub to: Option<String>,
    pub value: String,
    pub gas_price: String,
    pub gas: u64,
    pub input: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionReceipt {
    pub transaction_hash: String,
    pub transaction_index: u64,
    pub block_hash: String,
    pub block_number: u64,
    pub from: String,
    pub to: Option<String>,
    pub cumulative_gas_used: u64,
    pub gas_used: u64,
    pub contract_address: Option<String>,
    pub logs: Vec<Log>,
    pub status: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Log {
    pub address: String,
    pub topics: Vec<String>,
    pub data: String,
    pub block_number: u64,
    pub block_hash: String,
    pub transaction_hash: String,
    pub transaction_index: u64,
    pub log_index: u64,
    pub removed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub number: u64,
    pub hash: String,
    pub parent_hash: String,
    pub nonce: String,
    pub sha3_uncles: String,
    pub logs_bloom: String,
    pub transactions_root: String,
    pub state_root: String,
    pub receipts_root: String,
    pub miner: String,
    pub difficulty: String,
    pub total_difficulty: String,
    pub size: u64,
    pub extra_data: String,
    pub gas_limit: u64,
    pub gas_used: u64,
    pub timestamp: u64,
    pub transactions: Vec<String>,
}

#[rpc]
pub trait RelayApi {
    #[rpc(name = "relay_getTransactionByHash")]
    fn get_transaction_by_hash(&self, hash: String) -> Result<Option<Transaction>>;

    #[rpc(name = "relay_getTransactionReceipt")]
    fn get_transaction_receipt(&self, hash: String) -> Result<Option<TransactionReceipt>>;

    #[rpc(name = "relay_getBlockByNumber")]
    fn get_block_by_number(&self, number: u64, include_txs: bool) -> Result<Option<Block>>;

    #[rpc(name = "relay_getBlockByHash")]
    fn get_block_by_hash(&self, hash: String, include_txs: bool) -> Result<Option<Block>>;

    #[rpc(name = "relay_getLogs")]
    fn get_logs(&self, filter: LogFilter) -> Result<Vec<Log>>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogFilter {
    pub from_block: Option<u64>,
    pub to_block: Option<u64>,
    pub address: Option<String>,
    pub topics: Option<Vec<Option<String>>>,
    pub block_hash: Option<String>,
}

pub struct RelayApiImpl;

impl RelayApi for RelayApiImpl {
    fn get_transaction_by_hash(&self, _hash: String) -> Result<Option<Transaction>> {
        // TODO: Implement actual transaction lookup
        Ok(None)
    }

    fn get_transaction_receipt(&self, _hash: String) -> Result<Option<TransactionReceipt>> {
        // TODO: Implement actual receipt lookup
        Ok(None)
    }

    fn get_block_by_number(&self, _number: u64, _include_txs: bool) -> Result<Option<Block>> {
        // TODO: Implement actual block lookup
        Ok(None)
    }

    fn get_block_by_hash(&self, _hash: String, _include_txs: bool) -> Result<Option<Block>> {
        // TODO: Implement actual block lookup
        Ok(None)
    }

    fn get_logs(&self, _filter: LogFilter) -> Result<Vec<Log>> {
        // TODO: Implement actual log filtering
        Ok(Vec::new())
    }
}

pub struct RpcServer {
    handler: IoHandler,
}

impl RpcServer {
    pub fn new() -> Self {
        let mut io = IoHandler::default();
        let api = RelayApiImpl;
        io.extend_with(api.to_delegate());
        Self { handler: io }
    }

    pub async fn start(self, addr: SocketAddr) -> anyhow::Result<()> {
        let server = jsonrpc_http_server::ServerBuilder::new(self.handler).start_http(&addr)?;

        server.wait();
        Ok(())
    }
}

impl Default for RpcServer {
    fn default() -> Self {
        Self::new()
    }
}
