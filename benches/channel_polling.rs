use criterion::{black_box, criterion_group, criterion_main, Criterion};
use crossbeam_channel::{bounded, unbounded};

fn benchmark_channel_check(c: &mut Criterion) {
    // 채널 생성 함수와 이름을 튜플로 저장
    let channels = vec![
        ("bounded(1)", bounded(1)),
        ("unbounded", unbounded()),
    ];

    for (name, (sender, receiver)) in channels {
        let mut group = c.benchmark_group(format!("channel_{}", name));
        
        group.bench_function("try_recv_empty", |b| {
            b.iter(|| {
                // 비어있는 채널에서 try_recv 시도
                black_box(receiver.try_recv().is_err());
            });
        });

        group.bench_function("try_recv_with_data", |b| {
            b.iter_with_setup(
                || {
                    // 매 반복마다 데이터 전송
                    sender.send(1).unwrap();
                },
                |_| {
                    // 데이터가 있는 채널에서 try_recv 시도
                    black_box(receiver.try_recv().is_ok());
                }
            );
        });

        group.bench_function("is_empty", |b| {
            b.iter(|| {
                // is_empty 체크
                black_box(receiver.is_empty());
            });
        });

        group.bench_function("is_full", |b| {
            b.iter(|| {
                // is_full 체크
                black_box(sender.is_full());
            });
        });

        group.finish();
    }
}

criterion_group!(benches, benchmark_channel_check);
criterion_main!(benches);