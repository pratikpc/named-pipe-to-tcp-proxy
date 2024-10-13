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
    println!("Hello, world!");
}
