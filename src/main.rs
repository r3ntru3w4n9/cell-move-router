use anyhow::Result;
use cell_move_router::{Args, Chip};
use structopt::StructOpt;

fn main() -> Result<()> {
    let args = Args::from_args();

    let mut chip = Chip::default();

    let read_result = match args.infile {
        Some(infile) => chip.read_file(&infile),
        None => panic!("No input file specified"),
    };

    if let Err(e) = read_result {
        return Err(e);
    }

    Ok(())
}
