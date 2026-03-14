use clap::Parser;
use anyhow::{Context, Result};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}
fn main() -> Result<()>{
    let arg = Cli::parse();
    let content = std::fs::read_to_string(&arg.path).with_context(|| format!("could not read file `{}`", arg.path.display()))?;

    for line in content.lines() {
        if line.contains(&arg.pattern){
            println!("{:?}",line);
        }
    }

    Ok(())
}
