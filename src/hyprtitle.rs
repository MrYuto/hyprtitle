use hyprland::data::*;
use hyprland::event_listener::{EventListener, WorkspaceEventData};
use hyprland::prelude::*;
use hyprland::shared::{WorkspaceId, WorkspaceType};
use serde_json::json;
use std::cell::RefCell;
use std::rc::Rc;

const WORKSPACE_ICON: &str = " ";
const GROUPED_WINDOWS_ICON: &str = " ";
const NORMAL_WINDOW_ICON: &str = " ";
const XWAYLAND_WINDOW_ICON: &str = " ";
const PINNED_WINDOW_ICON: &str = " ";
const FLOATING_WINDOW_ICON: &str = "󰨦 ";
const FULLSCREEN_WINDOW_ICON: &str = "󰊓 ";
const MAXIMIZED_WINDOW_ICON: &str = " ";
const WINDOW_CLASS_ICON: &str = " ";
const WINDOW_SIZE_ICON: &str = "󰳂 ";
const WINDOW_POSITION_ICON: &str = " ";

#[derive(Clone, Debug)]
pub struct WorkspaceInfo {
    pub id: Option<WorkspaceId>,
    pub name: Option<String>,
}

impl Default for WorkspaceInfo {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct Hyprtitle {
    pub active_window: Option<Client>,
    pub workspace_info: WorkspaceInfo,
}

impl Default for Hyprtitle {
    fn default() -> Self {
        Self {
            active_window: Default::default(),
            workspace_info: Default::default(),
        }
    }
}

impl Hyprtitle {
    pub fn new() -> Self {
        let mut hyprtitle = Hyprtitle::default();
        hyprtitle.update(None);
        hyprtitle
    }

    pub fn update(&mut self, workspace_name: Option<String>) -> &Self {
        let active_window = Client::get_active().unwrap();
        let mut workspaces = Workspaces::get().unwrap().to_vec().into_iter();

        let mut workspace = if let Some(name) = workspace_name {
            workspaces.clone().find(|workspace| workspace.name == name)
        } else {
            None
        };

        if let Some(active_window) = &active_window {
            self.workspace_info = WorkspaceInfo {
                id: Some(active_window.workspace.id),
                name: Some(active_window.workspace.name.clone()),
            };
        }

        if workspace.is_none() || active_window.is_some() {
            workspace = if let Some(id) = self.workspace_info.id {
                workspaces.find(|workspace| workspace.id == id)
            } else if let Some(name) = self.workspace_info.name.as_ref() {
                workspaces.find(|workspace| &workspace.name == name)
            } else {
                None
            };
        }

        if let Some(workspace) = workspace {
            self.workspace_info = WorkspaceInfo {
                id: Some(workspace.id),
                name: Some(workspace.name),
            };
        }

        self.active_window = active_window;
        self
    }

    pub fn print(&self) {
        let workspace_id_text = self.workspace_info.id.unwrap_or(0).to_string();
        let workspace_text = self
            .workspace_info
            .name
            .as_ref()
            .unwrap_or(&workspace_id_text);

        let workspace = WORKSPACE_ICON.to_string() + workspace_text;

        let mut title = String::new();
        let mut class = String::new();
        let mut size = String::new();
        let mut position = String::new();
        let mut grouped_windows = String::new();

        if let Some(active_window) = self.active_window.as_ref() {
            let grouped_window_count = active_window.grouped.len();
            if grouped_window_count > 0 {
                grouped_windows.push_str(format!(" {GROUPED_WINDOWS_ICON}").as_str());

                if grouped_window_count > 1 {
                    let index = active_window
                        .grouped
                        .iter()
                        .position(|address| **address == active_window.address)
                        .unwrap_or(0)
                        + 1;

                    grouped_windows
                        .push_str(format!("{}/{}", index, grouped_window_count).as_str());
                }
            }

            let mut title_icon = NORMAL_WINDOW_ICON;

            if active_window.pinned {
                title_icon = PINNED_WINDOW_ICON
            } else if active_window.floating {
                title_icon = FLOATING_WINDOW_ICON
            } else if active_window.fullscreen == FullscreenMode::Maximized {
                title_icon = MAXIMIZED_WINDOW_ICON
            } else if active_window.fullscreen_client == FullscreenMode::Fullscreen {
                title_icon = FULLSCREEN_WINDOW_ICON
            } else if active_window.xwayland {
                title_icon = XWAYLAND_WINDOW_ICON
            }

            title = title_icon.to_string() + active_window.title.as_ref();
            class = WINDOW_CLASS_ICON.to_string() + active_window.class.as_ref();

            position = WINDOW_POSITION_ICON.to_string()
                + format!("{}x{}", active_window.at.0, active_window.at.1).as_ref();

            size = WINDOW_SIZE_ICON.to_string()
                + format!("{}x{}", active_window.size.0, active_window.size.1).as_ref();
        }

        let data = json!({
        "alt": "",
        "class": "",
        "percentage": 0,
        "tooltip": format!("{class}\n{position} {size}\n{title}").trim(),
        "text": format!("{workspace}{grouped_windows} {title}",).trim(),
        });

        println!("{data}");
    }

    pub fn start(self) {
        let mut listener = EventListener::new();
        let hyprtitle: Rc<RefCell<_>> = Rc::new(RefCell::new(self));

        let workspace_handler = {
            let hyprtitle = hyprtitle.clone();

            move |workspace: WorkspaceEventData| {
                let mut hyprtitle = hyprtitle.borrow_mut();
                let workspace_name = match workspace.name {
                    WorkspaceType::Regular(name) => Some(name),
                    WorkspaceType::Special(name) => name,
                };

                hyprtitle.update(workspace_name).print();
            }
        };

        listener.add_workspace_changed_handler(workspace_handler.clone());

        macro_rules! window_handler {
            ($hyprtitle:expr) => {{
                let hyprtitle = hyprtitle.clone();
                move |_| {
                    let mut hyprtitle = hyprtitle.borrow_mut();
                    hyprtitle.update(None).print();
                }
            }};
        }

        listener.add_active_window_changed_handler(window_handler!(hyprtitle));
        listener.add_float_state_changed_handler(window_handler!(hyprtitle));
        listener.add_fullscreen_state_changed_handler(window_handler!(hyprtitle));
        listener.add_group_toggled_handler(window_handler!(hyprtitle));
        listener.add_window_closed_handler(window_handler!(hyprtitle));
        listener.add_window_moved_handler(window_handler!(hyprtitle));
        listener.add_window_moved_into_group_handler(window_handler!(hyprtitle));
        listener.add_window_moved_out_of_group_handler(window_handler!(hyprtitle));
        listener.add_window_opened_handler(window_handler!(hyprtitle));
        listener.add_window_pinned_handler(window_handler!(hyprtitle));
        listener.add_window_title_changed_handler(window_handler!(hyprtitle));

        hyprtitle.borrow_mut().update(None).print();
        listener.start_listener().unwrap();
    }
}
