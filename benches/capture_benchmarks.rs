use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::time::Duration;
use watchdog::PlatformCapture;

fn benchmark_single_capture(c: &mut Criterion) {
    let capture = PlatformCapture::new().expect("Failed to create capture");

    c.bench_function("single_capture", |b| {
        b.iter(|| {
            let frame = capture.capture_full_screen().expect("Capture failed");
            black_box(frame); // Prevent optimization
        })
    });
}

fn benchmark_capture_throughput(c: &mut Criterion) {
    let capture = PlatformCapture::new().expect("Failed to create capture");

    let mut group = c.benchmark_group("capture_throughput");

    // Test different batch sizes
    for batch_size in [1, 5, 10, 25, 50].iter() {
        group.bench_with_input(
            BenchmarkId::new("batch_capture", batch_size),
            batch_size,
            |b, &size| {
                b.iter(|| {
                    for _ in 0..size {
                        let frame = capture.capture_full_screen().expect("Capture failed");
                        black_box(frame);
                    }
                })
            },
        );
    }
    group.finish();
}

fn benchmark_fps_targets(c: &mut Criterion) {
    let capture = PlatformCapture::new().expect("Failed to create capture");

    let mut group = c.benchmark_group("fps_simulation");
    group.measurement_time(Duration::from_secs(10)); // Longer measurement for stability

    // Test if we can sustain different FPS targets
    for fps in [15, 30, 60].iter() {
        let frame_time = Duration::from_millis(1000 / fps);

        group.bench_with_input(
            BenchmarkId::new("sustained_fps", fps),
            &frame_time,
            |b, &target_interval| {
                b.iter_custom(|iters| {
                    let start = std::time::Instant::now();

                    for _ in 0..iters {
                        let capture_start = std::time::Instant::now();
                        let frame = capture.capture_full_screen().expect("Capture failed");
                        black_box(frame);

                        // Try to maintain target interval
                        let elapsed = capture_start.elapsed();
                        if elapsed < target_interval {
                            std::thread::sleep(target_interval - elapsed);
                        }
                    }

                    start.elapsed()
                })
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    benchmark_single_capture,
    benchmark_capture_throughput,
    benchmark_fps_targets
);
criterion_main!(benches);
