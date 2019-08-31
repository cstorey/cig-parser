use std::borrow::Cow;

use nom::{
    branch::alt, bytes::streaming::*, character::is_space, character::streaming::*,
    combinator::map, IResult,
};

use crate::errors::CIFParseError;
use crate::helpers::{mandatory, string};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FullOrUpdate {
    Full,
    Update,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Header<'a> {
    pub file_mainframe_identity: Cow<'a, str>,
    pub extract_date: Cow<'a, str>,
    pub extract_time: Cow<'a, str>,
    pub current_file: Cow<'a, str>,
    pub last_file: Option<Cow<'a, str>>,
    pub update_indicator: FullOrUpdate,
    pub version: Cow<'a, str>,
    pub user_start_date: Cow<'a, str>,
    pub user_end_date: Cow<'a, str>,
}

pub(super) fn parse_header<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Header, CIFParseError> {
    |i: &'a [u8]| -> IResult<&'a [u8], Header, CIFParseError> {
        let (i, _) = tag("HD")(i)?;
        let (i, file_mainframe_identity) = mandatory(string(20usize))(i)?;
        let (i, extract_date) = mandatory(string(6usize))(i)?;
        let (i, extract_time) = mandatory(string(4usize))(i)?;
        let (i, current_file) = mandatory(string(7usize))(i)?;
        let (i, last_file) = string(7usize)(i)?;
        let (i, update_indicator) = alt((
            map(char('U'), |_| FullOrUpdate::Update),
            map(char('F'), |_| FullOrUpdate::Full),
        ))(i)?;
        let (i, version) = mandatory(string(1usize))(i)?;
        let (i, user_start_date) = mandatory(string(6usize))(i)?;
        let (i, user_end_date) = mandatory(string(6usize))(i)?;
        let (i, _spare) = take_while_m_n(20, 20, is_space)(i)?;

        Ok((
            i,
            Header {
                file_mainframe_identity: file_mainframe_identity.into(),
                extract_date: extract_date.into(),
                extract_time: extract_time.into(),
                current_file: current_file.into(),
                last_file: last_file.map(Into::into),
                update_indicator: update_indicator,
                version: version.into(),
                user_start_date: user_start_date.into(),
                user_end_date: user_end_date.into(),
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_full_header() {
        let p = parse_header();
        let hdr =
            b"HDTPS.UDFROC1.PD1907050507191939DFROC2S       FA050719040720                    ";
        let (rest, _val) = p(hdr).expect("parse_header");
        assert_eq!(String::from_utf8_lossy(rest), "");
    }
}
