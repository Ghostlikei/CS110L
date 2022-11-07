use addr2line::gimli::LocationListsOffset;
use nix::sys::ptrace;
use nix::sys::signal;
use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use nix::unistd::Pid;
use std::collections::HashMap;
use std::convert::TryInto;
use std::process::Child;
use std::process::Command;
use std::os::unix::process::CommandExt;
use std::fmt;
use std::mem::size_of;

use crate::dwarf_data::DwarfData;

#[derive(Debug)]
pub enum Status {
    /// Indicates inferior stopped. Contains the signal that stopped the process, as well as the
    /// current instruction pointer that it is stopped at.
    Stopped(signal::Signal, usize),

    /// Indicates inferior exited normally. Contains the exit status code.
    Exited(i32),

    /// Indicates the inferior exited due to a signal. Contains the signal that killed the
    /// process.
    Signaled(signal::Signal),
}

/// This function calls ptrace with PTRACE_TRACEME to enable debugging on a process. You should use
/// pre_exec with Command to call this in the child process.
fn child_traceme() -> Result<(), std::io::Error> {
    ptrace::traceme().or(Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "ptrace TRACEME failed",
    )))
}

#[derive(Debug)]
pub struct Inferior {
    child: Child,
    breakpoints: Vec<usize>,
    pub map: HashMap<usize, u8>
}

impl Inferior {
    /// Attempts to start a new inferior process. Returns Some(Inferior) if successful, or None if
    /// an error is encountered.
    pub fn new(target: &str, args: &Vec<String>, bp: &Vec<usize>) -> Option<Inferior> {
        // TODO: implement me!
        // println!(
        //     "Inferior::new not implemented! target={}, args={:?}",
        //     target, args
        // );
        
        let mut cmd: Command = Command::new(target);
        

        unsafe {
            cmd.pre_exec(child_traceme);
        }

        if let Ok(child) = cmd.args(args).spawn() {
            let breakpoints = bp.clone();
            // println!("inf bps: {:?}", &breakpoints);
            let mut inferior = 
            Inferior{
                child: child, 
                breakpoints: breakpoints, 
                map: HashMap::new(),
            };

            if let Ok(status) = inferior.wait(Some(WaitPidFlag::empty())) {
                if let Status::Stopped(sig, addr) = status {
                    if sig != signal::SIGTRAP {
                        None
                    }
                    else {
                        if let Err(_) = inferior.write_bp(&bp){
                            println!("Error writing breakpoints");
                        }
                        Some(inferior)
                    }
                }
                else {
                    None
                }
            }
            else {
                None
            }
        }
        else {
            None
        }
    }

    /// Returns the pid of this inferior.
    pub fn pid(&self) -> Pid {
        nix::unistd::Pid::from_raw(self.child.id() as i32)
    }

    /// Calls waitpid on this inferior and returns a Status to indicate the state of the process
    /// after the waitpid call.
    pub fn wait(&self, options: Option<WaitPidFlag>) -> Result<Status, nix::Error> {
        Ok(match waitpid(self.pid(), options)? {
            WaitStatus::Exited(_pid, exit_code) => Status::Exited(exit_code),
            WaitStatus::Signaled(_pid, signal, _core_dumped) => Status::Signaled(signal),
            WaitStatus::Stopped(_pid, signal) => {
                let regs = ptrace::getregs(self.pid())?;
                Status::Stopped(signal, regs.rip as usize)
            }
            other => panic!("waitpid returned unexpected status: {:?}", other),
        })
    }
    
    pub fn kill(&mut self){
        println!("Killing running inferior (pid {})", self.pid());
        self.child.kill().unwrap();
    }

    pub fn print_backtrace(&self, debug_data: &DwarfData) -> Result<(), nix::Error> {
        let regs = ptrace::getregs(self.pid()).unwrap();
        let mut rip: usize = regs.rip.try_into().unwrap();
        let mut rbp: usize = regs.rbp.try_into().unwrap();
        // println!("%rbp register: {:#x}", rbp);
        // let line_info: LineInfo = get_address_info(rip, debug_data);
        // println!("{}", line_info);
        loop {
            let line_info: LineInfo = get_address_info(rip, debug_data);
            if line_info.info == String::from("NULL") {
                println!("Failed to backtrace!");
                return Ok(()); // return fake ok (actually unknown error)
            }
            println!("{}", line_info);
            if debug_data.get_function_from_addr(rip).unwrap() == String::from("main") {
                break;
            }

            rip = ptrace::read(self.pid(), (rbp + 8) as ptrace::AddressType)? as usize;
            // println!("Before read: {}", rbp);
            rbp = ptrace::read(self.pid(), rbp as ptrace::AddressType)? as usize;
            // println!("After read: {}", rbp);

        }
        
        Ok(()) 
    }

    // #[allow(unused)]
    pub fn write_bp(&mut self, breakpoints: &Vec<usize>) -> Result<(), nix::Error> {
        for bp in breakpoints {
            match self.write_byte(*bp, 0xcc){
                Ok(byte) => {
                    self.map.insert(*bp, byte);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        Ok(())
    }

    pub fn write_byte(&mut self, addr: usize, val: u8) -> Result<u8, nix::Error> {
        let aligned_addr = align_addr_to_word(addr);
        let byte_offset = addr - aligned_addr;
        let word = ptrace::read(self.pid(), aligned_addr as ptrace::AddressType)? as u64;
        let orig_byte = (word >> 8 * byte_offset) & 0xff;
        let masked_word = word & !(0xff << 8 * byte_offset);
        let updated_word = masked_word | ((val as u64) << 8 * byte_offset);
        ptrace::write(
            self.pid(),
            aligned_addr as ptrace::AddressType,
            updated_word as *mut std::ffi::c_void,
        )?;
        Ok(orig_byte as u8)
    }

    
}

fn align_addr_to_word(addr: usize) -> usize {
    addr & (-(size_of::<usize>() as isize) as usize)
}

#[derive(Debug, Default)]
pub struct LineInfo {
    pub func_name: String,
    pub info: String,
}

impl fmt::Display for LineInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.func_name, self.info)
    } 
}

pub fn get_address_info(curr_addr: usize, debug_data: &DwarfData) -> LineInfo {
    let info = match debug_data.get_line_from_addr(curr_addr) {
        Some(line) => format!("{}", line),
        None => {
            String::from("NULL")
        }
    };

    let name = match debug_data.get_function_from_addr(curr_addr) {
        Some(func) => func,
        None => {
            String::from("NULL")
        }
    };
    LineInfo{func_name: name, info: info}
}
