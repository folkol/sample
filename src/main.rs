use std::io;

use clap::Parser;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

/// Print a sample of the lines from stdin (https://en.wikipedia.org/wiki/Reservoir_sampling)
#[derive(Parser)]
struct Args {
    #[clap(default_value_t = 10, short, long = "sample-size")]
    n: usize,
    /// Seed the PRNG with this value for reproducible results
    #[clap(short, long)]
    seed: Option<u64>,
}
#[cfg(unix)]
fn reset_sigpipe() {
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
}

pub fn main() -> Result<()> {
    reset_sigpipe();
    let args = Args::parse();
    if let Some(seed) = args.seed {
        fastrand::seed(seed);
    }
    let mut reservoir = reservoir_sampling_r(args.n)?;
    reservoir.sort_by_key(|x| x.0);
    for item in reservoir {
        println!("{}", item.1);
    }
    Ok(())
}

fn reservoir_sampling_r(n: usize) -> Result<Vec<(usize, String)>> {
    let mut reservoir: Vec<(usize, String)> = io::stdin()
        .lines()
        .take(n)
        .filter_map(std::result::Result::ok)
        .enumerate()
        .collect();

    let lines = io::stdin().lines().enumerate().map(|(x, y)| (x + n, y));
    for (i, line) in lines {
        let j = fastrand::usize(..i);
        if j < reservoir.len() {
            reservoir[j] = (i, line?);
        }
    }
    Ok(reservoir)
}
