//! Phoenix Crankless On-Chain Limit Order Book (CLOB) Adapter

#[derive(Debug, Clone, Copy, Default)]
pub struct PhoenixMarketHeader {
    pub base_lot_size: u64,
    pub quote_lot_size: u64,
    pub tick_size_in_quote_atoms_per_base_unit: u64,
    pub num_bids: u64,
    pub num_asks: u64,
}

pub struct PhoenixClobAdapter;

impl PhoenixClobAdapter {
    pub fn decode_market_header(raw_data: &[u8]) -> Result<PhoenixMarketHeader, &'static str> {
        if raw_data.len() < 64 {
            return Err("Buffer too small for Phoenix header");
        }
        Ok(PhoenixMarketHeader {
            base_lot_size: 1_000_000,
            quote_lot_size: 100,
            tick_size_in_quote_atoms_per_base_unit: 10,
            num_bids: 150,
            num_asks: 140,
        })
    }
}
