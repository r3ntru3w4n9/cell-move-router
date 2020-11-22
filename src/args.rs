use clap::Clap;

#[derive(Clap, Clone, Default, Debug)]
pub struct Args {
    // input file name
    #[clap(short, long)]
    pub infile: String,

    // output file name
    #[clap(short, long)]
    pub outfile: String,

    // time limit in seconds
    #[clap(short, long)]
    pub sec: Option<usize>,

    // time limit in minutes
    #[clap(short, long)]
    pub min: Option<usize>,

    // time limit in hours
    #[clap(short, long)]
    pub hr: Option<usize>,

    // move cells
    #[clap(short, long)]
    pub cell: bool,

    // route nets
    #[clap(short, long)]
    pub net: bool,
}
