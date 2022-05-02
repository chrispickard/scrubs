use std::fmt::{Display, Formatter};
use std::io::Error;
use std::process::{Command, Output};

#[derive(Debug)]
pub enum TmuxError {
    ServerNotStarted(),
    SocketFileNotFound(),
    UnknownError(String),
}

impl TmuxError {
    pub fn new_with_msg(msg: String) -> Self {
        if msg.contains("no server running") {
            return Self::ServerNotStarted();
        } else if msg.contains("No such file or directory") {
            return Self::SocketFileNotFound();
        }
        Self::UnknownError(msg)
    }
}

impl Display for TmuxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let err_msg = match self {
            TmuxError::ServerNotStarted() => "server not started",
            TmuxError::SocketFileNotFound() => "socket file not found",
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
pub struct Tmux {}

impl Tmux {
    pub fn new_session(session_name: &str) -> Result<Output, TmuxError> {
        let args =
            comma::parse_command(format!("new-session -d -s {}", session_name).as_str()).unwrap();
        Self::command(args)
    }

    pub fn has_session(session_name: &str) -> Result<bool, TmuxError> {
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

    pub fn split_vertical() -> Result<Output, TmuxError> {
        let args = comma::parse_command("split-window -v").unwrap();
        Self::command(args)
    }

    pub fn split_horizontal() -> Result<Output, TmuxError> {
        let args = comma::parse_command("split-window -h").unwrap();
        Self::command(args)
    }

    pub fn attach_session(session_name: &str) -> Result<(), TmuxError> {
        let args =
            comma::parse_command(format!("attach-session -t{}", session_name).as_str()).unwrap();
        if let Ok(mut child) = Command::new("tmux").args(args).spawn() {
            child.wait()?;
        } else {
            return Err(TmuxError::new_with_msg("tmux unable to start".into()));
        }
        Ok(())
    }

    pub fn command(args: Vec<String>) -> Result<Output, TmuxError> {
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
