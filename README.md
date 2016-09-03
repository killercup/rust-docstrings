# Markdown doc string parser

This is a **proof of concept**.

This Rust library can be used to extract some data from documentation formatted as described [here](https://scribbles.pascalhertleif.de/machine-readable-inline-markdown-code-cocumentation.html).

For example, given a string like this one:

```markdown
Lorem ipsum

A longer description lorem ipsum dolor sit amet.

# Parameters

- `param1`: Foo
- `param2`: Bar
```

it will return structure like this:

```rust
DocBlock {
    teaser: "Lorem ipsum",
    description: Some("A longer description lorem ipsum dolor sit amet."),
    sections: [
        Parameters([
            ("param1", "Foo"),
            ("param2", "Bar")
        ])
    ]
}
```

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
