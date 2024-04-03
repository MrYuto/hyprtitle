mod hyprtitle;

use clap::{Parser, Subcommand};
use hyprland::event_listener::EventListener;
use hyprland::shared::WorkspaceType;
use hyprtitle::{print_hyprtitle, Hyprtitle};
use std::process::Command;
use std::sync::{Arc, Mutex};

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
                let hyprtitle = Hyprtitle::new(None);

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

    //TODO: Move this to the struct

    let mut listener = EventListener::new();
    let current_workspace_name: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));

    let handle_workspace_event = {
        let current_workspace_name = Arc::clone(&current_workspace_name);

        move |workspace_type| {
            let mut workspace_name = current_workspace_name.lock().unwrap();

            *workspace_name = match workspace_type {
                WorkspaceType::Regular(name) => Some(name),
                WorkspaceType::Special(name) => name,
            };

            print_hyprtitle(workspace_name.clone());
        }
    };

    listener.add_workspace_change_handler(handle_workspace_event.clone());
    // listener.add_workspace_destroy_handler(handle_workspace_event.clone());

    listener.add_window_close_handler({
        let current_workspace_name = Arc::clone(&current_workspace_name);

        move |_| {
            let workspace_name = current_workspace_name.lock().unwrap();
            print_hyprtitle(workspace_name.clone());
        }
    });

    listener.add_window_open_handler(|_| print_hyprtitle(None));
    listener.add_window_moved_handler(|_| print_hyprtitle(None));
    listener.add_window_title_change_handler(|_| print_hyprtitle(None));
    listener.add_active_window_change_handler(|_| print_hyprtitle(None));

    print_hyprtitle(None);
    listener.start_listener().unwrap();
}
