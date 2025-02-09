use std::path::PathBuf;
use structopt::StructOpt;

mod chip8;
use chip8::Chip8;
use std::{thread, time};

use std::time::Duration;

#[derive(Debug, StructOpt)]
#[structopt(name = "Example", about = "CHIP8-rs options")]
struct Opt {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
    debug: bool,

    /// Set speed
    // we don't want to name it "speed", need to look smart
    #[structopt(short = "c", long = "clock", default_value = "10")]
    speed: f64,

    /// Input file
    #[structopt(short = "r", long = "rom", parse(from_os_str))]
    rom: PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    println!("{:?}", opt);

    let mut chip8: Chip8 = Chip8::new();
    //chip8.debug();

    // TODO: make it so that the file is taken as a positional argument
    chip8.load_rom(opt.rom);


    loop {
        chip8.Cycle();
        chip8.pretty_print_video();
        thread::sleep(time::Duration::from_millis(10));
    }

}

