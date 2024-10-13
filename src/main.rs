mod named_pipe_server;
mod splice;

use clap::Parser;
use splice::splice;
use tokio::{
    io,
    net::{windows::named_pipe::NamedPipeServer, TcpStream},
};

/// Implement the `TryRead` trait for our structures  
/// Remember that this is just needed because Tokio
/// does not provide traits for the below functions
impl splice::Readable for NamedPipeServer {
    fn try_read(&self, buf: &mut [u8]) -> io::Result<usize> {
        Self::try_read(self, buf)
    }
    async fn readable(&self) -> io::Result<()> {
        Self::readable(self).await
    }
}

/// Implement the `TryRead` trait for our structures  
/// Remember that this is just needed because Tokio
/// does not provide traits for the below functions
impl splice::Readable for TcpStream {
    fn try_read(&self, buf: &mut [u8]) -> io::Result<usize> {
        return Self::try_read(self, buf);
    }
    async fn readable(&self) -> io::Result<()> {
        Self::readable(self).await
    }
}

/// Named Pipe to TCP Server Proxy.  
/// The primary use case is for example a Docker Engine Socket
/// Which on Windows might be listening to the Windows File Socket  
/// But behind the scenes, could actually be proxied to an indepedent
/// TCP client without setting DOCKER_HOST on terminals etc.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the Windows Named Pipe to listen on
    #[arg(long, required = true)]
    pipe: String,

    /// Name of the TCP Server to connect to
    #[arg(long, required = true)]
    tcp: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = Args::parse();
    use named_pipe_server::NamedPipeServerManager;
    let mut server = NamedPipeServerManager::new(args.pipe);
    loop {
        // A client has connected to the server
        let mut pipe_stream = server.accept().await.unwrap();
        // Each Pipe Stream should have its own TCP Connection
        // This makes it easier to logically read and write code
        let mut tcp_stream = TcpStream::connect(&args.tcp).await.unwrap();
        // Spawn a new Future to achieve concurrency
        tokio::spawn(async move {
            // Each connection should have its own Buffer
            let mut buf = vec![0; 1024 * 8];
            // Note that even if this returns an error we do not really care
            // The only reason the let exists here is so we do not get warnings
            let _ = splice(&mut pipe_stream, &mut tcp_stream, &mut buf).await;
        });
    }
}
