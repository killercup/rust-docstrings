extern crate pulldown_cmark;

fn debug_markdown_events(markdown: &str) {
    let parser = pulldown_cmark::Parser::new(markdown);
    for event in parser {
        println!("{:?}", event);
    }
}

fn main() {
    debug_markdown_events("\
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

- `param1`: Foo lorem ipsum dolor long line right here
    yes this was it
- `param2`: Bar

# Examples

```rust
fn name(arg: Type) -> RetType {
    unimplemented!();
}
```
    ");
}
