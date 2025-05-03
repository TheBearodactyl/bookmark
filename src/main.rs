mod cli;
mod data;

fn main() -> anyhow::Result<()> {
    cli::cli()
}
