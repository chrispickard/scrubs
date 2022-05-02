use scrubs::{Tmux, TmuxError};
use std::process::exit;

use clap::Parser;

/// Simplest tmux session manager you'll ever see
#[derive(Parser, Debug)]
#[clap(name="umux", author, version, about, long_about = None)]
struct Cli {
    /// Name of the session
    #[clap(default_value_t = String::from("work"))]
    session_name: String,
}

fn main() {
    let cli = Cli::parse();
    let session_name = cli.session_name;
    let has_session = match Tmux::has_session(&session_name) {
        Ok(has_session) => has_session,
        Err(e) => match e {
            TmuxError::ServerNotStarted() => false,
            TmuxError::SocketFileNotFound() => false,
            TmuxError::UnknownError(e) => {
                eprintln!("{}", e);
                exit(1);
            }
        },
    };
    if !has_session {
        if let Err(e) = Tmux::new_session(&session_name) {
            eprintln!("{}", e);
            exit(1);
        }
        if let Err(e) = Tmux::split_vertical() {
            eprintln!("{}", e);
            exit(1);
        }
    }

    if let Err(e) = Tmux::attach_session(&session_name) {
        eprintln!("{}", e);
        exit(1);
    }
}
