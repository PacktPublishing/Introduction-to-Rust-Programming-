use std::error; //Has an Error Trait that we will find useful
use std::io::{Read, Write};

use thiserror::*;

#[derive(Error, Debug, PartialEq)]
pub enum BErr {
    #[error("No Target Provided")]
    NoTarget,
    //Format string can reference parts with .0
    #[error("Divide by Zero Error")]
    DivErr,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut it = std::env::args().skip(1);
    let target = it.next().ok_or(BErr::NoTarget)?;
    let mut tot = 0;
    let mut count = 0;
    let mut target = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(target)?;
    for a in it {
        let s = sum_file(&a)?;
        tot += s;
        count += 1;
        writeln!(target, "{: <25}= {}", a, s)?;
    }

    if count == 0 {
        return Err(BErr::DivErr.into());
    }

    writeln!(target, "----------------")?;

    writeln!(target, "AVG = {}", tot / count)?;
    println!("DONE");

    Ok(())
}

pub fn sum_file(fname: &str) -> anyhow::Result<isize> {
    let mut s = String::new();
    std::fs::File::open(fname)?.read_to_string(&mut s)?;
    let mut res = 0;
    for n in s.trim().split('\n') {
        res += n.parse::<isize>()?;
    }
    Ok(res)
}
