mod cli;

use clap::Parser;
use cli::Cli;

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    if !args.project_path.is_dir() {
        return Err(anyhow::anyhow!(
                "The provided path '{}' is not a valid directory.",
                args.project_path.display()
        ));
    }

    

    Ok(())
}
