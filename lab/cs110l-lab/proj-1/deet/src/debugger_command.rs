use crate::debugger::Debugger;

pub enum DebuggerCommand {
    Quit,
    Run(Vec<String>),
    Continue,
    Backtrace,
    Break(Option<usize>, String),
}

impl DebuggerCommand {
    pub fn from_tokens(tokens: &Vec<&str>) -> Option<DebuggerCommand> {
        match tokens[0] {
            "q" | "quit" => Some(DebuggerCommand::Quit),
            "r" | "run" => {
                let args = tokens[1..].to_vec();
                Some(DebuggerCommand::Run(
                    args.iter().map(|s| s.to_string()).collect(),
                ))
            }
            "c" | "cont" => {
                Some(DebuggerCommand::Continue)
            }
            "bt" | "back" | "backtrace" => {
                Some(DebuggerCommand::Backtrace)
            }
            "b" | "break" => {
                let bpoint = tokens[1];
                if bpoint.starts_with("*"){
                    Some(DebuggerCommand::Break(
                        parse_address(&bpoint[1..]),
                        String::new(),
                    ))
                }
                else {
                    Some(DebuggerCommand::Break(
                        None,
                        String::from(bpoint),
                    ))
                }
                

            }

            _ => None,
        }
    }

    
}

fn parse_address(addr: &str) -> Option<usize> {
    let addr_without_0x = if addr.to_lowercase().starts_with("0x") {
        &addr[2..]
    } else {
        &addr
    };
    usize::from_str_radix(addr_without_0x, 16).ok()
}

