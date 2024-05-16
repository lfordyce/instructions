use futures::io::{ReadHalf, WriteHalf};
use futures::StreamExt;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
// use tokio::net::udp::{RecvHalf, SendHalf};
use tokio::net::UdpSocket;
use tokio::sync::mpsc::{self, Receiver, Sender};

type PacketBuffer = Vec<u8>;

#[derive(Debug)]
pub struct UDPConnection {
    max_packet_size: usize,
    packets: Receiver<PacketBuffer>,
    remote_addr: SocketAddr,
    send_socket: Arc<UdpSocket>,
    // send_socket: WriteHalf<UdpSocket>,
}

struct UDPRecvTask {
    max_packet_size: usize,
    recv_socket: Arc<UdpSocket>,
    // recv_socket: ReadHalf<UdpSocket>,
    chan: Sender<PacketBuffer>,
}

impl UDPConnection {
    pub async fn new(remote_addr: SocketAddr, max_packet_size: usize) -> Self {
        // let local_addr: SocketAddr = "0.0.0.0:0".parse().unwrap();
        // let socket = UdpSocket::bind(local_addr).await.unwrap();
        // socket.connect(&remote_addr).await.unwrap();
        // let (recv_socket, send_socket) = futures::StreamExt::split(socket);

        let sock = UdpSocket::bind("0.0.0.0:8080".parse::<SocketAddr>().unwrap())
            .await
            .unwrap();
        let recv_socket = Arc::new(sock);
        let send_socket = recv_socket.clone();

        // let (recv_socket, send_socket) = socket.split();

        let (send_chan, recv_chan) = mpsc::channel(16);

        let connection = UDPConnection {
            max_packet_size,
            packets: recv_chan,
            remote_addr,
            send_socket,
        };

        let task = UDPRecvTask {
            max_packet_size,
            recv_socket,
            chan: send_chan,
        };
        tokio::spawn(async move {
            task.receive_loop().await;
        });
        connection
    }

    pub async fn send(&mut self, packet: PacketBuffer) -> Result<(), Box<dyn Error>> {
        //debug!("OUT : {:?}", packet);
        self.send_socket.send(packet.as_slice()).await?;
        Ok(())
    }

    pub async fn pop_packet(&mut self) -> Option<PacketBuffer> {
        self.packets.recv().await
    }
}

impl UDPRecvTask {
    async fn receive_loop(mut self) {
        //info!("UDPConnection init receive loop");
        loop {
            let result = self.receive().await;
            if let Ok(buffer) = result {
                if !buffer.is_empty() {
                    //debug!(" IN : {:?}", packet_buffer);
                    if self.chan.send(buffer).await.is_err() {
                        // receiver is dropped
                        //info!("UDPConnection exit receive loop");
                        return;
                    }
                }
            }
        }
    }

    async fn receive(&mut self) -> std::io::Result<Vec<u8>> {
        let mut buffer = vec![0u8; self.max_packet_size];
        let size = self.recv_socket.recv(buffer.as_mut_slice()).await?;
        buffer.truncate(size);
        Ok(buffer)
    }
}
