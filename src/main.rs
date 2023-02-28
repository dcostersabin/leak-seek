extern crate clap;
extern crate walkdir;
mod commit_info;
mod file_pipeline;
mod filename;
mod fragment;
mod git_added;
mod git_diff;
mod git_logs;
mod git_pipeline;
mod leaks;
mod match_pipeline;
mod rules;
mod utils;

use clap::{Args, Parser, Subcommand};
use commit_info::CommitInfo;
use file_pipeline::FilePipeline;
use filename::get_file_name;
use fragment::Fragment;
use git_added::get_added;
use git_diff::get_diff_blocks;
use git_logs::Logs;
use git_pipeline::GitPipeline;
use leaks::Leak;
use match_pipeline::MatchPipeline;
use rules::get_rules;
use rules::get_stop_words;
use rules::Rule;
use serde_json::to_string;
use shannon_entropy::shannon_entropy;
use utils::get_fragment;

#[derive(Parser)]
#[command(author,version,about,long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long)]
    git: bool,
}

#[derive(Subcommand)]
enum Commands {
    Detect(DetectArgs),
}

#[derive(Args)]
struct DetectArgs {
    path: Option<String>,
}

fn print_json(data: &Vec<Leak>) {
    let json_data = to_string(data);
    match json_data {
        Ok(data) => {
            println!("{}", data);
        }
        Err(_) => {}
    }
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Detect(path) => {
            let dir_path = path.path.as_ref().unwrap();

            if !cli.git {
                let mut file_obj = FilePipeline::new(dir_path);
                file_obj.start();
                print_json(&file_obj.data);
            } else {
                let mut git_obj = GitPipeline::new(dir_path);
                git_obj.start();
                print_json(&git_obj.data);
            }
        }
    }
}
