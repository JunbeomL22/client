use crate::UnixNano;
use std::net::UdpSocket;
use std::io::{Error, ErrorKind};
use flashlog::get_unix_nano;

/// A UDP client that sends and receives packets.
/// when receiving packets, it will return the last packet size
/// If there is no packet to read during the idle_timeout, it will return None so that other tasks can be executed.
/// Of course, you can choose not to do any other tasks and just wait for the next packet by re-calling the recv method.
pub struct UdpClient {
    socket: UdpSocket,
    idle_timeout: Option<UnixNano>,
}

impl UdpClient {
    pub fn new(
        address: &str,
        idle_timeout: Option<UnixNano>,
    ) -> Result<Self, Error> {
        let socket = UdpSocket::bind(address)?;
        socket.set_nonblocking(true)?;
        Ok(Self { 
            socket,
            idle_timeout,
         })
    }

    pub fn send(&self, buf: &[u8]) -> Result<usize, Error> {
        self.socket.send(buf)
    }

    /// Set the non-blocking mode of the socket.
    /// handle the last packet received
    /// returning Ok(None) means that the socket is idle
    pub fn recv(&self, buf: &mut [u8]) -> Result<Option<usize>, Error> {
        let mut res_size = None;
        let start_nano = get_unix_nano();
        if let Some(idle_timeout) = self.idle_timeout {
            loop {
                if get_unix_nano() - start_nano >= idle_timeout {
                    flashlog::flash_trace!("UDP";"Idle timeout: {}", idle_timeout);
                    return Ok(None);  // when timeout, return the last packet size
                }
        
                match self.socket.recv(buf) {
                    Ok(size) => {
                        // if the packet is received, update the size
                        res_size = Some(size);
                        continue;  // check if there is another packet
                    }
                    Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                        // there is no packet to read, so return the last packet size
                        return Ok(res_size);
                    }
                    Err(e) => return Err(e),
                }
            }
        } else {
            loop {
                match self.socket.recv(buf) {
                    Ok(size) => {
                        // if the packet is received, update the size
                        res_size = Some(size);
                        continue;  // check if there is another packet
                    }
                    Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                        // there is no packet to read, so return the last packet size
                        return Ok(res_size);
                    }
                    Err(e) => return Err(e),
                }
            }
        }
    }
}

impl From<&str> for UdpClient {
    fn from(address: &str) -> Self {
        Self::new(address, None).expect("Failed to create UDP client")
    }
}