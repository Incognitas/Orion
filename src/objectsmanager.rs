use objects::JCVMObject;
use exceptions::InterpreterException;

pub struct ObjectManager {
    objects_container: Vec<JCVMObject>,
}

impl ObjectManager {
    pub fn new() -> ObjectManager {
        ObjectManager {
            objects_container: Vec::new(),
        }
    }

    pub fn get_object(&self, index: usize) -> Result<&JCVMObject, InterpreterException> {
        if index < self.objects_container.len() {
            return Ok(&self.objects_container[index]);
        }
        Err(InterpreterException::ArrayIndexOutOfBoundsException)
    }
}
