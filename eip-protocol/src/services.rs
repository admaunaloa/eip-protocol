use crate::eip::{EipResult, Serializing};
use crate::encapsulation::VERSION;
use crate::error_code::{NOT_ENOUGH_DATA, REPLY_DATA_TOO_LARGE};
use crate::item::Item;
use bytes::{Buf, BufMut, BytesMut};
use core::mem::size_of;
use core::ops::BitOr;

const NAME_LEN: usize = 16;
const NAME: &[u8; NAME_LEN] = b"Communications\0\0";

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Capability(u16);

impl BitOr for Capability {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Services {
    pub item: Item,                 // TypeId
    pub encapsulation_version: u16, // Version of the encapsulation
    pub capability: Capability,     // Capability flags
    pub name: [u8; NAME_LEN],       // Service name
}

impl Services {
    pub const EIP_ENCAPSULATION: Capability = Capability(0x0020);
    pub const SUPPORT_CLASS_01: Capability = Capability(0x0100);

    /// Create a new server
    ///
    /// # Arguments
    ///
    /// * `buf` - The message buffer to write to
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytes::BytesMut;
    /// use eip_protocol::services::Services;
    /// use eip_protocol::eip::Serializing;
    ///
    /// let srv = Services::server();
    /// let mut buf = BytesMut::with_capacity(100);
    ///
    /// assert_eq!(
    ///    Ok(()),
    ///    srv.serialize(&mut buf)
    /// );
    ///
    /// assert_eq!(24, buf.len());
    /// assert_eq!(&b"\0\x01\x14\0\x01\0\x20\x01Communications\0\0"[..], buf);
    /// ```
    ///
    pub fn server() -> Self {
        Services {
            item: Item::new(
                Item::SERVICES,
                size_of::<u16>() // encapsulation_version
            + size_of::<u16>() // capability
            + NAME_LEN,
            ),
            encapsulation_version: VERSION,
            capability: Self::EIP_ENCAPSULATION | Self::SUPPORT_CLASS_01,
            name: *NAME,
        }
    }

    const fn serial_size(&self) -> usize {
        size_of::<u16>() // encapsulation_version
        + size_of::<u16>() // capability
        + NAME_LEN // name
    }

    /// List services request
    ///
    /// # Arguments
    ///
    /// * `buf` - The message buffer to write to
    ///
    /// # Errors
    ///
    /// An error variant will be returned if there is not enough room.
    ///
    pub fn list(&self, buf: &mut BytesMut) -> EipResult {
        if buf.remaining_mut() < size_of::<u16>() {
            // room for item_count
            return Err(REPLY_DATA_TOO_LARGE);
        }
        buf.put_u16_le(1); // item_count only one item
        self.serialize(buf)?;
        Ok(())
    }
}

impl Serializing for Services {
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
        self.item.deserialize(buf)?;

        if buf.remaining() < self.serial_size() {
            return Err(NOT_ENOUGH_DATA);
        }
        self.encapsulation_version = buf.get_u16_le();
        self.capability = Capability(buf.get_u16_le());
        for n in 0..NAME_LEN {
            self.name[n] = buf.get_u8();
        }
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
        self.item.serialize(buf)?;

        if buf.remaining_mut() < self.serial_size() {
            return Err(REPLY_DATA_TOO_LARGE);
        }
        buf.put_u16_le(self.encapsulation_version);
        buf.put_u16_le(self.capability.0);
        buf.put(&self.name[..]);
        Ok(())
    }
}

#[test]
fn deserialize() {
    let mut srv = Services::default();
    let mut buf = &b"\0\x01\x14\0\x01\0\x20\x01Communications\0\0"[..];

    assert_eq!(Ok(()), srv.deserialize(&mut buf));
    assert_eq!(0, buf.len());
    assert_eq!(Item::SERVICES, srv.item.type_id);
    assert_eq!(20, srv.item.len);
    assert_eq!(1, srv.encapsulation_version);
    assert_eq!(
        Services::EIP_ENCAPSULATION | Services::SUPPORT_CLASS_01,
        srv.capability
    );
    assert_eq!(&b"Communications\0\0"[..], &srv.name);

    buf = &b"\0\x01\x14\0\x01\0\x20\x01Communications\0"[..];
    assert_eq!(Err(NOT_ENOUGH_DATA), srv.deserialize(&mut buf));
}

#[test]
fn serialize() {
    let srv = Services::server();
    let mut buf = BytesMut::with_capacity(100);

    assert_eq!(Ok(()), srv.serialize(&mut buf));
    assert_eq!(24, buf.len());
    assert_eq!(&b"\0\x01\x14\0\x01\0\x20\x01Communications\0\0"[..], buf);
}

#[test]
fn auto_traits() {
    use crate::eip::check_auto_traits;

    check_auto_traits::<Capability>();
    check_auto_traits::<Services>();
}
