use reedline_repl_rs::*;
use reedline_repl_rs::clap::{Arg, ArgMatches, Command};

fn hello<T>(args: ArgMatches, _context: &mut T) -> Result<Option<String>> {
    Ok(Some(format!("Hello, {}", args.value_of("who").unwrap())))
}

fn new_term<T>(args: ArgMatches, _context: &mut T) -> Result<Option<String>> {
    let mut repl = Repl::new(())
        .with_name("Alrust2")
        .with_stop_on_ctrl_d(true)
        .with_command(
            Command::new("hello")
                .arg(
                    Arg::new("who").required(true)
                ), 
            hello
        );
    repl.run().unwrap();
    Ok(None)
}

pub fn main() {
    let mut repl = Repl::new(())
        .with_name("Alrust")
        .with_stop_on_ctrl_d(true)
        .with_command(
            Command::new("hello")
                .arg(
                    Arg::new("who").required(true)), 
            hello)
        .with_command(
            Command::new("another"),
            new_term);

    repl.run();
}
