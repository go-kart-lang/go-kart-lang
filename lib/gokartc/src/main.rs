use clap::Parser;
use gokart_compile::compile;
use gokart_decay::decay;
use gokart_parse::parse;
use gokart_serde::Serialize;
use gokart_verify::verify;
use std::{
    fs::{self, File},
    io::BufWriter,
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

impl Cli {
    fn execute(&self) -> Result<(), miette::Result<()>> {
        let input = fs::read_to_string(&self.file).map_err(|e| {
            eprintln!("[ERROR]: unable to read input file");
            eprintln!("{e}");
            Ok(())
        })?;

        let ast = parse(&input)
            .map_err(|e| Err(miette::Error::from(e).with_source_code(input.clone())))?;

        verify(&ast).map_err(|e| Err(miette::Error::from(e).with_source_code(input.clone())))?;

        let exp = decay(&ast);
        let code = compile(&exp);

        let file_bin = self.file.with_extension("bin");
        let output = match &self.output {
            Some(path) => Ok(path.as_os_str()),
            None => match file_bin.file_name() {
                Some(name) => Ok(name),
                None => {
                    eprintln!("[ERROR]: unable to generate output filename");
                    Err(Ok(()))
                }
            },
        }?;

        let file = File::create(output).map_err(|e| {
            eprintln!("[ERROR]: unable to open output file");
            eprintln!("{e}");
            Ok(())
        })?;
        let mut writer = BufWriter::new(file);

        code.serialize(&mut writer);

        println!("Done");
        Ok(())
    }
}

fn main() -> miette::Result<()> {
    let cli = Cli::parse();
    match cli.execute() {
        Ok(_) => Ok(()),
        Err(e) => e,
    }
}
