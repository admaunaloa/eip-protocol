use crate::eip;
use crate::eip::{EipResult, Serializing};
use crate::error_code::{ErrorCode, NOT_ENOUGH_DATA, PATH_SEGMENT_ERROR, REPLY_DATA_TOO_LARGE};
use bytes::{Buf, BufMut, BytesMut};
use core::mem::size_of;
use log::warn;

const ADDITIONAL_STATUS_MAX: u8 = 2;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Service(u8);

impl From<u8> for Service {
    fn from(val: u8) -> Self {
        Self(val)
    }
}

impl From<Service> for u8 {
    fn from(val: Service) -> Self {
        val.0
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Request {
    pub service: Service,
    pub class: Option<u16>,
    pub instance: Option<u16>,
    pub attribute: Option<u16>,
}

impl Request {
    pub const GET_ATTRIBUTE_SINGLE: Service = Service(0x0e);
    pub const SET_ATTRIBUTE_SINGLE: Service = Service(0x0f);
    pub const GET_ATTRIBUTE_ALL: Service = Service(0x01);
    pub const SET_ATTRIBUTE_ALL: Service = Service(0x10);
    pub const NO_OPERATION: Service = Service(0x17);
    pub const RESPONSE: Service = Service(0x80);

    const TYPE_MASK: u8 = 0xe0;
    const TYPE_LOGICAL: u8 = 0x20;
    const LEVEL_MASK: u8 = 0x1c;
    const LEVEL_CLASS: u8 = 0x00;
    const LEVEL_INSTANCE: u8 = 0x04;
    const LEVEL_ATTRIBUTE: u8 = 0x10;
    const FORMAT_MASK: u8 = 0x03;
    const FORMAT_8: u8 = 0x00;
    const FORMAT_16: u8 = 0x01;

    /// Deserialize a logical segment
    ///
    /// # Arguments
    ///
    /// * `buf` - The message buffer to read from
    /// * `seg` - Segment type value
    ///
    /// # Errors
    ///
    /// An error variant will be returned if there is not enough data or the data is invalid.
    ///
    fn deserialize_logical(&mut self, buf: &mut dyn Buf, seg: u8) -> EipResult {
        let val = match seg & Self::FORMAT_MASK {
            Self::FORMAT_8 => {
                if buf.remaining() < 1 {
                    return Err(PATH_SEGMENT_ERROR);
                }
                Some(buf.get_u8() as u16)
            }
            Self::FORMAT_16 => {
                if buf.remaining() < 3 {
                    return Err(PATH_SEGMENT_ERROR);
                }
                buf.get_u8();
                Some(buf.get_u16_le())
            }
            _ => return Err(PATH_SEGMENT_ERROR),
        };

        match seg & Self::LEVEL_MASK {
            Self::LEVEL_CLASS => self.class = val,
            Self::LEVEL_INSTANCE => self.instance = val,
            Self::LEVEL_ATTRIBUTE => self.attribute = val,
            _ => return Err(PATH_SEGMENT_ERROR),
        }
        Ok(())
    }

    /// Serialize one logical segment
    ///
    /// # Arguments
    ///
    /// * `buf` - The message buffer to write to
    /// * `val` - The value to write
    /// * `tag` - The type tag
    ///
    /// # Errors
    ///
    /// An error variant will be returned if there is not enough room.
    ///
    fn serialize_logical(buf: &mut BytesMut, val: u16, tag: u8) -> EipResult {
        if val <= u8::MAX as u16 {
            if buf.remaining_mut() < 2 {
                // 8 bit tag + 8 bit value
                return Err(REPLY_DATA_TOO_LARGE);
            }
            buf.put_u8(Self::TYPE_LOGICAL | Self::FORMAT_8 | tag);
            buf.put_u8(val as u8);
        } else {
            if buf.remaining_mut() < 4 {
                // 8 bit tag + 8bit dummy + 16 bit value
                return Err(REPLY_DATA_TOO_LARGE);
            }
            buf.put_u8(Self::TYPE_LOGICAL | Self::FORMAT_16 | tag);
            buf.put_u8(0);
            buf.put_u16_le(val);
        }
        Ok(())
    }

    /// Get the minumum serialized size in Bytes.
    ///
    /// # Returns
    ///
    /// * The minimum number of bytes when serialized
    ///
    pub const fn serial_size(&self) -> usize {
        size_of::<u8>() // service
        + size_of::<u8>() // segment count
    }
}

impl Serializing for Request {
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
            return Err(PATH_SEGMENT_ERROR);
        }

        self.service = Service(buf.get_u8());
        for _ in 0..buf.get_u8() {
            // number of path segments
            if buf.remaining() < 1 {
                return Err(PATH_SEGMENT_ERROR);
            }
            let seg = buf.get_u8();

            match seg & Self::TYPE_MASK {
                Self::TYPE_LOGICAL => self.deserialize_logical(buf, seg)?,
                _ => return Err(PATH_SEGMENT_ERROR),
            };
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
        let mut n: u8 = 0; // Segment counter

        if buf.remaining_mut() < self.serial_size() {
            return Err(REPLY_DATA_TOO_LARGE);
        }
        let mut b = buf.split_off(self.serial_size()); // room for n

        if let Some(c) = self.class {
            Self::serialize_logical(&mut b, c, Self::LEVEL_CLASS)?;
            n += 1;
            if let Some(i) = self.instance {
                Self::serialize_logical(&mut b, i, Self::LEVEL_INSTANCE)?;
                n += 1;
                if let Some(a) = self.attribute {
                    Self::serialize_logical(&mut b, a, Self::LEVEL_ATTRIBUTE)?;
                    n += 1;
                }
            }
        }

        buf.put_u8(self.service.0);
        buf.put_u8(n);
        buf.unsplit(b);
        Ok(())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Response {
    pub service: Service,
    pub general_status: ErrorCode,
    pub additional_status_size: u8, // number of 16 bit segments
    pub additional_status: [u16; ADDITIONAL_STATUS_MAX as usize],
}

impl Response {
    /// Get the serialized size in Bytes.
    /// Note: additional_status_size must be set correctly
    ///
    /// # Returns
    ///
    /// * The minimum number of bytes when serialized
    ///
    pub fn serial_size(&self) -> usize {
        size_of::<u8>() // service
        + size_of::<u8>() // reserved
        + size_of::<u8>() // general_status
        + size_of::<u8>() // additional_status_size
        + (self.additional_status_size as usize * 2) // 2 because of 16 bit segments
    }

    /// Get the serialized maximum size in Bytes.
    ///
    /// # Returns
    ///
    /// * The maximum number of bytes when serialized
    ///
    const fn serial_size_max(&self) -> usize {
        size_of::<u8>() // service
        + size_of::<u8>() // reserved
        + size_of::<u8>() // general_status
        + size_of::<u8>() // additional_status_size
        + (ADDITIONAL_STATUS_MAX as usize * 2) // 2 because of 16 bit segments
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
        eip::split_off(buf, self.serial_size_max())
    }
}

impl Serializing for Response {
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
        self.additional_status_size = 0;
        if buf.remaining() < self.serial_size() {
            return Err(NOT_ENOUGH_DATA);
        }

        self.service = buf.get_u8().into();
        buf.get_u8();
        self.general_status = buf.get_u8().into();
        let size = buf.get_u8();
        if buf.remaining() < (size as usize * 2) {
            return Err(NOT_ENOUGH_DATA);
        }
        self.additional_status_size = size;

        for i in 0..size {
            if i < ADDITIONAL_STATUS_MAX {
                self.additional_status[i as usize] = buf.get_u16_le();
            } else {
                self.additional_status_size = ADDITIONAL_STATUS_MAX;
                warn!("message_router Response::deserialize() Too much additional status. Discarded: {:#04x}",
                    buf.get_u16_le());
            }
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
        if buf.remaining_mut() < self.serial_size() {
            return Err(REPLY_DATA_TOO_LARGE);
        }

        buf.put_u8(self.service.into());
        buf.put_u8(0);
        buf.put_u8(self.general_status.into());

        let mut size = self.additional_status_size;

        if size > ADDITIONAL_STATUS_MAX {
            size = ADDITIONAL_STATUS_MAX;
            warn!("message_router Response::serialize() Too much additional status, truncated. size: {}", size);
        }

        buf.put_u8(size);
        for i in 0..size {
            buf.put_u16_le(self.additional_status[i as usize]);
        }

        Ok(())
    }
}

#[test]
fn request_deserialize_logic_8() {
    // 8 bit 3 segments
    let mut request = Request::default();
    let mut buf = &b"\x0e\x03\x20\x12\x24\x34\x30\x56"[..];
    assert_eq!(Ok(()), request.deserialize(&mut buf));
    assert_eq!(0, buf.len());
    assert_eq!(Some(0x12), request.class);
    assert_eq!(Some(0x34), request.instance);
    assert_eq!(Some(0x56), request.attribute);

    // 8 bit 2 segments
    buf = &b"\x0e\x02\x20\xff\x24\x00"[..];
    request = Request::default();
    assert_eq!(Ok(()), request.deserialize(&mut buf));
    assert_eq!(0, buf.len());
    assert_eq!(Some(0xff), request.class);
    assert_eq!(Some(0x00), request.instance);
    assert_eq!(None, request.attribute);
}

#[test]
fn request_deserialize_logic_16() {
    // 16 bit 3 segments
    let mut buf = &b"\x0e\x03\x21\0\x34\x12\x25\0\x78\x56\x31\0\x12\x90"[..];
    let mut request = Request::default();
    assert_eq!(Ok(()), request.deserialize(&mut buf));
    assert_eq!(0, buf.len());
    assert_eq!(Some(0x1234), request.class);
    assert_eq!(Some(0x5678), request.instance);
    assert_eq!(Some(0x9012), request.attribute);
}

#[test]
fn request_deserialize_logic_bounds() {
    // no data, empty buffer
    let mut buf = &b""[..];
    let mut request = Request::default();
    assert_eq!(Err(PATH_SEGMENT_ERROR), request.deserialize(&mut buf));

    // 8 bit 3 segments, one missing segment
    buf = &b"\x0e\x03\x20\xff\x24\x00"[..];
    request = Request::default();
    assert_eq!(Err(PATH_SEGMENT_ERROR), request.deserialize(&mut buf));

    // 8 bit 3 segments, one missing value
    buf = &b"\x0e\x03\x20\x12\x24\x34\x30"[..];
    request = Request::default();
    assert_eq!(Err(PATH_SEGMENT_ERROR), request.deserialize(&mut buf));

    // 8 bit 3 segments, one false formatted segment (\x24 -> \x26)
    buf = &b"\x0e\x03\x20\x12\x26\x34\x30\x56"[..];
    request = Request::default();
    assert_eq!(Err(PATH_SEGMENT_ERROR), request.deserialize(&mut buf));
}

#[test]
fn request_serialize_logic_8() {
    // 8 bit 3 segments
    let mut request = Request::default();
    request.service = Service(0x0e);
    request.class = Some(0x12);
    request.instance = Some(0x34);
    request.attribute = Some(0x56);

    let mut buf = BytesMut::with_capacity(100);

    assert_eq!(Ok(()), request.serialize(&mut buf));
    assert_eq!(8, buf.len());
    assert_eq!(&b"\x0e\x03\x20\x12\x24\x34\x30\x56"[..], buf);

    // 8 bit 2 segments
    request.attribute = None;

    buf.clear();

    assert_eq!(Ok(()), request.serialize(&mut buf));
    assert_eq!(6, buf.len());
    assert_eq!(&b"\x0e\x02\x20\x12\x24\x34"[..], buf);
}

#[test]
fn request_serialize_logic_16() {
    // 16 bit 3 segments
    let mut request = Request::default();
    request.service = Service(0x0e);
    request.class = Some(0x1234);
    request.instance = Some(0x5678);
    request.attribute = Some(0x9012);

    let mut buf = BytesMut::with_capacity(100);

    assert_eq!(Ok(()), request.serialize(&mut buf));
    assert_eq!(14, buf.len());
    assert_eq!(
        &b"\x0e\x03\x21\0\x34\x12\x25\0\x78\x56\x31\0\x12\x90"[..],
        buf
    );
}

#[test]
fn response_serialize() {
    let mut res = Response::default();
    res.service = Request::GET_ATTRIBUTE_SINGLE;
    res.general_status = REPLY_DATA_TOO_LARGE;

    let mut buf = BytesMut::with_capacity(100);

    assert_eq!(Ok(()), res.serialize(&mut buf));
    assert_eq!(4, buf.len());
    assert_eq!(&b"\x0e\0\x11\0"[..], buf);

    buf = BytesMut::with_capacity(100);
    res.additional_status_size = 2;
    res.additional_status[0] = 0x1234;
    res.additional_status[1] = 0x5678;

    assert_eq!(Ok(()), res.serialize(&mut buf));
    assert_eq!(8, buf.len());
    assert_eq!(&b"\x0e\0\x11\x02\x34\x12\x78\x56"[..], buf);
}

#[test]
fn response_deserialize() {
    let mut res = Response::default();
    let mut buf = &b"\x0e\0\x11\0"[..];
    assert_eq!(Ok(()), res.deserialize(&mut buf));
    assert_eq!(0, buf.len());
    assert_eq!(Request::GET_ATTRIBUTE_SINGLE, res.service.into());
    assert_eq!(REPLY_DATA_TOO_LARGE, res.general_status.into());
}

#[test]
fn response_deserialize_bounds() {
    let mut res = Response::default();
    let mut buf = &b"\x0e\0\x11"[..];
    assert_eq!(Err(NOT_ENOUGH_DATA), res.deserialize(&mut buf));

    buf = &b"\x0e\0\x11\x02\0\0\0"[..];
    assert_eq!(Err(NOT_ENOUGH_DATA), res.deserialize(&mut buf));
}

#[test]
fn auto_traits() {
    use crate::eip::check_auto_traits;

    check_auto_traits::<Request>();
    check_auto_traits::<Response>();
    check_auto_traits::<Service>();
}
