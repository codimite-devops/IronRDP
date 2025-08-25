//! RAIL client implementation
//! 
//! Provides a client-side implementation of the RAIL protocol for launching
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

/// RAIL client implementation
/// 
/// Handles client-side RAIL protocol operations including application launching,
/// window management, and session control.
#[derive(Debug)]
pub struct RailClient {
    /// Current RAIL protocol version
    version: u32,
    /// Whether the handshake has been completed
    handshake_complete: bool,
    /// Map of launched applications by their execution ID
    applications: HashMap<u32, ApplicationInfo>,
    /// Next execution ID to assign
    next_exec_id: u32,
}

/// Information about a launched application
#[derive(Debug, Clone)]
pub struct ApplicationInfo {
    pub exec_id: u32,
    pub executable: String,
    pub working_directory: String,
    pub arguments: String,
    pub flags: ExecuteFlags,
    pub windows: Vec<WindowId>,
}

impl RailClient {
    /// Create a new RAIL client with default settings
    pub fn new() -> Self {
        Self {
            version: RAIL_VERSION_4,
            handshake_complete: false,
            applications: HashMap::new(),
            next_exec_id: 1,
        }
    }

    /// Create a new RAIL client with specified version
    pub fn with_version(version: u32) -> Self {
        Self {
            version,
            handshake_complete: false,
            applications: HashMap::new(),
            next_exec_id: 1,
        }
    }

    /// Launch a remote application
    pub fn execute_application(
        &mut self,
        executable: String,
        working_directory: String,
        arguments: String,
        flags: ExecuteFlags,
    ) -> PduResult<SvcMessage> {
        let exec_id = self.next_exec_id;
        self.next_exec_id += 1;

        let app_info = ApplicationInfo {
            exec_id,
            executable: executable.clone(),
            working_directory: working_directory.clone(),
            arguments: arguments.clone(),
            flags,
            windows: Vec::new(),
        };

        self.applications.insert(exec_id, app_info);

        let execute_pdu = ExecutePdu::new(executable, working_directory, arguments)
            .with_flags(flags);
        
        let rail_pdu = RailPdu::new(OrderType::Execute, RailPduData::Execute(execute_pdu));
        
        Ok(SvcMessage::from(rail_pdu))
    }

    /// Create a handshake message
    pub fn create_handshake(&self) -> SvcMessage {
        let handshake_pdu = HandshakePdu::new(self.version);
        let rail_pdu = RailPdu::new(OrderType::Handshake, RailPduData::Handshake(handshake_pdu));
        SvcMessage::from(rail_pdu)
    }

    /// Get information about a launched application
    pub fn get_application(&self, exec_id: u32) -> Option<&ApplicationInfo> {
        self.applications.get(&exec_id)
    }

    /// Get all launched applications
    pub fn applications(&self) -> impl Iterator<Item = &ApplicationInfo> {
        self.applications.values()
    }

    /// Check if handshake is complete
    pub fn is_handshake_complete(&self) -> bool {
        self.handshake_complete
    }

    /// Process an incoming RAIL PDU
    fn process_rail_pdu(&mut self, pdu: RailPdu) -> PduResult<Vec<SvcMessage>> {
        match pdu.order_data {
            RailPduData::Handshake(_) => {
                self.handshake_complete = true;
                Ok(Vec::new())
            }
            RailPduData::HandshakeEx(_) => {
                self.handshake_complete = true;
                Ok(Vec::new())
            }
            RailPduData::NotifyEvent(_) => {
                // Handle window events, updates, etc.
                Ok(Vec::new())
            }
            RailPduData::WindowMove(_) => {
                // Handle window movement notifications
                Ok(Vec::new())
            }
            RailPduData::MinMaxInfo(_) => {
                // Handle window state changes
                Ok(Vec::new())
            }
            _ => {
                // Handle other PDU types as needed
                Ok(Vec::new())
            }
        }
    }
}

impl Default for RailClient {
    fn default() -> Self {
        Self::new()
    }
}

impl_as_any!(RailClient);

impl SvcProcessor for RailClient {
    fn channel_name(&self) -> ChannelName {
        ChannelName::from_utf8(RAIL_CHANNEL_NAME).expect("Invalid RAIL channel name")
    }

    fn compression_condition(&self) -> CompressionCondition {
        CompressionCondition::Never
    }

    fn start(&mut self) -> PduResult<Vec<SvcMessage>> {
        // Send handshake when channel starts
        Ok(vec![self.create_handshake()])
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
    fn test_rail_client_creation() {
        let client = RailClient::new();
        assert_eq!(client.version, RAIL_VERSION_4);
        assert!(!client.handshake_complete);
        assert_eq!(client.applications.len(), 0);
    }

    #[test]
    fn test_application_execution() {
        let mut client = RailClient::new();
        
        let _message = client.execute_application(
            "notepad.exe".to_string(),
            "C:\\".to_string(),
            "test.txt".to_string(),
            ExecuteFlags::NORMAL,
        ).unwrap();

        assert_eq!(client.applications.len(), 1);
        let app = client.get_application(1).unwrap();
        assert_eq!(app.executable, "notepad.exe");
        assert_eq!(app.arguments, "test.txt");
    }

    #[test]
    fn test_channel_name() {
        let client = RailClient::new();
        let channel_name = client.channel_name();
        assert_eq!(channel_name.as_str(), Some(RAIL_CHANNEL_NAME));
    }
}