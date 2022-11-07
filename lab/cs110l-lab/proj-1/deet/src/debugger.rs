use crate::debugger_command::DebuggerCommand;
use crate::dwarf_data;
// use crate::dwarf_data::DwarfData;
use crate::inferior;
use crate::inferior::Inferior;
use crate::inferior::Status;
use crate::inferior::get_address_info;
use libc::exit;
use rustyline::error::ReadlineError;
use rustyline::Editor;

use nix::sys::ptrace;
use nix::sys::signal;
use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};

use crate::dwarf_data::{DwarfData, Error as DwarfError};


pub struct Debugger {
    target: String,
    history_path: String,
    readline: Editor<()>,
    inferior: Option<Inferior>,
    debug_data: DwarfData,
    breakpoints: Vec<usize>,
}

impl Debugger {
    /// Initializes the debugger.
    pub fn new(target: &str) -> Debugger {
        // TODO (milestone 3): initialize the DwarfData
        let debug_data = match DwarfData::from_file(target) {
            Ok(val) => val,
            Err(DwarfError::ErrorOpeningFile) => {
                println!("Could not open file {}", target);
                std::process::exit(1);
            }
            Err(DwarfError::DwarfFormatError(err)) => {
                println!("Could not debugging symbols from {}: {:?}", target, err);
                std::process::exit(1);
            }
        };
        debug_data.print();

        let history_path = format!("{}/.deet_history", std::env::var("HOME").unwrap());
        let mut readline = Editor::<()>::new();
        // Attempt to load history from ~/.deet_history if it exists
        let _ = readline.load_history(&history_path);

        let mut bp: Vec<usize> = Vec::new();
        Debugger {
            target: target.to_string(),
            history_path,
            readline,
            inferior: None,
            debug_data,
            breakpoints: bp,
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.get_next_command() {
                DebuggerCommand::Run(args) => {
                    if self.inferior.is_some() {
                        self.inferior.as_mut().unwrap().kill();
                    }
                    if let Some(inferior) = Inferior::new(&self.target, &args, &self.breakpoints) {
                        // Create the inferior
                        self.inferior = Some(inferior);
                        // TODO (milestone 1): make the inferior run
                        // You may use self.inferior.as_mut().unwrap() to get a mutable reference
                        // to the Inferior object
                        self.resume();

                    } else {
                        println!("Error starting subprocess");
                    }
                    
                }
                DebuggerCommand::Quit => {
                    if self.inferior.is_some() {
                        self.inferior.as_mut().unwrap().kill();
                    }
                    return;
                }

                DebuggerCommand::Continue => {
                    if self.inferior.is_none() {
                        println!("No child process running");
                    }
                    self.resume();
                }

                DebuggerCommand::Backtrace => {
                    self.inferior.as_ref().unwrap().print_backtrace(&self.debug_data).unwrap();
                }

                DebuggerCommand::Break(addr, line) => {
                    match addr {
                        Some(a) => {
                            if self.breakpoints.contains(&a){
                                println!("Breakpoint 0x{:x} already exists!", a);
                            }
                            else{
                                self.breakpoints.push(a);
                                println!("Set breakpoint {} at 0x{:x}", self.breakpoints.len()-1, a);
                            }                  
                        }
                        None => {
                            let trans_bp: usize;
                            trans_bp = match line.parse::<usize>().ok() {
                                Some(l) => {
                                    match self.debug_data.get_addr_for_line(None, l){
                                        Some(a) => {
                                            self.breakpoints.push(a);
                                            println!("Set breakpoint {} at 0x{:x}", self.breakpoints.len()-1, a);
                                            a
                                        }
                                        None => {
                                            println!("Err: {} is not a valid line number", l);
                                            return;
                                        }
                                    }
                                }
                                None => {
                                    match self.debug_data.get_addr_for_function(None, line.as_str()){
                                        Some(a) => {
                                            self.breakpoints.push(a);
                                            println!("Set breakpoint {} at 0x{:x}", self.breakpoints.len()-1, a);
                                            a
                                        }
                                        None => {
                                            println!("Err: {} function doesn't exist", line);
                                            return;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                }
            }
        }
    }

    /// This function prompts the user to enter a command, and continues re-prompting until the user
    /// enters a valid command. It uses DebuggerCommand::from_tokens to do the command parsing.
    ///
    /// You don't need to read, understand, or modify this function.
    fn get_next_command(&mut self) -> DebuggerCommand {
        loop {
            // Print prompt and get next line of user input
            match self.readline.readline("(deet) ") {
                Err(ReadlineError::Interrupted) => {
                    // User pressed ctrl+c. We're going to ignore it
                    println!("Type \"quit\" to exit");
                }
                Err(ReadlineError::Eof) => {
                    // User pressed ctrl+d, which is the equivalent of "quit" for our purposes
                    return DebuggerCommand::Quit;
                }
                Err(err) => {
                    panic!("Unexpected I/O error: {:?}", err);
                }
                Ok(line) => {
                    if line.trim().len() == 0 {
                        continue;
                    }
                    self.readline.add_history_entry(line.as_str());
                    if let Err(err) = self.readline.save_history(&self.history_path) {
                        println!(
                            "Warning: failed to save history file at {}: {}",
                            self.history_path, err
                        );
                    }
                    let tokens: Vec<&str> = line.split_whitespace().collect();
                    if let Some(cmd) = DebuggerCommand::from_tokens(&tokens) {
                        return cmd;
                    } else {
                        println!("Unrecognized command.");
                    }
                }
            }
        }
    }

    pub fn resume(&mut self) {
        let inf = self.inferior.as_mut().unwrap();
        let pid = inf.pid();
        ptrace::cont(pid, None).unwrap();
        if let Ok(status) = inf.wait(Some(WaitPidFlag::empty())) {
            self.handle_stop(status);
        }
        else {
            panic!("Error resuming process");
        }
    }

    fn handle_stop(&mut self, status: Status) {
        match status {
            Status::Exited(sig) => {
                println!("Child Exited (status {})", sig);
                self.inferior = None;
            }

            Status::Signaled(sig) => {
                println!("Child Signaled with status {}", sig);
                self.inferior = None;
            }

            Status::Stopped(sig, regs) => {
                // regs returns current rip of the process
                println!("Child stopped (status {})", sig);
                let stopped_line = get_address_info(regs, &self.debug_data);
                if stopped_line.info == String::from("NULL") {
                    return;
                }
                println!("Stopped at {} ({})", stopped_line.func_name, stopped_line.info);
                let break_addr = regs - 1;
                if self.breakpoints.contains(&break_addr) {
                    println!("Program stopped at breakpoint: 0x{:x} with signal {}", &regs, sig);
                    let inf = self.inferior.as_mut().unwrap();
                    inf.write_byte(break_addr, inf.map.get(&break_addr).unwrap().clone()).unwrap();
                }
            }
        }
    }
}
