use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cwmp::generate;
use cwmp::protocol::{AddObject, BodyElement, CwmpVersion, Envelope, HeaderElement, ID};

fn criterion_benchmark(c: &mut Criterion) {
    let e: Envelope = Envelope::new(
        Some(CwmpVersion::new(1, 0)),
        vec![HeaderElement::ID(ID::new(true, "1234"))],
        vec![BodyElement::AddObject(AddObject::new("foo", "key"))],
    );
    c.bench_function("generate add_object", |b| {
        b.iter(|| generate(black_box(&e)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
