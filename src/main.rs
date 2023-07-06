use std::{io, io::prelude::*};
use std::collections::VecDeque;
use std::io::{BufWriter, StdoutLock};

use clap::Parser;
use rand::*;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Parser)]
struct Args {
    #[clap(default_value_t = false, short, long)]
    debug: bool,
    #[clap(default_value_t = 10)]
    n: usize,
    // seed: Option<usize>,
}

#[cfg(unix)]
fn reset_sigpipe() {
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
        // libc::signal(libc::SIG, libc::SIG_DFL);  TODO sigquit -> dump?
    }
}

pub fn main() -> Result<()> {
    reset_sigpipe();
    // rand::
    let args = Args::parse();
    let mut reservoir: Vec<String> = io::stdin()
        .lines()
        .take(args.n)
        .filter_map(std::result::Result::ok)
        .collect();

    let mut rng = rand::thread_rng();
    for (i, line) in io::stdin().lines().enumerate() {
        let j: usize = rng.gen_range(0..i + reservoir.len());
        if j < reservoir.len() {
            reservoir[j] = line?;
        }
    }
    dbg!(reservoir);
    Ok(())
}
