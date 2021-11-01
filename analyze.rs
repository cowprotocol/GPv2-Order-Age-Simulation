use std::io::BufRead;

fn main() {
    // all f64 variables represent time in seconds

    // unix timestamp when order arrived, sorted
    let orders: Vec<f64> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().split(',').nth(1).unwrap().parse().unwrap())
        .collect();

    // time single driver run takes if it doesn't settle
    let single_run_no_settlement = 5.;
    // time single driver run takes if it settles
    let single_run_settlement = 45.;

    // Simulate different min order age values.
    for min_order_age in [30., 60., 120., 180., 240., 300.] {
        // how often each settlement size occurred
        let mut settlements: std::collections::BTreeMap<u64, u64> = Default::default();
        // total time orders spent between creation and transaction getting mined
        let mut total_wait_time = 0.;

        // index of the first order in the current solvable orders
        let mut first_solvable_order = None;
        let mut now = orders[0];
        // next unhandled order index, might be in the future
        let mut next_order = 0;
        'outer: loop {
            loop {
                if next_order >= orders.len() {
                    break 'outer;
                }
                if orders[next_order] > now {
                    break;
                }
                if first_solvable_order.is_none() {
                    first_solvable_order = Some(next_order);
                }
                next_order += 1;
            }
            let first = match first_solvable_order {
                Some(first) if now > orders[first] + min_order_age => first,
                // no first order or all below min age
                _ => {
                    now += single_run_no_settlement;
                    continue;
                }
            };
            // at least one order above min age
            now += single_run_settlement;
            for order in &orders[first..next_order] {
                total_wait_time += now - order;
            }
            let settlement_size = next_order - first;
            *settlements.entry(settlement_size as u64).or_default() += 1;
            first_solvable_order = None;
        }

        println!("with {} seconds min_order_age", min_order_age);
        for (orders, count) in &settlements {
            println!("  {} -> {}", orders, count);
        }
        let total_settlements: u64 = settlements.values().sum();
        let total_trades: u64 = settlements
            .iter()
            .map(|(orders, count)| orders * count)
            .sum();
        println!(
            "  average trades per settlement {:.1}",
            total_trades as f64 / total_settlements as f64
        );
        println!(
            "  average order wait time {:.1} seconds",
            total_wait_time / total_trades as f64
        );
        println!();
    }
}
