use ::std::iter::Peekable;
use ::pulldown_cmark::{Event, Tag};

use ::errors::ParseError;
use ::offset_parser::fst;
use ::to_md::md;
use ::types::*;

macro_rules! not {
    (start $tag:pat) => (not!(Event::Start($tag)));
    (end $tag:pat) => (not!(Event::End($tag)));
    ($tag:pat) => (
        |event| {
            if let $tag = *event {
                false
            } else {
                true
            }
        }
    );
}

pub fn teaser<'a, I>(events: &mut I) -> Result<WithOffset<String>, ParseError> where
    I: Iterator<Item=(Event<'a>, usize)>,
{
    if let Some((Event::Start(Tag::Paragraph), offset)) = events.next() {
        Ok(WithOffset {
            inner: md(events.map(fst).take_while(not!(end Tag::Paragraph))),
            offset: offset,
        })
    } else {
        Err(ParseError::NoTeaser)
    }
}

pub fn description<'a, I>(events: &mut Peekable<I>) -> Result<Option<WithOffset<String>>, ParseError> where
    I: Iterator<Item=(Event<'a>, usize)>,
{
    let offset = events.peek().map(|&(_, offset)| offset).unwrap_or(0);
    let description = md(events.map(fst).take_while(not!(start Tag::Header(1))));

    if description.is_empty() {
        return Ok(None);
    }

    Ok(Some(WithOffset {
        inner: description,
        offset: offset,
    }))
}

pub fn sections<'a, I>(events: &mut Peekable<I>) -> Result<Vec<WithOffset<DocSection>>, ParseError> where
    I: Iterator<Item=(Event<'a>, usize)>,
{
    let mut sections: Vec<WithOffset<DocSection>> = vec![];

    loop {
        // Assume the previous item was the start of an `Header(1)`, so a
        // section begins witht he text content of that header.
        //
        // TODO: Handle that more nicely (see note on `take_while` above)
        // TODO: Deal with headlines not starting with text (i.e., those
        // starting with inline markdown).
        let (go_on, offset) = {
            // Peek next item without consuming
            //
            // This is inside a block to ahve the handle on events is dropped
            // sooner (so we can use it in `events.next()` below).
            if let Some(&(Event::Text(_), offset)) = events.peek() {
                (true, offset)
            } else {
                (false, 0)
            }
        };

        if go_on {
            sections.push(WithOffset {
                inner: try!(section(events)),
                offset: offset,
            });
            continue;
        }

        match events.next() {
            None => break,
            // Next section
            Some((Event::Start(Tag::Header(1)), offset)) => {
                sections.push(WithOffset {
                    inner: try!(section(events)),
                    offset: offset,
                });
            },
            Some(unexpected) =>
                return Err(ParseError::UnexpectedMarkdown(
                    "Sections".into(), format!("{:?}", unexpected))),
        }
    }

    Ok(sections)
}

fn section<'a, I>(events: &mut I) -> Result<DocSection, ParseError> where
    I: Iterator<Item=(Event<'a>, usize)>,
{
    // the next item is the value after a `Event::Start(Tag::Header(1))`
    let headline = md(events.map(fst).take_while(not!(end Tag::Header(1))));

    // What kind of headline are we dealing with?
    let section = match &headline.trim().to_lowercase()[..] {
        "parameters" =>
            DocSection::Parameters(try!(list(events, &headline))),
        "type parameters" =>
            DocSection::TypeParameters(try!(list(events, &headline))),
        "lifetime parameters" | "lifetimes" =>
            DocSection::LifetimeParameters(try!(list(events, &headline))),
        "returns" =>
            DocSection::Returns(
                md(events.map(fst).take_while(not!(start Tag::List(_)))),
                try!(list(events, &headline))),
        _ =>
            DocSection::Custom(
                headline,
                md(events.map(fst).take_while(not!(start Tag::Header(1))))),
    };

    Ok(section)
}

fn list<'a, I>(events: &mut I, section: &str) -> Result<Vec<(Identifier, Documentation)>, ParseError> where
    I: Iterator<Item=(Event<'a>, usize)>,
{
    let mut list = vec![];

    loop {
        match events.next() {
            Some((Event::Start(Tag::List(_)), _)) => continue,
            None | Some((Event::End(Tag::List(_)), _)) => break,
            Some((Event::Start(Tag::Item), _)) => list.push(try!(list_item(events))),
            Some(unexpected) => return Err(ParseError::UnexpectedMarkdown(
                section.into(), format!("{:?}", unexpected))),
        }
    }

    Ok(list)
}

fn list_item<'a, I>(events: &mut I) -> Result<(Identifier, Documentation), ParseError> where
    I: Iterator<Item=(Event<'a>, usize)>,
{
    let ident = if let Some((Event::Start(Tag::Code), _)) = events.next() {
        md(events.map(fst).take_while(not!(end Tag::Code)))
    } else {
        return Err(ParseError::NoIdent);
    };

    let mut docs = md(events.map(fst).take_while(not!(end Tag::Item)));

    if docs.starts_with(": ") {
        docs = docs.trim_left_matches(": ").into();
    } else {
        return Err(ParseError::WrongIdentDocsSeparator);
    }

    Ok((ident, docs))
}
