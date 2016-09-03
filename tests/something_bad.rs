extern crate docstrings;

use docstrings::*;

#[test]
fn no_teaser() {
    assert_eq!(
        parse_md_docblock("\
# Parameters

- `foo`: Bar").unwrap_err(),
        docstrings::ParseError::NoTeaser
    );
}

#[test]
fn section_not_starting_with_list() {
    assert_eq!(
        parse_md_docblock("\
Lorem ipsum

# Parameters

Dafuq

- `foo`: Bar").unwrap_err(),
        docstrings::ParseError::UnexpectedMarkdown("Parameters".into(), "Start(Paragraph)".into())
    );
}

#[test]
fn no_ident_in_ident_list() {
    assert_eq!(
        parse_md_docblock("\
Lorem ipsum

# Parameters

- Bar
    ").unwrap_err(),
        docstrings::ParseError::NoIdent
    );
}


#[test]
fn broken_ident_list() {
    assert_eq!(
        parse_md_docblock("\
Lorem ipsum

# Parameters

- `foo`: Bar
- `other` some other thing
    ").unwrap_err(),
        docstrings::ParseError::WrongIdentDocsSeparator
    );
}
