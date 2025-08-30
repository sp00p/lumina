use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    
    #[arg(required = true, default_value = ".")]
    pub project_path: PathBuf,
}
