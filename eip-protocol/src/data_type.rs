#![allow(dead_code)]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DataType(u8);

pub const BOOL: DataType = DataType(0xc1); // boolean
pub const SINT: DataType = DataType(0xc2); // signed 8 bit integer
pub const INT: DataType = DataType(0xc3); // signed 16 bit integer
pub const DINT: DataType = DataType(0xc4); // signed 32 bit integer
pub const LINT: DataType = DataType(0xc5); // signed 64 bit integer
pub const USINT: DataType = DataType(0xc6); // unsigned 8 bit integer
pub const UINT: DataType = DataType(0xc7); // unsigned 16 bit integer
pub const UDINT: DataType = DataType(0xc8); // unsigned 32 bit integer
pub const ULINT: DataType = DataType(0xc9); // unsigned 64 bit integer
pub const REAL: DataType = DataType(0xca); // float 32 bit
pub const LREAL: DataType = DataType(0xcb); // float 64 bit
pub const BYTE: DataType = DataType(0xd1); // bit string 8 bit
pub const WORD: DataType = DataType(0xd2); // bit string 16 bit
pub const DWORD: DataType = DataType(0xd3); // bit string 32 bit
pub const LWORD: DataType = DataType(0xd4); // bit string 64 bit
pub const SHORT_STRING: DataType = DataType(0xda); // Path segments
pub const EPATH: DataType = DataType(0xdc); // Character string, 1 byte character, 1 byte length

#[test]
fn auto_traits() {
    use crate::eip::check_auto_traits;

    check_auto_traits::<DataType>();
}
