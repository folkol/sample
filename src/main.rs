use std::io;

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
    let mut reservoir: Vec<(usize, String)> = io::stdin()
        .lines()
        .take(args.n)
        .filter_map(std::result::Result::ok)
        .enumerate()
        .collect();

    let mut rng = rand::thread_rng();
    for (i, line) in io::stdin().lines().enumerate() {
        let i = i + reservoir.len();
        let j: usize = rng.gen_range(0..i);
        if j < reservoir.len() {
            reservoir[j] = (i, line?);
        }
    }
    reservoir.sort_by_key(|x| x.0);
    for item in reservoir {
        println!("{}", item.1);
    }
    Ok(())
}
