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
use aptly_lib::Prefix;
use std::collections::HashMap;
use string_template::Template;
use structopt::StructOpt;

const MIRROR_CREATE_STRING: &str =
  "aptly mirror create {{options}} {{mirror_name}} {{url}} {{components}}";

fn main() {
  let _cli_arguments = CliArguments::from_args();

  let _ubuntu = ubuntu::initalize();
  create_mirrors(_ubuntu);
  let _debian = debian::initalize();
  create_mirrors(_debian);
}

#[derive(Debug, StructOpt)]
#[structopt(
  name = "aply_manager",
  about = "Eases management of multiple aptly mirrrors."
)]
enum CliArguments {
  #[structopt(name = "create_mirrors")]
  /// Create the required mirrors.
  CreateMirrors,
}

fn create_mirrors(set: Prefix) {
  let create_command = Template::new(MIRROR_CREATE_STRING);
  for distro in set.distributions.iter() {
    let mut args = HashMap::<&str, &str>::new();
    let a = distro.options.to_command_line_arguments();
    args.insert("options", &a);
    args.insert("mirror_name", distro.name);
    if let Some(uri) = distro.uri {
      args.insert("url", uri);
    } else {
      args.insert("url", set.uri);
    }
    let c;
    if let Some(comps) = &distro.components {
      c = comps.join(" ");
      args.insert("components", &c);
    } else {
      c = set.components.join(" ");
      args.insert("components", &c);
    }
    println!("{}", create_command.render(&args));
  }
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
