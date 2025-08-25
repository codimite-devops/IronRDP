# IronRDP RAIL

Implementation of the RAIL (Remote Application Integrated Locally) protocol for IronRDP, 
supporting RemoteApp functionality and seamless application delivery.

## What is RAIL?

RAIL (Remote Application Integrated Locally) is a Microsoft protocol that enables individual applications 
running on a remote server to appear as if they are running locally on the client desktop. This provides 
a seamless user experience where remote applications integrate with the local desktop environment.

## Features

- **Application Launching**: Start remote applications as if they were local
- **Window Management**: Seamless window operations (minimize, maximize, move, resize)
- **Session Management**: Handle remote application lifecycle
- **Desktop Integration**: Applications appear in taskbar and respond to Alt+Tab
- **Language Bar Support**: Docked language bar functionality
- **IME Synchronization**: Input method editor synchronization

## Protocol Implementation

This crate implements the MS-RDPERP specification including:

- RAIL capability negotiation
- Static virtual channel communication
- Core RAIL PDU structures and encoding/decoding
- Client and server-side processing

## Usage

```rust
use ironrdp_rail::{RailClient, RailServer, ExecuteRequest};
use ironrdp_svc::StaticVirtualChannel;

// Create a RAIL client
let rail_client = RailClient::new();
let mut channel = StaticVirtualChannel::new(rail_client);

// Launch a remote application
let execute_req = ExecuteRequest::new("notepad.exe", "");
let messages = channel.process(&execute_req.encode()?)?;
```

This crate is part of the [IronRDP] project.

[IronRDP]: https://github.com/Devolutions/IronRDP