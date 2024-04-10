mod hyprtitle;
use clap::{Parser, Subcommand};
use hyprtitle::Hyprtitle;
use std::process::Command;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Copy {
        #[arg(short, long)]
        title: bool,
        #[arg(short, long)]
        class: bool,
        #[arg(short, long)]
        size: bool,
        #[arg(short, long)]
        position: bool,
    },
}

fn copy_to_clipboard(value: &str) {
    Command::new("/usr/bin/wl-copy")
        .arg(value)
        .output()
        .expect("failed to copy value to clipboard.");
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Copy {
                size,
                title,
                class,
                position,
            } => {
                let hyprtitle = Hyprtitle::new();

                if let Some(active_window) = hyprtitle.active_window {
                    if title {
                        copy_to_clipboard(&active_window.title);
                    }
                    if class {
                        copy_to_clipboard(&active_window.class);
                    }
                    if size {
                        copy_to_clipboard(&format!(
                            "{} {}",
                            active_window.size.0, active_window.size.1
                        ));
                    }
                    if position {
                        copy_to_clipboard(&format!(
                            "{} {}",
                            active_window.at.0, active_window.at.1
                        ));
                    }
                }
            }
        }
        return;
    }

    Hyprtitle::new().start()
}
