extern crate docstrings;

use docstrings::parse_md_docblock;

fn main() {
    println!("{:#?}", parse_md_docblock(DOC_STR).unwrap());
}

const DOC_STR: &'static str = "\
Lorem ipsum

A longer description lorem ipsum dolor sit amet.

# Parameters

- `param1`: Foo
- `param2`: Bar
";
