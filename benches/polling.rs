use std::net::{UdpSocket, TcpStream, TcpListener};
use std::io::{ErrorKind, Read};
use criterion::{criterion_group, criterion_main, Criterion};
use std::thread;
use std::time::Duration;

fn setup_tcp_server() {
    thread::spawn(|| {
        let listener = TcpListener::bind("127.0.0.1:5001").unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(_) => {},
                Err(_) => break,
            }
        }
    });
    // 서버가 시작될 시간을 주기 위해 잠시 대기
    thread::sleep(Duration::from_millis(100));
}

fn bench_sockets_check(c: &mut Criterion) {
    // 먼저 TCP 서버 시작
    setup_tcp_server();
    
    let udp_socket = UdpSocket::bind("127.0.0.1:5000").unwrap();
    udp_socket.set_nonblocking(true).unwrap();
    
    // TCP 서버에 연결
    let mut tcp_stream = TcpStream::connect("127.0.0.1:5001").unwrap();
    tcp_stream.set_nonblocking(true).unwrap();
    
    let mut buf = [0; 1024];
    let mut group = c.benchmark_group("socket_checks");
    
    // UDP 소켓 체크 시간 측정
    group.bench_function("udp_check", |b| {
        b.iter(|| {
            match udp_socket.recv(&mut buf) {
                Ok(_) => {},
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {},
                Err(e) => panic!("Unexpected error: {}", e),
            }
        })
    });

    // TCP 스트림 체크 시간 측정
    group.bench_function("tcp_check", |b| {
        b.iter(|| {
            match tcp_stream.read(&mut buf) {
                Ok(_) => {},
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {},
                Err(e) => panic!("Unexpected error: {}", e),
            }
        })
    });

    // UDP와 TCP 연속 체크 시간 측정
    group.bench_function("both_sockets_check", |b| {
        b.iter(|| {
            match udp_socket.recv(&mut buf) {
                Ok(_) => {},
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {},
                Err(e) => panic!("Unexpected error: {}", e),
            }
            match tcp_stream.read(&mut buf) {
                Ok(_) => {},
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {},
                Err(e) => panic!("Unexpected error: {}", e),
            }
        })
    });

    group.finish();
}

criterion_group!(benches, bench_sockets_check);
criterion_main!(benches);