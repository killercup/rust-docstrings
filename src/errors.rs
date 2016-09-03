quick_error! {
    /// Errors while parsing documention
    #[derive(Debug)]
    pub enum ParseError {
        NoTeaser {
            description("Doc comment did not start with a teaser.")
        }
        UnexpectedMarkdown(section: String, event: String) {
            description("Unexpected Markdown")
            display("Unexpected Markdown in section `{sec}`: {event}",
                sec=section, event=event)
        }
        NoIdent {
            description("No identifier in list mapping ident -> docs")
        }
        WrongIdentDocsSeparator {
            description("List identifier and doc string must be written like this: `itend`: Docs")
        }
    }
}
