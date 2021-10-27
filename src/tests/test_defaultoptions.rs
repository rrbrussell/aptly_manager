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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_defaultoptions() {
    let test_string =
      "architectures: [\"i386\"]\ninstaller = false\nsources = false\nudebs = false\n[debian]\ninstaller=true";
    let manual = DefaultOptions {
      architectures: vec![String::from("i386")],
      installer: false,
      sources: false,
      udebs: false,
    };

    let parsed: DefaultOptions = toml::from_str(test_string).unwrap();
    assert_eq!(parsed.architectures, manual.architectures);
    assert_eq!(parsed.installer, manual.installer);
    assert_eq!(parsed.sources, manual.sources);
    assert_eq!(parsed.udebs, manual.udebs);
  }
}
