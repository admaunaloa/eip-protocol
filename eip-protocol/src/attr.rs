#![allow(dead_code)]
use crate::eip::{EipResult, Serializing};
use crate::error_code::{
    ATTRIBUTE_NOT_GETTABLE, ATTRIBUTE_NOT_SETTABLE, INVALID_PARAMETER, NOT_ENOUGH_DATA,
    REPLY_DATA_TOO_LARGE, TOO_MUCH_DATA,
};
use bytes::{Buf, BufMut, BytesMut};
use core::mem::size_of;
use log::warn;

// Attribute access levels
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub struct AccessCode(u8);

impl AccessCode {
    pub const NONE: u8 = 0x00;
    pub const GET: u8 = 0x01;
    pub const SET: u8 = 0x02;

    /// Create an instance
    ///
    /// # Arguments
    ///
    /// * `code` - The accessibility level, the values can be or-ed |
    ///
    /// # Returns
    ///
    /// * The created instance
    ///
    pub fn new(code: u8) -> Self {
        AccessCode(code)
    }

    /// Can the attribute be retrieved
    ///
    /// # Returns
    ///
    /// * true if the attribute can be retrieved
    ///
    #[inline]
    pub fn getable(&self) -> bool {
        (self.0 & AccessCode::GET) != 0
    }

    /// Can the attribute be changed
    ///
    /// # Returns
    ///
    /// * true if the attribute can be changed
    ///
    #[inline]
    pub fn settable(&self) -> bool {
        (self.0 & AccessCode::SET) != 0
    }
}

/// Default is access code both GET and SET
impl Default for AccessCode {
    fn default() -> Self {
        AccessCode(AccessCode::GET | AccessCode::SET)
    }
}

/// Attribute that holds an signed 8 bit integer
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Sint {
    /// The internal value
    val: i8,
    /// The allowed access methods
    acc: AccessCode,
}

impl Sint {
    /// Creates an attribute.
    ///
    /// # Arguments
    ///
    /// * `val` - The initial value
    /// * `access` - The accessibility via the eip interface, the internal get/set are not influenced.
    ///
    pub fn new(val: i8, acc: AccessCode) -> Self {
        Sint { val, acc }
    }

    /// Retrieves the value from an attribute.
    ///
    /// # Returns
    ///
    /// * The internal value
    ///
    #[inline]
    pub fn get(&self) -> i8 {
        self.val
    }

    /// Changes the value to an attribute.
    ///
    /// # Arguments
    ///
    /// * `val` - The value to copy
    ///
    #[inline]
    pub fn set(&mut self, val: i8) {
        self.val = val;
    }

    /// Get the serialized size in Bytes.
    ///
    /// # Returns
    ///
    /// * The number of bytes when serialized
    ///
    pub const fn serial_size(&self) -> usize {
        size_of::<i8>()
    }
}

impl Serializing for Sint {
    /// Read the value from a message buffer.
    ///
    /// # Arguments
    ///
    /// * `buf` - The message buffer to read from
    ///
    fn deserialize(&mut self, buf: &mut dyn Buf) -> EipResult {
        if !self.acc.settable() {
            return Err(ATTRIBUTE_NOT_SETTABLE);
        }
        if buf.remaining() < self.serial_size() {
            return Err(NOT_ENOUGH_DATA);
        }
        self.val = buf.get_i8();
        Ok(())
    }

    /// Write the value to a message buffer.
    ///
    /// # Arguments
    ///
    /// * `buf` - The message buffer to write to
    ///
    fn serialize(&self, buf: &mut BytesMut) -> EipResult {
        if !self.acc.getable() {
            return Err(ATTRIBUTE_NOT_GETTABLE);
        }
        if buf.remaining_mut() < self.serial_size() {
            return Err(REPLY_DATA_TOO_LARGE);
        }
        buf.put_i8(self.val);
        Ok(())
    }
}

/// Attribute that holds an signed 16 bit integer
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Int {
    /// The internal value
    val: i16,
    /// The allowed access methods
    acc: AccessCode,
}

impl Int {
    /// Creates an attribute.
    ///
    /// # Arguments
    ///
    /// * `val` - The initial value
    /// * `access` - The accessibility via the eip interface, the internal get/set are not influenced.
    ///
    pub fn new(val: i16, acc: AccessCode) -> Self {
        Int { val, acc }
    }

