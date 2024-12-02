use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct CapabilityFlags: u32 {
        const COMPRESSION     = 0x0001;
        const ENCRYPTION     = 0x0002;
        const STREAMING      = 0x0004;
        const EDGE_COMPUTE   = 0x0008;
        const STATE_SYNC     = 0x0010;
        const SERVICE_MESH   = 0x0020;
        const LOAD_BALANCING = 0x0040;
        const TRACING        = 0x0080;
        const METRICS        = 0x0100;
        const CONSENSUS      = 0x0200;
        const CACHING        = 0x0400;
        const COMPRESSION_ZSTD = 0x0800;
        const COMPRESSION_LZ4  = 0x1000;
        const TLS_1_3         = 0x2000;
        const QUIC            = 0x4000;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ExtensionFlags: u32 {
        const CUSTOM_AUTH    = 0x0001;
        const CUSTOM_CODEC   = 0x0002;
        const CUSTOM_CRYPTO  = 0x0004;
        const CUSTOM_ROUTING = 0x0008;
        const CUSTOM_TRACING = 0x0010;
        const EXPERIMENTAL   = 0x8000;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProtocolVersion {
    pub major: u16,
    pub minor: u16,
}

impl ProtocolVersion {
    pub const CURRENT: Self = Self {
        major: 2,
        minor: 0,
    };

    pub fn is_compatible(&self, other: &Self) -> bool {
        self.major == other.major && self.minor >= other.minor
    }
} 