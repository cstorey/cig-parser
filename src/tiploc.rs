use std::fmt;

use nom::IResult;

use crate::errors::CIFParseError;
use crate::helpers::{mandatory, string};

#[derive(Clone, Eq, PartialEq)]
pub struct Tiploc<'a>(&'a str);

impl<'a> Tiploc<'a> {
    pub fn parse(i: &'a [u8]) -> IResult<&'a [u8], Self, CIFParseError> {
        let (i, name) = mandatory("tiploc", string(7usize))(i)?;
        Ok((i, Tiploc(name)))
    }
}

impl<'a> Tiploc<'a> {
    pub fn of_str(s: &'a str) -> Self {
        Tiploc(s)
    }
}

impl fmt::Debug for Tiploc<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_tuple("Tiploc").field(&self.0).finish()
    }
}

impl fmt::Display for Tiploc<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

impl<'a> From<&'a str> for Tiploc<'a> {
    fn from(tl: &'a str) -> Self {
        Tiploc(tl)
    }
}

impl AsRef<str> for Tiploc<'_> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