    /// Retrieves the value from an attribute.
    ///
    /// # Returns
    ///
    /// * The internal value
    ///
    #[inline]
    pub fn get(&self) -> i16 {
        self.val
    }

    /// Changes the value to an attribute.
    ///
    /// # Arguments
    ///
    /// * `val` - The value to copy
    ///
    #[inline]
    pub fn set(&mut self, val: i16) {
        self.val = val;
    }

    /// Get the serialized size in Bytes.
    ///
    /// # Returns
    ///
    /// * The number of bytes when serialized
    ///
    pub const fn serial_size(&self) -> usize {
        size_of::<i16>()
    }
}

impl Serializing for Int {
    /// Read the value from a message buffer.
    ///
    /// # Arguments
    ///
    /// * `buf` - The message buffer to read from
    ///
    fn deserialize(&mut self, buf: &mut dyn Buf) -> EipResult {
        if !self.acc.settable() {
            return Err(ATTRIBUTE_NOT_SETTABLE);
        }
        if buf.remaining() < self.serial_size() {
            return Err(NOT_ENOUGH_DATA);
        }
        self.val = buf.get_i16_le();
        Ok(())
    }

    /// Write the value to a message buffer.
    ///
    /// # Arguments
    ///
    /// * `buf` - The message buffer to write to
    ///
    fn serialize(&self, buf: &mut BytesMut) -> EipResult {
        if !self.acc.getable() {
            return Err(ATTRIBUTE_NOT_GETTABLE);
        }
        if buf.remaining_mut() < self.serial_size() {
            return Err(REPLY_DATA_TOO_LARGE);
        }
        buf.put_i16_le(self.val);
        Ok(())
    }
}

/// Attribute that holds an signed 32 bit integer
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DInt {
    /// The internal value
    val: i32,
    /// The allowed access methods
    acc: AccessCode,
}

impl DInt {
    /// Creates an attribute.
    ///
    /// # Arguments
    ///
    /// * `val` - The initial value
    /// * `access` - The accessibility via the eip interface, the internal get/set are not influenced.
    ///
    pub fn new(val: i32, acc: AccessCode) -> Self {
        DInt { val, acc }
    }

    /// Retrieves the value from an attribute.
    ///
    /// # Returns
    ///
    /// * The internal value
    ///
    #[inline]
    pub fn get(&self) -> i32 {
        self.val
    }

    /// Changes the value to an attribute.
    ///
    /// # Arguments
    ///
    /// * `val` - The value to copy
    ///
    #[inline]
    pub fn set(&mut self, val: i32) {
        self.val = val;
    }

    /// Get the serialized size in Bytes.
    ///
    /// # Returns
    ///
    /// * The number of bytes when serialized
    ///
    pub const fn serial_size(&self) -> usize {
        size_of::<i32>()
    }
}

impl Serializing for DInt {
    /// Read the value from a message buffer.
    ///
    /// # Arguments
    ///
    /// * `buf` - The message buffer to read from
    ///
    fn deserialize(&mut self, buf: &mut dyn Buf) -> EipResult {
        if !self.acc.settable() {
            return Err(ATTRIBUTE_NOT_SETTABLE);
        }
        if buf.remaining() < self.serial_size() {
            return Err(NOT_ENOUGH_DATA);
        }
        self.val = buf.get_i32_le();
        Ok(())
    }

    /// Write the value to a message buffer.
    ///
    /// # Arguments
    ///
    /// * `buf` - The message buffer to write to
    ///
    fn serialize(&self, buf: &mut BytesMut) -> EipResult {
        if !self.acc.getable() {
            return Err(ATTRIBUTE_NOT_GETTABLE);
        }
        if buf.remaining_mut() < self.serial_size() {
            return Err(REPLY_DATA_TOO_LARGE);
        }
        buf.put_i32_le(self.val);
        Ok(())
    }
}

/// Attribute that holds an unsigned 8 bit integer
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Usint {
    /// The internal value
    val: u8,
    /// The allowed access methods
    acc: AccessCode,
}

impl Usint {
    /// Creates an attribute.
    ///
    /// # Arguments
    ///
    /// * `val` - The initial value
    /// * `access` - The accessibility via the eip interface, the internal get/set are not influenced.
    ///
    pub fn new(val: u8, acc: AccessCode) -> Self {
        Usint { val, acc }
    }

    /// Retrieves the value from an attribute.
    ///
    /// # Returns
    ///
    /// * The internal value
    ///
    #[inline]
    pub fn get(&self) -> u8 {
        self.val
    }

