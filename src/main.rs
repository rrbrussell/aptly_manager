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

// Command Lines
const MIRROR_CREATE_STRING: &str =
  "aptly mirror create {{options}} {{mirror_name}} {{url}} {{component}}";
const MIRROR_UPDATE_STRING: &str = "aptly mirror update {{mirror_name}}";
const DROP_SNAPSHOTS_STRING: &str = "aptly snapshot drop {{snapshot_name}}";
const RENAME_SNAPSHOTS_STRING: &str =
  "aptly snapshot rename {{old_snapshot_name}} {{new_snapshot_name}}";
const MAKE_SNAPSHOTS_STRING: &str =
  "aptly snapshot create {{snapshot_name}} from mirror {{mirror_name}}";
const PUBLISH_SNAPSHOTS_STRING: &str =
  "aptly publish snapshot -component={{components}} -distribution={{distro}} {{snapshot_names}} {{prefix}}";
const SWITCH_PUBLISHED_SNAPSHOTS_STRING: &str =
  "aptly publish switch -component={{components}} {{distro}} {{prefix}} {{snapshot_names}}";

/// Item Names
const MIRROR_NAME_TEMPLATE: &str = "{{prefix}}-{{distro}}-{{component}}";
const SNAPSHOT_NAME_TEMPLATE: &str = "{{prefix}}-{{distro}}-{{component}}-{{date}}";

fn main() {
  let _cli_arguments = CliArguments::from_args();

  let _ubuntu = ubuntu::initalize();
  let _debian = debian::initalize();
  match _cli_arguments {
    CliArguments::CreateMirrors => {
      create_mirrors(_ubuntu);
      create_mirrors(_debian);
    }
    CliArguments::UpdateMirrors => {
      update_mirrors(_ubuntu);
      update_mirrors(_debian);
    }
    CliArguments::DropOldSnapshots => {
      drop_old_snapshots(_ubuntu);
      drop_old_snapshots(_debian);
    }
    CliArguments::RenameSnapshots => {
      rename_snapshots(_ubuntu);
      rename_snapshots(_debian);
    }
    CliArguments::MakeSnapshots => {
      make_snapshots(_ubuntu);
      make_snapshots(_debian);
    }
    CliArguments::PublishSnapshots => {
      publish_snapshots(_ubuntu);
      publish_snapshots(_debian);
    }
    CliArguments::SwitchPublishedSnapshots => {
      switch_published_snapshots(_ubuntu);
      switch_published_snapshots(_debian);
    }
  }
}

#[derive(Debug, StructOpt)]
#[structopt(
  name = "aptly_manager",
  about = "Eases management of multiple aptly mirrrors."
)]
enum CliArguments {
  #[structopt(name = "create_mirrors")]
  /// Create the required mirrors.
  CreateMirrors,
  #[structopt(name = "update_mirrors")]
  /// Update the mirrors.
  UpdateMirrors,
  #[structopt(name = "drop_old_snapshots")]
  /// Drop the two day old snapshots of the mirrors.
  DropOldSnapshots,
  #[structopt(name = "rename_snapshots")]
  /// Rename the one day old snapshots.
  RenameSnapshots,
  #[structopt(name = "make_snapshots")]
  /// Make new snapshots.
  MakeSnapshots,
  #[structopt(name = "publish_snapshots")]
  /// Make new snapshots.
  PublishSnapshots,
  #[structopt(name = "switch_published_snapshots")]
  /// Make new snapshots.
  SwitchPublishedSnapshots,
}

