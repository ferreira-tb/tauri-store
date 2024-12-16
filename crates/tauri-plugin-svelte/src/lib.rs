#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub fn hello() {
  println!("Hello, world!");
}