    /// Changes the value to an attribute.
    ///
    /// # Arguments
    ///
    /// * `val` - The value to copy
    ///
    #[inline]
    pub fn set(&mut self, val: u8) {
        self.val = val;
    }

    /// Get the serialized size in Bytes.
    ///
    /// # Returns
    ///
    /// * The number of bytes when serialized
    ///
    pub const fn serial_size(&self) -> usize {
        size_of::<u8>()
    }
}

impl Serializing for Usint {
    /// Read the value from a message buffer.
    ///
    /// # Arguments
    ///
    /// * `buf` - The message buffer to read from
    ///
    fn deserialize(&mut self, buf: &mut dyn Buf) -> EipResult {
        if !self.acc.settable() {
            return Err(ATTRIBUTE_NOT_SETTABLE);
        }
        if buf.remaining() < self.serial_size() {
            return Err(NOT_ENOUGH_DATA);
        }
        self.val = buf.get_u8();
        Ok(())
    }

    /// Write the value to a message buffer.
    ///
    /// # Arguments
    ///
    /// * `buf` - The message buffer to write to
    ///
    fn serialize(&self, buf: &mut BytesMut) -> EipResult {
        if !self.acc.getable() {
            return Err(ATTRIBUTE_NOT_GETTABLE);
        }
        if buf.remaining_mut() < self.serial_size() {
            return Err(REPLY_DATA_TOO_LARGE);
        }
        buf.put_u8(self.val);
        Ok(())
    }
}

/// Attribute that holds an unsigned 16 bit integer
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Uint {
    /// The internal value
    val: u16,
    /// The allowed access methods
    acc: AccessCode,
}

impl Uint {
    /// Creates an attribute.
    ///
    /// # Arguments
    ///
    /// * `val` - The initial value
    /// * `access` - The accessibility via the eip interface, the internal get/set are not influenced.
    ///
    pub fn new(val: u16, acc: AccessCode) -> Self {
        Uint { val, acc }
    }

    /// Retrieves the value from an attribute.
    ///
    /// # Returns
    ///
    /// * The internal value
    ///
    #[inline]
    pub fn get(&self) -> u16 {
        self.val
    }

    /// Changes the value to an attribute.
    ///
    /// # Arguments
    ///
    /// * `val` - The value to copy
    ///
    #[inline]
    pub fn set(&mut self, val: u16) {
        self.val = val;
    }

    /// Get the serialized size in Bytes.
    ///
    /// # Returns
    ///
    /// * The number of bytes when serialized
    ///
    pub const fn serial_size(&self) -> usize {
        size_of::<u16>()
    }
}

impl Serializing for Uint {
    /// Read the value from a message buffer.
    ///
    /// # Arguments
    ///
    /// * `buf` - The message buffer to read from
    ///
    fn deserialize(&mut self, buf: &mut dyn Buf) -> EipResult {
        if !self.acc.settable() {
            return Err(ATTRIBUTE_NOT_SETTABLE);
        }
        if buf.remaining() < self.serial_size() {
            return Err(NOT_ENOUGH_DATA);
        }
        self.val = buf.get_u16_le();
        Ok(())
    }

    /// Write the value to a message buffer.
    ///
    /// # Arguments
    ///
    /// * `buf` - The message buffer to write to
    ///
    fn serialize(&self, buf: &mut BytesMut) -> EipResult {
        if !self.acc.getable() {
            return Err(ATTRIBUTE_NOT_GETTABLE);
        }
        if buf.remaining_mut() < self.serial_size() {
            return Err(REPLY_DATA_TOO_LARGE);
        }
        buf.put_u16_le(self.val);
        Ok(())
    }
}

/// Attribute that holds an unsigned 32 bit integer
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Duint {
    /// The internal value
    val: u32,
    /// The allowed access methods
    acc: AccessCode,
}

impl Duint {
    /// Creates an attribute.
    ///
    /// # Arguments
    ///
    /// * `val` - The initial value
    /// * `access` - The accessibility via the eip interface, the internal get/set are not influenced.
    ///
    pub fn new(val: u32, acc: AccessCode) -> Self {
        Duint { val, acc }
    }

    /// Retrieves the value from an attribute.
    ///
    /// # Returns
    ///
    /// * The internal value
    ///
    #[inline]
    pub fn get(&self) -> u32 {
        self.val
    }

