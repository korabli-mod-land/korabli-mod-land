#[derive(Debug, clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Commands {
  Serve {
    #[arg(long, default_value = "127.0.0.1")]
    host: String,
    #[arg(short, long, default_value_t = 6375)]
    port: u16,
    #[arg(long, default_value_t = true)]
    swagger_ui: bool,
    #[arg(long, default_value = "/api")]
    api_mount: String,
    #[arg(long, default_value = "/api-docs")]
    api_docs_mount: String,
    #[arg(long, default_value = "/swagger_ui")]
    swagger_ui_mount: String,
  },
  Generate {
    #[arg(short, long)]
    output: String,
    #[arg()]
    item: String,
  },
}
