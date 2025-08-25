//! RAIL PDU (Protocol Data Unit) structures and encoding/decoding
//! 
//! This module implements all the PDU types defined in the MS-RDPERP specification
//! for communication between RAIL client and server.

use ironrdp_core::{
    ensure_size, invalid_field_err, Decode, DecodeResult, Encode, 
    EncodeResult, ReadCursor, WriteCursor,
};
use ironrdp_svc::SvcEncode;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::types::{ExecuteFlags};

/// RAIL PDU header size in bytes
pub const RAIL_PDU_HEADER_SIZE: usize = 4;

/// Maximum size of RAIL PDU data
pub const RAIL_MAX_PDU_SIZE: usize = 16384;

/// Main RAIL PDU envelope that wraps all RAIL messages
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RailPdu {
    pub order_type: OrderType,
    pub order_length: u16,
    pub order_data: RailPduData,
}

impl RailPdu {
    pub fn new(order_type: OrderType, order_data: RailPduData) -> Self {
        let order_length = order_data.size() as u16;
        Self {
            order_type,
            order_length,
            order_data,
        }
    }
}

impl Encode for RailPdu {
    fn encode(&self, dst: &mut WriteCursor<'_>) -> EncodeResult<()> {
        ensure_size!(in: dst, size: self.size());

        dst.write_u16(self.order_type.to_u16().unwrap());
        dst.write_u16(self.order_length);
        self.order_data.encode(dst)?;

        Ok(())
    }

    fn name(&self) -> &'static str {
        "RailPdu"
    }

    fn size(&self) -> usize {
        RAIL_PDU_HEADER_SIZE + self.order_data.size()
    }
}

impl<'de> Decode<'de> for RailPdu {
    fn decode(src: &mut ReadCursor<'de>) -> DecodeResult<Self> {
        ensure_size!(in: src, size: RAIL_PDU_HEADER_SIZE);

        let order_type = OrderType::from_u16(src.read_u16())
            .ok_or_else(|| invalid_field_err!("orderType", "invalid RAIL order type"))?;
        let order_length = src.read_u16();

        let data_size = order_length as usize;
        ensure_size!(in: src, size: data_size);
        
        let data_slice = src.read_slice(data_size);
        let mut data_cursor = ReadCursor::new(data_slice);
        let order_data = RailPduData::decode_with_type(&mut data_cursor, order_type)?;

        Ok(Self {
            order_type,
            order_length,
            order_data,
        })
    }
}

impl SvcEncode for RailPdu {}

/// RAIL order types as defined in MS-RDPERP Section 2.2.1.1.1
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum OrderType {
    /// Handshake PDU
    Handshake = 0x0001,
    /// Handshake Extended PDU  
    HandshakeEx = 0x0002,
    /// Execute PDU
    Execute = 0x0003,
    /// Activate PDU
    Activate = 0x0004,
    /// System Parameters Update PDU
    SysParam = 0x0005,
    /// System Command PDU
    SysCommand = 0x0006,
    /// Notify Event PDU
    NotifyEvent = 0x0007,
    /// Window Move PDU
    WindowMove = 0x0008,
    /// Local Move/Size PDU
    LocalMoveSize = 0x0009,
    /// Minimize/Restore PDU
    MinMaxInfo = 0x000A,
    /// Client Status PDU
    ClientStatus = 0x000B,
    /// System Menu PDU
    SysMenu = 0x000C,
    /// Language Bar Info PDU
    LangBarInfo = 0x000D,
    /// Get Application ID PDU
    GetAppId = 0x000E,
    /// Get Application ID Response PDU
    GetAppIdResp = 0x000F,
}

/// Union of all possible RAIL PDU data types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RailPduData {
    Handshake(HandshakePdu),
    HandshakeEx(HandshakeExPdu),
    Execute(ExecutePdu),
    Activate(ActivatePdu),
    SysParam(SysParamPdu),
    SysCommand(SysCommandPdu),
    NotifyEvent(NotifyEventPdu),
    WindowMove(WindowMovePdu),
    LocalMoveSize(LocalMoveSizePdu),
    MinMaxInfo(MinMaxInfoPdu),
    ClientStatus(ClientStatusPdu),
    SysMenu(SysMenuPdu),
    LangBarInfo(LangBarInfoPdu),
    GetAppId(GetAppIdPdu),
    GetAppIdResp(GetAppIdRespPdu),
}

