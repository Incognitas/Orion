pub const NULL_HANDLE: i16 = 0;

// some sizes...
pub const REFERENCE_SIZE: i16 = 2;
pub const BYTE_SIZE: i16 = 1;

// constants
// visibility mask : global or not global (this will impact firewall rules)
#[derive(Debug)]
#[repr(u8)]
pub enum ObjectFlags {
    GLOBAL = 0x1,
    ARRAY = 0x2,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum PrimitiveType {
    UNKNOWN = 0x0,
    REFERENCE = 0x1,
    BYTE = 0x2,
    SHORT = 0x4,
    INTEGER = 0x8,
}
