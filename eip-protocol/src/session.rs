use crate::eip::EipResult;
use crate::error_code::{
    INVALID_SESSION, NOT_ENOUGH_DATA, REPLY_DATA_TOO_LARGE, UNSUPPORTED_VERSION,
};

use crate::encapsulation::VERSION;
use bytes::{Buf, BufMut, BytesMut};
use core::mem::size_of;
use std::collections::HashSet;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Session {
    id: u32,
    set: HashSet<u32>,
}

impl Session {
    /// Request to register a new session
    ///
    /// # Arguments
    ///
    /// * `req` - The message buffer to read from to
    /// * `res` - The message buffer to write to
    /// * `id` - The identifier value of the new session
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytes::BytesMut;
    /// use eip_protocol::session::Session;
    ///
    /// let mut req = &b"\x01\0\0\0"[..];
    /// let mut res = BytesMut::with_capacity(100);
    /// let mut id: u32 = 0;
    /// let mut session = Session::default();
    ///
    /// assert_eq!(
    ///    Ok(()),
    ///    session.register(&mut req, &mut res, &mut id)
    /// );
    ///
    /// assert_eq!(4, res.len());
    /// assert_eq!(&b"\x01\0\0\0"[..], res);
    ///
    /// assert_eq!(true, session.check(id));
    /// assert_eq!(Ok(()), session.unregister(id));
    ///
    /// ```
    ///
    /// # Errors
    ///
    /// If the size of the buffers is not sufficient or if the encapsualtion version is incompatible
    /// an error variant will be returned.
    ///
    pub fn register(&mut self, req: &mut dyn Buf, res: &mut BytesMut, id: &mut u32) -> EipResult {
        let size = size_of::<u16>() + size_of::<u16>(); // protocol version + options flags

        if req.remaining() < size {
            return Err(NOT_ENOUGH_DATA);
        }

        if res.remaining_mut() < size {
            return Err(REPLY_DATA_TOO_LARGE);
        }

        let version = req.get_u16_le();
        let _ = req.get_u16_le(); // OptionFlags

        // find a free session id
        loop {
            self.id += 1;
            if self.set.insert(self.id) {
                break;
            }
        }
        *id = self.id;

        res.put_u16_le(VERSION);
        res.put_u16_le(0); // OptionFlags

        if version > VERSION {
            return Err(UNSUPPORTED_VERSION);
        }

        Ok(())
    }

    /// Request to remove a session
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier value of the session to remove
    ///
    /// # Errors
    ///
    /// If the session does not exist an error variant will be returned.
    ///
    pub fn unregister(&mut self, id: u32) -> EipResult {
        if !self.set.remove(&id) {
            return Err(INVALID_SESSION);
        }
        Ok(())
    }

    /// Test if this is a valid session number
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier value of the session
    ///
    /// # Returns
    ///
    /// True if this is a valid session, false otherwise.
    ///
    pub fn check(&mut self, id: u32) -> bool {
        self.set.contains(&id)
    }
}

#[test]
fn bounds() {
    use bytes::BytesMut;

    let mut req = &b"\x01\0\0\0"[..];
    let mut req_too_short = &b"\x01\0\0"[..];
    let mut req_unsupported_version = &b"\x02\0\0\0"[..];
    let mut res = BytesMut::with_capacity(100);
    let mut id: u32 = 0;
    let mut session = Session::default();

    assert_eq!(
        Err(NOT_ENOUGH_DATA),
        session.register(&mut req_too_short, &mut res, &mut id)
    );
    assert_eq!(
        Err(UNSUPPORTED_VERSION),
        session.register(&mut req_unsupported_version, &mut res, &mut id)
    );
    assert_eq!(Ok(()), session.register(&mut req, &mut res, &mut id));

    let id_wrong = id + 1;

    assert_eq!(false, session.check(id_wrong));
    assert_eq!(Err(INVALID_SESSION), session.unregister(id_wrong));
}

#[test]
fn auto_traits() {
    use crate::eip::check_auto_traits;

    check_auto_traits::<Session>();
}
