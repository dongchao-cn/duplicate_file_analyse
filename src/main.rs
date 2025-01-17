use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short = 'p', value_name = "path")]
    path: String
}

// cargo run -- -p ~/duplicate_file_analyse
// cargo run -- -p ~/duplicate_file_analyse/tests/data
// cargo flamegraph -- -p ~/duplicate_file_analyse
fn main() {
    let args = Args::parse();

    println!("args.path: {}", args.path);

    let all_files = duplicate_file_analyse::get_all_files(&args.path);
    let all_duplicated_files = duplicate_file_analyse::get_duplicated_files(&all_files);
    dbg!(all_duplicated_files);
}
