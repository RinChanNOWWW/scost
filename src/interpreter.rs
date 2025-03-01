use anyhow::anyhow;
use prettytable::Table;
use rustyline::completion::Completer;
use rustyline::error::ReadlineError;
use rustyline::history::FileHistory;
use rustyline::Editor;
use rustyline_derive::Helper;
use rustyline_derive::Highlighter;
use rustyline_derive::Hinter;
use rustyline_derive::Validator;

use crate::command::CommandCopy;
use crate::command::CommandList;
use crate::command::CommandRemove;
use crate::command::CommandSign;
use crate::command::COMMANDS;
use crate::Result;

#[derive(Helper, Highlighter, Validator, Hinter)]
struct CommandHelper;

impl Completer for CommandHelper {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        _: usize,
        _: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let res = COMMANDS
            .iter()
            .filter(|s| s.starts_with(line))
            .map(|s| format!("{} ", s))
            .collect::<Vec<String>>();
        Ok((0, res))
    }
}

pub struct Interpreter {
    rl: Editor<CommandHelper, FileHistory>,
}

impl Interpreter {
    pub fn new() -> Result<Self> {
        let rustyline_config = rustyline::Config::builder()
            .history_ignore_dups(true)?
            .history_ignore_space(true)
            .completion_type(rustyline::CompletionType::List)
            .build();
        let mut rl = Editor::with_config(rustyline_config)?;
        rl.set_helper(Some(CommandHelper));

        Ok(Self { rl })
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            let readline = self.rl.readline(">> ");
            match readline {
                Ok(line) => {
                    self.rl.add_history_entry(line.as_str())?;
                    let cmd_with_args: Vec<String> = line
                        .trim()
                        .split(" ")
                        .filter(|&s| !s.is_empty())
                        .map(String::from)
                        .collect();
                    if cmd_with_args.is_empty() {
                        continue;
                    }
                    match self.interpret(&cmd_with_args).await {
                        Ok(res) => {
                            res.printstd();
                            println!();
                        }
                        Err(e) => {
                            println!("<Error> {}", &e);
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("<Keyboard Interrupted>");
                }
                Err(ReadlineError::Eof) => {
                    println!("<Bye>");
                    break;
                }
                Err(e) => {
                    println!("<Readlinne Error> {}", &e);
                    break;
                }
            }
        }

        Ok(())
    }

    async fn interpret(&self, cmd_with_args: &[String]) -> Result<Table> {
        if cmd_with_args.is_empty() {
            return Err(anyhow!("Command cannot be empty"));
        }
        let cmd = &cmd_with_args[0];
        let args = &cmd_with_args[1..];

        match cmd.as_ref() {
            CommandCopy::COMMAND => CommandCopy::execute(args).await,
            CommandList::COMMAND => CommandList::execute(args).await,
            CommandRemove::COMMAND => CommandRemove::execute(args).await,
            CommandSign::COMMAND => CommandSign::execute(args).await,
            _ => Err(anyhow!("Unkown command: {}", cmd)),
        }
    }
}
