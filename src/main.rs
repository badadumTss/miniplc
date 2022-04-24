mod compiler;
mod core;
mod parser;
mod scanner;
mod tests;

use clap::Parser;
use compiler::Compiler;
use std::fs::File;
use std::io::Error;
use std::io::Write;

/// Compiler for the MiniPascal language
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// File to interpret
    #[clap(short, long)]
    file: String,

    /// Output file name
    #[clap(short, long, default_value = "out.c")]
    output: String,

    /// Verbosity of the application
    #[clap(short, parse(from_occurrences))]
    verbose: usize,
}

#[cfg(not(tarpaulin_include))]
fn main() -> Result<(), Error> {
    let args = Args::parse();
    // let mut interpreter = Interpreter::new(args.interactive);

    stderrlog::new()
        .module(module_path!())
        .verbosity(args.verbose)
        .init()
        .unwrap();

    // trace! {"running file {}", args.file};
    // interpreter.run_file(args.file)?;

    use compiler::State;

    let mut compiler = Compiler::new();
    let source = compiler.compile_file(args.file)?;
    match compiler.state {
        State::Sane => {
            let mut output = File::create(args.output)?;
            write!(output, "{}", source)?;
        }
        State::Error => {
            for synerr in compiler.s_errors {
                println!("{}", synerr);
            }
            for cerr in compiler.c_errors {
                println!("{}", cerr);
            }
        }
    }

    Ok(())
}
