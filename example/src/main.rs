use enum_index::EnumIndexGet;
use enum_index_derive::EnumIndex;

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