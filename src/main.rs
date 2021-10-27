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

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use structopt::StructOpt;

fn main() {
  let cli_arguments = CliArguments::from_args();
  println!("{:?}", cli_arguments);
}

#[derive(Debug, StructOpt)]
#[structopt(
  name = "aply_manager",
  about = "Eases management of multiple aptly mirrrors."
)]
struct CliArguments {
  /// The control file. Defaults to $HOME/.aptly_manager.toml .
  #[structopt(parse(from_os_str), default_value = "$HOME/.aptly_manager.toml")]
  control_file: PathBuf,
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
struct DefaultOptions {
  architectures: Vec<String>,
  installer: bool,
  sources: bool,
  udebs: bool,
}

#[allow(dead_code)]
struct Distribution {
  architectures: Vec<String>,
  components: Vec<String>,
  installer: bool,
  sources: bool,
  udebs: bool,
  uri: String,
}

#[allow(dead_code)]
struct Prefix {
  architectures: Vec<String>,
  components: Vec<String>,
  distributions: Vec<String>,
  installer: bool,
  sources: bool,
  udebs: bool,
  uri: String,
}
