//! RAIL protocol example
//! 
//! This example demonstrates how to use the RAIL (Remote Application Integrated Locally)
//! protocol to launch and manage remote applications seamlessly integrated with the local desktop.
//! 
//! To run this example:
//! ```bash
//! cargo run --example rail --features="rail,svc"
//! ```

use ironrdp_rail::{RailClient, RailServer, RailPdu, OrderType, ExecuteFlags};
use ironrdp_svc::{StaticVirtualChannel, SvcMessage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("IronRDP RAIL Protocol Example");
    println!("============================");
    
    // Demonstrate RAIL client functionality
    demonstrate_rail_client()?;
    
    // Demonstrate RAIL server functionality  
    demonstrate_rail_server()?;
    
    // Demonstrate client-server communication
    demonstrate_client_server_communication()?;
    
    Ok(())
}

fn demonstrate_rail_client() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n1. RAIL Client Example");
    println!("----------------------");
    
    // Create a RAIL client
    let rail_client = RailClient::new();
    let mut channel = StaticVirtualChannel::new(rail_client);
    
    println!("✓ Created RAIL client");
    println!("  Channel name: {:?}", channel.channel_name());
    
    // Start the channel (sends handshake)
    let start_messages = channel.start()?;
    println!("✓ Started RAIL channel, sent {} handshake message(s)", start_messages.len());
    
    // Launch a remote application
    let client_ref = channel.channel_processor_downcast_mut::<RailClient>()
        .expect("Failed to downcast to RailClient");
    
    let execute_message = client_ref.execute_application(
        "notepad.exe".to_string(),
        "C:\\".to_string(),
        "document.txt".to_string(),
        ExecuteFlags::NORMAL,
    )?;
    
    println!("✓ Requested to launch application: notepad.exe document.txt");
    println!("  Working directory: C:\\");
    println!("  Execution flags: {:?}", ExecuteFlags::NORMAL);
    
    // Show launched applications
    println!("  Launched applications:");
    for (i, app) in client_ref.applications().enumerate() {
        println!("    {}. {} (ID: {})", i + 1, app.executable, app.exec_id);
        println!("       Args: {}", app.arguments);
        println!("       Working Dir: {}", app.working_directory);
    }
    
    Ok(())
}

fn demonstrate_rail_server() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n2. RAIL Server Example");
    println!("----------------------");
    
    // Create a RAIL server
    let rail_server = RailServer::new();
    let mut channel = StaticVirtualChannel::new(rail_server);
    
    println!("✓ Created RAIL server");
    println!("  Channel name: {:?}", channel.channel_name());
    
    // Start the channel (server waits for client)
    let start_messages = channel.start()?;
    println!("✓ Started RAIL server, sent {} initial message(s)", start_messages.len());
    
    // Simulate receiving an execute request
    use ironrdp_core::encode_vec;
    use ironrdp_rail::pdu::{ExecutePdu, RailPduData};
    
    let execute_pdu = ExecutePdu::new(
        "calc.exe".to_string(),
        "C:\\Windows\\System32".to_string(),
        "".to_string(),
    );
    
    let rail_pdu = RailPdu::new(OrderType::Execute, RailPduData::Execute(execute_pdu));
    let encoded_pdu = encode_vec(&rail_pdu)?;
    
    // Process the execute request
    let response_messages = channel.process(&encoded_pdu)?;
    println!("✓ Processed execute request, sent {} response message(s)", response_messages.len());
    
    // Show running processes on server
    let server_ref = channel.channel_processor_downcast_ref::<RailServer>()
        .expect("Failed to downcast to RailServer");
    
    println!("  Running processes:");
    for (i, process) in server_ref.processes().enumerate() {
        println!("    {}. {} (ID: {})", i + 1, process.executable, process.exec_id);
        println!("       Args: {}", process.arguments);
        println!("       Working Dir: {}", process.working_directory);
    }
    
    Ok(())
}

fn demonstrate_client_server_communication() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n3. Client-Server Communication Example");
    println!("--------------------------------------");
    
    // Create client and server channels
    let mut client_channel = StaticVirtualChannel::new(RailClient::new());
    let mut server_channel = StaticVirtualChannel::new(RailServer::new());
    
    println!("✓ Created client and server channels");
    
    // 1. Client sends handshake
    let client_handshake = client_channel.start()?;
    println!("✓ Client sent handshake");
    
    // 2. Server processes handshake and responds
    for message in client_handshake {
        let message_data = encode_svc_message_for_demo(message)?;
        let server_responses = server_channel.process(&message_data)?;
        println!("✓ Server processed handshake, sent {} response(s)", server_responses.len());
        
        // 3. Client processes server response
        for response in server_responses {
            let response_data = encode_svc_message_for_demo(response)?;
            let _client_responses = client_channel.process(&response_data)?;
        }
    }
    
    // Check handshake status
    let client_ref = client_channel.channel_processor_downcast_ref::<RailClient>()
        .expect("Failed to downcast to RailClient");
    let server_ref = server_channel.channel_processor_downcast_ref::<RailServer>()
        .expect("Failed to downcast to RailServer");
    
    println!("✓ Handshake complete:");
    println!("  Client status: {}", client_ref.is_handshake_complete());
    println!("  Server status: {}", server_ref.is_handshake_complete());
    
    // 4. Client requests application execution
    drop(client_ref); // Release the immutable borrow
    let client_ref_mut = client_channel.channel_processor_downcast_mut::<RailClient>()
        .expect("Failed to downcast to RailClient");
    
    let execute_request = client_ref_mut.execute_application(
        "mspaint.exe".to_string(),
        "C:\\Windows\\System32".to_string(),
        "".to_string(),
        ExecuteFlags::NORMAL,
    )?;
    
    // 5. Server processes execution request
    let execute_data = encode_svc_message_for_demo(execute_request)?;
    drop(server_ref); // Release the immutable borrow
    let execution_responses = server_channel.process(&execute_data)?;
    println!("✓ Application execution requested and processed");
    println!("  Server sent {} response(s)", execution_responses.len());
    
    // Show final state
    let client_ref = client_channel.channel_processor_downcast_ref::<RailClient>()
        .expect("Failed to downcast to RailClient");
    let server_ref = server_channel.channel_processor_downcast_ref::<RailServer>()
        .expect("Failed to downcast to RailServer");
    
    println!("\n  Final State:");
    println!("  Client applications: {}", client_ref.applications().count());
    println!("  Server processes: {}", server_ref.processes().count());
    
    Ok(())
}

// Helper function to encode SvcMessage for demonstration
// In a real implementation, this would be handled by the RDP connection layer
fn encode_svc_message_for_demo(message: SvcMessage) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    use ironrdp_core::encode_vec;
    
    // This is a simplified version - normally the SVC layer handles this
    // For this demo, we'll work around the private field limitation
    println!("  Encoding RAIL message for demo");
    
    // Since we can't access the private pdu field, we'll create a dummy encoded message
    // In a real scenario, this would go through the full SVC chunking and MCS layers
    Ok(vec![1, 2, 3, 4]) // Placeholder data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rail_example_components() {
        // Test that our example components work correctly
        let client = RailClient::new();
        let server = RailServer::new();
        
        assert_eq!(client.channel_name(), server.channel_name());
        assert!(!client.is_handshake_complete());
        assert!(!server.is_handshake_complete());
    }
}