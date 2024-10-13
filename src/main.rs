mod named_pipe_server;
mod splice;

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

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("Hello, world!");
}
