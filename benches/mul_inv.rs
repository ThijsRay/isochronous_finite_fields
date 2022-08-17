use criterion::*;
use isochronous_finite_fields::GF;

fn criterion_benchmark(c: &mut Criterion) {
    let mut mul_inv = c.benchmark_group("multiplicative inverse");

    for a in [0, 64, 128, 196, 255] {
        mul_inv.bench_with_input(format!("{a:?}"), &a, |b, a| {
            b.iter(|| black_box(GF(*a)).multiplicative_inverse())
        });
    }

    mul_inv.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
