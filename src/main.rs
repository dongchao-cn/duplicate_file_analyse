use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short = 'p', value_name = "path")]
    path: String
}

fn main() {
    println!("Hello, world!");
    let args = Args::parse();

    println!("{}", args.path);
}
