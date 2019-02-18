use prettytable::{cell, row, Table};
use std::collections::BTreeMap;

fn local_factor(n: u64) -> u32 {
    // Compute gray code for every group of 2 bits.
    let gray = n ^ ((n & 0xAAAA_AAAA_AAAA_AAAA) >> 1);
    // Count the upper bits.
    let upper = (gray & 0xAAAA_AAAA_AAAA_AAAA).count_ones();
    // Count the lower bits.
    let lower = (gray & 0x5555_5555_5555_5555).count_ones();
    // Weight the upper bits by 2 and lower bits by 1.
    (upper << 1) + lower
}

fn main() {
    let mut table = Table::new();
    table.add_row(row!["Factor", "Deviation"]);
    let mut set = BTreeMap::new();
    for i in 0..(1 << 20) {
        set.entry(local_factor(i))
            .and_modify(|n| {
                *n += 1;
            })
            .or_insert(1);
    }
    for i in 1..10 {
        let expected = (1 << 20) / i;
        table.add_row(row![
            i,
            (0..i)
                .map(|j| (set
                    .iter()
                    .filter(|&(factor, _)| factor % i == j)
                    .map(|(_, count)| count)
                    .sum::<i32>()
                    - expected as i32)
                    .abs())
                .sum::<i32>()
                / i as i32
        ]);
    }
    table.printstd();
}
