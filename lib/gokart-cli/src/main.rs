use clap::{Parser, Subcommand};
use gokart_compile::compile;
use gokart_decay::decay;
use gokart_parse::parse;
use gokart_verify::verify;
use gokart_vm::{GC, VM};
use miette as mt;
use std::{
    fs, io,
    path::{Path, PathBuf},
};
use thiserror::Error;

fn read_file<P: AsRef<Path>>(path: P) -> io::Result<String> {
    fs::read_to_string(path)
}

#[derive(Parser)]
#[command(name = "gokart-cli")]
#[command(version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        #[arg(required = true)]
        file: PathBuf,
    },
}

impl Commands {
    fn execute(&self) -> CliRes<()> {
        match self {
            Commands::Run { file } => {
                let input = read_file(file)?;
                let mut ast = {
                    let res = parse(&input);
                    res.map_err(|e| {
                        // todo
                        CliErr::MTError(mt::Error::from(e).with_source_code(input.clone()))
                    })?
                };
                {
                    let res = verify(&mut ast);
                    res.map_err(|e| {
                        // todo
                        CliErr::MTError(mt::Error::from(e).with_source_code(input.clone()))
                    })?
                }

                let exp = decay(&ast);
                let code = compile(&exp);
                let mut vm = VM::new(code, GC::default());
                vm.run();
                Ok(())
            }
        }
    }
}

// todo: better error display
#[derive(Debug, Error)]
enum CliErr {
    #[error(transparent)]
    IOError(#[from] io::Error),
    #[error("{0}")]
    MTError(mt::Error),
}

type CliRes<T> = Result<T, CliErr>;

fn main() -> CliRes<()> {
    let cli = Cli::parse();
    cli.command.execute()
}
