pub const NULL_HANDLE: i16 = 0;

// constants
// visibility mask : global or not global (this will impact firewall rules)
pub const VISIBILITY_MASK: u16 = 0x8000;
pub const VISIBILITY_GLOBAL: u16 = 0x8000;

// object type
pub const OBJECT_TYPE_MASK: u16 = 0x7000;
pub const OBJECT_TYPE_ARRAY: u16 = 0x4000;

// arrays of primitive types
pub const OBJECT_TYPE_ARRAY_REFERENCES: u16 = OBJECT_TYPE_ARRAY | 0x0000;
pub const OBJECT_TYPE_ARRAY_BYTES: u16 = OBJECT_TYPE_ARRAY | 0x0100;
pub const OBJECT_TYPE_ARRAY_SHORTS: u16 = OBJECT_TYPE_ARRAY | 0x0200;
pub const OBJECT_TYPE_ARRAY_INTEGERS: u16 = OBJECT_TYPE_ARRAY | 0x0300;
