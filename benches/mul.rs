use criterion::*;
use isochronous_finite_fields::GF;

fn criterion_benchmark(c: &mut Criterion) {
    let mut mul = c.benchmark_group("mul");

    for a in [0, 64, 128, 196, 255].windows(2) {
        mul.bench_with_input(format!("{a:?}"), a, |b, a| {
            b.iter(|| black_box(GF(a[0])) * black_box(GF(a[1])))
        });
    }

    mul.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
