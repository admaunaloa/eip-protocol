#![allow(dead_code)]
use crate::attr::{AccessCode, Uint};
use crate::eip::{EipResult, Serializing};
use crate::error_code::ATTRIBUTE_NOT_SUPPORTED;
use bytes::{Buf, BytesMut};

// This is a set of static attributes that is commonly applicable

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Attr(u16);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct StaticAttr {
    pub revision: Uint,
    pub max_instance: Uint,
    pub number_of_instances: Uint,
}

impl StaticAttr {
    pub const REVISION: Attr = Attr(1);
    pub const MAX_INSTANCE: Attr = Attr(2);
    pub const NUMBER_OF_INSTANCES: Attr = Attr(3);
    pub const ATTRIBUTE_END: Attr = Attr(4);

    /// Create an instance.
    /// Typically used for server side. Default is typically for the client side.
    ///
    /// # Arguments
    ///
    /// * `revision` - The accessibility level, the values can be or-ed |
    /// * `max_instance` - The capacity for instances
    /// * `number_of_instances` - The actual number of instances
    ///
    /// # Returns
    ///
    /// * The created instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytes::BytesMut;
    /// use eip_protocol::static_attr::StaticAttr;
    ///
    /// let sa = StaticAttr::new(0x1234, 2, 3);
    /// let mut buf = BytesMut::with_capacity(10);
    ///
    /// assert_eq!(
    ///    Ok(()),
    ///    sa.serialize_attribute_single(&mut buf, StaticAttr::REVISION)
    /// );
    ///
    /// assert_eq!(2, buf.len());
    /// assert_eq!(&b"\x34\x12"[..], buf);
    /// ```
    pub fn new(revision: u16, max_instance: u16, number_of_instances: u16) -> Self {
        let allow_serialize = AccessCode::new(AccessCode::GET);
        StaticAttr {
            revision: Uint::new(revision, allow_serialize.clone()),
            max_instance: Uint::new(max_instance, allow_serialize.clone()),
            number_of_instances: Uint::new(number_of_instances, allow_serialize.clone()),
        }
    }

    /// Serialize one specific attribute
    ///
    /// # Arguments
    ///
    /// * `buf` - The message buffer to write to
    /// * `attr` - The attribute identifier number
    ///
    /// # Errors
    ///
    /// If the attribute is non existent or is not getable, an error variant will be returned.
    ///
    pub fn serialize_attribute_single(&self, buf: &mut BytesMut, attr: Attr) -> EipResult {
        match attr {
            Self::REVISION => self.revision.serialize(buf)?,
            Self::MAX_INSTANCE => self.max_instance.serialize(buf)?,
            Self::NUMBER_OF_INSTANCES => self.number_of_instances.serialize(buf)?,
            _ => return Err(ATTRIBUTE_NOT_SUPPORTED),
        };
        Ok(())
    }

    /// Deserialize one specific attribute
    ///
    /// # Arguments
    ///
    /// * `buf` - The message buffer to read from
    /// * `attr` - The attribute identifier number
    ///
    /// # Errors
    ///
    /// If the attribute is non existent or is not set-able, an error variant will be returned.
    ///
    pub fn deserialize_attribute_single(&mut self, buf: &mut dyn Buf, attr: Attr) -> EipResult {
        match attr {
            Self::REVISION => self.revision.deserialize(buf)?,
            Self::MAX_INSTANCE => self.max_instance.deserialize(buf)?,
            Self::NUMBER_OF_INSTANCES => self.number_of_instances.deserialize(buf)?,
            _ => return Err(ATTRIBUTE_NOT_SUPPORTED),
        };
        Ok(())
    }
}

impl Serializing for StaticAttr {
    /// Deserialize all attributes
    ///
    /// # Arguments
    ///
    /// * `buf` - The message buffer to read from
    ///
    /// # Errors
    ///
    /// If one of the attributes is non existent or is not set-able, an error variant will be returned.
    ///
    fn deserialize(&mut self, buf: &mut dyn Buf) -> EipResult {
        for n in 1..Self::ATTRIBUTE_END.0 {
            self.deserialize_attribute_single(buf, Attr(n))?;
        }
        Ok(())
    }

    /// Serialize all attributes
    ///
    /// # Arguments
    ///
    /// * `buf` - The message buffer to write to
    ///
    /// # Errors
    ///
    /// If one of the attributes is non existent or is not getable,
    /// an error variant will be returned.
    ///
    fn serialize(&self, buf: &mut BytesMut) -> EipResult {
        for n in 1..Self::ATTRIBUTE_END.0 {
            self.serialize_attribute_single(buf, Attr(n))?;
        }
        Ok(())
    }
}

#[test]
fn deserialize() {
    let mut sa = StaticAttr::default();
    let mut buf = &b"\x02\x01\x04\x03\x06\x05"[..];
    assert_eq!(Ok(()), sa.deserialize(&mut buf));
    assert_eq!(0, buf.len());
    assert_eq!(0x0102, sa.revision.get());
    assert_eq!(0x0304, sa.max_instance.get());
    assert_eq!(0x0506, sa.number_of_instances.get());

    sa = StaticAttr::default();
    buf = &b"\x06\x07"[..];
    assert_eq!(
        Ok(()),
        sa.deserialize_attribute_single(&mut buf, StaticAttr::REVISION)
    );
    assert_eq!(0x0706, sa.revision.get());
}

#[test]
fn serialize() {
    let mut sa = StaticAttr::new(1, 2, 3);
    let mut buf = BytesMut::with_capacity(10);
    assert_eq!(Ok(()), sa.serialize(&mut buf));
    assert_eq!(6, buf.len());
    assert_eq!(&b"\x01\0\x02\0\x03\0"[..], buf);

    sa = StaticAttr::default();
    sa.revision.set(0x1234);
    buf = BytesMut::with_capacity(10);
    assert_eq!(
        Ok(()),
        sa.serialize_attribute_single(&mut buf, StaticAttr::REVISION)
    );
    assert_eq!(2, buf.len());
    assert_eq!(&b"\x34\x12"[..], buf);
}

#[test]
fn auto_traits() {
    use crate::eip::check_auto_traits;

    check_auto_traits::<StaticAttr>();
}
