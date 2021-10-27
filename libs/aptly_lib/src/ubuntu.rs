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

use super::Architectures;
use super::Distribution;
use super::Prefix;
use super::PrimaryOptions;

pub fn initalize<'a>() -> Prefix<'a> {
  let mut distributions: Vec<Distribution> = Vec::with_capacity(25);
  distributions.append(&mut initalize_focal());
  distributions.append(&mut initalize_groovy());
  distributions.append(&mut initalize_hirsute());
  distributions.append(&mut initalize_impish());
  distributions.append(&mut initalize_jammy());
  let output: Prefix = Prefix {
    components: vec!["main", "multiverse", "restricted", "universe"],
    distributions: distributions,
    uri: "http://mirror.pit.teraswitch.com/ubuntu/",
  };
  return output;
}

fn initalize_focal<'a>() -> Vec<Distribution<'a>> {
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

fn initalize_groovy<'a>() -> Vec<Distribution<'a>> {
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

fn initalize_hirsute<'a>() -> Vec<Distribution<'a>> {
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

fn initalize_impish<'a>() -> Vec<Distribution<'a>> {
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

fn initalize_jammy<'a>() -> Vec<Distribution<'a>> {
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
