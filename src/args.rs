use structopt::StructOpt;

#[derive(StructOpt, Default, Debug)]
pub struct Args {
    // input file name
    #[structopt(short, long)]
    pub infile: Option<String>,

    // output file name
    #[structopt(short, long)]
    pub outfile: Option<String>,
}
