extern crate pulldown_cmark;
#[macro_use] extern crate quick_error;

use std::iter::Peekable;
use pulldown_cmark::{Parser, Event, Tag};

mod types;
use types::*;

mod errors;
use errors::ParseError;

mod to_md;
use to_md::md;

// markdown event helpers

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

// Extract data from docs

fn extract_teaser<'a, I>(events: &mut I) -> Result<String, ParseError> where
    I: Iterator<Item=Event<'a>>,
{
    if let Some(Event::Start(Tag::Paragraph)) = events.next() {
        let mut teaser = String::new();
        loop {
            match events.next() {
                None | Some(Event::End(Tag::Paragraph)) => break,
                Some(Event::Text(text)) => teaser.push_str(&text),
                Some(Event::SoftBreak) => teaser.push_str("\n"),
                Some(unexpected) => return Err(ParseError::UnexpectedMarkdown(
                    "Teaser".into(), format!("{:?}", unexpected))),
            }
        }
        Ok(teaser)
    } else {
        Err(ParseError::NoTeaser)
    }
}

fn extract_description<'a, I>(events: &mut I) -> Result<Option<String>, ParseError> where
    I: Iterator<Item=Event<'a>>,
{
    let description = md(events.take_while(not!(start Tag::Header(1))));

    Ok(Some(description))
}

fn extract_sections<'a, I>(events: &mut Peekable<I>) -> Result<Vec<DocSection>, ParseError> where
    I: Iterator<Item=Event<'a>>,
{
    let mut sections: Vec<DocSection> = vec![];

    loop {
        let go_on = {
            if let Some(&Event::Text(_)) = events.peek() {
                true
            } else {
                false
            }
        };

        if go_on {
            sections.push(try!(extract_section(events)));
            continue;
        }

        match events.next() {
            None => break,
            Some(unexpected) => return Err(ParseError::UnexpectedMarkdown(
                "Sections".into(), format!("{:?}", unexpected))),
        }
    }

    Ok(sections)
}

fn extract_section<'a, I>(events: &mut I) -> Result<DocSection, ParseError> where
    I: Iterator<Item=Event<'a>>,
{
    // the next item is the value after a `Event::Start(Tag::Header(1))`
    let headline = md(events.take_while(not!(end Tag::Header(1))));

    // What kind of headline are we dealing with?
    let section = match &headline.trim().to_lowercase()[..] {
        "parameters" =>
            DocSection::Parameters(try!(extract_list(events))),
        "type parameters" =>
            DocSection::TypeParameters(try!(extract_list(events))),
        "lifetime parameters" | "lifetimes" =>
            DocSection::LifetimeParameters(try!(extract_list(events))),
        "returns" =>
            DocSection::Returns(
                md(events.take_while(not!(start Tag::List(_)))),
                try!(extract_list(events))),
        _ =>
            DocSection::Custom(
                headline,
                md(events.take_while(not!(start Tag::Header(1))))),
    };

    Ok(section)
}

fn extract_list<'a, I>(events: &mut I) -> Result<Vec<(Identifier, Documentation)>, ParseError> where
    I: Iterator<Item=Event<'a>>,
{
    let mut list = vec![];

    loop {
        match events.next() {
            Some(Event::Start(Tag::List(_))) => continue,
            None | Some(Event::End(Tag::List(_))) => break,
            Some(Event::Start(Tag::Item)) => list.push(try!(extract_list_item(events))),
            Some(unexpected) => return Err(ParseError::UnexpectedMarkdown(
                "List item".into(), format!("{:?}", unexpected))),
        }
    }

    Ok(list)
}

fn extract_list_item<'a, I>(events: &mut I) -> Result<(Identifier, Documentation), ParseError> where
    I: Iterator<Item=Event<'a>>,
{
    let ident = if let Some(Event::Start(Tag::Code)) = events.next() {
        md(events.take_while(not!(end Tag::Code)))
    } else {
        return Err(ParseError::NoIdent);
    };

    let mut docs = md(events.take_while(not!(end Tag::Item)));

    if docs.starts_with(": ") {
        docs = docs.trim_left_matches(": ").into();
    } else {
        return Err(ParseError::WrongIdentDocsSeparator);
    }

    Ok((ident, docs))
}

// And go!
pub fn parse_md_docblock(md: &str) -> Result<DocBlock, ParseError> {
    let mut md_events = Parser::new(md).peekable();

    Ok(DocBlock {
        teaser: try!(extract_teaser(&mut md_events)),
        description: try!(extract_description(&mut md_events)),
        sections: try!(extract_sections(&mut md_events)),
    })
}
