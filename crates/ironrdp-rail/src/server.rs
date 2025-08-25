//! RAIL server implementation
//! 
//! Provides a server-side implementation of the RAIL protocol for hosting
//! and managing remote applications.

use std::collections::HashMap;

use ironrdp_core::impl_as_any;
use ironrdp_pdu::{gcc::ChannelName, PduResult};
use ironrdp_svc::{CompressionCondition, SvcMessage, SvcProcessor};

use crate::{
    pdu::{RailPdu, RailPduData, OrderType, HandshakePdu, ExecutePdu},
    types::{WindowId, ExecuteFlags},
    RAIL_CHANNEL_NAME,
    version::RAIL_VERSION_4,
};

/// RAIL server implementation
/// 
/// Handles server-side RAIL protocol operations including application execution,
/// window management, and client communication.
#[derive(Debug)]
pub struct RailServer {
    /// Current RAIL protocol version
    version: u32,
    /// Whether the handshake has been completed
    handshake_complete: bool,
    /// Map of running processes by their execution ID
    processes: HashMap<u32, ProcessInfo>,
    /// Next execution ID to assign
    next_exec_id: u32,
}

/// Information about a running process
#[derive(Debug)]
pub struct ProcessInfo {
    pub exec_id: u32,
    pub executable: String,
    pub working_directory: String,
    pub arguments: String,
    pub flags: ExecuteFlags,
    pub windows: Vec<WindowId>,
}

impl RailServer {
    /// Create a new RAIL server with default settings
    pub fn new() -> Self {
        Self {
            version: RAIL_VERSION_4,
            handshake_complete: false,
            processes: HashMap::new(),
            next_exec_id: 1,
        }
    }

    /// Create a new RAIL server with specified version
    pub fn with_version(version: u32) -> Self {
        Self {
            version,
            handshake_complete: false,
            processes: HashMap::new(),
            next_exec_id: 1,
        }
    }

    /// Create a handshake response message
    pub fn create_handshake_response(&self) -> SvcMessage {
        let handshake_pdu = HandshakePdu::new(self.version);
        let rail_pdu = RailPdu::new(OrderType::Handshake, RailPduData::Handshake(handshake_pdu));
        SvcMessage::from(rail_pdu)
    }

    /// Execute an application on the server
    fn execute_application(&mut self, execute_pdu: ExecutePdu) -> PduResult<Vec<SvcMessage>> {
        let exec_id = self.next_exec_id;
        self.next_exec_id += 1;

        // In a real implementation, this would launch the actual process
        // For this sample, we'll just store the information
        let process_info = ProcessInfo {
            exec_id,
            executable: execute_pdu.exe_or_file.clone(),
            working_directory: execute_pdu.working_directory.clone(),
            arguments: execute_pdu.arguments.clone(),
            flags: execute_pdu.flags,
            windows: Vec::new(),
        };

        self.processes.insert(exec_id, process_info);

        // In a real implementation, would return appropriate response PDUs
        // For now, just acknowledge the execution
        Ok(Vec::new())
    }

    /// Get information about a running process
    pub fn get_process(&self, exec_id: u32) -> Option<&ProcessInfo> {
        self.processes.get(&exec_id)
    }

    /// Get all running processes
    pub fn processes(&self) -> impl Iterator<Item = &ProcessInfo> {
        self.processes.values()
    }

    /// Check if handshake is complete
    pub fn is_handshake_complete(&self) -> bool {
        self.handshake_complete
    }

    /// Terminate a process
    pub fn terminate_process(&mut self, exec_id: u32) -> PduResult<bool> {
        if let Some(_process_info) = self.processes.remove(&exec_id) {
            // In a real implementation, would terminate the actual process
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Process an incoming RAIL PDU
    fn process_rail_pdu(&mut self, pdu: RailPdu) -> PduResult<Vec<SvcMessage>> {
        match pdu.order_data {
            RailPduData::Handshake(_) => {
                self.handshake_complete = true;
                Ok(vec![self.create_handshake_response()])
            }
            RailPduData::HandshakeEx(_) => {
                self.handshake_complete = true;
                // Would send HandshakeEx response in full implementation
                Ok(vec![self.create_handshake_response()])
            }
            RailPduData::Execute(execute_pdu) => {
                self.execute_application(execute_pdu)
            }
            RailPduData::Activate(_) => {
                // Handle window activation requests
                Ok(Vec::new())
            }
            RailPduData::SysCommand(_) => {
                // Handle system commands (minimize, maximize, etc.)
                Ok(Vec::new())
            }
            RailPduData::WindowMove(_) => {
                // Handle window movement requests
                Ok(Vec::new())
            }
            _ => {
                // Handle other PDU types as needed
                Ok(Vec::new())
            }
        }
    }
}

impl Default for RailServer {
    fn default() -> Self {
        Self::new()
    }
}

impl_as_any!(RailServer);

impl SvcProcessor for RailServer {
    fn channel_name(&self) -> ChannelName {
        ChannelName::from_utf8(RAIL_CHANNEL_NAME).expect("Invalid RAIL channel name")
    }

    fn compression_condition(&self) -> CompressionCondition {
        CompressionCondition::Never
    }

    fn start(&mut self) -> PduResult<Vec<SvcMessage>> {
        // Server waits for client handshake
        Ok(Vec::new())
    }

    fn process(&mut self, payload: &[u8]) -> PduResult<Vec<SvcMessage>> {
        use ironrdp_core::decode;
        use ironrdp_pdu::decode_err;

        let rail_pdu: RailPdu = decode(payload).map_err(|e| decode_err!(e))?;
        self.process_rail_pdu(rail_pdu)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rail_server_creation() {
        let server = RailServer::new();
        assert_eq!(server.version, RAIL_VERSION_4);
        assert!(!server.handshake_complete);
        assert_eq!(server.processes.len(), 0);
    }

    #[test]
    fn test_process_execution() {
        let mut server = RailServer::new();
        
        let execute_pdu = ExecutePdu::new(
            "notepad.exe".to_string(),
            "C:\\".to_string(),
            "test.txt".to_string(),
        ).with_flags(ExecuteFlags::NORMAL);

        let _messages = server.execute_application(execute_pdu).unwrap();

        assert_eq!(server.processes.len(), 1);
        let process = server.get_process(1).unwrap();
        assert_eq!(process.executable, "notepad.exe");
        assert_eq!(process.arguments, "test.txt");
    }

    #[test]
    fn test_channel_name() {
        let server = RailServer::new();
        let channel_name = server.channel_name();
        assert_eq!(channel_name.as_str(), Some(RAIL_CHANNEL_NAME));
    }
}