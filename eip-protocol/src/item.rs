use crate::eip;
use crate::eip::{EipResult, Serializing};
use crate::error_code::{ErrorCode, NOT_ENOUGH_DATA, REPLY_DATA_TOO_LARGE};
use bytes::{Buf, BufMut, BytesMut};
use core::mem::size_of;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Id(u16);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Item {
    pub type_id: Id,
    pub len: usize,
}

impl Item {
    pub const NULL_ADDRESS: Id = Id(0x0000);
    pub const IDENTITY: Id = Id(0x000c);
    pub const CONNECTED_ADDRESS: Id = Id(0x00a1);
    pub const CONNECTED_DATA: Id = Id(0x00b1);
    pub const UNCONNECTED_DATA: Id = Id(0x00b2);
    pub const SERVICES: Id = Id(0x0100);
    pub const SOCKET_OT: Id = Id(0x8000);
    pub const SOCKET_TO: Id = Id(0x8001);
    pub const SEQUENCED_ADDRESS: Id = Id(0x8002);

    /// Create an instance.
    /// Typically used for server side. Default is typically for the client side.
    ///
    /// # Arguments
    ///
    /// * `type_id` - The type identification
    /// * `len` - The number of bytes
    ///
    /// # Returns
    ///
    /// * The created instance
    ///
    pub fn new(type_id: Id, len: usize) -> Self {
        Item { type_id, len }
    }

    /// Get the serialized size in Bytes.
    ///
    /// # Returns
    ///
    /// * The number of bytes when serialized
    ///
    pub const fn serial_size(&self) -> usize {
        size_of::<u16>() // type_id
        + size_of::<u16>() // len
    }

    /// Reserve room in a buffer to serialize this later.
    ///
    /// # Returns
    ///
    /// * The rest of the buffer after the reservation
    ///
    /// # Arguments
    ///
    /// * `buf` - The message buffer to write to
    ///
    /// # Errors
    ///
    /// An error variant will be returned if there is not enough room for the reservation.
    ///
    pub fn split_off(&self, buf: &mut BytesMut) -> Result<BytesMut, ErrorCode> {
        eip::split_off(buf, self.serial_size())
    }
}

impl Serializing for Item {
    /// Deserialize all fields
    ///
    /// # Arguments
    ///
    /// * `buf` - The message buffer to read from
    ///
    /// # Errors
    ///
    /// An error variant will be returned if there is not enough data.
    ///
    fn deserialize(&mut self, buf: &mut dyn Buf) -> EipResult {
        if buf.remaining() < self.serial_size() {
            return Err(NOT_ENOUGH_DATA);
        }
        self.type_id.0 = buf.get_u16_le();
        self.len = buf.get_u16_le() as usize;
        Ok(())
    }

    /// Serialize all fields
    ///
    /// # Arguments
    ///
    /// * `buf` - The message buffer to write to
    ///
    /// # Errors
    ///
    /// An error variant will be returned if there is not enough room or invalid length.
    ///
    fn serialize(&self, buf: &mut BytesMut) -> EipResult {
        if (buf.remaining_mut() < self.serial_size()) || (self.len > u16::MAX as usize) {
            return Err(REPLY_DATA_TOO_LARGE);
        }
        buf.put_u16_le(self.type_id.0);
        buf.put_u16_le(self.len as u16);
        Ok(())
    }
}

#[test]
fn deserialize() {
    let mut item = Item::default();
    let mut buf = &b"\0\0\x02\0"[..];
    assert_eq!(Ok(()), item.deserialize(&mut buf));
    assert_eq!(0, buf.len());
    assert_eq!(2, item.len);

    buf = &b"\0\0\x02"[..];
    assert_eq!(Err(NOT_ENOUGH_DATA), item.deserialize(&mut buf));
}

#[test]
fn serialize() {
    let mut item = Item::new(Item::SEQUENCED_ADDRESS, 0);
    item.len = 3;
    let mut buf = BytesMut::with_capacity(100);
    assert_eq!(Ok(()), item.serialize(&mut buf));
    assert_eq!(4, buf.len());
    assert_eq!(&b"\x02\x80\x03\0"[..], buf);
}

#[test]
fn auto_traits() {
    use crate::eip::check_auto_traits;

    check_auto_traits::<Id>();
    check_auto_traits::<Item>();
}
