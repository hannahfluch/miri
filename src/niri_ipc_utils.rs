use niri_ipc::socket::Socket;
use niri_ipc::{Request, Response, Window, Workspace, state::EventStreamState};

pub fn get_focused_window(event_state: &EventStreamState) -> Option<&Window> {
    event_state.windows.windows.values().find(|window| window.is_focused)
}

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

pub fn get_focused_window_id(action_socket: &mut Socket) -> Option<u64> {
    let reply = action_socket.send(Request::FocusedWindow).ok()?;
    let response = reply.ok()?;
    match response {
        Response::FocusedWindow(Some(window)) => Some(window.id),
        _ => None,
    }
}
