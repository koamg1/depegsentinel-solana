//! # DepegSentinel Solana Core
//! 
//! High-performance pre-oracle liquidity risk telemetry, sub-slot Order Flow Imbalance (OFI),
//! and non-linear invariant numerical solver for the Solana Sealevel runtime.
//! 
//! Designed and optimized for sub-15 microsecond execution within Solana's 400ms slot constraints.

pub mod curve;
pub mod ofi;
pub mod slippage;
pub mod dex;
pub mod pre_oracle;

pub use curve::StableswapSolver;
pub use ofi::SlotOrderFlowImbalance;
pub use slippage::ExecutionSlippageCliff;
pub use pre_oracle::PreOracleHazardIndex;
