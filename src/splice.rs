use tokio::io::{self, AsyncWriteExt};

/// Implements the Readable Trait for Splice
/// # Note
/// Needed because Tokio Net does not have
/// a trait for readable and try_read which means we have
/// to create our own.  
/// This helps us keep our code generic and independent of
/// specific handle abstractions.
/// This would help us integrate additional handles in the future
/// As and when needed
pub trait Readable {
    /// Implement this in a manner such that
    /// It un-blockingly reads into the buf
    /// And returns the number of bytes read
    /// Or Error
    fn try_read(&self, buf: &mut [u8]) -> io::Result<usize>;
    /// Can be used to await till the socket is readable
    async fn readable(&self) -> io::Result<()>;
}

/// Implementation of a Linux Splice-like system call
/// Reads from the `readable` source into the `buf`
/// And forwards the received bytes to `writeable`
/// <div class="warning">
/// Always ensure that this function is called
/// after `readable`.readable() returns.  
/// It indicates that the socket has
/// some data to read
/// </div>
async fn splice_source_to_dest(
    readable: &mut impl Readable,
    writeable: &mut (impl AsyncWriteExt + Unpin),
    buf: &mut [u8],
) -> io::Result<()> {
    // Splice is to be only called
    // After the Readable Socket has notified
    // that there are some bytes available to read
    // So we try and read some bytes
    match readable.try_read(buf) {
        // If there are 0 bytes available to read
        // After readable returned
        // This indicates that the connection has disconnected
        Ok(0) => Err(io::Error::new(
            io::ErrorKind::Other,
            "Read source disconnected",
        )),
        // Received n bytes over the network
        Ok(n) => {
            // Implementation of the Splice system call
            // Forward the received n bytes
            // from the network buffer
            // To the writeable socket
            writeable.write_all(&buf[0..n]).await?;
            Ok(())
        }
        // If the readiness event is a false positive,
        // it would fail with the same
        // In this case we do nothing
        Err(e) if e.kind() == io::ErrorKind::WouldBlock => Ok(()),
        // Received a genuine error
        // Forward
        Err(e) => Err(e),
    }
}

/// Splice data received bi-directionally between left and right
/// # Note
/// - Inspired by the Linux splice system call
/// - If right has data to read, write it to left
/// - If left has data to read, write it to right
/// - Does so till the sockets are open
/// - Choose an appropriate `buf` size.  
///   Providing a buffer size which is too small
///   could potentially reduce performance.
pub async fn splice(
    left: &mut (impl Readable + AsyncWriteExt + Unpin),
    right: &mut (impl Readable + AsyncWriteExt + Unpin),
    buf: &mut [u8],
) -> io::Result<()> {
    loop {
        // Wait for the first of the two to be available for reading
        // If any of the two is readable
        // Read bytes from one and forward to the other
        tokio::select! {
            Ok(_) = left.readable() =>
                splice_source_to_dest(left, right, buf).await?,
            Ok(_) = right.readable() =>
                splice_source_to_dest(right, left, buf).await?
        }
    }
}
