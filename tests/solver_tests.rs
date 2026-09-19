use depegsentinel_solana::curve::StableswapSolver;
use depegsentinel_solana::ofi::{SlotOrderFlowImbalance, LevelQuote};
use depegsentinel_solana::slippage::ExecutionSlippageCliff;
use depegsentinel_solana::pre_oracle::PreOracleHazardIndex;

#[test]
fn test_stableswap_invariant_convergence() {
    let balances = vec![10_000_000.0, 10_000_000.0, 10_000_000.0];
    let a = 100.0;
    let d = StableswapSolver::compute_d(&balances, a);
    assert!((d - 30_000_000.0).abs() < 1.0);
}

#[test]
fn test_stableswap_simulate_swap() {
    let balances = vec![10_000_000.0, 10_000_000.0];
    let a = 100.0;
    let metrics = StableswapSolver::simulate_swap(&balances, a, 0, 1, 100_000.0).unwrap();
    assert!(metrics.output_dy > 99_000.0);
    assert!(metrics.price_impact_pct < 1.0);
}

#[test]
fn test_ofi_calculation() {
    let mut ofi = SlotOrderFlowImbalance::new();
    let q1 = LevelQuote { bid_price: 1.0, bid_qty: 1000.0, ask_price: 1.0001, ask_qty: 1000.0 };
    let q2 = LevelQuote { bid_price: 1.0, bid_qty: 1500.0, ask_price: 1.0001, ask_qty: 800.0 };
    let val1 = ofi.update(q1);
    let val2 = ofi.update(q2);
    assert_eq!(val1, 0.0);
    assert!(val2 > 0.0); // Buying pressure detected
}

#[test]
fn test_pre_oracle_hazard_index() {
    let hazard_safe = PreOracleHazardIndex::evaluate_hazard(0.1, 1.2, 1.000, 0.001, 0.9998, 0.95);
    assert!(hazard_safe < 0.50);

    let hazard_danger = PreOracleHazardIndex::evaluate_hazard(3.8, -4.5, 1.000, 0.001, 0.9250, 0.20);
    assert!(hazard_danger > 0.85); // High alert for bad debt
}
