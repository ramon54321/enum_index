use enum_index_repr::EnumIndexGet;
use enum_index_repr_derive::EnumIndex;

#[repr(u16)]
#[derive(EnumIndex)]
enum Tag {
    Hello,
    World,
    How,
    Are,
    You,
}

fn main() {
    assert_eq!(Tag::How.index(), 2);
}
