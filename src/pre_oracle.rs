//! Pre-Oracle Hazard Index Lambda(t)
//!
//! Synthesizes DEX order book depth, OFI, and Pyth Network / Switchboard
//! confidence intervals to detect liquidation hazard before on-chain oracle heartbeats.

pub struct PreOracleHazardIndex;

impl PreOracleHazardIndex {
    /// Computes continuous hazard function Lambda(t) in [0.0, 1.0].
    /// If Lambda(t) > 0.82, liquidation slippage will cause bad debt.
    pub fn evaluate_hazard(
        z_score_vol_400ms: f64,
        ofi_slot: f64,
        oracle_price: f64,
        oracle_conf: f64,
        dex_execution_price: f64,
        depth_ratio_top5: f64,
    ) -> f64 {
        let delta_oracle = if oracle_price > 0.0 {
            (oracle_price - dex_execution_price) / oracle_price
        } else {
            0.0
        };

        let conf_ratio = if oracle_price > 0.0 {
            oracle_conf / oracle_price
        } else {
            0.001
        };

        // Linear hazard score z
        let alpha_1 = 0.35;
        let alpha_2 = 0.40;
        let alpha_3 = 1.20;
        let alpha_4 = 0.50;

        let z = (alpha_1 * z_score_vol_400ms) - (alpha_2 * ofi_slot)
            + (alpha_3 * (delta_oracle / conf_ratio.max(1e-4)))
            + (alpha_4 * (1.0 - depth_ratio_top5));

        // Sigmoid activation
        1.0 / (1.0 + (-z).exp())
    }
}