fn switch_published_snapshots(prefix: Prefix) {
  let switch_published_snapshots_command = Template::new(SWITCH_PUBLISHED_SNAPSHOTS_STRING);
  let mut component_list: Vec<&str> = Vec::with_capacity(5);
  let mut snapshot_list: Vec<String> = Vec::with_capacity(5);
  let snapshot_name = Template::new(SNAPSHOT_NAME_TEMPLATE);
  for distro in prefix.distributions.iter() {
    let component_iterator;
    if let Some(comps) = &distro.components {
      component_iterator = comps.iter();
    } else {
      component_iterator = prefix.components.iter();
    }
    for component in component_iterator {
      component_list.push(component);
      let mut snapshot_name_args = HashMap::<&str, &str>::new();

      snapshot_name_args.insert("prefix", prefix.name);
      snapshot_name_args.insert("distro", distro.name);
      snapshot_name_args.insert("component", component);
      snapshot_name_args.insert("date", "today");

      let snapshot = snapshot_name.render(&snapshot_name_args);
      snapshot_list.push(snapshot.clone());
    }
    let mut command_args = HashMap::<&str, &str>::new();
    command_args.insert("prefix", prefix.name);
    command_args.insert("distro", distro.name);
    let a = component_list.join(",");
    command_args.insert("components", &a);
    let b = snapshot_list.join(" ");
    command_args.insert("snapshot_names", &b);
    println!(
      "{}",
      switch_published_snapshots_command.render(&command_args)
    );
    component_list.clear();
    snapshot_list.clear();
  }
}

fn publish_snapshots(prefix: Prefix) {
  let publish_snapshots_command = Template::new(PUBLISH_SNAPSHOTS_STRING);
  let mut component_list: Vec<&str> = Vec::with_capacity(5);
  let mut snapshot_list: Vec<String> = Vec::with_capacity(5);
  let snapshot_name = Template::new(SNAPSHOT_NAME_TEMPLATE);
  for distro in prefix.distributions.iter() {
    let component_iterator;
    if let Some(comps) = &distro.components {
      component_iterator = comps.iter();
    } else {
      component_iterator = prefix.components.iter();
    }
    for component in component_iterator {
      component_list.push(component);
      let mut snapshot_name_args = HashMap::<&str, &str>::new();

      snapshot_name_args.insert("prefix", prefix.name);
      snapshot_name_args.insert("distro", distro.name);
      snapshot_name_args.insert("component", component);
      snapshot_name_args.insert("date", "today");

      let snapshot = snapshot_name.render(&snapshot_name_args);
      snapshot_list.push(snapshot.clone());
    }
    let mut command_args = HashMap::<&str, &str>::new();
    command_args.insert("prefix", prefix.name);
    command_args.insert("distro", distro.name);
    let a = component_list.join(",");
    command_args.insert("components", &a);
    let b = snapshot_list.join(" ");
    command_args.insert("snapshot_names", &b);
    println!("{}", publish_snapshots_command.render(&command_args));
    component_list.clear();
    snapshot_list.clear();
  }
}

fn make_snapshots(prefix: Prefix) {
  let make_snapshots_command = Template::new(MAKE_SNAPSHOTS_STRING);
  let mirror_name = Template::new(MIRROR_NAME_TEMPLATE);
  let snapshot_name = Template::new(SNAPSHOT_NAME_TEMPLATE);
  for distro in prefix.distributions.iter() {
    let component_iterator;
    if let Some(comps) = &distro.components {
      component_iterator = comps.iter();
    } else {
      component_iterator = prefix.components.iter();
    }
    for component in component_iterator {
      let mut mirror_name_args = HashMap::<&str, &str>::new();
      let mut snapshot_name_args = HashMap::<&str, &str>::new();
      mirror_name_args.insert("prefix", prefix.name);
      mirror_name_args.insert("distro", distro.name);
      mirror_name_args.insert("component", component);

      snapshot_name_args.insert("prefix", prefix.name);
      snapshot_name_args.insert("distro", distro.name);
      snapshot_name_args.insert("component", component);
      snapshot_name_args.insert("date", "today");

      let mut command_args = HashMap::<&str, &str>::new();
      let mirror_name = mirror_name.render(&mirror_name_args);
      let snapshot_name = snapshot_name.render(&snapshot_name_args);
      command_args.insert("mirror_name", &mirror_name);
      command_args.insert("snapshot_name", &snapshot_name);
      println!("{}", make_snapshots_command.render(&command_args));
    }
  }
}

