use prettytable::{cell, row, Row, Table};
use std::collections::BTreeMap;

const NUM_SAMPLES: i64 = 1 << 4;

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
    let mut set = BTreeMap::new();
    for i in 0..NUM_SAMPLES {
        set.entry(local_factor(i as u64))
            .and_modify(|n| {
                *n += 1;
            })
            .or_insert(1);
    }
    let mut table = Table::new();
    table.add_row(row!["Factor", "Deviation"]);
    for i in 1..10 {
        let expected = NUM_SAMPLES / i;
        table.add_row(row![
            i,
            (0..i)
                .map(|j| (set
                    .iter()
                    .filter(|&(&factor, _)| i64::from(factor) % i == j)
                    .map(|(_, count)| count)
                    .sum::<i64>()
                    - expected as i64)
                    .abs())
                .sum::<i64>()
        ]);
    }
    table.printstd();

    let mut table = Table::new();
    table.add_row(row![
        "Samples",
        "3 Deviation",
        "4 Deviation",
        "5 Deviation",
        "6 Deviation",
        "31 Deviation"
    ]);
    for samples in (1..20).map(|n| 1 << n) {
        let mut row = vec![cell!(samples)];
        for bucket_size in (3..=6).chain(Some(31)) {
            let expected = samples / bucket_size;
            row.push(cell!((0..bucket_size)
                .map(|j| ((0..samples)
                    .map(local_factor)
                    .filter(|&factor| i64::from(factor) % bucket_size as i64 == j as i64)
                    .count() as i64
                    - expected as i64)
                    .abs())
                .sum::<i64>()));
        }
        table.add_row(Row::new(row));
    }
    table.printstd();

    let mut table = Table::new();
    table.add_row(row!["Position", "31 Size Count"]);
    let expected = (1i64 << 21) / 31;
    for j in 0i64..31 {
        table.add_row(row![
            j,
            ((0..1 << 21)
                .map(local_factor)
                .filter(|&factor| i64::from(factor) % 31 as i64 == j)
                .count() as i64
                - expected)
                .abs()
        ]);
    }
    table.printstd();

    let mut table = Table::new();
    table.add_row(row![
        "Bits",
        "Distance 1 In Binary",
        "Distance 1 In Gray",
        "Distance 1 In Rev Gray",
        "Distance 2 In Binary",
        "Distance 2 In Gray",
        "Distance 2 In Rev Gray",
    ]);
    for bits in 2i64..16 {
        println!("on {} bits", bits);
        let test = |conv: fn(i64) -> i64| {
            (0i64..1 << bits)
                .map(|n| {
                    (0i64..bits)
                        .map(|b| n ^ (1i64 << b))
                        .map(|m| (conv(m) - conv(n)).abs() % (1 << (bits - 1)))
                        .sum::<i64>() as f64
                        / bits as f64
                })
                .sum::<f64>()
                / f64::from(1 << bits)
        };
        table.add_row(row![
            bits,
            test(|n| n),
            test(|n| n ^ (n >> 1)),
            test(|mut num| {
                num = num ^ (num >> 16);
                num = num ^ (num >> 8);
                num = num ^ (num >> 4);
                num = num ^ (num >> 2);
                num = num ^ (num >> 1);
                num
            })
        ]);
    }
    table.printstd();
}
