use clap::Parser;
use gokart_compile::compile;
use gokart_decay::decay;
use gokart_parse::parse;
use gokart_serde::Serialize;
use gokart_verify::verify;
use miette as mt;
use std::{
    fs::{self, File},
    path::PathBuf,
};

#[derive(Parser)]
#[command(name = "gokartc")]
#[command(version = "1.0")]
struct Cli {
    file: PathBuf,

    #[arg(short, long)]
    output: Option<PathBuf>,
}

type CliRes<T> = Result<T, ()>;

impl Cli {
    fn execute(&self) -> CliRes<()> {
        let input = fs::read_to_string(&self.file).map_err(|e| {
            eprintln!("[ERROR]: unable to read input file");
            eprintln!("{e}");
        })?;

        let ast = parse(&input).map_err(|e| {
            let report = mt::Error::from(e).with_source_code(input.clone());
            eprintln!("[ERROR]: unable to parse file");
            eprintln!("{report}");
        })?;

        verify(&ast).map_err(|e| {
            let report = mt::Error::from(e).with_source_code(input.clone());
            eprintln!("[ERROR]: unable to verify file");
            eprintln!("{report}");
        })?;

        let exp = decay(&ast);
        let code = compile(&exp);

        let output = match &self.output {
            Some(path) => path,
            None => &self.file.with_extension(".bin"),
        };

        let mut file = File::create(output).map_err(|e| {
            eprintln!("[ERROR]: unable to open output file");
            eprintln!("{e}");
        })?;

        code.serialize(&mut file);

        println!("Done");
        Ok(())
    }
}

fn main() -> CliRes<()> {
    let cli = Cli::parse();
    cli.execute()
}
