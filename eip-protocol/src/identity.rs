use crate::attr::{AccessCode, Duint, ShortString, Uint, Usint};
use crate::eip::{EipResult, Serializing};
use crate::encapsulation;
use crate::error_code::ATTRIBUTE_NOT_SUPPORTED;
use crate::item::Item;
use crate::socket_address::SocketAddress;
use bytes::{Buf, BufMut, BytesMut};

/// This object provides identification of and general information about the device.

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Attr(u16);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Identity {
    pub vendor_id: Uint,      // Identification of each vendor by number
    pub device_type: Uint,    // Indication of general type of product
    pub product_code: Uint,   // Identification of a particular product of an individual vendor
    pub revision: Uint,       // Revision of the item the Identity Object represents
    pub status: Uint,         // Summary status of device
    pub serial_number: Duint, // Serial number of device
    pub product_name: ShortString,
    pub state: Usint, // Present state of the device as represented by the state transition diagram
    pub configuration_consistency_value: Uint, // Contents identify configuration of device
    pub heartbeat_interval: Usint, // The nominal interval between heartbeat messages in seconds
    pub socket_address: SocketAddress,
}

impl Identity {
    pub const VENDOR_ID: Attr = Attr(1);
    pub const DEVICE_TYPE: Attr = Attr(2);
    pub const PRODUCT_CODE: Attr = Attr(3);
    pub const REVISION: Attr = Attr(4);
    pub const STATUS: Attr = Attr(5);
    pub const SERIAL_NUMBER: Attr = Attr(6);
    pub const PRODUCT_NAME: Attr = Attr(7);
    pub const STATE: Attr = Attr(8);
    pub const CONFIGURATION_CONSISTENCY_VALUE: Attr = Attr(9);
    pub const HEARTBEAT_INTERVAL: Attr = Attr(10);
    pub const ATTRIBUTE_END: Attr = Attr(11);

    /// Create an instance.
    /// Typically used for server side. Default is typically for the client side.
    ///
    /// # Arguments
    ///
    /// * `vendor_id` - The EIP vendor identification number
    /// * `device_type` - The EIP device identification number
    /// * `product_code` - The product type number
    /// * `revision` - The software revision, major and minor
    /// * `serial_number` - The serial number
    /// * `product_name` - The human readable product description
    ///
    /// # Returns
    ///
    /// * The created instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytes::BytesMut;
    /// use eip_protocol::identity::Identity;
    ///
    /// let name: String = str::to_string("Hello");
    /// let identity = Identity::new(1, 2, 3, 4, 5, name);
    /// let mut buf = BytesMut::with_capacity(10);
    ///
    /// identity.serialize_attribute_single(&mut buf, Identity::SERIAL_NUMBER);
    ///
    /// assert_eq!(4, buf.len());
    /// assert_eq!(&b"\x05\0\0\0"[..], buf);
    /// ```
    pub fn new(
        vendor_id: u16,
        device_type: u16,
        product_code: u16,
        revision: u16,
        serial_number: u32,
        product_name: String,
    ) -> Self {
        let gettable = AccessCode::new(AccessCode::GET);
        Identity {
            vendor_id: Uint::new(vendor_id, gettable.clone()),
            device_type: Uint::new(device_type, gettable.clone()),
            product_code: Uint::new(product_code, gettable.clone()),
            revision: Uint::new(revision, gettable.clone()),
            status: Uint::default(),
            serial_number: Duint::new(serial_number, gettable.clone()),
            product_name: ShortString::with_capacity(product_name, gettable.clone(), 32),
            state: Usint::default(),
            configuration_consistency_value: Uint::default(),
            heartbeat_interval: Usint::default(),
            socket_address: SocketAddress::default(),
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
            Self::VENDOR_ID => self.vendor_id.serialize(buf)?,
            Self::DEVICE_TYPE => self.device_type.serialize(buf)?,
            Self::PRODUCT_CODE => self.product_code.serialize(buf)?,
            Self::REVISION => self.revision.serialize(buf)?,
            Self::STATUS => self.status.serialize(buf)?,
            Self::SERIAL_NUMBER => self.serial_number.serialize(buf)?,
            Self::PRODUCT_NAME => self.product_name.serialize(buf)?,
            Self::STATE => self.state.serialize(buf)?,
            Self::CONFIGURATION_CONSISTENCY_VALUE => {
                self.configuration_consistency_value.serialize(buf)?
            }
            Self::HEARTBEAT_INTERVAL => self.heartbeat_interval.serialize(buf)?,
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
            Self::VENDOR_ID => self.vendor_id.deserialize(buf)?,
            Self::DEVICE_TYPE => self.device_type.deserialize(buf)?,
            Self::PRODUCT_CODE => self.product_code.deserialize(buf)?,
            Self::REVISION => self.revision.deserialize(buf)?,
            Self::STATUS => self.status.deserialize(buf)?,
            Self::SERIAL_NUMBER => self.serial_number.deserialize(buf)?,
            Self::PRODUCT_NAME => self.product_name.deserialize(buf)?,
            Self::STATE => self.state.deserialize(buf)?,
            Self::CONFIGURATION_CONSISTENCY_VALUE => {
                self.configuration_consistency_value.deserialize(buf)?
            }
            Self::HEARTBEAT_INTERVAL => self.heartbeat_interval.deserialize(buf)?,
            _ => return Err(ATTRIBUTE_NOT_SUPPORTED),
        };
        Ok(())
    }

