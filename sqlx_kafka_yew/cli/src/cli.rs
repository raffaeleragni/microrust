use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Clone, Debug)]
pub struct Args {
    #[clap(value_enum, default_value_t=Env::Local)]
    pub env: Env,
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Env {
    Local,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Command {
    New {
        #[clap(subcommand)]
        object: Object,
    },
}

#[derive(Subcommand, Clone, Debug)]
pub enum Object {
    Tenant { id: u64, name: String },
}
