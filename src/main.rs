use clap::Parser;
use duplicate_file_analyse::{*};
use log::info;
use std::env;

#[derive(Parser)]
struct Args {
    #[arg(short = 'p', value_name = "path")]
    path: String,

    #[arg(short = 'm', value_name = "mode", value_parser = ["get_duplicated_files", "analyse_duplicated_floder"])]
    mode: String,
}

// cargo run -- -p ~/duplicate_file_analyse
// cargo run -- -p ~/duplicate_file_analyse/tests/data -m get_duplicated_files
// cargo run -- -p ~/duplicate_file_analyse/tests/data -m analyse_duplicated_floder
// cargo flamegraph -- -p ~/duplicate_file_analyse
// cargo build --release
// nohup ./duplicate_file_analyse -p /var/services/homes/dongchao/Photos &
fn main() {
    env::set_var("RUST_LOG", "info"); // 设置日志级别
    // env_logger::init();
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let args = Args::parse();

    info!("path: {}", args.path);
    info!("mode: {}", args.mode);

    let file_name = "duplicated_files.json".to_string();
    if args.mode == "get_duplicated_files" {
        info!("get_all_files...");
        let all_files = get_all_files(&args.path);
        info!("all_files: {}", all_files.len());
        let all_duplicated_files = get_duplicated_files(&all_files);
        serialize(&all_duplicated_files, &file_name);
    
    } else if args.mode == "analyse_duplicated_floder" {
        let all_duplicated_files = deserialize(&file_name);
        info!("all_duplicated_files: {:#?}", all_duplicated_files);
        let analyse_result = analyse_duplicated_floder(all_duplicated_files);
        info!("analyse_result: {:#?}", analyse_result);
        gen_delete_cmd(analyse_result);
    }
    info!("done!");
}
