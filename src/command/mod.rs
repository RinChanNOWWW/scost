use std::marker::PhantomData;

use anyhow::anyhow;
use prettytable::Table;

use crate::bucket::BucketPtr;
use crate::GlobalInstance;
use crate::Result;

mod copy;
mod list;
mod remove;
mod sign;

pub trait Command {
    const COMMAND: &'static str;
    const NUM_ARGS: usize;
    const HELP: &'static str;

    async fn execute(buckets: &[BucketPtr], path: &str) -> Result<Table>;
}

pub struct CommandHelper<C> {
    _c: PhantomData<C>,
}

impl<C: Command> CommandHelper<C> {
    pub const COMMAND: &'static str = C::COMMAND;

    /// Parse and execute the command.
    ///
    /// A command's `args` always starts with a list of bucket aliases,
    /// and the last argument is a path.
    pub async fn execute(args: &[String]) -> Result<Table> {
        if args.len() != C::NUM_ARGS {
            return Err(anyhow!(
                "Command {} requires {} argument(s), but got {}. Usage: {}",
                C::COMMAND,
                C::NUM_ARGS,
                args.len(),
                C::HELP
            ));
        }

        let instance = GlobalInstance::instance();

        let mut buckets = Vec::with_capacity(C::NUM_ARGS - 1);
        for b in args[0..C::NUM_ARGS - 1].iter() {
            if b == "*" {
                buckets.extend(instance.get_buckets());
            } else {
                let op = instance
                    .get_bucket(b)
                    .ok_or(anyhow!("Bucket '{}' not found", b))?;
                buckets.push(op);
            }
        }
        let path = &args.last().unwrap();

        C::execute(&buckets, path).await
    }
}

pub type CommandCopy = CommandHelper<copy::Copy>;
pub type CommandList = CommandHelper<list::List>;
pub type CommandRemove = CommandHelper<remove::Remove>;
pub type CommandSign = CommandHelper<sign::Sign>;

pub const COMMANDS: [&str; 4] = [
    CommandCopy::COMMAND,
    CommandList::COMMAND,
    CommandRemove::COMMAND,
    CommandSign::COMMAND,
];
