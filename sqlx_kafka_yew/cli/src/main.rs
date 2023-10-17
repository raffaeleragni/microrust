mod cli;
mod commands;

use clap::Parser;
use cli::Args;
use commands::new_tenant;

fn main() {
    let _ = dotenv::vars();
    let args = Args::parse();
    let env = args.env;
    match args.command {
        cli::Command::New { object } => match object {
            cli::Object::Tenant { id, name } => new_tenant(env, id, name),
        },
    }
}
