use clap::Parser;
use duplicate_file_analyse::{*};

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

    let all_files = get_all_files(&args.path);
    let all_duplicated_files = get_duplicated_files(&all_files);
    let analyse_result = analyse_duplicated_floder(all_duplicated_files);
    dbg!(analyse_result);
}
