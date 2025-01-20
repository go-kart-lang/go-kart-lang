use clap::Parser;
use gokart_core::OpCode;
use gokart_serde::Deserialize;
use gokart_vm::{ConstantFolding, DeadCodeElimination, Optimization, TailCallOptimization, VM};
use std::{fs::File, io::BufReader, path::PathBuf};

#[derive(Parser)]
#[command(name = "gokart")]
#[command(version = "1.0")]
struct Cli {
    file: PathBuf,
}

type CliRes<T> = Result<T, ()>;

impl Cli {
    fn execute(&self) -> CliRes<()> {
        let file = File::open(&self.file).map_err(|e| {
            eprintln!("[ERROR]: unable to open input file");
            eprintln!("{e}");
        })?;
        let mut reader = BufReader::new(file);

        let code = Vec::<OpCode>::deserialize(&mut reader).map_err(|e| {
            eprintln!("[ERROR]: unable to deserialize input file");
            eprintln!("{e}");
        })?;

        // println!("{:?}", code);

        let optimizations: Vec<Box<dyn Optimization>> = vec![
            Box::new(TailCallOptimization),
            // Box::new(DeadCodeElimination),
            Box::new(ConstantFolding),
        ];

        let mut vm = VM::new(code, optimizations);

        vm.gc().objects_threshold = 1_000_000;
        vm.gc().bytes_threshold = 8 * 1024 * 1024;

        vm.run();
        vm.cleanup();

        Ok(())
    }
}

fn main() {
    let cli = Cli::parse();
    let _ = cli.execute();
}
