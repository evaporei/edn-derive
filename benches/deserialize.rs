use criterion::{criterion_group, criterion_main};

criterion_group!(benches, edn::criterion_benchmark);
criterion_main!(benches);

mod edn {
    use criterion::Criterion;
    use edn_derive::{Deserialize, Serialize};
    use edn_rs;

    pub fn criterion_benchmark(c: &mut Criterion) {
        c.bench_function("to_str", |b| b.iter(|| edn_rs::to_string(val())));
        let val_str = edn_rs::to_string(val());
        c.bench_function("from_str", |b| {
            b.iter(|| edn_rs::from_str::<ValEdn>(&val_str))
        });
    }

    fn val() -> ValEdn {
        ValEdn {
            tuples: 'd',
            foo_vec: vec![Foo { value: 2 }, Foo { value: 3 }],
            this_is__a___crazy_string: "Crazyness".to_string(),
            bar: Baz::BarBaz,
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct ValEdn {
        tuples: char,
        foo_vec: Vec<Foo>,
        this_is__a___crazy_string: String,
        bar: Baz,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct Foo {
        value: usize,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    enum Baz {
        FooBar,
        BarBaz,
        FooBaz,
    }
}
