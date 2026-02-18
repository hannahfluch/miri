use common::{Command, IPCMessage, IPCMessageContainer, MIRI_SOCKET_PATH, MiriAction, MiriGet, WorkspaceModes};
use niri_ipc::state::{EventStreamState, EventStreamStatePart};
use niri_ipc::{Request, socket::Socket};
use service::niri_ipc_utils::get_focused_workspace;
use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

#[derive(Default)]
struct ServiceState {
    workspace_modes: WorkspaceModes,
}

trait CliRunner {
    fn run(
        &self,
        action_socket: Arc<Mutex<Socket>>,
        event_state: Arc<RwLock<EventStreamState>>,
        service_state: Arc<Mutex<ServiceState>>,
    );
}

impl CliRunner for Command {
    fn run(
        &self,
        action_socket: Arc<Mutex<Socket>>,
        event_state: Arc<RwLock<EventStreamState>>,
        service_state: Arc<Mutex<ServiceState>>,
    ) {
        match self {
            Command::Action { action } => action.run(action_socket, event_state, service_state),
            Command::Get { get } => get.run(action_socket, event_state, service_state),
        }
    }
}

impl CliRunner for MiriAction {
    fn run(
        &self,
        _action_socket: Arc<Mutex<Socket>>,
        event_state: Arc<RwLock<EventStreamState>>,
        service_state: Arc<Mutex<ServiceState>>,
    ) {
        match self {
            MiriAction::CycleFocusedWorkspaceMode => {
                println!("[ACTION]: CycleFocusedWorkspaceMode");
                let event_state = event_state.read().expect("Failed to get read lock on event_state");

                let Some(workspace) = get_focused_workspace(&event_state) else {
                    eprintln!("No focused workspace was found");
                    return;
                };

                // workspace is now evaluated has having a value
                println!("focused workspace on {:?}", workspace);

                let Some(output) = workspace.output.as_ref() else {
                    eprintln!("Focused workspace had no output");
                    return;
                };

                println!("focused workspace on {:?}", workspace.output);

                let mut service_state = service_state.lock().expect("Failed to get lock for service state");

                service_state.workspace_modes.cycle_mode(output, workspace.idx);

                println!(
                    "mode {}",
                    service_state.workspace_modes.get_mode(output, workspace.idx).as_str()
                )
            }
            MiriAction::Spawn => {
                println!("[ACTION]: Spawn");
            }
        }
    }
}

impl CliRunner for MiriGet {
    fn run(
        &self,
        _action_socket: Arc<Mutex<Socket>>,
        _event_state: Arc<RwLock<EventStreamState>>,
        _service_state: Arc<Mutex<ServiceState>>,
    ) {
        match self {
            MiriGet::FocusedWorkspaceMode => {
                println!("[GET]: FocusedWorkspaceMode");
            }
            MiriGet::OtherThing => {
                println!("[GET]: OtherThing");
            }
        }
    }
}

// TODO: this function is half ai generated, review later
fn handle_cli(
    stream: UnixStream,
    action_socket: Arc<Mutex<Socket>>,
    event_state: Arc<RwLock<EventStreamState>>,
    service_state: Arc<Mutex<ServiceState>>,
) {
    let reader = BufReader::new(stream);
    for line in reader.lines() {
        match line {
            Ok(command_str) => {
                let command_str = command_str.trim();
                if command_str.is_empty() {
                    continue;
                }

                match serde_json::from_str::<IPCMessageContainer>(command_str) {
                    Ok(container) => match container.message {
                        IPCMessage::CliExecute(command) => {
                            command.run(action_socket.clone(), event_state.clone(), service_state.clone());
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to parse command '{}': {}", command_str, e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading from client: {}", e);
                break;
            }
        }
    }
}

fn main() {
    let socket_path = MIRI_SOCKET_PATH;
    let _ = std::fs::remove_file(socket_path);

    let cli_listener = UnixListener::bind(socket_path).expect("Failed to bind to miri unix socket");

    let action_socket = Arc::new(Mutex::new(
        Socket::connect().expect("Failed to connect to niri_ipc action socket"),
    ));

    let event_state = Arc::new(RwLock::new(EventStreamState::default()));
    let service_state = Arc::new(Mutex::new(ServiceState::default()));

    let event_state_clone = event_state.clone();
    let service_state_clone = service_state.clone();
    thread::spawn(move || {
        event_loop(event_state_clone, service_state_clone);
    });

    // accept cli socket connections on main thread
    for stream in cli_listener.incoming() {
        match stream {
            Ok(client_stream) => {
                let action_socket = action_socket.clone();
                let event_state = event_state.clone();
                let service_state = service_state.clone();
                thread::spawn(move || {
                    handle_cli(client_stream, action_socket, event_state, service_state);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}

fn event_loop(event_state: Arc<RwLock<EventStreamState>>, _service_state: Arc<Mutex<ServiceState>>) {
    let mut event_socket = Socket::connect().expect("Failed to connect to niri_ipc event socket");

    if let Err(e) = event_socket.send(Request::EventStream) {
        eprintln!("Failed to subscribe to event stream: {e}");
        std::process::exit(1);
    }

    let mut read_next = event_socket.read_events();

    loop {
        // FIXME: this is not a good way to handle this lol
        let event = read_next().expect("Failed to read event");

        match &event {
            niri_ipc::Event::WindowOpenedOrChanged { window } => {
                println!("[EVENT]: window opened or changed");
                let local_event_state = event_state.read().expect("Could not hold lock on event_state");
                if service::niri_ipc_utils::window_is_new(&window.id, &local_event_state) {
                    println!("window opened");
                } else {
                    println!("window changed");
                }
            }
            niri_ipc::Event::WindowClosed { id: _ } => {
                println!("[EVENT]: window closed");
            }
            niri_ipc::Event::WindowsChanged { windows: _ } => {
                println!("[EVENT]: windows changed");
            }
            _ => {}
        }

        let mut state = event_state
            .write()
            .expect("Failed to acquire write lock on event state");
        state.apply(event);
    }
}
