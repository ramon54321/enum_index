<div align="center">
    <span><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Rust_programming_language_black_logo.svg/1920px-Rust_programming_language_black_logo.svg.png" width="100"></span>
</div>

### EnumIndex

EnumIndex provides a small macro which implementes EnumIndexGet, permitting the following:

```
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
```