fn rename_snapshots(prefix: Prefix) {
  let rename_snapshots_command = Template::new(RENAME_SNAPSHOTS_STRING);
  let old_snapshot_name = Template::new(SNAPSHOT_NAME_TEMPLATE);
  let new_snapshot_name = Template::new(SNAPSHOT_NAME_TEMPLATE);
  for distro in prefix.distributions.iter() {
    let component_iterator;
    if let Some(comps) = &distro.components {
      component_iterator = comps.iter();
    } else {
      component_iterator = prefix.components.iter();
    }
    for component in component_iterator {
      let mut old_name_args = HashMap::<&str, &str>::new();
      let mut new_name_args = HashMap::<&str, &str>::new();
      old_name_args.insert("prefix", prefix.name);
      old_name_args.insert("distro", distro.name);
      old_name_args.insert("component", component);
      old_name_args.insert("date", "today");

      new_name_args.insert("prefix", prefix.name);
      new_name_args.insert("distro", distro.name);
      new_name_args.insert("component", component);
      new_name_args.insert("date", "yesterday");

      let mut command_args = HashMap::<&str, &str>::new();
      let old_snapshot_name = old_snapshot_name.render(&old_name_args);
      let new_snapshot_name = new_snapshot_name.render(&new_name_args);
      command_args.insert("old_snapshot_name", &old_snapshot_name);
      command_args.insert("new_snapshot_name", &new_snapshot_name);
      println!("{}", rename_snapshots_command.render(&command_args));
    }
  }
}

fn drop_old_snapshots(prefix: Prefix) {
  let drop_snapshots_command = Template::new(DROP_SNAPSHOTS_STRING);
  let snapshot_name = Template::new(SNAPSHOT_NAME_TEMPLATE);
  for distro in prefix.distributions.iter() {
    let component_iterator;
    if let Some(comps) = &distro.components {
      component_iterator = comps.iter();
    } else {
      component_iterator = prefix.components.iter();
    }
    for component in component_iterator {
      let mut name_args = HashMap::<&str, &str>::new();
      name_args.insert("prefix", prefix.name);
      name_args.insert("distro", distro.name);
      name_args.insert("component", component);
      name_args.insert("date", "yesterday");

      let mut command_args = HashMap::<&str, &str>::new();
      let snapshot_name = snapshot_name.render(&name_args);
      command_args.insert("snapshot_name", &snapshot_name);
      println!("{}", drop_snapshots_command.render(&command_args));
    }
  }
}

fn update_mirrors(prefix: Prefix) {
  let update_comand = Template::new(MIRROR_UPDATE_STRING);
  let mirror_name = Template::new(MIRROR_NAME_TEMPLATE);
  for distro in prefix.distributions.iter() {
    let component_iterator;
    if let Some(comps) = &distro.components {
      component_iterator = comps.iter();
    } else {
      component_iterator = prefix.components.iter();
    }
    for component in component_iterator {
      let mut name_args = HashMap::<&str, &str>::new();
      name_args.insert("prefix", prefix.name);
      name_args.insert("distro", distro.name);
      name_args.insert("component", component);

      let mut command_args = HashMap::<&str, &str>::new();
      let mirror_name = mirror_name.render(&name_args);
      command_args.insert("mirror_name", &mirror_name);
      println!("{}", update_comand.render(&command_args));
    }
  }
}

fn create_mirrors(prefix: Prefix) {
  let create_command = Template::new(MIRROR_CREATE_STRING);
  let mirror_name = Template::new(MIRROR_NAME_TEMPLATE);
  for distro in prefix.distributions.iter() {
    let component_iterator;
    if let Some(comps) = &distro.components {
      component_iterator = comps.iter();
    } else {
      component_iterator = prefix.components.iter();
    }
    for component in component_iterator {
      let mut name_args = HashMap::<&str, &str>::new();
      name_args.insert("prefix", prefix.name);
      name_args.insert("distro", distro.name);
      name_args.insert("component", component);

      let mut command_args = HashMap::<&str, &str>::new();
      let options = distro.options.to_command_line_arguments();
      command_args.insert("options", &options);
      command_args.insert("component", component);
      let mname = mirror_name.render(&name_args);
      command_args.insert("mirror_name", &mname);
      if let Some(uri) = distro.uri {
        command_args.insert("url", uri);
      } else {
        command_args.insert("url", prefix.uri);
      }
      println!("{}", create_command.render(&command_args));
    }
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
