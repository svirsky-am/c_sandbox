
use futures::future::join_all;
use std::time::Duration;
use tokio::time::sleep;

// напишите здесь асинхронную функцию handle_connections

// async fn handle_connections(connections: Vec<impl std::future::Future<Output = ()>>) {
//     join_all(connections).await;
// }


async fn handle_connections<I, F>(connections: I)
where
    <F as Future>::Output: Send,
    I: IntoIterator<Item = F>,
    F: Future + Send + 'static,
{
    let mut handles = Vec::new();
    for connection in connections.into_iter() {
        let handle = tokio::spawn(connection);
        handles.push(handle);
    }
    join_all(handles).await;
} 

// Тесты
#[tokio::main]
async fn main() {
    use std::time::Instant;

    let connections = {
        let mut connections = Vec::new();
        for i in 0..10 {
            let connection = async move {
                sleep(Duration::from_millis(100)).await;
                println!("Hello from connection {i}");
            };
            connections.push(connection);
        }
        connections
    };
    let start = Instant::now();
    handle_connections(connections).await;
    let end = start.elapsed();

    assert!(end < Duration::from_millis(500))
} 