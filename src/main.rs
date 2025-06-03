use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(required = true)]
    input: String,

    // optional
    #[clap(long, default_value_t = false)]
    lex: bool,
    #[clap(long, default_value_t = false)]
    parse: bool,
    #[clap(long, default_value_t = false)]
    codegen: bool,
}

fn main() {
    let args = Args::parse();
    if args.lex {
        println!("Lexing input: {}", args.input);
        // Call the lexing function here
    }
    if args.parse {
        println!("Parsing input: {}", args.input);
        // Call the parsing function here
    }
    if args.codegen { 
        println!("Generating code for input: {}", args.input);
        // Call the code generation function here
    }
    if !args.lex && !args.parse && !args.codegen {
        println!("No action specified. Use --lex, --parse, or --codegen.");
    }
}