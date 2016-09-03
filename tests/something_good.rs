extern crate docstrings;

use docstrings::*;

#[test]
fn teaser() {
    assert_eq!(
        parse_md_docblock("\
Lorem ipsum
        ").unwrap(),
        DocBlock {
            teaser: "Lorem ipsum".into(),
            description: None,
            sections: vec![],
        }
    );
}

#[test]
fn description() {
    assert_eq!(
        parse_md_docblock("\
Lorem ipsum

A longer description lorem ipsum dolor sit amet. With multiple lines, of course,
to see that this is actually just one more paragraph in Markdown land.
        ").unwrap(),
        DocBlock {
            teaser: "Lorem ipsum".into(),
            description: Some("A longer description lorem ipsum dolor sit amet. With multiple lines, of course,
to see that this is actually just one more paragraph in Markdown land.".into()),
            sections: vec![],
        }
    );
}

#[test]
fn fancy_description() {
    assert_eq!(
        parse_md_docblock("\
Lorem ipsum

A longer description lorem ipsum dolor sit amet. With multiple lines, of course,
to see that this is actually just one more paragraph in Markdown land.

But actually, let's add more fancy stuff, e.g., a list:

- Yes, a list
- Lists are cool
- I write lists now

## Sub-headline for description

Yes, we are using subheadlines as well. To confuse everyone.

Very nice.
        ").unwrap(),
        DocBlock {
            teaser: "Lorem ipsum".into(),
            description: Some("A longer description lorem ipsum dolor sit amet. With multiple lines, of course,
to see that this is actually just one more paragraph in Markdown land.

But actually, let's add more fancy stuff, e.g., a list:

- Yes, a list
- Lists are cool
- I write lists now


## Sub-headline for description

Yes, we are using subheadlines as well. To confuse everyone.

Very nice.".into()),
            sections: vec![],
        }
    );
}

#[test]
fn parameters() {
    assert_eq!(
        parse_md_docblock("\
Lorem ipsum

# Parameters

- `foo`: Bar
        ").unwrap(),
        DocBlock {
            teaser: "Lorem ipsum".into(),
            description: None,
            sections: vec![
                DocSection::Parameters(vec![
                    ("foo".into(), "Bar".into()),
                ]),
            ],
        }
    );
}

#[test]
fn type_parameters() {
    assert_eq!(
        parse_md_docblock("\
Lorem ipsum

# Type Parameters

- `T`: Some type
        ").unwrap(),
        DocBlock {
            teaser: "Lorem ipsum".into(),
            description: None,
            sections: vec![
                DocSection::TypeParameters(vec![
                    ("T".into(), "Some type".into()),
                ]),
            ],
        }
    );
}

#[test]
fn lifetimes() {
    assert_eq!(
        parse_md_docblock("\
Lorem ipsum

# Lifetimes

- `'foo`: The life time of foo
        ").unwrap(),
        DocBlock {
            teaser: "Lorem ipsum".into(),
            description: None,
            sections: vec![
                DocSection::LifetimeParameters(vec![
                    ("'foo".into(), "The life time of foo".into()),
                ]),
            ],
        }
    );
}

#[test]
fn returns() {
    assert_eq!(
        parse_md_docblock("\
Lorem ipsum

# Returns

This returns a wonderful `Result`, with is either:

- `Ok(Wonderful)`: A gloriously positive answer
- `Err(Misantropy)`: Also a valid answer
        ").unwrap(),
        DocBlock {
            teaser: "Lorem ipsum".into(),
            description: None,
            sections: vec![
                DocSection::Returns(
                    "This returns a wonderful `Result`, with is either:".into(),
                    vec![
                        ("Ok(Wonderful)".into(), "A gloriously positive answer".into()),
                        ("Err(Misantropy)".into(), "Also a valid answer".into()),
                    ]
                ),
            ],
        }
    );
}

#[test]
fn custom_sections() {
    assert_eq!(
        parse_md_docblock("\
Lorem ipsum

# Custom1

Lorem ipsum

# Custom Two

dolor sit amet
        ").unwrap(),
        DocBlock {
            teaser: "Lorem ipsum".into(),
            description: None,
            sections: vec![
                DocSection::Custom("Custom1".into(), "Lorem ipsum".into()),
                DocSection::Custom("Custom Two".into(), "dolor sit amet".into()),
            ],
        }
    );
}

#[test]
fn kitchensink() {
    assert_eq!(
        parse_md_docblock(r#"Fooify a `Foo` with a label

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

- `label`: A string labelling the foo
- `magic`: A `Foo` that will be labeled

# Returns

A `Result` which is:

- `Ok`: A `Bar` that is the labeled `Foo` and thus lives as long as the
    `Foo` given in `magic`.
- `Err`: Returns the number of gravely appalled people (per half-century
    per country) if you were to use that label *and* `Foo`'s acceptance
    indicator is less than it.

# Type parameters

- `T`: A type that can be converted into a `FooLabel`, e.g. a `String`, a
    `BananaPeelRope`, or a `Cow<str>`.

# Lifetimes

- `floof`: The life time of the given foo as determined by the floof source
    it was originally loaded from.

# Examples

```rust
assert_eq!(fooify("lorem", Foo::extract_from_global_floof_resource()).label(),
           Bar::with_label("lorem"))
```
    "#).unwrap(),
        DocBlock {
            teaser: "Fooify a `Foo` with a label".into(),
            description: Some("A longer description lorem ipsum dolor sit amet. With multiple lines, of course,
to see that this is actually just one more paragraph in Markdown land.

But actually, let's add more fancy stuff, e.g., a list:

- Yes, a list
- Lists are cool
- I write lists now


## Sub-headline for description

Yes, we are using subheadlines as well. To confuse everyone.

Very nice.".into()),
            sections: vec![
                DocSection::Parameters(vec![
                    ("label".into(), "A string labelling the foo".into()),
                    ("magic".into(), "A `Foo` that will be labeled".into()),
                ]),
                DocSection::Returns(
                    "A `Result` which is:".into(),
                    vec![
                        ("Ok".into(), "A `Bar` that is the labeled `Foo` and thus lives as long as the
`Foo` given in `magic`.".into()),
                        ("Err".into(), "Returns the number of gravely appalled people (per half-century
per country) if you were to use that label *and* `Foo`'s acceptance
indicator is less than it.".into()),
                    ]
                ),
                DocSection::TypeParameters(vec![
                    ("T".into(), "A type that can be converted into a `FooLabel`, e.g. a `String`, a
`BananaPeelRope`, or a `Cow<str>`.".into()),
                ]),
                DocSection::LifetimeParameters(vec![
                    ("floof".into(), "The life time of the given foo as determined by the floof source
it was originally loaded from.".into()),
                ]),
                DocSection::Custom("Examples".into(), r#"```rust
assert_eq!(fooify("lorem", Foo::extract_from_global_floof_resource()).label(),
           Bar::with_label("lorem"))
```"#.into()),
            ],
        }
    );
}
