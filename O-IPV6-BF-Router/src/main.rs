//  ----------------------------------------------------
//          Example
//  ----------------------------------------------------  
use neuralmesh_router::{HierarchicalRouter, LevelConfig, Prefix};
use std::net::Ipv6Addr;
use std::time::Instant;

fn main() {
    // 5 levels 
    let configs = vec![
        LevelConfig { prefix_len: 48,  expected_entries: 1_000_000, seed: 0xA1 },
        LevelConfig { prefix_len: 64,  expected_entries: 1_000_000, seed: 0xA2 },
        LevelConfig { prefix_len: 80,  expected_entries: 1_000_000, seed: 0xA3 },
        LevelConfig { prefix_len: 96,  expected_entries: 1_000_000, seed: 0xA4 },
        LevelConfig { prefix_len: 128, expected_entries: 1_000_000, seed: 0xA5 },
    ];

    let mut router = HierarchicalRouter::new(configs);

    println!("Inserting synthetic experts...");
    let t = Instant::now();

    // 1000 experts on 10 areas, on 10 domains each.
    for area in 0u16..10 {
        // area prefix
        let area_addr = Ipv6Addr::new(0x2001, 0xdb8, area, 0, 0, 0, 0, 0);
        router.level_mut(0).insert(&Prefix::new(area_addr, 48));

        for domain in 0u16..10 {
            let domain_addr = Ipv6Addr::new(0x2001, 0xdb8, area, domain, 0, 0, 0, 0);
            router.level_mut(1).insert(&Prefix::new(domain_addr, 64));

            for expert in 0u16..10 {
                let expert_addr =
                    Ipv6Addr::new(0x2001, 0xdb8, area, domain, expert, 0, 0, 0);
                router.level_mut(2).insert(&Prefix::new(expert_addr, 80));

                let expert_full = Ipv6Addr::new(
                    0x2001, 0xdb8, area, domain, expert, 0, 0, expert,
                );
                router.level_mut(4).insert(&Prefix::new(expert_full, 128));
            }
        }
    }

    router.finalize();
    println!(
        "Built in {:?} | memory: {:.2} MB",
        t.elapsed(),
        router.memory_bytes() as f64 / 1e6
    );

    println!("\nRouting query...");
    let query = Prefix::new(Ipv6Addr::new(0x2001, 0xdb8, 3, 0, 0, 0, 0, 0), 48);
    let t = Instant::now();
    let results = router.route(&[query]);
    let elapsed = t.elapsed();

    println!("Results: {} candidates in {:?}", results.len(), elapsed);
    for r in results.iter().take(5) {
        println!("  → {:?}/{}", r.addr, r.len);
    }

    // benchmark 
    // println!("\nBenchmarking 10,000 queries...");
    // let queries: Vec<Prefix> = (0u16..10)
    //     .map(|a| Prefix::new(Ipv6Addr::new(0x2001, 0xdb8, a, 0, 0, 0, 0, 0), 48))
    //     .collect();

    // let t = Instant::now();
    // let mut total = 0usize;
    // for _ in 0..1_000 {
    //     for q in &queries {
    //         total += router.route(std::slice::from_ref(q)).len();
    //     }
    // }
    // let elapsed = t.elapsed();
    // println!(
    //     "{} results in {:?} ({:.0} queries/sec)",
    //     total,
    //     elapsed,
    //     10_000.0 / elapsed.as_secs_f64()
    // );

    // // memory check
    // println!("\nMemory breakdown:");
    // for (i, level) in (0..router.num_levels()).map(|i| (i, router.level(i))) {
    //     println!(
    //         "  Level {} (/{:3}): {:>6} prefixes, {:>8.2} MB bloom",
    //         i,
    //         level.config.prefix_len,
    //         level.prefixes.len(),
    //         level.bloom.memory_bytes() as f64 / 1e6,
    //     );
    // }
}