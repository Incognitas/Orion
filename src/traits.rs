use constants::PrimitiveType;

pub trait HasType {
    fn is_of_type(&self, cmp_val: PrimitiveType) -> bool;
}
