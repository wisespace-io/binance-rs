use criterion::{black_box, criterion_group, criterion_main, Criterion};

use binance::api::*;
use binance::userstream::*;
use binance::websockets::*;
use std::sync::atomic::{AtomicBool, Ordering};
use core::time::Duration;

fn handle_msg(web_socket: &mut WebSockets<'_>, body: &str) {
    web_socket.test_handle_msg(&body);
}

fn criterion_benchmark(c: &mut Criterion) {

    let mut group = c.benchmark_group("websockets-decoder");

    let all_symbols_json = reqwest::blocking::get("https://api.binance.com/api/v3/ticker/price")
        .unwrap().text().unwrap();

    let btc_symbol_json = reqwest::blocking::get("https://api.binance.com/api/v3/ticker/price?symbol=BTCUSDT")
        .unwrap().text().unwrap();

    let mut web_socket_subscribed: WebSockets<'_> = WebSockets::new_with_subscription("!ticker@arr", |event: WebsocketEvent| {
        Ok(())
    });

    let mut web_socket: WebSockets<'_> = WebSockets::new(|event: WebsocketEvent| {
        Ok(())
    });

    group.sample_size(200);
    group.measurement_time(Duration::new(35, 0));
    group.bench_function("handle_msg all symbols", |b| b.iter(|| handle_msg(&mut web_socket_subscribed, &all_symbols_json)));
    group.bench_function("handle_msg BTCUSDT symbol", |b| b.iter(|| handle_msg(&mut web_socket, &btc_symbol_json)));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);