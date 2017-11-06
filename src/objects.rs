use constants;

// a structure representing an object
pub struct JCVMObject {
    owner: i16,
    object_type: i16,
    object_length: i16, // length in terms of raw length (not in terms of items etc)
    persistent: bool,
    pub content: Vec<u8>, // sometimes we don't have arrays
}

impl JCVMObject {
    pub fn new(owner: i16, type_: i16, length: i16, persistent: bool) -> JCVMObject {
        JCVMObject {
            owner: owner,
            object_type: type_,
            object_length: length,
            persistent: persistent,
            content: Vec::with_capacity(length as usize),
        }
    }

    pub fn owner(&self) -> i16 {
        self.owner
    }

    pub fn object_type(&self) -> i16 {
        self.object_type
    }

    pub fn object_length(&self) -> i16 {
        self.object_length
    }

    pub fn is_persistent(&self) -> bool {
        self.persistent
    }
}
