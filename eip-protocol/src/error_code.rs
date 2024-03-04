#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ErrorCode(u8);

pub const SUCCESS: ErrorCode = ErrorCode(0x00);
pub const UNSUPPORTED_COMMAND: ErrorCode = ErrorCode(0x01);
pub const INSUFFICIENT_MEMORY: ErrorCode = ErrorCode(0x02);
pub const INCORRECT_DATA: ErrorCode = ErrorCode(0x03);
pub const PATH_SEGMENT_ERROR: ErrorCode = ErrorCode(0x04);
pub const PATH_DESTINATION_UNKNOWN: ErrorCode = ErrorCode(0x05);
pub const ATTRIBUTE_NOT_SETTABLE: ErrorCode = ErrorCode(0x0e);
pub const REPLY_DATA_TOO_LARGE: ErrorCode = ErrorCode(0x11);
pub const NOT_ENOUGH_DATA: ErrorCode = ErrorCode(0x13);
pub const ATTRIBUTE_NOT_SUPPORTED: ErrorCode = ErrorCode(0x14);
pub const TOO_MUCH_DATA: ErrorCode = ErrorCode(0x15);
pub const OBJECT_DOES_NOT_EXIST: ErrorCode = ErrorCode(0x16);
pub const INVALID_PARAMETER: ErrorCode = ErrorCode(0x20);
pub const MESSAGE_FORMAT_ERROR: ErrorCode = ErrorCode(0x24);
pub const ATTRIBUTE_NOT_GETTABLE: ErrorCode = ErrorCode(0x2c);
pub const INVALID_SESSION: ErrorCode = ErrorCode(0x64);
pub const UNSUPPORTED_VERSION: ErrorCode = ErrorCode(0x69);

impl From<u8> for ErrorCode {
    fn from(val: u8) -> Self {
        Self(val)
    }
}

impl From<ErrorCode> for u8 {
    fn from(val: ErrorCode) -> Self {
        val.0
    }
}

impl From<ErrorCode> for u32 {
    fn from(val: ErrorCode) -> Self {
        val.0 as u32
    }
}

#[test]
fn auto_traits() {
    use crate::eip::check_auto_traits;

    check_auto_traits::<ErrorCode>();
}
