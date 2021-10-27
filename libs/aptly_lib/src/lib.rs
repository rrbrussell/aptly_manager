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

pub mod debian;

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
#[derive(Debug, strum_macros::EnumString, strum_macros::Display)]
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
  pub uri: &'a str,
}

pub fn initalize_ubuntu<'a>() -> Prefix<'a> {
  let mut distributions: Vec<Distribution> = Vec::with_capacity(25);
  distributions.append(&mut initalize_ubuntu_focal());
  distributions.append(&mut initalize_ubuntu_groovy());
  distributions.append(&mut initalize_ubuntu_hirsute());
  distributions.append(&mut initalize_ubuntu_impish());
  distributions.append(&mut initalize_ubuntu_jammy());
  let output: Prefix = Prefix {
    components: vec!["main", "multiverse", "restricted", "universe"],
    distributions: distributions,
    uri: "http://mirror.pit.teraswitch.com/ubuntu/",
  };
  return output;
}

fn initalize_ubuntu_focal<'a>() -> Vec<Distribution<'a>> {
  let mut output: Vec<Distribution> = Vec::with_capacity(5);
  output.push(Distribution {
    components: None,
    name: "focal",
    options: PrimaryOptions {
      architectures: vec![Architectures::Amd64, Architectures::I386],
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "focal-backports",
    options: PrimaryOptions {
      architectures: vec![Architectures::Amd64, Architectures::I386],
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "focal-proposed",
    options: PrimaryOptions {
      architectures: vec![Architectures::Amd64, Architectures::I386],
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "focal-security",
    options: PrimaryOptions {
      architectures: vec![Architectures::Amd64, Architectures::I386],
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "focal-updates",
    options: PrimaryOptions {
      architectures: vec![Architectures::Amd64, Architectures::I386],
      ..Default::default()
    },
    uri: None,
  });
  return output;
}

fn initalize_ubuntu_groovy<'a>() -> Vec<Distribution<'a>> {
  let mut output: Vec<Distribution> = Vec::with_capacity(5);
  output.push(Distribution {
    components: None,
    name: "groovy",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "groovy-backports",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "groovy-proposed",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "groovy-security",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "groovy-updates",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  return output;
}

fn initalize_ubuntu_hirsute<'a>() -> Vec<Distribution<'a>> {
  let mut output: Vec<Distribution> = Vec::with_capacity(5);
  output.push(Distribution {
    components: None,
    name: "hirsute",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "hirsute-backports",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "hirsute-proposed",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "hirsute-security",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "hirsute-updates",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  return output;
}

fn initalize_ubuntu_impish<'a>() -> Vec<Distribution<'a>> {
  let mut output: Vec<Distribution> = Vec::with_capacity(5);
  output.push(Distribution {
    components: None,
    name: "impish",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "impish-backports",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "impish-proposed",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "impish-security",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "impish-updates",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  return output;
}

fn initalize_ubuntu_jammy<'a>() -> Vec<Distribution<'a>> {
  let mut output: Vec<Distribution> = Vec::with_capacity(5);
  output.push(Distribution {
    components: None,
    name: "jammy",
    options: PrimaryOptions {
      architectures: vec![Architectures::Amd64, Architectures::I386],
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "jammy-backports",
    options: PrimaryOptions {
      architectures: vec![Architectures::Amd64, Architectures::I386],
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "jammy-proposed",
    options: PrimaryOptions {
      architectures: vec![Architectures::Amd64, Architectures::I386],
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "jammy-security",
    options: PrimaryOptions {
      architectures: vec![Architectures::Amd64, Architectures::I386],
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "jammy-updates",
    options: PrimaryOptions {
      architectures: vec![Architectures::Amd64, Architectures::I386],
      ..Default::default()
    },
    uri: None,
  });
  return output;
}
