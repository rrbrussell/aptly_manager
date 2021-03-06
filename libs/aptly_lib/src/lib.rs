//Copyright (C) 2021 Robert R. Russell
//This program is free software; you can redistribute it and/or modify it under
//the terms of the GNU General Public License as published by the Free Software
//Foundation; version 2.
//
//This program is distributed in the hope that it will be useful, but WITHOUT
//ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
//FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
//You should have received a copy of the GNU General Public License along with
//this program; if not, write to the Free Software Foundation, Inc., 51 Franklin
//Street, Fifth Floor, Boston, MA 02110-1301, USA.

use std::string::ToString;
use strum_macros::Display;
use strum_macros::EnumString;

pub mod debian;
pub mod ubuntu;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }
}

/// Architectures represent all of the architectures used in Debian formated repositories.
/// All is assumed.
/// Ubuntu: amd64 arm64 armhf i386 ppc64el riscv64 s390x
/// Debian: amd64 arm64 armel armhf i386 mips64el mipsel ppc64el s390x
#[derive(Debug, EnumString, Display)]
pub enum Architectures {
  #[strum(serialize = "amd64")]
  Amd64,
  #[strum(serialize = "arm64")]
  Arm64,
  #[strum(serialize = "armel")]
  Armel,
  #[strum(serialize = "armhf")]
  Armhf,
  #[strum(serialize = "i386")]
  I386,
  #[strum(serialize = "mips64el")]
  Mips64el,
  #[strum(serialize = "mipsel")]
  Mipsel,
  #[strum(serialize = "ppc64el")]
  Ppc64el,
  #[strum(serialize = "riscv64")]
  Riscv64,
  #[strum(serialize = "s390x")]
  S390x,
}

impl Default for Architectures {
  fn default() -> Self {
    Architectures::Amd64
  }
}

#[derive(Debug)]
pub struct PrimaryOptions {
  pub architectures: Vec<Architectures>,
  pub installer: bool,
  pub sources: bool,
  pub udebs: bool,
}

impl PrimaryOptions {
  pub fn to_command_line_arguments(&self) -> String {
    let mut output = String::with_capacity(64);
    output.push_str("-architectures=");
    output.push_str(&self.architectures[0].to_string());
    let mut i = 1;
    while i < self.architectures.len() {
      output.push(',');
      output.push_str(&self.architectures[i].to_string());
      i += 1;
    }
    output.push(' ');
    if self.installer {
      output.push_str("-with-installer ");
    }
    if self.sources {
      output.push_str("-with-sources ");
    }
    if self.udebs {
      output.push_str("-with-udebs");
    }
    return output;
  }
}

impl Default for PrimaryOptions {
  fn default() -> Self {
    PrimaryOptions {
      architectures: vec![Architectures::Amd64],
      installer: false,
      sources: false,
      udebs: false,
    }
  }
}

pub struct Distribution<'a> {
  pub components: Option<Vec<&'a str>>,
  pub name: &'a str,
  pub options: PrimaryOptions,
  pub uri: Option<&'a str>,
}

pub struct Prefix<'a> {
  pub components: Vec<&'a str>,
  pub distributions: Vec<Distribution<'a>>,
  pub name: &'a str,
  pub uri: &'a str,
}
