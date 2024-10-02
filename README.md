----
### Description

----
This project is part of the Ukraine Rust Community bootcamp study process. It introduces a basic implementation of a Terminal User Interface (TUI) for Zookeeper.

Zookeeper + TUI + Rust = zui.rs

At present, the application supports basic operations such as create, read, update, and delete (CRUD) for Zookeeper nodes. The backbone of this project is the Ratatui framework, which provides robust tools for creating TUI-based applications.

### Available Operations
---
1. Connect to Zookeeper using provided connection parameters (connection string can be passed as an app parameter or configured within the app).
2. Create persistent nodes (other types of nodes will be added later).
3. Delete nodes.
4. Retrieve node children.
5. Retrieve node statistics.
6. Set node data.
7. Retrieve node data in different formats:
   1. Raw bytes representation
   2. String representation
   3. JSON representation
### TODO:
---
1. Add options for node creation:
  1. Ephemeral node
  2. Persistent sequential node
  3. Ephemeral sequential node
  4. Node with TTL
2. Add a DeleteAll operation to recursively delete nodes.
3. Add functionality for retrieving ephemeral nodes.
4. Implement node watching functionality.
5. Add functionality for working with ACLs.
6. Add different application modes:
  1. ReadOnly mode - only read operations are allowed.
  2. NonDelete mode - node deletion is forbidden.
7. Refactor code to resolve architectural issues.
8. Polish the UI/UX.
## Disclaimer
---
This tool may contain bugs. Therefore, be very careful when working with real data.

If you have the ability and desire to contribute, you are welcome!

### Demo video
--- 

[![Demo Video](https://img.youtube.com/vi/TBEHMzSEzW8/0.jpg)](https://www.youtube.com/watch?v=TBEHMzSEzW8)
