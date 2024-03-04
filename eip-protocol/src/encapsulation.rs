use crate::eip;
use crate::eip::{EipResult, Serializing};
use crate::error_code::{ErrorCode, NOT_ENOUGH_DATA, REPLY_DATA_TOO_LARGE};
use bytes::{Buf, BufMut, BytesMut};
use core::mem::size_of;
const CONTEXT_LEN: usize = 8;
pub const VERSION: u16 = 1;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Command(u16);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Encapsulation {
    pub command: Command,           // Request
    pub len: u16,                   // Number of bytes
    pub session: u32,               // Session identifier
    pub status: u32,                // Status code
    pub context: [u8; CONTEXT_LEN], // Sender context
    pub options: u32,               // Options flags
}

impl Encapsulation {
    pub const NOP: Command = Command(0x00);
    pub const LIST_SERVICES: Command = Command(0x04);
    pub const LIST_IDENTITY: Command = Command(0x63);
    pub const REGISTER_SESSION: Command = Command(0x65);
    pub const UNREGISTER_SESSION: Command = Command(0x66);
    pub const SEND_RR_DATA: Command = Command(0x6f);

    /// Get the serialized size in Bytes.
    ///
    /// # Returns
    ///
    /// * The number of bytes when serialized
    ///
    pub const fn serial_size(&self) -> usize {
        size_of::<u16>() // command
        + size_of::<u16>() // len
        + size_of::<u32>() // session
        + size_of::<u32>() // status
        + CONTEXT_LEN // context
        + size_of::<u32>() // options
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

impl Serializing for Encapsulation {
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
        self.command = Command(buf.get_u16_le());
        self.len = buf.get_u16_le();
        self.session = buf.get_u32_le();
        self.status = buf.get_u32_le();
        for n in 0..CONTEXT_LEN {
            self.context[n] = buf.get_u8();
        }
        self.options = buf.get_u32_le();
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
    /// An error variant will be returned if there is not enough room.
    ///
    fn serialize(&self, buf: &mut BytesMut) -> EipResult {
        if buf.remaining_mut() < self.serial_size() {
            return Err(REPLY_DATA_TOO_LARGE);
        }
        buf.put_u16_le(self.command.0);
        buf.put_u16_le(self.len);
        buf.put_u32_le(self.session);
        buf.put_u32_le(self.status);
        buf.put(&self.context[..]);
        buf.put_u32_le(self.options);
        Ok(())
    }
}

#[test]
fn deserialize() {
    let mut enc = Encapsulation::default();
    let mut buf =
        &b"\x01\0\x02\0\x03\0\0\0\x04\0\0\0\x01\x02\x03\x04\x05\x06\x07\x08\x05\0\0\0"[..];

    assert_eq!(Ok(()), enc.deserialize(&mut buf));
    assert_eq!(0, buf.len());
    assert_eq!(Command(1), enc.command);
    assert_eq!(2, enc.len);
    assert_eq!(3, enc.session);
    assert_eq!(4, enc.status);
    assert_eq!(&[1, 2, 3, 4, 5, 6, 7, 8], &enc.context);
    assert_eq!(5, enc.options);

    buf = &b"\x01\0\x02\0\x03\0\0\0\x04\0\0\0\x01\x02\x03\x04\x05\x06\x07\x08\x05\0\0"[..];
    assert_eq!(Err(NOT_ENOUGH_DATA), enc.deserialize(&mut buf));
}

#[test]
fn serialize() {
    let mut enc = Encapsulation::default();
    enc.command = Command(1);
    enc.len = 2;
    enc.session = 3;
    enc.status = 4;
    enc.context = [1, 2, 3, 4, 5, 6, 7, 8];
    enc.options = 5;
    let mut buf = BytesMut::with_capacity(100);
    assert_eq!(Ok(()), enc.serialize(&mut buf));
    assert_eq!(24, buf.len());
    assert_eq!(
        &b"\x01\0\x02\0\x03\0\0\0\x04\0\0\0\x01\x02\x03\x04\x05\x06\x07\x08\x05\0\0\0"[..],
        buf
    );
}

#[test]
fn auto_traits() {
    use crate::eip::check_auto_traits;

    check_auto_traits::<Command>();
    check_auto_traits::<Encapsulation>();
}
