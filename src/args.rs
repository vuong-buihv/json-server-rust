use clap::Parser;

#[derive(Parser)]
#[clap(version)]
pub(crate) struct Args {

    pub json_filename: String,

    #[clap(short, long)]
    pub port: Option<String>,
}