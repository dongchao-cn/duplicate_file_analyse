use clap::Parser;
use duplicate_file_analyse::{*};
use log::info;
use std::env;

#[derive(Parser)]
struct Args {
    #[arg(short = 'p', value_name = "path")]
    path: String
}

// cargo run -- -p ~/duplicate_file_analyse
// cargo run -- -p ~/duplicate_file_analyse/tests/data
// cargo flamegraph -- -p ~/duplicate_file_analyse
// cargo build --release
// nohup ./duplicate_file_analyse -p /var/services/homes/dongchao/Photos &
fn main() {
    env::set_var("RUST_LOG", "info"); // 设置日志级别
    // env_logger::init();
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let args = Args::parse();

    info!("path: {}", args.path);

    info!("get_all_files...");
    let all_files = get_all_files(&args.path);
    info!("all_files: {}", all_files.len());
    let all_duplicated_files = get_duplicated_files(&all_files);
    info!("all_duplicated_files: {:#?}", all_duplicated_files);
    let (analyse_result, delete_cmd) = analyse_duplicated_floder(all_duplicated_files);
    info!("analyse_result: {:#?}", analyse_result);
    info!("delete_cmd: {}", delete_cmd);
    info!("done!");
}
