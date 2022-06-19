#![feature(let_else)]

use std::env;

use crate::parser::classfile::ClassFile;

mod parser;
mod stream_reader;

fn main() {
  let path = env::args().skip(1).next().expect("Expected File Name");

  let cf = ClassFile::read(path.clone());
  println!("{:#?}", cf);
}
