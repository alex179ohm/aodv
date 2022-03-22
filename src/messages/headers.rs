// SPDX-License-Identifier: MIT

#![allow(dead_code)]

#[derive(Clone, Copy, Debug)]
pub enum Type {
    Req,
    Rep,
    Err,
    Ack,
}

impl Type {
    /// Returns `true` if the type is [`RReq`].
    ///
    /// [`RReq`]: Type::RReq
    pub fn is_rreq(&self) -> bool {
        matches!(self, Self::Req)
    }

    /// Returns `true` if the type is [`RRep`].
    ///
    /// [`RRep`]: Type::RRep
    pub fn is_rrep(&self) -> bool {
        matches!(self, Self::Rep)
    }

    /// Returns `true` if the type is [`RErr`].
    ///
    /// [`RErr`]: Type::RErr
    pub fn is_rerr(&self) -> bool {
        matches!(self, Self::Err)
    }

    /// Returns `true` if the type is [`Ack`].
    ///
    /// [`Ack`]: Type::Ack
    pub fn is_ack(&self) -> bool {
        matches!(self, Self::Ack)
    }
}

impl TryFrom<u8> for Type {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Type::Req),
            2 => Ok(Type::Rep),
            3 => Ok(Type::Err),
            4 => Ok(Type::Ack),
            _ => Err("Invalid value".to_string()),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Count(u8);

#[derive(Clone, Copy, Debug)]
pub struct PrefixSize(u8);

#[derive(Clone, Copy, Debug)]
pub struct Lifetime(u32);

impl PrefixSize {
    const MAX: u8 = 31;
}

impl TryFrom<u8> for PrefixSize {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0..=PrefixSize::MAX => Ok(PrefixSize(value)),
            _ => Err(format!("value too big,  {} > {}", value, PrefixSize::MAX)),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Flag {
    Join,
    Repair,
    Gratuitous,
    DestinationOnly,
    UnknownSequence,
    AckRequired,
    NoDelete,
}

#[derive(Debug, Clone, Copy)]
pub struct ReqHeader {
    ty: Type,
    flag: Flag,
    hop: Count,
}

impl Default for ReqHeader {
    fn default() -> Self {
        ReqHeader {
            ty: Type::Req,
            flag: Flag::Gratuitous,
            hop: Count(0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RepHeader {
    ty: Type,
    flag: Flag,
    prefix_sz: PrefixSize,
    hop: Count,
}

impl Default for RepHeader {
    fn default() -> Self {
        RepHeader {
            ty: Type::Rep,
            flag: Flag::AckRequired,
            prefix_sz: PrefixSize(0),
            hop: Count(0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ErrHeader {
    ty: Type,
    flag: Flag,
    dest: Count,
}

impl Default for ErrHeader {
    fn default() -> Self {
        Self {
            ty: Type::Err,
            flag: Flag::NoDelete,
            dest: Count(0),
        }
    }
}
