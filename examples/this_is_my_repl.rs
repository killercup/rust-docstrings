extern crate docstrings;

use docstrings::parse_md_docblock;

fn main() {
    println!("{:#?}", parse_md_docblock(DOC_STR).unwrap());
}

const DOC_STR: &'static str = "\
A short teaser. Very short.
But on two lines. Haha!

A longer description lorem ipsum dolor sit amet. With multiple lines, of course,
to see that this is actually just one more paragraph in Markdown land.

But actually, let's add more fancy stuff, e.g., a list:

- Yes, a list
- Lists are cool
- I write lists now

## Sub-headline for description

Yes, we are using subheadlines as well. To confuse everyone.

Very nice.

# Parameters

- `param1`: Foo
- `param2`: Bar
";
