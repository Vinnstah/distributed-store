use distributed_client::memory::ClientMemory;
use distributed_client::message_dispatch::Client;
use models::message::CircularList;
use models::node::NodeID;
use std::env;
use std::sync::{Arc, Mutex};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let port: u16 = args
        .get(1)
        .expect("Failed to get port argument")
        .parse()
        .expect("Failed to parse arg as u16");

    let list_of_servers = Arc::new(CircularList::new(vec![port, port + 1, port + 2]));
    let mut client_memory = Arc::new(Mutex::new(ClientMemory::<String>::new(CircularList::new(
        vec![
            NodeID::from_u16(port),
            NodeID::from_u16(port + 1),
            NodeID::from_u16(port + 2),
        ],
    ))));
    let servers = Client::initialize_nodes(list_of_servers);

    let message_stack = Client::create_message_stack(9000);
    Client::dispatch_messages(message_stack, servers);

    loop {}
}
