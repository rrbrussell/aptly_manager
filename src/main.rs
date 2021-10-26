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
