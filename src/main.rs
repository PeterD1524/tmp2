use std::os::unix::process::CommandExt;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    home: Option<std::ffi::OsString>,
    #[arg(long)]
    shell: Option<std::ffi::OsString>,
    #[arg(long)]
    env_clear: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let home = cli.home.as_deref();
    if let Some(home) = home {
        std::env::set_current_dir(home)?;
    }
    if let Some(shell) = cli.shell.as_deref() {
        let mut command = std::process::Command::new(shell);
        if cli.env_clear {
            command.env_clear();
        }
        if let Some(home) = home {
            command.env("HOME", home);
        }
        Err(command.exec())?;
    }
    Ok(())
}
