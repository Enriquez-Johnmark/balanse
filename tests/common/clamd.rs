//! A stand-in for ClamAV's daemon: it speaks clamd's INSTREAM protocol and
//! reports any stream containing `EICAR` as infected.

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

/// What the fake does with each scan.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Behaviour {
    /// Answers like clamd.
    Scans,
    /// Answers with an error, as clamd does when a stream is too long.
    Errors,
    /// Reads the file and never answers.
    Hangs,
}

/// A running fake clamd.
#[derive(Clone)]
pub struct FakeClamd {
    pub addr: SocketAddr,
    /// Every stream it received, reassembled from its chunks.
    pub streams: Arc<Mutex<Vec<Vec<u8>>>>,
}

impl FakeClamd {
    pub async fn start(behaviour: Behaviour) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let fake = Self {
            addr: listener.local_addr().unwrap(),
            streams: Arc::default(),
        };
        let streams = fake.streams.clone();
        tokio::spawn(async move {
            while let Ok((socket, _)) = listener.accept().await {
                let streams = streams.clone();
                tokio::spawn(async move { serve(socket, behaviour, streams).await });
            }
        });
        fake
    }

    pub fn addr(&self) -> String {
        self.addr.to_string()
    }
}

async fn serve(mut socket: TcpStream, behaviour: Behaviour, streams: Arc<Mutex<Vec<Vec<u8>>>>) {
    let mut command = Vec::new();
    while command.last() != Some(&0) {
        command.push(socket.read_u8().await.unwrap());
    }
    if command == b"zPING\0" {
        match behaviour {
            Behaviour::Hangs => std::future::pending::<()>().await,
            _ => socket.write_all(b"PONG\0").await.unwrap(),
        }
        return;
    }
    assert_eq!(command, b"zINSTREAM\0", "unexpected clamd command");
    let mut stream = Vec::new();
    loop {
        let length = socket.read_u32().await.unwrap() as usize;
        if length == 0 {
            break;
        }
        let mut chunk = vec![0; length];
        socket.read_exact(&mut chunk).await.unwrap();
        stream.extend_from_slice(&chunk);
    }
    let infected = stream.windows(5).any(|window| window == b"EICAR");
    streams.lock().unwrap().push(stream);
    let reply: &[u8] = match behaviour {
        Behaviour::Scans if infected => b"stream: Eicar-Test-Signature FOUND\0",
        Behaviour::Scans => b"stream: OK\0",
        Behaviour::Errors => b"INSTREAM size limit exceeded. ERROR\0",
        Behaviour::Hangs => {
            std::future::pending::<()>().await;
            return;
        }
    };
    socket.write_all(reply).await.unwrap();
}

/// An address where nothing listens.
pub async fn nobody_listening() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    listener.local_addr().unwrap().to_string()
}
