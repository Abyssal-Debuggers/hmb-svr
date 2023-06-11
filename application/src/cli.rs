use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CLI {
    #[arg(short, long, default_value_t = ("./config.json".to_string()))]
    pub config: String,
}