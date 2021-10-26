fn main() {
  println!("Hello, world!");
}

struct DefaultOptions {
  architectures: Vec<String>,
  installer: bool,
  sources: bool,
  udebs: bool,
}

struct Distribution {
  architectures: Vec<String>,
  components: Vec<String>,
  installer: bool,
  sources: bool,
  udebs: bool,
  uri: String,
}

struct Prefix {
  architectures: Vec<String>,
  components: Vec<String>,
  distributions: Vec<String>,
  installer: bool,
  sources: bool,
  udebs: bool,
  uri: String,
}
