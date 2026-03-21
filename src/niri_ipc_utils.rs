use niri_ipc::{Window, Workspace, state::EventStreamState};

fn get_focused_workspace(event_state: &EventStreamState) -> Option<&Workspace> {
    event_state
        .workspaces
        .workspaces
        .values()
        .find(|workspace| workspace.is_focused)
}

pub fn get_windows_on_focused_workspace(event_state: &EventStreamState) -> Option<Vec<&Window>> {
    let Some(focused_workspace) = get_focused_workspace(event_state) else {
        eprintln!("Could not get focused workspace");
        return None;
    };
    let workspace_windows: Vec<&Window> = event_state
        .windows
        .windows
        .iter()
        .filter(|(_, window)| window.workspace_id == Some(focused_workspace.id))
        .map(|(_, window)| window)
        .collect();

    Some(workspace_windows)
}
