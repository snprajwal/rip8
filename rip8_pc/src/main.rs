use std::{env, fs::File, io::Read};

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err("Usage: rip8 /path/to/rom");
    }

    let mut rom = File::open(&args[1]).expect("failed to open ROM");
    let mut buf = Vec::default();
    rom.read_to_end(&mut buf).expect("failed to read ROM");

    rip8_pc::run(&buf);
    Ok(())
}
