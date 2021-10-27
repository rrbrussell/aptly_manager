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

use aptly_lib::debian;
use aptly_lib::ubuntu;
use std::path::PathBuf;
use structopt::StructOpt;

fn main() {
  let _cli_arguments = CliArguments::from_args();

  let _ubuntu = ubuntu::initalize();
  let _debian = debian::initalize();
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

/*
fn parse_default_options(input: &mut Table) -> Option<DefaultOptions> {
  let mut options: DefaultOptions = Default::default();

  if input.contains_key("architectures") {
    if input.entry("architectures").
  }

  return Some(options);
}
*/
/*
let mut test_parse: Table = toml::from_str(
  r#"architectures= ['i386']
  installer = false
  sources = false
  udebs = false
  [debian]
  installer = true
  distributions = ['bullseye','bookworm']
  components = ['main', 'contrib', 'non-free']
  [debian.bullseye]
  sources = true"#,
)
.unwrap();

let defaults: DefaultOptions = parse_default_options(&mut test_parse).unwrap();

println!("{:#?}", defaults);
println!("{:#?}", test_parse);
*/
