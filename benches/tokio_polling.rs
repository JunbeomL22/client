use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio_tungstenite::{connect_async, accept_async};
use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;
use futures_util::{StreamExt, FutureExt};

async fn setup_tcp_server() {
    tokio::spawn(async move {
        let listener = TcpListener::bind("127.0.0.1:5001").await.unwrap();
        loop {
            match listener.accept().await {
                Ok(_) => {},
                Err(_) => break,
            }
        }
    });
    tokio::time::sleep(Duration::from_millis(100)).await;
}

async fn setup_ws_server() {
    tokio::spawn(async move {
        let listener = TcpListener::bind("127.0.0.1:5002").await.unwrap();
        while let Ok((stream, _)) = listener.accept().await {
            tokio::spawn(async move {
                let ws_stream = accept_async(stream).await.unwrap();
                let (_write, _read) = ws_stream.split();
            });
        }
    });
    tokio::time::sleep(Duration::from_millis(100)).await;
}

fn bench_sockets_check(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    
    // 소켓과 스트림을 미리 설정
    let (udp_socket, tcp_stream, mut ws_stream) = runtime.block_on(async {
        setup_tcp_server().await;
        setup_ws_server().await;
        
        let socket = UdpSocket::bind("127.0.0.1:5000").await.unwrap();
        let stream = TcpStream::connect("127.0.0.1:5001").await.unwrap();
        let (ws_stream, _) = connect_async("ws://127.0.0.1:5002").await.unwrap();
        let (_, ws_read) = ws_stream.split();
        
        (socket, stream, ws_read)
    });
    
    let mut group = c.benchmark_group("async_socket_checks");
    
    // UDP 소켓 체크
    group.bench_function("async_udp_check", |b| {
        let mut buf = [0; 1024];
        b.iter(|| {
            match udp_socket.try_recv(&mut buf) {
                Ok(_) => {},
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {},
                Err(e) => panic!("Unexpected error: {}", e),
            }
        });
    });

    // TCP 스트림 체크
    group.bench_function("async_tcp_check", |b| {
        let mut buf = [0; 1024];
        b.iter(|| {
            match tcp_stream.try_read(&mut buf) {
                Ok(_) => {},
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {},
                Err(e) => panic!("Unexpected error: {}", e),
            }
        });
    });

    // WebSocket 체크
    group.bench_function("async_ws_check", |b| {
        b.iter(|| {
            runtime.block_on(async {
                match ws_stream.next().now_or_never() {
                    Some(_) => {},
                    None => {},
                }
            })
        });
    });

    // 모든 소켓 연속 체크
    group.bench_function("async_all_sockets_check", |b| {
        let mut buf = [0; 1024];
        b.iter(|| {
            match udp_socket.try_recv(&mut buf) {
                Ok(_) => {},
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {},
                Err(e) => panic!("Unexpected error: {}", e),
            }
            
            match tcp_stream.try_read(&mut buf) {
                Ok(_) => {},
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {},
                Err(e) => panic!("Unexpected error: {}", e),
            }

            runtime.block_on(async {
                match ws_stream.next().now_or_never() {
                    Some(_) => {},
                    None => {},
                }
            })
        });
    });

    group.finish();
}

criterion_group!(benches, bench_sockets_check);
criterion_main!(benches);