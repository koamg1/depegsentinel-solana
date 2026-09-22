//! Execution Slippage Cliff Analyzer
//!
//! Predicts non-linear execution slippage across discrete tick arrays
//! in Orca Whirlpools, Raydium CLMM, and Meteora DLMM bins.

#[derive(Debug, Clone)]
pub struct ExecutionSlippageCliff;

impl ExecutionSlippageCliff {
    /// Computes the effective slippage penalty Lambda(Q) for liquidation volume Q.
    /// If liquidation volume exceeds cumulative active tick depth, slippage spikes exponentially.
    pub fn calculate_slippage_cliff(
        liquidation_usd: f64,
        active_tick_depth_usd: f64,
        reserve_asymmetry: f64,
    ) -> f64 {
        if active_tick_depth_usd <= 0.0 {
            return 100.0; // 100% slippage collapse
        }

        let depth_ratio = liquidation_usd / active_tick_depth_usd;
        if depth_ratio <= 0.25 {
            // Linear low-impact regime
            depth_ratio * 0.4 * (1.0 + reserve_asymmetry)
        } else if depth_ratio <= 1.0 {
            // Quadratic compression regime
            (depth_ratio.powi(2) * 1.8) * (1.0 + reserve_asymmetry)
        } else {
            // Slippage Cliff regime (liquidity exhaustion)
            let excess = depth_ratio - 1.0;
            2.0 + (excess.powf(2.5) * 8.5) + (reserve_asymmetry * 5.0)
        }
    }
}