    /// Changes the value to an attribute.
    ///
    /// # Arguments
    ///
    /// * `val` - The value to copy
    ///
    #[inline]
    pub fn set(&mut self, val: u32) {
        self.val = val;
    }

    /// Get the serialized size in Bytes.
    ///
    /// # Returns
    ///
    /// * The number of bytes when serialized
    ///
    pub const fn serial_size(&self) -> usize {
        size_of::<u32>()
    }
}

impl Serializing for Duint {
    /// Read the value from a message buffer.
    ///
    /// # Arguments
    ///
    /// * `buf` - The message buffer to read from
    ///
    fn deserialize(&mut self, buf: &mut dyn Buf) -> EipResult {
        if !self.acc.settable() {
            return Err(ATTRIBUTE_NOT_SETTABLE);
        }
        if buf.remaining() < self.serial_size() {
            return Err(NOT_ENOUGH_DATA);
        }
        self.val = buf.get_u32_le();
        Ok(())
    }

    /// Write the value to a message buffer.
    ///
    /// # Arguments
    ///
    /// * `buf` - The message buffer to write to
    ///
    fn serialize(&self, buf: &mut BytesMut) -> EipResult {
        if !self.acc.getable() {
            return Err(ATTRIBUTE_NOT_GETTABLE);
        }
        if buf.remaining_mut() < self.serial_size() {
            return Err(REPLY_DATA_TOO_LARGE);
        }
        buf.put_u32_le(self.val);
        Ok(())
    }
}

/// Attribute that holds an character string. Maximum length is 255 characters.
#[derive(Clone, Debug, PartialEq)]
pub struct ShortString {
    buf: String, // Is deliberatly not Cow, favor simplicity over saving bytes in this case.
    cap: usize,
    acc: AccessCode,
}

impl ShortString {
    /// Creates an attribute with a maximum capacity
    ///
    /// # Arguments
    ///
    /// * `val` - The initial value
    /// * `acc` - The accessibility via the eip interface, the internal get/set are not influenced.
    /// * `capacity` - The capacity. Maximum is 255.
    ///
    pub fn with_capacity(buf: String, acc: AccessCode, capacity: u8) -> Self {
        let len = buf.len();
        let cap = capacity as usize;
        if len > cap {
            warn!(
                "ShortString::with_capacity() String too long, truncated. Length: {}",
                len
            );
        }
        ShortString { buf, cap, acc }
    }

    /// Set a string to the attribute.
    ///
    /// # Arguments
    ///
    /// * `buf` - The string to copy
    ///
    pub fn set(&mut self, buf: String) {
        let len = buf.len();
        if len > self.cap {
            warn!(
                "ShortString::set() String too long, truncated. Length: {}",
                len
            );
            self.buf = (&buf[..self.cap]).into();
        } else {
            self.buf = buf;
        }
    }

    /// Get the serialized size in Bytes.
    ///
    /// # Returns
    ///
    /// * The number of bytes when serialized
    ///
    pub fn serial_size(&self) -> usize {
        size_of::<u8>() + self.buf.len() // one for the size byte
    }
}

impl Serializing for ShortString {
    /// Read the value from a message buffer.
    ///
    /// # Arguments
    ///
    /// * `buf` - The message buffer to read from
    ///
    fn deserialize(&mut self, buf: &mut dyn Buf) -> EipResult {
        if !self.acc.settable() {
            return Err(ATTRIBUTE_NOT_SETTABLE);
        }

        // check if the incoming buffer can have the size byte
        if buf.remaining() < 1 {
            return Err(NOT_ENOUGH_DATA);
        }

        let l = buf.get_u8() as usize; // get size

        // check if the size is available in the incoming buffer
        if buf.remaining() < l {
            return Err(NOT_ENOUGH_DATA);
        }

        // check if the internal capacity is enough to hold the string
        if self.cap < l {
            return Err(TOO_MUCH_DATA);
        }

        // check if the incoming is unicode format
        let s = match String::from_utf8(buf.copy_to_bytes(l).to_vec()) {
            Ok(v) => v,
            Err(_) => return Err(INVALID_PARAMETER),
        };
        self.buf = s;

        Ok(()) // one for the size byte
    }

