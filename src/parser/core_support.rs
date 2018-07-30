// Copyright 2013-2014 The Rust Project Developers.
// Copyright 2018 The Uuid Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::fmt;
use parser;

impl<'a> fmt::Display for parser::Expected<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            parser::Expected::Any(ref crits) => write!(f, "one of {:?}", crits),
            parser::Expected::Exact(crit) => write!(f, "{}", crit),
            parser::Expected::Range { min, max } => {
                write!(f, "{}..{} inclusive", min, max)
            }
        }
    }
}

impl<'a> fmt::Display for parser::UuidParseError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: ", self._description())?;

        match *self {
            parser::UuidParseError::InvalidCharacter {
                expected,
                found,
                index,
            } => write!(
                f,
                "expected {:?}, found {} at {}",
                expected.chars(),
                found,
                index
            ),
            parser::UuidParseError::InvalidGroupCount {
                ref expected,
                found,
            } => write!(f, "expected {}, found {}", expected, found),
            parser::UuidParseError::InvalidGroupLength {
                ref expected,
                found,
                group,
            } => write!(
                f,
                "expected {}, found {} in group {}",
                expected, found, group,
            ),
            parser::UuidParseError::InvalidLength {
                ref expected,
                found,
            } => write!(f, "expected {}, found {}", expected, found),
        }
    }
}
