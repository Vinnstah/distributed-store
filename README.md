# Distributed Systems Challenge
A distributed systems challenge generated by ChatGPT. This exercise was generated in order to practice coding distributed systems in Rust.
The actual outline and description of the challenge is generated but none of the code in the project.

# Distributed Systems Challenge: A Simple Distributed Key-Value Store

## Objective
Design and implement a basic distributed key-value store that can handle GET, PUT, and DELETE operations over a network. 
The system should consist of multiple nodes (servers) that can store key-value pairs and a client that can interact with these nodes to perform operations. The challenge focuses on understanding the basics of distributed systems, including networking, data replication, and fault tolerance.

### Key Features
**Data Partitioning**: Implement a simple partitioning mechanism to distribute data across different nodes.
**Replication**: Ensure data is replicated across at least two nodes for fault tolerance.
**Consistency**: Implement eventual consistency for replication.
**Basic Operations**: Support for GET, PUT, and DELETE operations.
**Fault Tolerance**: The system should handle node failures gracefully during operations.
**Client Interaction**: A simple client that can perform operations on the distributed store.

### Tools and Languages
**Programming Language**: You can use any programming language you are comfortable with. Python, Go, and Java are good choices for their networking libraries.
**Networking**: Use TCP or HTTP for communication between nodes and the client.
**Data Storage**: In-memory storage on each node is sufficient.

## System Components
**Node (Server)**:
Each node stores a subset of the key-value pairs.
Handles replication to ensure data is stored on multiple nodes.
Responds to client requests for GET, PUT, and DELETE operations.
**Client**:
Sends requests to nodes to perform operations.
Determines which node to contact for a given key (simple partitioning logic).

## Testing Your System
**Partitioning and Basic Operations**:
Start 3 nodes.
Use the client to PUT 10 key-value pairs into the store.
Ensure that the keys are evenly distributed across the nodes.
Use GET to retrieve all keys and validate the values.
DELETE a few keys and ensure they are removed.
**Replication and Fault Tolerance**:
With the 10 key-value pairs stored and replicated, randomly stop one node.
Ensure that all data is still accessible through the remaining nodes.
Restart the node and ensure it can be repopulated with the missing data.

**Consistency**:
Perform concurrent PUT operations on the same key with different values from multiple clients.
Ensure that eventually, all replicas agree on the same value.