    /// Write the value to a message buffer.
    ///
    /// # Arguments
    ///
    /// * `buf` - The message buffer to write to
    ///
    fn serialize(&self, buf: &mut BytesMut) -> EipResult {
        if !self.acc.getable() {
            return Err(ATTRIBUTE_NOT_GETTABLE);
        }

        if buf.remaining_mut() < self.serial_size() {
            return Err(REPLY_DATA_TOO_LARGE);
        }

        buf.put_u8(self.buf.len() as u8); // len is limited during assignment
        buf.put(self.buf.as_bytes());
        Ok(()) // one for the size byte
    }
}

/// Default is only capacity set to max
impl Default for ShortString {
    fn default() -> Self {
        ShortString {
            buf: Default::default(),
            cap: u8::MAX as usize,
            acc: Default::default(),
        }
    }
}

#[test]
fn auto_traits() {
    use crate::eip::check_auto_traits;

    check_auto_traits::<AccessCode>();
    check_auto_traits::<Usint>();
    check_auto_traits::<Uint>();
    check_auto_traits::<Duint>();
    check_auto_traits::<ShortString>();
}

#[test]
fn access_codes() {
    let mut access = AccessCode::new(AccessCode::GET);
    assert_eq!(true, access.getable());
    assert_eq!(false, access.settable());

    access = AccessCode::new(AccessCode::SET);
    assert_eq!(false, access.getable());
    assert_eq!(true, access.settable());

    access = AccessCode::new(AccessCode::NONE);
    assert_eq!(false, access.getable());
    assert_eq!(false, access.settable());
}

#[test]
fn sint() {
    let mut sint = Sint::new(123, AccessCode::new(AccessCode::GET));
    sint.set(122);
    assert_eq!(122, sint.get());

    let mut buf = &b"\x06\x07\x08\x09"[..];
    sint = Sint::new(123, AccessCode::new(AccessCode::SET));
    assert_eq!(Ok(()), sint.deserialize(&mut buf));
    assert_eq!(buf.remaining(), 3);
    assert_eq!(0x06, sint.get());

    let mut buf2 = BytesMut::with_capacity(10);
    sint = Sint::new(0x12, AccessCode::new(AccessCode::GET));
    assert_eq!(Ok(()), sint.serialize(&mut buf2));
    assert_eq!(1, buf2.len());
    assert_eq!(&b"\x12"[..], &buf2);

    sint = Sint::new(123, AccessCode::new(AccessCode::GET | AccessCode::SET));
    assert_eq!(123, sint.get());
}

#[test]
fn sint_bounds() {
    let mut buf = BytesMut::with_capacity(10);
    let bounds_list: [i8; 3] = [-128, 0, 127];
    let getable = AccessCode::new(AccessCode::GET);
    let mut inst = Sint::new(123, getable.clone());

    for i in 0..bounds_list.len() {
        let bound = bounds_list[i];
        inst.set(bound);
        assert_eq!(bound, inst.get());
    }

    for i in 0..bounds_list.len() {
        buf.clear();
        let bound = bounds_list[i];
        inst = Sint::new(bound, getable.clone());
        assert_eq!(bound, inst.get());
        assert_eq!(Ok(()), inst.serialize(&mut buf));
        assert_eq!(1, buf.len());
        assert_eq!(bound, buf[0] as i8);
    }
}

#[test]
fn int() {
    let mut int = Int::new(12345, AccessCode::new(AccessCode::GET));
    int.set(22222);
    assert_eq!(22222, int.get());

    let mut buf = &b"\x06\x07\x08\x09"[..];
    int = Int::new(123, AccessCode::new(AccessCode::SET));
    assert_eq!(Ok(()), int.deserialize(&mut buf));
    assert_eq!(buf.remaining(), 2);
    assert_eq!(0x0706, int.get());

    let mut buf2 = BytesMut::with_capacity(10);
    int = Int::new(0x1234, AccessCode::new(AccessCode::GET));
    assert_eq!(Ok(()), int.serialize(&mut buf2));
    assert_eq!(2, buf2.len());
    assert_eq!(&b"\x34\x12"[..], &buf2);

    int = Int::new(12345, AccessCode::new(AccessCode::GET | AccessCode::SET));
    assert_eq!(12345, int.get());
}

