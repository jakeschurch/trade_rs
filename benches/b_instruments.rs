#[macro_use]
extern crate criterion;
extern crate chrono;
extern crate trade_rs;

use criterion::Criterion;

fn bench_currency_from(c: &mut Criterion) {
    use trade_rs::instruments::Currency;

    c.bench_function("Currency::from", |b| b.iter(|| Currency::from(10.00)));
}

fn bench_quote_new(c: &mut Criterion) {
    use chrono::Utc;
    use trade_rs::instruments::{Order, Side};

    c.bench_function("Quote::new", |b| {
        b.iter(|| Order::new("aapl", 10.00, 10, Side::Buy, Utc::now()))
    });
}

criterion_group!(benches, bench_currency_from, bench_quote_new);
criterion_main!(benches);
