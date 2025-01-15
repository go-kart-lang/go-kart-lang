use clap::Parser;
use gokart_core::OpCode;
use gokart_serde::Deserialize;
use gokart_vm::{GC, VM};
use std::{fs::File, path::PathBuf};

#[derive(Parser)]
#[command(name = "gokart")]
#[command(version = "1.0")]
struct Cli {
    file: PathBuf,
}

type CliRes<T> = Result<T, ()>;

impl Cli {
    fn execute(&self) -> CliRes<()> {
        let mut file = File::create(&self.file).map_err(|e| {
            eprintln!("[ERROR]: unable to open input file");
            eprintln!("{e}");
        })?;

        let code = Vec::<OpCode>::deserialize(&mut file).map_err(|e| {
            eprintln!("[ERROR]: unable to deserialize input file");
            eprintln!("{e}");
        })?;

        let mut vm = VM::new(code, GC::default());
        vm.run();

        Ok(())
    }
}

fn main() -> CliRes<()> {
    let cli = Cli::parse();
    cli.execute()
}
