extern crate pulldown_cmark;
#[macro_use] extern crate quick_error;

use pulldown_cmark::Parser;

mod types;
mod errors;
mod to_md;
mod extractors;

pub use ::errors::ParseError;
pub use types::*;

pub fn parse_md_docblock(md: &str) -> Result<DocBlock, ParseError> {
    let mut md_events = Parser::new(md).peekable();

    Ok(DocBlock {
        teaser: try!(extractors::teaser(&mut md_events)),
        description: try!(extractors::description(&mut md_events)),
        sections: try!(extractors::sections(&mut md_events)),
    })
}