#[test]
fn int_bounds() {
    let mut buf = BytesMut::with_capacity(10);
    let bounds_list: [i16; 3] = [-32768, 0, 32767];
    let getable = AccessCode::new(AccessCode::GET);
    let mut inst = Int::new(12345, getable.clone());

    for i in 0..bounds_list.len() {
        let bound = bounds_list[i];
        inst.set(bound);
        assert_eq!(bound, inst.get());
    }

    for i in 0..bounds_list.len() {
        buf.clear();
        let bound = bounds_list[i];
        inst = Int::new(bound, getable.clone());
        assert_eq!(bound, inst.get());
        assert_eq!(Ok(()), inst.serialize(&mut buf));
        assert_eq!(2, buf.len());
        assert_eq!(bound & 0xff, buf[0] as i16);
        assert_eq!((bound >> 8) & 0xff, buf[1] as i16);
    }
}

#[test]
fn dint() {
    let mut dint = DInt::new(123, AccessCode::new(AccessCode::GET));
    dint.set(22222222);
    assert_eq!(22222222, dint.get());

    let mut buf = &b"\x06\x07\x08\x09"[..];
    dint = DInt::new(123, AccessCode::new(AccessCode::SET));
    assert_eq!(Ok(()), dint.deserialize(&mut buf));
    assert_eq!(buf.remaining(), 0);
    assert_eq!(0x09080706, dint.get());

    let mut buf2 = BytesMut::with_capacity(10);
    dint = DInt::new(0x12345678, AccessCode::new(AccessCode::GET));
    assert_eq!(Ok(()), dint.serialize(&mut buf2));
    assert_eq!(4, buf2.len());
    assert_eq!(&b"\x78\x56\x34\x12"[..], &buf2);

    dint = DInt::new(12345678, AccessCode::new(AccessCode::GET | AccessCode::SET));
    assert_eq!(12345678, dint.get());
}

#[test]
fn dint_bounds() {
    let mut buf = BytesMut::with_capacity(10);
    let bounds_list: [i32; 3] = [-2147483648, 0, 2147483647];
    let getable = AccessCode::new(AccessCode::GET);
    let mut inst = DInt::new(123456789, getable.clone());

    for i in 0..bounds_list.len() {
        let bound = bounds_list[i];
        inst.set(bound);
        assert_eq!(bound, inst.get());
    }

    for i in 0..bounds_list.len() {
        buf.clear();
        let bound = bounds_list[i];
        inst = DInt::new(bound, getable.clone());
        assert_eq!(bound, inst.get());
        assert_eq!(Ok(()), inst.serialize(&mut buf));
        assert_eq!(4, buf.len());
        assert_eq!(bound & 0xff, buf[0] as i32);
        assert_eq!((bound >> 8) & 0xff, buf[1] as i32);
        assert_eq!((bound >> 16) & 0xff, buf[2] as i32);
        assert_eq!((bound >> 24) & 0xff, buf[3] as i32);
    }
}

#[test]
fn usint() {
    let mut sint = Usint::new(123, AccessCode::new(AccessCode::GET));
    sint.set(222);
    assert_eq!(222, sint.get());

    let mut buf = &b"\x06\x07\x08\x09"[..];
    sint = Usint::new(123, AccessCode::new(AccessCode::SET));
    assert_eq!(Ok(()), sint.deserialize(&mut buf));
    assert_eq!(buf.remaining(), 3);
    assert_eq!(0x06, sint.get());

    let mut buf2 = BytesMut::with_capacity(10);
    sint = Usint::new(0x12, AccessCode::new(AccessCode::GET));
    assert_eq!(Ok(()), sint.serialize(&mut buf2));
    assert_eq!(1, buf2.len());
    assert_eq!(&b"\x12"[..], &buf2);

    sint = Usint::new(123, AccessCode::new(AccessCode::GET | AccessCode::SET));
    assert_eq!(123, sint.get());
}

#[test]
fn usint_bounds() {
    let mut buf = BytesMut::with_capacity(10);
    let bounds_list: [u8; 3] = [0, 127, 255];
    let getable = AccessCode::new(AccessCode::GET);
    let mut inst = Usint::new(123, getable.clone());

    for i in 0..bounds_list.len() {
        let bound = bounds_list[i];
        inst.set(bound);
        assert_eq!(bound, inst.get());
    }

    for i in 0..bounds_list.len() {
        buf.clear();
        let bound = bounds_list[i];
        inst = Usint::new(bound, getable.clone());
        assert_eq!(bound, inst.get());
        assert_eq!(Ok(()), inst.serialize(&mut buf));
        assert_eq!(1, buf.len());
        assert_eq!(bound, buf[0]);
    }
}

