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


#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::File;
    use crate::{parse, vm};

    #[test]
    fn test_hello() {
        test("hello-world.lua")
    }

    fn test(filename:&str) {
        let dir = env::current_dir().unwrap();
        let file = File::open(dir.join(format!("lua/{}", filename))).unwrap();
        println!("{:?}", file);
        let proto = parse::load(file);
        vm::ExeState::new().execute(&proto);
    }

}
