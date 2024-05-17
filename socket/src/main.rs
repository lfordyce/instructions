use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
const ADDR: &str = "localhost:8000";

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Create listener instance that bounds to certain address
    let listener = TcpListener::bind(ADDR).await?;
    // Create communication-channel between clients which connect each with server
    let (tx, _) = tokio::sync::broadcast::channel(10);
    loop {
        match listener.accept().await {
            Ok((mut socket, addr)) => {
                // Cloning (tx, rx) for each client
                // rx should be mutable
                let tx = tx.clone();
                let mut rx = tx.subscribe();
                // Handling task for each client on different thread
                // Ownership of the environment should be moved to the closure. Ex) tx, rx, socket, addr etc..
                tokio::spawn(async move {
                    let (r, mut w) = socket.split();
                    let mut r = BufReader::new(r);
                    let mut text = String::new();
                    // Reading line from client & receiving message should run concurrently
                    // On same thread, 'r.read_line' & 'rx.recv()' are running concurrently. After one task is done, remaining task will run
                    // For example, if 'Client 1' send message 'r.read_line() will call first.
                    // While 'rx.recv()' will be called for other 'Client' and vise versa.
                    tokio::select! {
                        result = r.read_line(&mut text) => {
                            match result {
                                Ok(_) => {
                                    tx.send((text.clone(), addr )).unwrap();
                                },
                                Err(e) => {
                                    println!("couldn't read message from client: {:?}", e)
                                }
                            }
                        }
                        result = rx.recv() => {
                            let (msg, recv_addr) = result.unwrap();
                            // For preventing echoing from 'server'
                            if recv_addr != addr {
                                match w.write_all(msg.as_bytes()).await {
                                    Ok(()) => (),
                                    Err(e) => {
                                        println!("couldn't write message to client: {:?}", e)
                                    }
                                }
                            }
                        }
                    }
                });
            }
            Err(e) => {
                println!("couldn't get client: {:?}", e)
            }
        }
    }
}
