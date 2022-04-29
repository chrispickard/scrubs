use std::fmt::{Display, Formatter};
use std::io::Error;
use std::process::{exit, Command, Output};

#[derive(Debug)]
enum TmuxError {
    ServerNotStarted(),
    UnknownError(String),
}

impl TmuxError {
    fn new_with_msg(msg: String) -> Self {
        if msg.contains("no server running") {
            return Self::ServerNotStarted();
        }
        Self::UnknownError(msg)
    }
}

impl Display for TmuxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let err_msg = match self {
            TmuxError::ServerNotStarted() => "server not started",
            TmuxError::UnknownError(s) => s,
        };
        write!(f, "{}", err_msg)
    }
}

impl From<Error> for TmuxError {
    fn from(e: Error) -> Self {
        Self::new_with_msg(e.to_string())
    }
}
struct Tmux {}

impl Tmux {
    fn new_session(session_name: &str) -> Result<Output, TmuxError> {
        let args =
            comma::parse_command(format!("new-session -d -s {}", session_name).as_str()).unwrap();
        Self::command(args)
    }

    fn has_session(session_name: &str) -> Result<bool, TmuxError> {
        let args = comma::parse_command("list-sessions -F '#{session_name}'").unwrap();
        let output = Self::command(args)?;
        let output = String::from_utf8_lossy(output.stdout.as_slice());
        for line in output.lines() {
            if line == session_name {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn split_window() -> Result<Output, TmuxError> {
        let args = comma::parse_command("split-window -v").unwrap();
        Self::command(args)
    }

    fn attach_session(session_name: &str) -> Result<(), TmuxError> {
        let args =
            comma::parse_command(format!("attach-session -t{}", session_name).as_str()).unwrap();
        if let Ok(mut child) = Command::new("tmux").args(args).spawn() {
            child.wait()?;
        } else {
            return Err(TmuxError::UnknownError("tmux unable to start".into()));
        }
        Ok(())
    }

    fn command(args: Vec<String>) -> Result<Output, TmuxError> {
        let out = Command::new("tmux").args(args).output()?;
        if !out.status.success() {
            let error = TmuxError::new_with_msg(
                String::from_utf8_lossy(out.stderr.as_slice())
                    .parse()
                    .unwrap(),
            );
            return Err(error);
        }
        Ok(out)
    }
}

fn main() {
    let session_name = "work";
    let has_session = match Tmux::has_session(session_name) {
        Ok(has_session) => has_session,
        Err(e) => match e {
            TmuxError::ServerNotStarted() => false,
            TmuxError::UnknownError(e) => {
                eprintln!("here? {}", e);
                exit(1);
            }
        },
    };
    if !has_session {
        if let Err(e) = Tmux::new_session(session_name) {
            eprintln!("{}", e);
            exit(1);
        }
        if let Err(e) = Tmux::split_window() {
            eprintln!("{}", e);
            exit(1);
        }
    }

    if let Err(e) = Tmux::attach_session(session_name) {
        eprintln!("{}", e);
        exit(1);
    }
}
