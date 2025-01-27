use criterion::{black_box, criterion_group, criterion_main, Criterion};
use client::data::snapshot::QuoteSnapshot;
use client::order::{
    core::{OrderCore, LimitOrder},
    request::OrderRequest,
    enums::{OrderStatus, OrderSide},
};
use client::InstId;
use crossbeam_channel::{bounded, unbounded};
use std::time::Duration;

fn benchmark_quote_snapshot(c: &mut Criterion) {
    let mut group = c.benchmark_group("quote_snapshot_channel");
    
    // 다양한 레벨 크기로 테스트
    let levels = vec![4];
    
    for level in levels {
        let sample = QuoteSnapshot::sample(level);
        // Clone only (비교를 위한 기준점)
        group.bench_function(format!("clone_only_level_{}", level), |b| {
            let sample = sample.clone();
            
            b.iter(|| {
                black_box(sample.clone());
            });
        });

        // Clone and send
        group.bench_function(format!("clone_and_send_level_{}", level), |b| {
            let (sender, receiver) = bounded(1);
            let sample = sample.clone();
            
            b.iter_with_setup(
                || {
                    // 채널 비우기
                    while receiver.try_recv().is_ok() {}
                    sender.try_send(sample.clone()).unwrap();
                },
                |_| {
                    let _received = black_box(receiver.recv().unwrap());
                }
            );
        });
        
        // Bounded channel recv
        group.bench_function(format!("bounded_channel_recv_level_{}", level), |b| {
            let (sender, receiver) = bounded(1);
            let sample = sample.clone();
            
            b.iter_with_setup(|| {
                // empty the channel
                while receiver.try_recv().is_ok() {}
                sender.send(sample.clone()).unwrap();
            }, |_| {
                let _received = black_box(receiver.recv().unwrap());
            });
        });
        
        // Unbounded channel recv
        group.bench_function(format!("unbounded_channel_recv_level_{}", level), |b| {
            let (sender, receiver) = unbounded();
            let sample = sample.clone();
            
            b.iter_with_setup(|| {
                while receiver.try_recv().is_ok() {}
                sender.send(sample.clone()).unwrap();
            }, |_| {
                let _received = black_box(receiver.recv().unwrap());
            });
        });
        
        // Try recv
        group.bench_function(format!("try_recv_level_{}", level), |b| {
            let (sender, receiver) = bounded(1);
            let sample = sample.clone();
            
            b.iter_with_setup(
                || {
                    sender.send(sample.clone()).unwrap();
                },
                |_| {
                    let _received = black_box(receiver.try_recv().unwrap());
                }
            );
        });
    }

    group.finish();
}

fn create_sample_order_request() -> OrderRequest {
    let limit_order = LimitOrder {
        price: 1000,
        quantity: 100,
        order_side: OrderSide::Bid,
        order_id: 12345,
    };

    OrderRequest {
        order_core: OrderCore::LimitOrder(limit_order),
        instid: InstId::default(),
        systemtime: 1234567890,
        status: OrderStatus::PendingNew,
        filled: None,
    }
}

fn benchmark_order_request(c: &mut Criterion) {
    let mut group = c.benchmark_group("order_request_channel");
    
    let sample = create_sample_order_request();
    // Clone only (비교를 위한 기준점)
    group.bench_function("clone_only", |b| {
        let sample = sample.clone();
        
        b.iter(|| {
            black_box(sample.clone());
        });
    });

    // Clone and send
    group.bench_function("clone_and_send", |b| {
        let (sender, receiver) = bounded(1);
        let sample = sample.clone();
        
        b.iter_with_setup(
            || {
                while receiver.try_recv().is_ok() {}
                sender.try_send(sample.clone()).unwrap();
            },
            |_| {
                let _received = black_box(receiver.recv().unwrap());
            }
        );
    });

    // Bounded channel recv
    group.bench_function("bounded_channel_recv", |b| {
        let (sender, receiver) = bounded(1);
        let sample = sample.clone();
        
        b.iter_with_setup(|| {
            while receiver.try_recv().is_ok() {}
            sender.send(sample.clone()).unwrap();
        }, |_| {
            let received: OrderRequest = black_box(receiver.recv().unwrap());
            // 받은 주문이 LimitOrder인지 확인
            if let OrderCore::LimitOrder(_) = received.order_core {
                black_box(());
            }
        });
    });
    
    // Unbounded channel recv
    group.bench_function("unbounded_channel_recv", |b| {
        let (sender, receiver) = unbounded();
        let sample = sample.clone();
        
        b.iter_with_setup(|| {
            while receiver.try_recv().is_ok() {}
            sender.send(sample.clone()).unwrap();
        }, |_| {
            let received: OrderRequest = black_box(receiver.recv().unwrap());
            // 받은 주문이 LimitOrder인지 확인
            if let OrderCore::LimitOrder(_) = received.order_core {
                black_box(());
            }
        });
    });
    
    // Try recv
    group.bench_function("try_recv", |b| {
        let (sender, receiver) = bounded(1);
        let sample = sample.clone();
        
        b.iter_with_setup(
            || {
                sender.send(sample.clone()).unwrap();
            },
            |_| {
                let received: OrderRequest = black_box(receiver.recv().unwrap());
                // 받은 주문이 LimitOrder인지 확인
                if let OrderCore::LimitOrder(_) = received.order_core {
                    black_box(());
                }
            }
        );
    });

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(10))
        .sample_size(100);
    targets = 
    benchmark_order_request,
    benchmark_quote_snapshot
}
criterion_main!(benches);