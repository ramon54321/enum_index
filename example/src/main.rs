use enum_index_repr::{EnumIndex, EnumIndexGet};

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
