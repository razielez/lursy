use std::fs::File;
use std::path::Path;
use clap::Parser;
use crate::args::Args;

mod value;
mod vm;
mod parse;
mod byte_code;
mod lex;
mod args;

fn main() {
    let args = Args::parse();
    let proto = parse::load(File::open(Path::new(&args.f)).unwrap());
    vm::ExeState::new().execute(&proto);
}
