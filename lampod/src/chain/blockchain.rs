use std::sync::Arc;

use bitcoin::Transaction;
use lightning::chain::chaininterface::{BroadcasterInterface, ConfirmationTarget, FeeEstimator};
use lightning::chain::Filter;
use lightning::routing::utxo::UtxoLookup;

use crate::backend::Backend;
use crate::keys::keys::LampoKeys;

/// Lampo FeeEstimator implementation
#[derive(Clone)]
pub struct LampoChainManager {
    pub backend: Arc<dyn Backend>,
    pub keymanager: Arc<LampoKeys>,
}

/// Personal Lampo implementation
impl LampoChainManager {
    /// Create a new instance of LampoFeeEstimator with the specified
    /// Backend.
    pub fn new(client: Arc<dyn Backend>, keys: Arc<LampoKeys>) -> Self {
        LampoChainManager {
            backend: client,
            keymanager: keys,
        }
    }

    pub fn is_lightway(&self) -> bool {
        self.backend.is_lightway()
    }
}

/// Rust lightning FeeEstimator implementation
impl FeeEstimator for LampoChainManager {
    fn get_est_sat_per_1000_weight(&self, confirmation_target: ConfirmationTarget) -> u32 {
        return match confirmation_target {
            ConfirmationTarget::Background => self.backend.fee_rate_estimation(24),
            ConfirmationTarget::Normal => self.backend.fee_rate_estimation(6),
            ConfirmationTarget::HighPriority => self.backend.fee_rate_estimation(2),
        };
    }
}

/// Brodcaster Interface implementation for Lampo.
impl BroadcasterInterface for LampoChainManager {
    fn broadcast_transaction(&self, tx: &Transaction) {
        self.backend.brodcast_tx(tx);
    }
}

// FIXME: todo implement it.
impl Filter for LampoChainManager {
    fn register_output(&self, output: lightning::chain::WatchedOutput) {
        self.backend.register_output(output);
    }

    fn register_tx(&self, txid: &bitcoin::Txid, script_pubkey: &bitcoin::Script) {
        self.backend.watch_utxo(txid, script_pubkey)
    }
}

impl lightning_block_sync::BlockSource for LampoChainManager {
    fn get_best_block<'a>(
        &'a self,
    ) -> lightning_block_sync::AsyncBlockSourceResult<(bitcoin::BlockHash, Option<u32>)> {
        self.backend.get_best_block()
    }

    fn get_block<'a>(
        &'a self,
        header_hash: &'a bitcoin::BlockHash,
    ) -> lightning_block_sync::AsyncBlockSourceResult<'a, lightning_block_sync::BlockData> {
        self.backend.get_block(header_hash)
    }

    fn get_header<'a>(
        &'a self,
        header_hash: &'a bitcoin::BlockHash,
        height_hint: Option<u32>,
    ) -> lightning_block_sync::AsyncBlockSourceResult<'a, lightning_block_sync::BlockHeaderData>
    {
        self.backend.get_header(header_hash, height_hint)
    }
}

// FIXME: implement this
impl UtxoLookup for LampoChainManager {
    fn get_utxo(
        &self,
        genesis_hash: &bitcoin::BlockHash,
        short_channel_id: u64,
    ) -> lightning::routing::utxo::UtxoResult {
        unimplemented!()
    }
}

// FIXME: fix this
unsafe impl Send for LampoChainManager {}
unsafe impl Sync for LampoChainManager {}