#[test]
fn uint() {
    let mut int = Uint::new(12345, AccessCode::new(AccessCode::GET));
    int.set(22222);
    assert_eq!(22222, int.get());

    let mut buf = &b"\x06\x07\x08\x09"[..];
    int = Uint::new(123, AccessCode::new(AccessCode::SET));
    assert_eq!(Ok(()), int.deserialize(&mut buf));
    assert_eq!(buf.remaining(), 2);
    assert_eq!(0x0706, int.get());

    let mut buf2 = BytesMut::with_capacity(10);
    int = Uint::new(0x1234, AccessCode::new(AccessCode::GET));
    assert_eq!(Ok(()), int.serialize(&mut buf2));
    assert_eq!(2, buf2.len());
    assert_eq!(&b"\x34\x12"[..], &buf2);

    int = Uint::new(12345, AccessCode::new(AccessCode::GET | AccessCode::SET));
    assert_eq!(12345, int.get());
}

#[test]
fn uint_bounds() {
    let mut buf = BytesMut::with_capacity(10);
    let bounds_list: [u16; 3] = [0, 32768, 65535];
    let getable = AccessCode::new(AccessCode::GET);
    let mut inst = Uint::new(12345, getable.clone());

    for i in 0..bounds_list.len() {
        let bound = bounds_list[i];
        inst.set(bound);
        assert_eq!(bound, inst.get());
    }

    for i in 0..bounds_list.len() {
        buf.clear();
        let bound = bounds_list[i];
        inst = Uint::new(bound, getable.clone());
        assert_eq!(bound, inst.get());
        assert_eq!(Ok(()), inst.serialize(&mut buf));
        assert_eq!(2, buf.len());
        assert_eq!(bound & 0xff, buf[0] as u16);
        assert_eq!((bound >> 8) & 0xff, buf[1] as u16);
    }
}

#[test]
fn duint() {
    let mut dint = Duint::new(123, AccessCode::new(AccessCode::GET));
    dint.set(22222222);
    assert_eq!(22222222, dint.get());

    let mut buf = &b"\x06\x07\x08\x09"[..];
    dint = Duint::new(123, AccessCode::new(AccessCode::SET));
    assert_eq!(Ok(()), dint.deserialize(&mut buf));
    assert_eq!(buf.remaining(), 0);
    assert_eq!(0x09080706, dint.get());

    let mut buf2 = BytesMut::with_capacity(10);
    dint = Duint::new(0x12345678, AccessCode::new(AccessCode::GET));
    assert_eq!(Ok(()), dint.serialize(&mut buf2));
    assert_eq!(4, buf2.len());
    assert_eq!(&b"\x78\x56\x34\x12"[..], &buf2);

    dint = Duint::new(12345678, AccessCode::new(AccessCode::GET | AccessCode::SET));
    assert_eq!(12345678, dint.get());
}

#[test]
fn duint_bounds() {
    let mut buf = BytesMut::with_capacity(10);
    let bounds_list: [u32; 3] = [0, 2147483648, 4294967295];
    let getable = AccessCode::new(AccessCode::GET);
    let mut inst = Duint::new(123456789, getable.clone());

    for i in 0..bounds_list.len() {
        let bound = bounds_list[i];
        inst.set(bound);
        assert_eq!(bound, inst.get());
    }

    for i in 0..bounds_list.len() {
        buf.clear();
        let bound = bounds_list[i];
        inst = Duint::new(bound, getable.clone());
        assert_eq!(bound, inst.get());
        assert_eq!(Ok(()), inst.serialize(&mut buf));
        assert_eq!(4, buf.len());
        assert_eq!(bound & 0xff, buf[0] as u32);
        assert_eq!((bound >> 8) & 0xff, buf[1] as u32);
        assert_eq!((bound >> 16) & 0xff, buf[2] as u32);
        assert_eq!((bound >> 24) & 0xff, buf[3] as u32);
    }
}

#[test]
fn short_string() {
    let mut buf = &b"\x05Hello"[..];
    let mut ss = ShortString::default();
    assert_eq!(Ok(()), ss.deserialize(&mut buf));
    assert_eq!(buf.remaining(), 0);

    let mut buf2 = BytesMut::with_capacity(10);
    let ss = ShortString::with_capacity(
        "Hello".into(),
        AccessCode::new(AccessCode::GET | AccessCode::SET),
        100,
    );
    assert_eq!(Ok(()), ss.serialize(&mut buf2));
    assert_eq!(6, buf2.len());
    assert_eq!(&b"\x05Hello"[..], &buf2);
}
