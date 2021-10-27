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

use super::Distribution;
use super::Prefix;
use super::PrimaryOptions;

pub fn initalize<'a>() -> Prefix<'a> {
  let mut distributions: Vec<Distribution> = Vec::with_capacity(22);
  distributions.append(&mut initalize_bookworm());
  distributions.append(&mut initalize_bullseye());
  distributions.append(&mut initalize_stable());
  distributions.append(&mut initalize_testing());
  let output: Prefix = Prefix {
    components: vec!["contrib", "main", "non-free"],
    distributions: distributions,
    uri: "http://mirror.pit.teraswitch.com/ubuntu/",
  };
  return output;
}

fn initalize_bookworm<'a>() -> Vec<Distribution<'a>> {
  let mut output: Vec<Distribution> = Vec::with_capacity(5);
  output.push(Distribution {
    components: None,
    name: "bookworm",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "bookworm-backports",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "bookworm-proposed-updates",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "bookworm-proposed-updates",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "bookworm-updates",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  return output;
}

fn initalize_bullseye<'a>() -> Vec<Distribution<'a>> {
  let mut output: Vec<Distribution> = Vec::with_capacity(6);
  output.push(Distribution {
    components: None,
    name: "bullseye",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "bullseye-backports",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "bullseye-backports-sloppy",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "bullseye-proposed-updates",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "bullseye-proposed-updates",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "bullseye-updates",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  return output;
}

fn initalize_stable<'a>() -> Vec<Distribution<'a>> {
  let mut output: Vec<Distribution> = Vec::with_capacity(6);
  output.push(Distribution {
    components: None,
    name: "stable",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "stable-backports",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "stable-backports-sloppy",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "stable-proposed-updates",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "stable-proposed-updates",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "stable-updates",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  return output;
}

fn initalize_testing<'a>() -> Vec<Distribution<'a>> {
  let mut output: Vec<Distribution> = Vec::with_capacity(5);
  output.push(Distribution {
    components: None,
    name: "testing",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "testing-backports",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "testing-proposed-updates",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "testing-proposed-updates",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  output.push(Distribution {
    components: None,
    name: "testing-updates",
    options: PrimaryOptions {
      ..Default::default()
    },
    uri: None,
  });
  return output;
}
