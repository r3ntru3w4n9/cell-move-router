use anyhow::Result;
use cell_move_router::{Args, Chip};
use clap::Clap;

fn main() -> Result<()> {
    let args = Args::parse();

    let mut chip = Chip::default();

    chip.read_file(&args.infile)?;
    chip.run(&args)?;
    chip.write_file(&args.outfile)?;

    Ok(())
}
