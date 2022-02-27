use clap::Parser;

#[derive(Parser)]
#[clap(version)]
pub(crate) struct Args {

    /// JSON file to load
    pub json_filename: String,

    /// Host
    #[clap(short, long)]
    pub host: Option<String>,

    /// Port
    #[clap(short, long)]
    pub port: Option<String>,
}