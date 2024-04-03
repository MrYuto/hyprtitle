use hyprland::data::*;
use hyprland::prelude::*;
use hyprland::shared::WorkspaceId;
use serde_json::json;

#[derive(Clone)]
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

pub struct Hyprtitle {
    pub windows: Option<u16>,
    pub active_window: Option<Client>,
    pub workspace_info: WorkspaceInfo,
}

impl Default for Hyprtitle {
    fn default() -> Self {
        Self {
            windows: Default::default(),
            active_window: Default::default(),
            workspace_info: Default::default(),
        }
    }
}

impl Hyprtitle {
    pub fn new(workspace_name: Option<String>) -> Self {
        let mut hyprtitle = Hyprtitle::default();

        if let Some(name) = workspace_name {
            hyprtitle.workspace_info.name = Some(name)
        }

        hyprtitle.update();
        hyprtitle
    }

    pub fn update(&mut self) {
        let active_window = Client::get_active().unwrap();
        let mut workspace_info = self.workspace_info.clone();

        if let Some(active_window) = &active_window {
            workspace_info = WorkspaceInfo {
                id: Some(active_window.workspace.id),
                name: Some(active_window.workspace.name.clone()),
            };
        }

        let mut workspaces = Workspaces::get().unwrap().to_vec().into_iter();

        let workspace = if let Some(id) = workspace_info.id {
            workspaces.find(|workspace| workspace.id == id)
        } else if let Some(name) = workspace_info.name.as_ref() {
            workspaces.find(|workspace| &workspace.name == name)
        } else {
            None
        };

        let mut windows = None;

        if let Some(workspace) = workspace {
            windows = Some(workspace.windows);
        }

        self.windows = windows;
        self.active_window = active_window;
        self.workspace_info = workspace_info;
    }

    pub fn print(&self) {
        let windows = String::from(" ") + self.windows.unwrap_or(0).to_string().as_ref();
        let workspace_id_text = self.workspace_info.id.unwrap_or(0).to_string();

        let workspace_text = self
            .workspace_info
            .name
            .as_ref()
            .unwrap_or(&workspace_id_text);

        let workspace = String::from(" ") + workspace_text;

        let mut title_icon = " ";
        let mut title_text = "";
        let mut class_text = "";
        let mut size_text = String::new();
        let mut position_text = String::new();

        if let Some(active_window) = self.active_window.as_ref() {
            if active_window.pinned {
                title_icon = " "
            } else if active_window.floating {
                title_icon = "󰨦 "
            }

            title_text = &active_window.title;
            class_text = &active_window.class;
            position_text = format!("{}x{}", active_window.at.0, active_window.at.1);
            size_text = format!("{}x{}", active_window.size.0, active_window.size.1);
        }

        let title = title_icon.to_string() + title_text;
        let class = String::from(" ") + class_text;
        let size = String::from("󰳂 ") + &size_text;
        let position = String::from(" ") + &position_text;

        let data = json!({
        "alt": "",
        "class": "",
        "percentage": 0,
        "tooltip": format!("{class}\n{position} {size}\n{title}"),
        "text": format!("{workspace} {windows} {title}"),
        });

        println!("{data}");
    }
}

pub fn print_hyprtitle(workspace_name: Option<String>) {
    Hyprtitle::new(workspace_name).print();
}
