use crate::UnixNano;
use std::net::{TcpStream, ToSocketAddrs};
use std::io::{Error, ErrorKind, Read, Write};
use flashlog::get_unix_nano;

/// A TCP client that sends and receives data.
/// When receiving data, it will return the last read size.
/// If there is no data to read during the idle_timeout, it will return None so that other tasks can be executed.
/// Of course, you can choose not to do any other tasks and just wait for the next data by re-calling the recv method.
pub struct TcpClient {
    stream: TcpStream,
    idle_timeout: Option<UnixNano>,
}

impl TcpClient {
    pub fn new<A: ToSocketAddrs>(
        address: A,
        idle_timeout: Option<UnixNano>,
    ) -> Result<Self, Error> {
        let stream = TcpStream::connect(address)?;
        stream.set_nonblocking(true)?;
        Ok(Self {
            stream,
            idle_timeout,
        })
    }

    pub fn send(&mut self, buf: &[u8]) -> Result<usize, Error> {
        self.stream.write(buf)
    }

    /// Set the non-blocking mode of the stream.
    /// Handle the last data received
    /// returning Ok(None) means that the stream is idle
    pub fn recv(&mut self, buf: &mut [u8]) -> Result<Option<usize>, Error> {
        let start_nano = get_unix_nano();

        if let Some(idle_timeout) = self.idle_timeout {
            loop {
                if get_unix_nano() - start_nano >= idle_timeout {
                    flashlog::flash_trace!("TCP";"Idle timeout: {}", idle_timeout);
                    return Ok(None);  // when timeout, return None
                }

                match self.stream.read(buf) {
                    Ok(size) => {
                        if size == 0 {
                            // Connection was closed by peer
                            return Err(Error::new(ErrorKind::ConnectionReset, "Connection closed by peer"));
                        }
                        // if data is received, return the size immediately
                        return Ok(Some(size));  // TCP is stream-based, so we return immediately
                    }
                    Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                        // there is no data to read, continue waiting if within timeout
                        continue;
                    }
                    Err(e) => return Err(e),
                }
            }
        } else {
            loop {
                match self.stream.read(buf) {
                    Ok(size) => {
                        if size == 0 {
                            // Connection was closed by peer
                            return Err(Error::new(ErrorKind::ConnectionReset, "Connection closed by peer"));
                        }
                        // if data is received, return the size immediately
                        return Ok(Some(size));  // TCP is stream-based, so we return immediately
                    }
                    Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                        // there is no data to read, continue waiting
                        continue;
                    }
                    Err(e) => return Err(e),
                }
            }
        }
    }
}

impl From<&str> for TcpClient {
    fn from(address: &str) -> Self {
        Self::new(address, None).expect("Failed to create TCP client")
    }
}