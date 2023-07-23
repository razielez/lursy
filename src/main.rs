use crate::args::Args;
use clap::Parser;
use std::fs::File;
use std::path::Path;

mod args;
mod byte_code;
mod lex;
mod parse;
mod value;
mod vm;

fn main() {
    let args = Args::parse();
    let proto = parse::load(File::open(Path::new(&args.f)).unwrap());
    vm::ExeState::new().execute(&proto);
}

#[cfg(test)]
mod tests {
    use crate::{parse, vm};
    use std::env;
    use std::fs::File;

    #[test]
    fn test_hello() {
        test("hello-world.lua")
    }

    fn test(filename: &str) {
        let dir = env::current_dir().unwrap();
        let file = File::open(dir.join(format!("lua/{}", filename))).unwrap();
        println!("{:?}", file);
        let proto = parse::load(file);
        vm::ExeState::new().execute(&proto);
    }
}
