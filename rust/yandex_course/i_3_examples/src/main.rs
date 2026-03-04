use tokio::time::sleep;
use std::time::Duration;

use futures::future::join_all;

use tokio::net::UdpSocket;

use std::io;

async fn task1() {
    sleep(Duration::from_millis(1000)).await;
    println!("task1");
}

async fn task2() {
    sleep(Duration::from_millis(500)).await;
    println!("task2");
} 

// #[tokio::main]
#[tokio::main(flavor = "current_thread")]
async fn main()  -> io::Result<()> {
    println!("Starting...");
    let res1 = task1().await;
    let res2 = task2().await;


    // Макрос join создаёт фьючерсы и ожидает окончания выполнения их всех, переданных ему, а потом возвращает результат их всех:
    let (res1, res2) = tokio::join!(
        task1(),
        task2(),
    );


    // join_all
    async fn foo(i: u32) -> u32 { i }

    let futures = vec![foo(1), foo(2), foo(3)];

    assert_eq!(join_all(futures).await, [1, 2, 3]); 

    // select exanmple. Отменяется , елси выполнен хотя бы один 
    let sock = UdpSocket::bind("0.0.0.0:8080").await?;
    let mut buf = [0; 1024];
    loop {
        tokio::select! {
            data = sock.recv_from(&mut buf) => {
                let (len, addr) = data?;
                println!("{:?} bytes received from {:?}", len, addr);
            }
            _ = sleep(Duration::from_secs(10)) => {}
        }
    }
} 