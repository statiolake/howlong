extern crate time;

use std::env;
use std::error::Error;
use std::process::{Command, Stdio};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    drop(args.next()); // drop this application name

    let cmd = args.next().ok_or("command not specified".to_string())?;
    let args: Vec<_> = args.collect();

    let start = time::now();
    Command::new(cmd)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait()?;
    let end = time::now();

    eprintln!("");
    eprintln!("==================== PROCESS FINISHED ====================");
    eprintln!("* time elapsed: {} ms", (end - start).num_milliseconds());
    eprintln!("==========================================================");
    eprintln!("");

    Ok(())
}
