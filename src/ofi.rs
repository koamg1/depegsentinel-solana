//! Slot-by-Slot Order Flow Imbalance (OFI) Engine for Solana
//!
//! Measures net order flow pressure across 400ms slots to detect aggressive
//! liquidity drainage before prices re-anchor.

#[derive(Debug, Clone, Copy, Default)]
pub struct LevelQuote {
    pub bid_price: f64,
    pub bid_qty: f64,
    pub ask_price: f64,
    pub ask_qty: f64,
}

#[derive(Debug, Clone)]
pub struct SlotOrderFlowImbalance {
    prev_quote: Option<LevelQuote>,
}

impl SlotOrderFlowImbalance {
    pub fn new() -> Self {
        Self { prev_quote: None }
    }

    /// Evaluates OFI for the current slot k compared to slot k-1:
    /// OFI = (I_B - I_A) / (0.5 * (Q_B + Q_A))
    pub fn update(&mut self, curr: LevelQuote) -> f64 {
        let prev = match self.prev_quote {
            Some(p) => p,
            None => {
                self.prev_quote = Some(curr);
                return 0.0;
            }
        };

        // Bid contribution I_B
        let i_b = if curr.bid_price > prev.bid_price {
            curr.bid_qty
        } else if (curr.bid_price - prev.bid_price).abs() < 1e-9 {
            curr.bid_qty - prev.bid_qty
        } else {
            -prev.bid_qty
        };

        // Ask contribution I_A
        let i_a = if curr.ask_price < prev.ask_price {
            curr.ask_qty
        } else if (curr.ask_price - prev.ask_price).abs() < 1e-9 {
            curr.ask_qty - prev.ask_qty
        } else {
            -prev.ask_qty
        };

        let depth_denominator = 0.5 * (curr.bid_qty + curr.ask_qty);
        let ofi = if depth_denominator > 0.0 {
            (i_b - i_a) / depth_denominator
        } else {
            0.0
        };

        self.prev_quote = Some(curr);
        ofi
    }
}
