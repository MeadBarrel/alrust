use reedline_repl_rs::*;
use reedline_repl_rs::clap::{Arg, ArgMatches, Command};
use super::message::Message;
use super::printer::*;
use super::build::Optimizator;
use super::error::{OptimizationError, Result};
use std::sync::mpsc::{Sender, channel};
use std::sync::{Arc, Mutex};
use std::thread;


struct Context {
    last_output: Option<String>,
    populations: Arc<Mutex<PopulationsSerializable>>,
    selected_generation: Option<usize>,
    //sender: Sender<Message>,
}

impl Context {
    pub fn population(&self) -> Result<PopulationSerializable> {
        match self.selected_generation {
            Some(x) => match self.populations.lock().unwrap().populations.get(x) {
                    Some(x) => Ok(x.clone()),
                    None => Err(OptimizationError::GenericError("The index is greater than the number of populations".to_string()))
            }
            None => match self.populations.lock().unwrap().populations.last() {
                    Some(x) => Ok(x.clone()),
                    None => Err(OptimizationError::GenericError("No populations yet".to_string()))
                }
            }
    }
}

pub fn run_repl(mut optimizator: Optimizator) {
    let populations = optimizator.populations.clone();
    let (sender, receiver) = channel();
    let handle = thread::spawn(move || optimizator.run(receiver).unwrap());

    let context = Context {
        last_output: None,
        populations,
        selected_generation: None,
    };

    let repl = Repl::new(context)
        .with_name("Alrust genetic optimization")
        .with_stop_on_ctrl_d(true)
        .with_command(
            Command::new("generations"), 
            generations
        )
        .run();

    sender.send(Message::Stop).unwrap();
    handle.join().unwrap();
}

fn select(args: ArgMatches, context_: &mut Context) -> Result<Option<String>> {
    let index: usize = *args
        .get_one::<usize>("index")
        .ok_or(OptimizationError::GenericError("Argument not provided".to_string()))?;
    context_.selected_generation = Some(index);
    Ok(Some("Selected".to_string()))
}

fn select_last(args: ArgMatches, context_: &mut Context) -> Result<Option<String>> {
    context_.selected_generation = None;
    Ok(Some("Tracking last generation".to_string()))
}

fn generations(args: ArgMatches, context_: &mut Context) -> Result<Option<String>> {
    let result = context_.populations.lock().unwrap().populations.len();
    Ok(Some(result.to_string()))
}