// SPDX-License-Identifier: GPL-3.0-or-later

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    start: bool,
}

fn main() {
    let args = Args::parse();

    if args.start {
        println!("Starting...")
    } else {
        println!("Use --start to begin.")
    }
}
