use benchmarks::*;
use criterion::*;
use futures::executor::block_on;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

fn bench_tcp(c: &mut Criterion) {
    let mut g = c.benchmark_group("TCP");

    g.throughput(Throughput::Bytes(1 * GB as u64));
    g.sample_size(10);

    g.bench_function("tokio", |b| {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        let (tx_ready, rx_ready) = tokio::sync::oneshot::channel();

        let task = rt.spawn(async {
            let server = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
            let addr = server.local_addr().unwrap();
            let client = tokio::net::TcpStream::connect(addr).await.unwrap();
            let (mut stream, _) = server.accept().await.unwrap();
            let mut buffer = [0u8; 64 * 1024];

            tx_ready.send(client).unwrap();

            loop {
                let n = stream.read(&mut buffer).await.unwrap();

                if n == 0 {
                    return;
                }

                stream.write_all(&[0; 1 * GB]).await.unwrap();
            }
        });

        let mut client = block_on(rx_ready).unwrap();

        b.iter(|| {
            futures::executor::block_on(async {
                let mut buffer = [0; 8 * 1024];
                client.write_all(&[0x00]).await.unwrap();

                let mut n = 0;
                while n < GB {
                    n += client.read(&mut buffer).await.unwrap();
                }

                assert_eq!(n, 1 * GB);
            })
        });

        task.abort();
    });

    g.bench_function("tokio-200B-header-256KiB-response", |b| {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        let (tx_ready, rx_ready) = tokio::sync::oneshot::channel();

        let task = rt.spawn(async {
            let server = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
            let addr = server.local_addr().unwrap();
            let client = tokio::net::TcpStream::connect(addr).await.unwrap();
            let (mut stream, _) = server.accept().await.unwrap();
            let mut buffer = [0u8; 200];

            tx_ready.send(client).unwrap();

            loop {
                let mut n = 0;
                while n < 200 {
                    let r = stream.read(&mut buffer[n..]).await.unwrap();
                    n += r;

                    if r == 0 {
                        return;
                    }
                }

                stream.write_all(&[0; 256 * KB]).await.unwrap();
            }
        });

        let mut client = block_on(rx_ready).unwrap();

        b.iter(|| {
            futures::executor::block_on(async {
                let mut total = 0;

                for _ in 0..4 * 1024 {
                    client.write_all(&[0x00; 200]).await.unwrap();
                    let mut buffer = [0; 256 * 1024];

                    let mut n = 0;
                    while n < 256 * KB {
                        n += client.read(&mut buffer[n..]).await.unwrap();
                    }

                    total += n;
                }

                assert_eq!(total, 1 * GB);
            })
        });

        task.abort();
    });

    g.throughput(Throughput::Bytes(32 as u64));
    g.bench_function("tokio-32B-header-32B-response", |b| {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        let (tx_ready, rx_ready) = tokio::sync::oneshot::channel();

        let task = rt.spawn(async {
            let server = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
            let addr = server.local_addr().unwrap();
            let client = tokio::net::TcpStream::connect(addr).await.unwrap();
            let (mut stream, _) = server.accept().await.unwrap();
            let mut buffer = [0u8; 32];

            tx_ready.send(client).unwrap();

            loop {
                let mut n = 0;
                while n < 32 {
                    let r = stream.read(&mut buffer[n..]).await.unwrap();
                    n += r;

                    if r == 0 {
                        return;
                    }
                }

                stream.write_all(&[0; 32]).await.unwrap();
            }
        });

        let mut client = block_on(rx_ready).unwrap();

        b.iter(|| {
            futures::executor::block_on(async {
                client.write_all(&[0x00; 32]).await.unwrap();
                let mut buffer = [0; 32];

                let mut n = 0;
                while n < 32 {
                    n += client.read(&mut buffer[n..]).await.unwrap();
                }
            })
        });

        task.abort();
    });
}

criterion_group!(benches, bench_tcp);
criterion_main!(benches);