impl RailPduData {
    fn decode_with_type(src: &mut ReadCursor<'_>, order_type: OrderType) -> DecodeResult<Self> {
        match order_type {
            OrderType::Handshake => Ok(Self::Handshake(HandshakePdu::decode(src)?)),
            OrderType::HandshakeEx => Ok(Self::HandshakeEx(HandshakeExPdu::decode(src)?)),
            OrderType::Execute => Ok(Self::Execute(ExecutePdu::decode(src)?)),
            OrderType::Activate => Ok(Self::Activate(ActivatePdu::decode(src)?)),
            OrderType::SysParam => Ok(Self::SysParam(SysParamPdu::decode(src)?)),
            OrderType::SysCommand => Ok(Self::SysCommand(SysCommandPdu::decode(src)?)),
            OrderType::NotifyEvent => Ok(Self::NotifyEvent(NotifyEventPdu::decode(src)?)),
            OrderType::WindowMove => Ok(Self::WindowMove(WindowMovePdu::decode(src)?)),
            OrderType::LocalMoveSize => Ok(Self::LocalMoveSize(LocalMoveSizePdu::decode(src)?)),
            OrderType::MinMaxInfo => Ok(Self::MinMaxInfo(MinMaxInfoPdu::decode(src)?)),
            OrderType::ClientStatus => Ok(Self::ClientStatus(ClientStatusPdu::decode(src)?)),
            OrderType::SysMenu => Ok(Self::SysMenu(SysMenuPdu::decode(src)?)),
            OrderType::LangBarInfo => Ok(Self::LangBarInfo(LangBarInfoPdu::decode(src)?)),
            OrderType::GetAppId => Ok(Self::GetAppId(GetAppIdPdu::decode(src)?)),
            OrderType::GetAppIdResp => Ok(Self::GetAppIdResp(GetAppIdRespPdu::decode(src)?)),
        }
    }
}

impl Encode for RailPduData {
    fn encode(&self, dst: &mut WriteCursor<'_>) -> EncodeResult<()> {
        match self {
            Self::Handshake(pdu) => pdu.encode(dst),
            Self::HandshakeEx(pdu) => pdu.encode(dst),
            Self::Execute(pdu) => pdu.encode(dst),
            Self::Activate(pdu) => pdu.encode(dst),
            Self::SysParam(pdu) => pdu.encode(dst),
            Self::SysCommand(pdu) => pdu.encode(dst),
            Self::NotifyEvent(pdu) => pdu.encode(dst),
            Self::WindowMove(pdu) => pdu.encode(dst),
            Self::LocalMoveSize(pdu) => pdu.encode(dst),
            Self::MinMaxInfo(pdu) => pdu.encode(dst),
            Self::ClientStatus(pdu) => pdu.encode(dst),
            Self::SysMenu(pdu) => pdu.encode(dst),
            Self::LangBarInfo(pdu) => pdu.encode(dst),
            Self::GetAppId(pdu) => pdu.encode(dst),
            Self::GetAppIdResp(pdu) => pdu.encode(dst),
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Handshake(_) => "HandshakePdu",
            Self::HandshakeEx(_) => "HandshakeExPdu",
            Self::Execute(_) => "ExecutePdu",
            Self::Activate(_) => "ActivatePdu",
            Self::SysParam(_) => "SysParamPdu",
            Self::SysCommand(_) => "SysCommandPdu",
            Self::NotifyEvent(_) => "NotifyEventPdu",
            Self::WindowMove(_) => "WindowMovePdu",
            Self::LocalMoveSize(_) => "LocalMoveSizePdu",
            Self::MinMaxInfo(_) => "MinMaxInfoPdu",
            Self::ClientStatus(_) => "ClientStatusPdu",
            Self::SysMenu(_) => "SysMenuPdu",
            Self::LangBarInfo(_) => "LangBarInfoPdu",
            Self::GetAppId(_) => "GetAppIdPdu",
            Self::GetAppIdResp(_) => "GetAppIdRespPdu",
        }
    }

    fn size(&self) -> usize {
        match self {
            Self::Handshake(pdu) => pdu.size(),
            Self::HandshakeEx(pdu) => pdu.size(),
            Self::Execute(pdu) => pdu.size(),
            Self::Activate(pdu) => pdu.size(),
            Self::SysParam(pdu) => pdu.size(),
            Self::SysCommand(pdu) => pdu.size(),
            Self::NotifyEvent(pdu) => pdu.size(),
            Self::WindowMove(pdu) => pdu.size(),
            Self::LocalMoveSize(pdu) => pdu.size(),
            Self::MinMaxInfo(pdu) => pdu.size(),
            Self::ClientStatus(pdu) => pdu.size(),
            Self::SysMenu(pdu) => pdu.size(),
            Self::LangBarInfo(pdu) => pdu.size(),
            Self::GetAppId(pdu) => pdu.size(),
            Self::GetAppIdResp(pdu) => pdu.size(),
        }
    }
}

