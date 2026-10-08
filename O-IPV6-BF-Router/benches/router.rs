//  ----------------------------------------------------
//          benchmark
//  ----------------------------------------------------  
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use neuralmesh_router::{HierarchicalRouter, LevelConfig, Prefix};
use std::net::Ipv6Addr;

fn build_router(n: usize) -> HierarchicalRouter {
    let configs = vec![
        LevelConfig { prefix_len: 48,  expected_entries: n, seed: 1 },
        LevelConfig { prefix_len: 64,  expected_entries: n, seed: 2 },
        LevelConfig { prefix_len: 80,  expected_entries: n, seed: 3 },
        LevelConfig { prefix_len: 128, expected_entries: n, seed: 4 },
    ];
    let mut r = HierarchicalRouter::new(configs);
    for i in 0..n {
        let area = (i & 0xFF) as u16;
        let domain = ((i >> 8) & 0xFF) as u16;
        let sub = ((i >> 16) & 0xFF) as u16;
        r.level_mut(0).insert(&Prefix::new(Ipv6Addr::new(0x2001, 0xdb8, area, 0, 0, 0, 0, 0), 48));
        r.level_mut(1).insert(&Prefix::new(Ipv6Addr::new(0x2001, 0xdb8, area, domain, 0, 0, 0, 0), 64));
        r.level_mut(2).insert(&Prefix::new(Ipv6Addr::new(0x2001, 0xdb8, area, domain, sub, 0, 0, 0), 80));
        r.level_mut(3).insert(&Prefix::new(Ipv6Addr::new(0x2001, 0xdb8, area, domain, sub, 0, 0, (i & 0xFFFF) as u16), 128));
    }
    r.finalize();
    r
}

fn bench_route(c: &mut Criterion) {
    let mut group = c.benchmark_group("route");
    for &n in &[1_000usize, 10_000, 100_000] {
        let router = build_router(n);
        let query = Prefix::new(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0), 48);
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, _| {
            b.iter(|| router.route(black_box(&[query])));
        });
    }
    group.finish();
}

fn bench_bloom_check(c: &mut Criterion) {
    let router = build_router(100_000);
    let addr = [0x20, 0x01, 0x0d, 0xb8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    c.bench_function("bloom_check", |b| {
        b.iter(|| router.level(0).bloom_contains(black_box(&addr)));
    });
}

criterion_group!(benches, bench_route, bench_bloom_check);
criterion_main!(benches);