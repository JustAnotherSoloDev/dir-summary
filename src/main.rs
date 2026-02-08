use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};

use crate::modules::dir_stat;

mod modules {
    pub mod dir_stat;
}

#[derive(Parser, Debug)]
#[command(name = "Directory Size Calculator")]
#[command(author, version, about, long_about)]
struct CliArgs {
    #[arg(short = 'm', long = "mode", value_enum)]
    mode: Mode,
    #[arg(short = 'd', long = "target-dir",required_if_eq_any([("mode","calculate"),("mode","create")]))]
    target_dir: Option<String>,
    #[arg(short = 'r', long = "report-path", required_if_eq_any([("mode", "Calculate"),("mode","Summary"),("mode", "Print")]))]
    report_path: Option<String>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
enum Mode {
    Create,
    Calculate,
    Summary,
    Print,
}

fn main() -> Result<()> {
    let args = CliArgs::parse();

    match args.mode {
        Mode::Create => {
            let report_path = dir_stat::get_report_name("report")?;
            let target_dir = args
                .target_dir
                .context("source directory path is required in create mode")?;
            dir_stat::create_report(&target_dir, &report_path)?;
            println!("ReportFile={:?}", report_path)
        }
        Mode::Calculate => {
            let report_path = args
                .report_path
                .context("Report file name is required in calculate mode")?;

            let target_dir = args
                .target_dir
                .context("directory path is required in calculate mode")?;

            let (total_size, total_files) =
                dir_stat::directory_summary(&report_path, &target_dir)?;
            let total_size = bytesize::ByteSize(total_size);
            println!("total files={} and size={}", total_files, total_size);
        }
        Mode::Summary => {
            let report_path = args
                .report_path
                .context("Report file name is required in summary mode")?;
            let report_path = PathBuf::from(report_path);
            let report_path = dir_stat::summary(&report_path)?;
            println!("reportPath={}", report_path)
        }
        Mode::Print => {
            let report_path = args
                .report_path
                .context("Report file name is required in summary mode")?;
            let report_path = PathBuf::from(report_path);
            dir_stat::print(&report_path)?;
        }
    }

    return Ok(());
}
