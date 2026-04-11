use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::path::Path;

use cachefetch::get_os::detect_system;
use cachefetch::get_size::get_size;

fn benchmark_detect_system(c: &mut Criterion) {
    c.bench_function("detect_system", |b| b.iter(|| black_box(detect_system())));
}

fn benchmark_get_size(c: &mut Criterion) {
    let folders = detect_system();
    if let Some(folder) = folders.first() {
        let path = Path::new(folder);
        if path.exists() {
            c.bench_function("get_size", |b| b.iter(|| black_box(get_size(path))));
        }
    }
}

fn benchmark_full_scan(c: &mut Criterion) {
    c.bench_function("full_cache_scan", |b| {
        b.iter_custom(|iters| {
            let mut total = std::time::Duration::ZERO;
            for _ in 0..iters {
                // Clear page cache for cold run (requires root on Linux)
                #[cfg(target_os = "linux")]
                if std::fs::write("/proc/sys/vm/drop_caches", "3").is_err() {
                    // Only warn once per benchmark, not every iteration
                    static WARNED: std::sync::Once = std::sync::Once::new();
                    WARNED.call_once(|| {
                         eprintln!("Warning: Unable to drop caches (requires root). Running warm cache benchmark.");
                    });
                }
                let start = std::time::Instant::now();
                let folders = detect_system();
                let cache_data: Vec<(String, u64)> = folders
                    .iter()
                    .filter_map(|folder| {
                        let path = Path::new(folder);
                        match get_size(path) {
                            Ok(size) if size > 0 => Some((folder.to_string(), size)),
                            _ => None,
                        }
                    })
                    .collect();
                total += start.elapsed();
                black_box(cache_data);
            }
            total
        })
    });
}

criterion_group!(
    benches,
    benchmark_detect_system,
    benchmark_get_size,
    benchmark_full_scan
);
criterion_main!(benches);
