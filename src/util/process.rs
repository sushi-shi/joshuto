use std::process;
use std::sync::mpsc;
use std::thread;

use crate::config::AppMimetypeEntry;
use crate::event::AppEvent;

use nix::{
    sys::wait::{waitpid, WaitStatus},
    unistd::{fork, setsid, ForkResult, Pid},
};

pub fn fork_execute<I, S>(
    entry: &AppMimetypeEntry,
    paths: I,
    event_tx: mpsc::Sender<AppEvent>,
) -> nix::Result<(Pid, thread::JoinHandle<()>)>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let program = String::from(entry.get_command());

    let mut command = process::Command::new(program);
    if entry.get_silent() {
        command.stdout(process::Stdio::null());
        command.stderr(process::Stdio::null());
    }
    command.args(entry.get_args());
    command.args(paths);

    match unsafe { fork()? } {
        ForkResult::Parent { child } => {
            let handle = thread::spawn(move || {
                loop {
                    if let WaitStatus::Exited(_, _) =
                        waitpid(Some(child), None).expect("child handle to exist after fork")
                    {
                        break;
                    }
                }
                let _ = event_tx.send(AppEvent::ChildProcessComplete(child));
            });
            Ok((child, handle))
        }
        ForkResult::Child => {
            setsid().expect("to never fail after forking");
            let code = match command.spawn() {
                Ok(_child) => 0,
                Err(error) => error.raw_os_error().unwrap_or(1),
            };
            std::process::exit(code)
        }
    }
}

pub fn execute_and_wait<I, S>(entry: &AppMimetypeEntry, paths: I) -> std::io::Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let program = String::from(entry.get_command());

    let mut command = process::Command::new(program);
    if entry.get_silent() {
        command.stdout(process::Stdio::null());
        command.stderr(process::Stdio::null());
    }

    command.args(entry.get_args());
    command.args(paths);

    let _ = command.status()?;
    Ok(())
}
