use distributed_client::client::Client;
use models::message::CircularList;
use models::node::NodeID;
use models::tcp_client::Stream;
use std::env;
use std::sync::Arc;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let port: u16 = args
        .get(1)
        .expect("Failed to get port argument")
        .parse()
        .expect("Failed to parse arg as u16");

    let servers = vec![
        NodeID::from_u16(port),
        NodeID::from_u16(port + 1),
        NodeID::from_u16(port + 2),
    ];
    let mut client = Client::new(Stream::new(), servers);

    let list_of_servers = Arc::new(CircularList::new(vec![port, port + 1, port + 2]));

    client.initialize_nodes(list_of_servers);

    client.create_message_stack(900);

    client.dispatch_messages();

    loop {}
}
