use ironrdp_core::{
    ensure_fixed_part_size, invalid_field_err, Decode, DecodeResult, Encode, EncodeResult, ReadCursor, WriteCursor,
};

/// [2.2.1.1.1.1.17] Rail Capability Set (TS_RAIL_CAPABILITYSET)
///
/// The Rail Capability Set is used to advertise support for RemoteApp,
/// which is the Microsoft implementation of RAIL (Remote Application
/// Integrated Locally).
///
/// [2.2.1.1.1.1.17]: https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-rdpbcgr/963b02a3-4829-4da5-86c5-d6aa4ba58fb8
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rail {
    /// Flags indicating supported RAIL features
    pub rail_support_level: RailSupportLevel,
}

impl Rail {
    const NAME: &'static str = "Rail";

    const FIXED_PART_SIZE: usize = 4; // railSupportLevel (4 bytes)
}

impl Encode for Rail {
    fn encode(&self, dst: &mut WriteCursor<'_>) -> EncodeResult<()> {
        ensure_fixed_part_size!(in: dst);

        dst.write_u32(self.rail_support_level.bits());

        Ok(())
    }

    fn name(&self) -> &'static str {
        Self::NAME
    }

    fn size(&self) -> usize {
        Self::FIXED_PART_SIZE
    }
}

impl<'de> Decode<'de> for Rail {
    fn decode(src: &mut ReadCursor<'de>) -> DecodeResult<Self> {
        ensure_fixed_part_size!(in: src);

        let rail_support_level_bits = src.read_u32();
        let rail_support_level = RailSupportLevel::from_bits(rail_support_level_bits)
            .ok_or_else(|| invalid_field_err!("railSupportLevel", "invalid Rail support level"))?;

        Ok(Self { rail_support_level })
    }
}

impl Default for Rail {
    fn default() -> Self {
        Self {
            rail_support_level: RailSupportLevel::SUPPORTED,
        }
    }
}

bitflags::bitflags! {
    /// Rail Support Level flags as defined in MS-RDPBCGR 2.2.1.1.1.1.17
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct RailSupportLevel: u32 {
        /// TS_RAIL_LEVEL_SUPPORTED (0x00000001)
        /// RemoteApp is supported.
        const SUPPORTED = 0x0000_0001;

        /// TS_RAIL_LEVEL_DOCKED_LANGBAR_SUPPORTED (0x00000002)  
        /// Docked Language Bar is supported.
        const DOCKED_LANGBAR_SUPPORTED = 0x0000_0002;

        /// TS_RAIL_LEVEL_SHELL_INTEGRATION_SUPPORTED (0x00000004)
        /// Enhanced RemoteApp shell integration is supported.
        const SHELL_INTEGRATION_SUPPORTED = 0x0000_0004;

        /// TS_RAIL_LEVEL_LANGUAGE_IME_SYNC_SUPPORTED (0x00000008)
        /// Language and IME synchronization is supported.
        const LANGUAGE_IME_SYNC_SUPPORTED = 0x0000_0008;

        /// TS_RAIL_LEVEL_SERVER_TO_CLIENT_IME_SYNC_SUPPORTED (0x00000010)
        /// Server-to-client IME synchronization is supported.
        const SERVER_TO_CLIENT_IME_SYNC_SUPPORTED = 0x0000_0010;

        /// TS_RAIL_LEVEL_HIDE_MINIMIZED_APPS_SUPPORTED (0x00000020)
        /// Hiding minimized applications is supported.
        const HIDE_MINIMIZED_APPS_SUPPORTED = 0x0000_0020;

        /// TS_RAIL_LEVEL_WINDOW_CLOAKING_SUPPORTED (0x00000040)
        /// Window cloaking is supported.
        const WINDOW_CLOAKING_SUPPORTED = 0x0000_0040;

        /// TS_RAIL_LEVEL_HANDSHAKE_EX_SUPPORTED (0x00000080)
        /// Extended handshake is supported.
        const HANDSHAKE_EX_SUPPORTED = 0x0000_0080;
    }
}

impl Default for RailSupportLevel {
    fn default() -> Self {
        Self::SUPPORTED
    }
}

#[cfg(test)]
mod tests {
    use ironrdp_core::{decode, encode_vec};

    use super::*;

    #[test]
    fn rail_capability_set_basic_encode_decode() {
        let rail = Rail {
            rail_support_level: RailSupportLevel::SUPPORTED | RailSupportLevel::DOCKED_LANGBAR_SUPPORTED,
        };

        let encoded = encode_vec(&rail).unwrap();
        let decoded: Rail = decode(&encoded).unwrap();

        assert_eq!(rail, decoded);
    }

    #[test]
    fn rail_capability_set_all_flags_encode_decode() {
        let rail = Rail {
            rail_support_level: RailSupportLevel::all(),
        };

        let encoded = encode_vec(&rail).unwrap();
        let decoded: Rail = decode(&encoded).unwrap();

        assert_eq!(rail, decoded);
    }

    #[test]
    fn rail_capability_set_default() {
        let rail = Rail::default();
        assert_eq!(rail.rail_support_level, RailSupportLevel::SUPPORTED);
    }

    #[test]
    fn rail_capability_set_size() {
        let rail = Rail::default();
        assert_eq!(rail.size(), 4);
    }
}