//! Extract data from documentation strings.
//!
//! The expected format is described [here][1].
//!
//! [1]: https://scribbles.pascalhertleif.de/machine-readable-inline-markdown-code-cocumentation.html

#![deny(missing_docs, warnings, unsafe_code, missing_debug_implementations)]

extern crate pulldown_cmark;
#[macro_use] extern crate quick_error;

use pulldown_cmark::Parser;
use pulldown_cmark::Event;

mod types;
mod errors;
mod to_md;
mod extractors;

pub use errors::ParseError;
pub use types::*;

use ::std::iter::Peekable;

/// Parse documentation and extract data
///
/// # Parameters
///
/// - `md`: Markdown string, needs to be parseable by `pulldown-cmark`
///
/// # Returns
///
/// A `Result`, which is either
///
/// - `Ok(DocBlock)`: A type that contains all extracted information (including
///     all unknown sections as `Custom` sections).
/// - `Err(ParseError)`: The first encountered error while parsing the
///     documentation string.
///
/// # Examples
///
/// Please excuse the weird way the input is formatting in this example.
/// Embedding Markdown strings in Rust code examples, which are just code blocks
/// in Markdown documentation strings inside a Rust program is kinda hard:
/// Rustdoc treads `#` at the beginning of code example line as a sign it
/// should omit the line from output. Sadly, this means I can't write Markdown
/// headlines as usual.
///
/// ```rust
/// # use self::docstrings::*;
/// assert_eq!(parse_md_docblock(
///     "Lorem ipsum\n\nDolor sit amet.\n\n# Parameters\n\n- `param1`: Foo\n- `param2`: Bar\n"
/// ).unwrap(),
///     DocBlock {
///         teaser: "Lorem ipsum".into(),
///         description: Some("Dolor sit amet.".into()),
///         sections: vec![
///             DocSection::Parameters(vec![
///                 ("param1".into(), "Foo".into()),
///                 ("param2".into(), "Bar".into())
///             ])
///         ]
///     }
/// );
/// ```
pub fn parse_md_docblock(md: &str) -> Result<DocBlock, ParseError> {
    let mut md_events = Parser::new(md).peekable();
    parse_md_docblock_events(&mut md_events)
}

/// Parse documentation and extract data
///
/// # Parameters
///
/// - `md`: Markdown
///
/// # Returns
///
/// A `Result`, which is either
///
/// - `Ok(DocBlock)`: A type that contains all extracted information (including
///     all unknown sections as `Custom` sections).
/// - `Err(ParseError)`: The first encountered error while parsing the
///     documentation string.
pub fn parse_md_docblock_events<'a, I>(events: &mut Peekable<I>) -> Result<DocBlock, ParseError> where
    I: Iterator<Item=Event<'a>>,
{
    Ok(DocBlock {
        teaser: try!(extractors::teaser(events)),
        description: try!(extractors::description(events)),
        sections: try!(extractors::sections(events)),
    })
}
