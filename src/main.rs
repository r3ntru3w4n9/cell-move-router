use anyhow::{anyhow, Result};
use cell_move_router::{Args, Chip};
use structopt::StructOpt;

fn main() -> Result<()> {
    let args = Args::from_args();

    let mut chip = Chip::default();

    let infile = &args.infile.ok_or(anyhow!("Input file not specified"))?;
    let outfile = &args.outfile.ok_or(anyhow!("Output file not specified"))?;

    chip.read_file(infile)?;
    chip.write_file(outfile)?;

    Ok(())
}
