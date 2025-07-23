use clap::Parser;
use std::{panic, path::PathBuf};

mod compiler;

use compiler::compile;

#[derive(Debug, Parser)]
#[command(version, about, arg_required_else_help = true)]
struct Cli {
    /// Code file path
    file: PathBuf,

    /// Enable debug-mode
    #[arg(short, long)]
    debug: bool,
}

fn main() -> Result<(), ()> {
    let cli = Cli::parse();

    // コードファイルパス
    let code_path = cli.file;

    let exec_path = compile(&code_path);
    if let Err(err) = exec_path {
        println!("{}", err);
    }

    Ok(())
}
