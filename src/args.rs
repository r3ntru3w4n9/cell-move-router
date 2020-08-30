use structopt::StructOpt;

#[derive(StructOpt, Default, Debug)]
pub struct Args {
    // input file name
    #[structopt(short, long)]
    pub infile: Option<String>,

    // output file name
    #[structopt(short, long)]
    pub outfile: Option<String>,

    // time limit in seconds
    #[structopt(short, long)]
    pub sec: Option<usize>,

    // time limit in minutes
    #[structopt(short, long)]
    pub min: Option<usize>,

    // time limit in hours
    #[structopt(short, long)]
    pub hours: Option<usize>,
}
