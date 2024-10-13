//! Named Pipe Server Manager manages
//! 1. Creates Named Pipe Server(s)
//! 2. Waits for a Client to connect
//! 3. And after a Client connects, returns the
//! Connection Server which can be used to
//! communicate with the client
//! 4. Needed because of the design of the Windows API

use tokio::io;
use tokio::net::windows::named_pipe::{self, NamedPipeServer};

/// Creates and manages client connections for Named Pipe servers
/// # Note
/// This is needed because:-  
/// - Need to use Windows API
/// - Windows API does not create a new Socket  
///   upon Client Connect
/// - Instead the existing Socket becomes a new Client.
/// - This means that if we expect multiple clients,  
///   We need to create multiple server objects
/// - This serves as an abstraction over the same  
///   improving readability and easing maintainability
pub struct NamedPipeServerManager {
    /// Named Pipe to connect to
    name: String,
    /// Builder to construct the Server
    options: named_pipe::ServerOptions,
}

impl NamedPipeServerManager {
    /// Create an object of the server manager
    /// * `name` The name of the Named Pipe this server has to bind to
    pub fn new(name: impl Into<String>) -> Self {
        use tokio::net::windows::named_pipe::PipeMode;
        Self::with_options(
            name.into(),
            named_pipe::ServerOptions::new()
                .pipe_mode(PipeMode::Message)
                .to_owned(),
        )
    }

    /// Create an object of the server manager
    /// * `name` The name of the Named Pipe this server has to bind to
    /// * `options` The options to create the named pipe with
    pub fn with_options(name: impl Into<String>, options: named_pipe::ServerOptions) -> Self {
        NamedPipeServerManager {
            name: name.into(),
            options,
        }
    }

    /// Wait for a client to connect
    /// to the Named Pipe Server.  
    /// After a client connects
    /// returns a Read/Write Stream
    pub async fn accept(self: &mut Self) -> io::Result<NamedPipeServer> {
        // The reason this is needed is due to a peculiarity in the
        // Windows API where
        // After the first client connects to the server
        // Instead of creating and returning a new Object Handle
        // The existing object handle becomes a connection stream
        // To the Connected Client
        // So every time, a new Server object has to be created
        let server = self.options.create(&self.name)?;
        // Wait for a client to connect
        server.connect().await?;
        // Upon client connection, the server object
        // becomes the client
        // This is a peculiarity of the Windows API
        let stream = server;
        return Ok(stream);
    }
}
