pub trait EnumIndexGet {
    fn index(&self) -> u16;
}

pub use enum_index_repr_derive::EnumIndex;
