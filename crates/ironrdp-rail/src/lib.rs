#![cfg_attr(doc, doc = include_str!("../README.md"))]
#![doc(html_logo_url = "https://cdnweb.devolutions.net/images/projects/devolutions/logos/devolutions-icon-shadow.svg")]

//! RAIL (Remote Application Integrated Locally) protocol implementation
//! 
//! This crate provides a complete implementation of the MS-RDPERP protocol
//! for seamless application delivery and desktop integration.

// Re-export core dependencies for convenience
pub use ironrdp_core;
pub use ironrdp_pdu;
pub use ironrdp_svc;

pub mod pdu;
pub mod client;
pub mod server;
pub mod types;

// Re-export main types
pub use client::RailClient;
pub use server::RailServer;
pub use pdu::{RailPdu, OrderType};
pub use types::*;

/// The RAIL static virtual channel name as defined in MS-RDPERP
pub const RAIL_CHANNEL_NAME: &str = "rail";

/// RAIL protocol version constants
pub mod version {
    /// RAIL version 1.0 (Windows Vista/2008)
    pub const RAIL_VERSION_1: u32 = 0x0001_0000;
    
    /// RAIL version 2.0 (Windows 7/2008 R2)
    pub const RAIL_VERSION_2: u32 = 0x0002_0000;
    
    /// RAIL version 3.0 (Windows 8/2012)
    pub const RAIL_VERSION_3: u32 = 0x0003_0000;
    
    /// RAIL version 4.0 (Windows 10/2016)
    pub const RAIL_VERSION_4: u32 = 0x0004_0000;
}