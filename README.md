### EnumIndex

EnumIndex provides a small macro which implementes EnumIndexGet, permitting the following:

```
use enums::EnumIndexGet;
use enums_derive::EnumIndex;

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
```
