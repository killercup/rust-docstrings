use pulldown_cmark::{Event, Tag};

pub fn md<'a, I>(events: I) -> String where
    I: Iterator<Item = Event<'a>>,
{
    let mut res = String::new();

    for event in events {
        match event {
            Event::Text(t) => {
                res.push_str(&t);
            }
            Event::SoftBreak => {
                res.push_str("\n");
            }
            Event::Start(Tag::Header(n)) => {
                res.push_str("\n");
                res.extend(::std::iter::repeat("#").take(n as usize));
                res.push_str(" ");
            }
            Event::Start(Tag::Item) => {
                res.push_str("- ");
            }
            Event::End(Tag::Item) |
            Event::End(Tag::List(_)) => {
                res.push_str("\n");
            }
            Event::End(Tag::Paragraph) |
            Event::End(Tag::Header(_)) => {
                res.push_str("\n\n");
            }
            _ => {}
        }
    }

    res
}