    /// List the mandatory attributes
    /// State is the last mandatory attribute.
    ///
    /// # Arguments
    ///
    /// * `buf` - The message buffer to write to
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytes::BytesMut;
    /// use eip_protocol::identity::Identity;
    ///
    /// let name: String = str::to_string("Hello");
    /// let id = Identity::new(1, 2, 3, 4, 5, name);
    /// let mut buf = BytesMut::with_capacity(100);
    ///
    /// assert_eq!(
    ///    Ok(()),
    ///    id.list(&mut buf)
    /// );
    ///
    /// assert_eq!(43, buf.len());
    /// assert_eq!(&b"\x0c\0'\0\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\0\0\x05\0\0\0\x05Hello\0"[..], buf);
    /// ```
    ///
    /// # Errors
    ///
    /// If one of the attributes is non existent or is not getable or there is not enough room,
    /// an error variant will be returned.
    ///
    pub fn list(&self, buf: &mut BytesMut) -> EipResult {
        let mut item = Item::new(Item::IDENTITY, 0);
        let mut rest = item.split_off(buf)?;

        rest.put_u16_le(encapsulation::VERSION);

        self.socket_address.serialize(&mut rest)?;

        for n in 1..Self::STATE.0 + 1 {
            self.serialize_attribute_single(&mut rest, Attr(n))?;
        }

        item.len = rest.len();
        item.serialize(buf)?;
        buf.unsplit(rest);
        Ok(())
    }
}

impl Serializing for Identity {
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
    let mut id = Identity::default();
    let mut buf = &b"\x01\0\x02\0\x03\0\x04\0\0\0\x05\0\0\0\x05Hello\0\0\0\0"[..];

    assert_eq!(Ok(()), id.deserialize(&mut buf));

    assert_eq!(0, buf.len());
    assert_eq!(1, id.vendor_id.get());
    assert_eq!(2, id.device_type.get());
    assert_eq!(3, id.product_code.get());
    assert_eq!(4, id.revision.get());
    assert_eq!(5, id.serial_number.get());
}

#[test]
fn deserialize_single() {
    let mut id = Identity::default();
    let mut buf = &b"\x06\x07"[..];
    assert_eq!(
        Ok(()),
        id.deserialize_attribute_single(&mut buf, Identity::STATUS)
    );
    assert_eq!(buf.remaining(), 0);
    assert_eq!(0x0706, id.status.get());
}

#[test]
fn serialize() {
    let name: String = str::to_string("Hello");
    let id = Identity::new(1, 2, 3, 4, 5, name);
    let mut buf = BytesMut::with_capacity(100);
    assert_eq!(Ok(()), id.serialize(&mut buf));
    assert_eq!(24, buf.len());
    assert_eq!(
        &b"\x01\0\x02\0\x03\0\x04\0\0\0\x05\0\0\0\x05Hello\0\0\0\0"[..],
        buf
    );
}

#[test]
fn serialize_single() {
    let name: String = str::to_string("Hello");
    let id = Identity::new(1, 2, 3, 4, 5, name);
    let mut buf = BytesMut::with_capacity(10);
    assert_eq!(
        Ok(()),
        id.serialize_attribute_single(&mut buf, Identity::PRODUCT_NAME)
    );
    assert_eq!(6, buf.len());
    assert_eq!(&b"\x05Hello"[..], buf);
}

#[cfg(test)]
fn setup_test_identity() -> Identity {
    let name: String = str::to_string("Hello");
    let mut identity = Identity::new(1, 2, 3, 4, 5, name);

    identity.status.set(6);
    identity.state.set(7);
    identity.configuration_consistency_value.set(8);
    identity.heartbeat_interval.set(9);
    identity
}

#[test]
fn get_attribute_device_type() {
    let mut buf = BytesMut::with_capacity(10);
    assert_eq!(
        Ok(()),
        setup_test_identity().serialize_attribute_single(&mut buf, Identity::DEVICE_TYPE)
    );
    assert_eq!(2, buf.len());
    assert_eq!(buf[0], 2);
    assert_eq!(buf[1], 0);
}

#[test]
fn get_attribute_state() {
    let mut buf = BytesMut::with_capacity(10);
    assert_eq!(
        Ok(()),
        setup_test_identity().serialize_attribute_single(&mut buf, Identity::STATE)
    );
    assert_eq!(1, buf.len());
    assert_eq!(7, buf[0]);
}

#[test]
fn auto_traits() {
    use crate::eip::check_auto_traits;

    check_auto_traits::<Attr>();
    check_auto_traits::<Identity>();
}