/// RAIL Handshake PDU - Initial handshake message
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandshakePdu {
    pub build_number: u32,
}

impl HandshakePdu {
    pub fn new(build_number: u32) -> Self {
        Self { build_number }
    }
}

impl Encode for HandshakePdu {
    fn encode(&self, dst: &mut WriteCursor<'_>) -> EncodeResult<()> {
        ensure_size!(in: dst, size: self.size());
        dst.write_u32(self.build_number);
        Ok(())
    }

    fn name(&self) -> &'static str {
        "HandshakePdu"
    }

    fn size(&self) -> usize {
        4
    }
}

impl<'de> Decode<'de> for HandshakePdu {
    fn decode(src: &mut ReadCursor<'de>) -> DecodeResult<Self> {
        ensure_size!(in: src, size: 4);
        let build_number = src.read_u32();
        Ok(Self { build_number })
    }
}

/// RAIL Extended Handshake PDU - Extended handshake with version info
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandshakeExPdu {
    pub build_number: u32,
    pub rail_handshake_flags: u32,
}

impl HandshakeExPdu {
    pub fn new(build_number: u32, rail_handshake_flags: u32) -> Self {
        Self { build_number, rail_handshake_flags }
    }
}

impl Encode for HandshakeExPdu {
    fn encode(&self, dst: &mut WriteCursor<'_>) -> EncodeResult<()> {
        ensure_size!(in: dst, size: self.size());
        dst.write_u32(self.build_number);
        dst.write_u32(self.rail_handshake_flags);
        Ok(())
    }

    fn name(&self) -> &'static str {
        "HandshakeExPdu"
    }

    fn size(&self) -> usize {
        8
    }
}

impl<'de> Decode<'de> for HandshakeExPdu {
    fn decode(src: &mut ReadCursor<'de>) -> DecodeResult<Self> {
        ensure_size!(in: src, size: 8);
        let build_number = src.read_u32();
        let rail_handshake_flags = src.read_u32();
        Ok(Self { build_number, rail_handshake_flags })
    }
}

/// RAIL Execute PDU - Launch application request
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutePdu {
    pub flags: ExecuteFlags,
    pub exe_or_file: String,
    pub working_directory: String,
    pub arguments: String,
}

impl ExecutePdu {
    pub fn new(exe_or_file: String, working_directory: String, arguments: String) -> Self {
        Self {
            flags: ExecuteFlags::NORMAL,
            exe_or_file,
            working_directory,
            arguments,
        }
    }

    pub fn with_flags(mut self, flags: ExecuteFlags) -> Self {
        self.flags = flags;
        self
    }
}

impl Encode for ExecutePdu {
    fn encode(&self, dst: &mut WriteCursor<'_>) -> EncodeResult<()> {
        ensure_size!(in: dst, size: self.size());

        dst.write_u32(self.flags.bits());
        
        // Encode null-terminated UTF-16 strings
        let exe_utf16 = self.exe_or_file.encode_utf16().chain(std::iter::once(0)).collect::<Vec<u16>>();
        let working_dir_utf16 = self.working_directory.encode_utf16().chain(std::iter::once(0)).collect::<Vec<u16>>();
        let args_utf16 = self.arguments.encode_utf16().chain(std::iter::once(0)).collect::<Vec<u16>>();

        dst.write_u16((exe_utf16.len() * 2) as u16);
        dst.write_u16((working_dir_utf16.len() * 2) as u16);
        dst.write_u16((args_utf16.len() * 2) as u16);

        for &ch in &exe_utf16 {
            dst.write_u16(ch);
        }
        for &ch in &working_dir_utf16 {
            dst.write_u16(ch);
        }
        for &ch in &args_utf16 {
            dst.write_u16(ch);
        }

        Ok(())
    }

    fn name(&self) -> &'static str {
        "ExecutePdu"
    }

    fn size(&self) -> usize {
        let exe_len = (self.exe_or_file.encode_utf16().count() + 1) * 2;
        let working_dir_len = (self.working_directory.encode_utf16().count() + 1) * 2;
        let args_len = (self.arguments.encode_utf16().count() + 1) * 2;
        
        4 + 2 + 2 + 2 + exe_len + working_dir_len + args_len
    }
}

