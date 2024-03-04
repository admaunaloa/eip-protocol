use crate::error_code::{ErrorCode, REPLY_DATA_TOO_LARGE};
use bytes::{Buf, BufMut, BytesMut};

pub type EipResult = Result<(), ErrorCode>;

/// EIP marshalling functions
pub trait Serializing {
    /// Un-marshalling
    fn deserialize(&mut self, buf: &mut dyn Buf) -> EipResult;
    /// Marshalling
    fn serialize(&self, buf: &mut BytesMut) -> EipResult;
}

/// Reserve room in a buffer to serialize some object later.
///
/// # Returns
///
/// * The rest of the buffer after the reservation
///
/// # Arguments
///
/// * `buf` - The message buffer to write to
/// * `s` - the number of bytes to reserve
///
/// # Errors
///
/// An error variant will be returned if there is not enough room for the reservation.
///
pub fn split_off(buf: &mut BytesMut, s: usize) -> Result<BytesMut, ErrorCode> {
    if buf.remaining_mut() < s {
        return Err(REPLY_DATA_TOO_LARGE);
    }
    Ok(buf.split_off(s))
}

#[cfg(test)]
pub fn check_auto_traits<T: Sized + Send + Sync + Unpin>() {}
