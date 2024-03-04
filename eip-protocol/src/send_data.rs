use crate::eip;
use crate::eip::{EipResult, Serializing};
use crate::error_code::{ErrorCode, NOT_ENOUGH_DATA, REPLY_DATA_TOO_LARGE};
use bytes::{Buf, BufMut, BytesMut};
use core::mem::size_of;

/// This is the SendRRData and SendUnitData implementation

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SendData {
    interface_handle: u32,
    time_out: u16,
    pub item_count: u16,
}

impl SendData {
    const fn serial_size(&self) -> usize {
        size_of::<u32>() // interface_handle
        + size_of::<u16>() // time_out
        + size_of::<u16>() // item_count
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

impl Serializing for SendData {
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
        self.interface_handle = buf.get_u32_le();
        self.time_out = buf.get_u16_le();
        self.item_count = buf.get_u16_le();
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
        if buf.remaining_mut() < self.serial_size() {
            return Err(REPLY_DATA_TOO_LARGE);
        }
        buf.put_u32_le(self.interface_handle);
        buf.put_u16_le(self.time_out);
        buf.put_u16_le(self.item_count);
        Ok(())
    }
}

#[test]
fn deserialize() {
    let mut send_data = SendData::default();
    let mut buf = &b"\0\0\0\0\0\0\x34\x12"[..];
    assert_eq!(Ok(()), send_data.deserialize(&mut buf));
    assert_eq!(0, buf.len());
    assert_eq!(0x1234, send_data.item_count);

    buf = &b"\0\0\0\0\0\0\x34"[..];
    assert_eq!(Err(NOT_ENOUGH_DATA), send_data.deserialize(&mut buf));
}

#[test]
fn serialize() {
    let mut send_data = SendData::default();
    send_data.item_count = 0x1234;
    let mut buf = BytesMut::with_capacity(100);
    assert_eq!(Ok(()), send_data.serialize(&mut buf));
    assert_eq!(8, buf.len());
    assert_eq!(&b"\0\0\0\0\0\0\x34\x12"[..], buf);
}

#[test]
fn auto_traits() {
    use crate::eip::check_auto_traits;

    check_auto_traits::<SendData>();
}
