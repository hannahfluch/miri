use niri_ipc::{Workspace, state::EventStreamState};

pub fn window_is_new(window_id: &u64, event_state: &EventStreamState) -> bool {
    event_state.windows.windows.contains_key(window_id)
}

pub fn get_focused_workspace(event_state: &EventStreamState) -> Option<&Workspace> {
    event_state
        .workspaces
        .workspaces
        .values()
        .find(|workspace| workspace.is_focused)
}