impl<'de> Decode<'de> for ExecutePdu {
    fn decode(src: &mut ReadCursor<'de>) -> DecodeResult<Self> {
        ensure_size!(in: src, size: 10);

        let flags = ExecuteFlags::from_bits(src.read_u32())
            .ok_or_else(|| invalid_field_err!("flags", "invalid execute flags"))?;
        
        let exe_len = src.read_u16() as usize;
        let working_dir_len = src.read_u16() as usize;
        let args_len = src.read_u16() as usize;

        ensure_size!(in: src, size: exe_len + working_dir_len + args_len);

        let exe_or_file = decode_utf16_string(src, exe_len)?;
        let working_directory = decode_utf16_string(src, working_dir_len)?;
        let arguments = decode_utf16_string(src, args_len)?;

        Ok(Self {
            flags,
            exe_or_file,
            working_directory,
            arguments,
        })
    }
}

// Helper function to decode UTF-16 strings
fn decode_utf16_string(src: &mut ReadCursor<'_>, byte_len: usize) -> DecodeResult<String> {
    let char_count = byte_len / 2;
    let mut utf16_chars = Vec::with_capacity(char_count);
    
    for _ in 0..char_count {
        utf16_chars.push(src.read_u16());
    }
    
    // Remove null terminator if present
    if let Some(&0) = utf16_chars.last() {
        utf16_chars.pop();
    }
    
    String::from_utf16(&utf16_chars)
        .map_err(|_| invalid_field_err!("string", "invalid UTF-16 string"))
}

// Placeholder implementations for remaining PDU types
// These would be fully implemented in a complete RAIL implementation

macro_rules! impl_placeholder_pdu {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name {
            _placeholder: u32,
        }

        impl $name {
            pub fn new() -> Self {
                Self { _placeholder: 0 }
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl Encode for $name {
            fn encode(&self, dst: &mut WriteCursor<'_>) -> EncodeResult<()> {
                dst.write_u32(self._placeholder);
                Ok(())
            }

            fn name(&self) -> &'static str {
                stringify!($name)
            }

            fn size(&self) -> usize {
                4
            }
        }

        impl<'de> Decode<'de> for $name {
            fn decode(src: &mut ReadCursor<'de>) -> DecodeResult<Self> {
                let _placeholder = src.read_u32();
                Ok(Self { _placeholder })
            }
        }
    };
}

impl_placeholder_pdu!(ActivatePdu);
impl_placeholder_pdu!(SysParamPdu);
impl_placeholder_pdu!(SysCommandPdu);
impl_placeholder_pdu!(NotifyEventPdu);
impl_placeholder_pdu!(WindowMovePdu);
impl_placeholder_pdu!(LocalMoveSizePdu);
impl_placeholder_pdu!(MinMaxInfoPdu);
impl_placeholder_pdu!(ClientStatusPdu);
impl_placeholder_pdu!(SysMenuPdu);
impl_placeholder_pdu!(LangBarInfoPdu);
impl_placeholder_pdu!(GetAppIdPdu);
impl_placeholder_pdu!(GetAppIdRespPdu);

#[cfg(test)]
mod tests {
    use super::*;
    use ironrdp_core::{encode_vec, decode};

    #[test]
    fn test_handshake_pdu_encode_decode() {
        let pdu = HandshakePdu::new(12345);
        let encoded = encode_vec(&pdu).unwrap();
        let decoded: HandshakePdu = decode(&encoded).unwrap();
        assert_eq!(pdu, decoded);
    }

    #[test]
    fn test_execute_pdu_encode_decode() {
        let pdu = ExecutePdu::new(
            "notepad.exe".to_string(),
            "C:\\".to_string(),
            "test.txt".to_string(),
        );
        
        let encoded = encode_vec(&pdu).unwrap();
        let decoded: ExecutePdu = decode(&encoded).unwrap();
        assert_eq!(pdu, decoded);
    }

    #[test]
    fn test_rail_pdu_wrapper() {
        let handshake = HandshakePdu::new(12345);
        let rail_pdu = RailPdu::new(OrderType::Handshake, RailPduData::Handshake(handshake.clone()));
        
        let encoded = encode_vec(&rail_pdu).unwrap();
        let decoded: RailPdu = decode(&encoded).unwrap();
        
        assert_eq!(rail_pdu.order_type, decoded.order_type);
        assert_eq!(rail_pdu.order_length, decoded.order_length);
        
        if let RailPduData::Handshake(decoded_handshake) = decoded.order_data {
            assert_eq!(handshake, decoded_handshake);
        } else {
            panic!("Expected HandshakePdu");
        }
    }
}