use crate::service_state::WorkspaceModes;
use common::Mode;
use niri_ipc::{Workspace, state::EventStreamState};

pub fn window_is_new(window_id: &u64, event_state: &EventStreamState) -> bool {
    !event_state.windows.windows.contains_key(window_id)
}

pub fn get_focused_workspace(event_state: &EventStreamState) -> Option<&Workspace> {
    event_state
        .workspaces
        .workspaces
        .values()
        .find(|workspace| workspace.is_focused)
}

pub fn get_focused_workspace_mode(service_state: &WorkspaceModes, event_state: &EventStreamState) -> Option<Mode> {
    let Some(focused_workspace) = get_focused_workspace(&event_state) else {
        eprintln!("Failed to get focused workspace");
        return None;
    };

    let Some(output) = focused_workspace.output.as_ref() else {
        eprintln!("Focused workspace has no output");
        return None;
    };

    Some(service_state.get_mode(output, focused_workspace.idx))
}
