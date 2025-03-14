#[derive(Debug, clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Commands {
  Serve {
    #[arg(long)]
    host: Option<String>,
    #[arg(short, long)]
    port: Option<u16>,
  },
  Generate {
    #[arg(short, long)]
    output: String,
    #[arg()]
    item: String,
  },
}
