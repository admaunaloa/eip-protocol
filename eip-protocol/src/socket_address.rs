use crate::eip::{EipResult, Serializing};
use crate::error_code::{NOT_ENOUGH_DATA, REPLY_DATA_TOO_LARGE};
use bytes::{Buf, BufMut, BytesMut};
use core::mem::{size_of, size_of_val};

const ZERO_LEN: usize = 8;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Family(i16);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SocketAddress {
    pub family: Family,
    pub port: u16,
    pub addr: u32,
    zero: [u8; ZERO_LEN],
}

impl SocketAddress {
    pub const AF_INET: Family = Family(2);

    /// Create a new server side instance
    ///
    /// # Arguments
    ///
    /// * `addr` - The IP address
    /// * `port` - The IP port number
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytes::BytesMut;
    /// use eip_protocol::socket_address::SocketAddress;
    /// use eip_protocol::eip::Serializing;
    ///
    /// let mut sa = SocketAddress::server(0x12345678, 44818);
    /// let mut buf = BytesMut::with_capacity(100);
    ///
    /// assert_eq!(
    ///    Ok(()),
    ///    sa.serialize(&mut buf)
    /// );
    ///
    /// assert_eq!(16, buf.len());
    /// assert_eq!(&b"\0\x02\xaf\x12\x12\x34\x56\x78\0\0\0\0\0\0\0\0"[..], buf);
    /// ```
    ///
    /// # Errors
    ///
    /// If there is not enough room an error variant will be returned.
    ///
    pub fn server(addr: u32, port: u16) -> Self {
        SocketAddress {
            family: Self::AF_INET,
            port,
            addr,
            zero: Default::default(),
        }
    }

    const fn serial_size(&self) -> usize {
        size_of::<i16>() // family
        + size_of::<u16>() // port
        + size_of::<u32>() // addr
        + ZERO_LEN // zero
    }
}

impl Serializing for SocketAddress {
    /// Deserialize all fields
    /// Note: is received in big-endian
    ///
    /// # Arguments
    ///
    /// * `buf` - The message buffer to read from
    ///
    /// # Errors
    ///
    /// If there is not enough data an error variant will be returned.
    ///
    fn deserialize(&mut self, buf: &mut dyn Buf) -> EipResult {
        if buf.remaining() < self.serial_size() {
            return Err(NOT_ENOUGH_DATA);
        }
        self.family = Family(buf.get_i16());
        self.port = buf.get_u16();
        self.addr = buf.get_u32();
        for n in 0..size_of_val(&self.zero) {
            self.zero[n] = buf.get_u8();
        }
        Ok(())
    }

    /// Serialize all fields
    /// Note: must be send in big-endian
    ///
    /// # Arguments
    ///
    /// * `buf` - The message buffer to write to
    ///
    /// # Errors
    ///
    /// If there is not enough room an error variant will be returned.
    ///
    fn serialize(&self, buf: &mut BytesMut) -> EipResult {
        if buf.remaining_mut() < self.serial_size() {
            return Err(REPLY_DATA_TOO_LARGE);
        }
        buf.put_i16(self.family.0);
        buf.put_u16(self.port);
        buf.put_u32(self.addr);
        buf.put(&self.zero[..]);
        Ok(())
    }
}

#[test]
fn deserialize() {
    let mut sa = SocketAddress::default();
    let mut buf = &b"\0\x02\xaf\x12\x12\x34\x56\x78\0\0\0\0\0\0\0\0"[..];

    assert_eq!(Ok(()), sa.deserialize(&mut buf));
    assert_eq!(0, buf.len());
    assert_eq!(Family(2), sa.family);
    assert_eq!(44818, sa.port);
    assert_eq!(0x12345678, sa.addr);
}

#[test]
fn deserialize_bounds() {
    let mut sa = SocketAddress::default();
    let mut buf = &b"\0\x02\xaf\x12\x12\x34\x56\x78\0\0\0\0\0\0\0"[..];

    assert_eq!(Err(NOT_ENOUGH_DATA), sa.deserialize(&mut buf));
}

#[test]
fn serialize() {
    let sa = SocketAddress::server(0x12345678, 44818);
    let mut buf = BytesMut::with_capacity(100);

    assert_eq!(Ok(()), sa.serialize(&mut buf));
    assert_eq!(16, buf.len());
    assert_eq!(&b"\0\x02\xaf\x12\x12\x34\x56\x78\0\0\0\0\0\0\0\0"[..], buf);
}

#[test]
fn auto_traits() {
    use crate::eip::check_auto_traits;

    check_auto_traits::<Family>();
    check_auto_traits::<SocketAddress>();
}
