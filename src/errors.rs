quick_error! {
    /// Errors while parsing documention
    #[derive(Debug, PartialEq, Eq, Clone, Hash)]
    pub enum ParseError {
        /// Missing teaser
        NoTeaser {
            description("Doc comment did not start with a teaser.")
        }
        /// Unexpected markdown text
        UnexpectedMarkdown(section: String, event: String) {
            description("Unexpected Markdown")
            display("Unexpected Markdown in section `{sec}`: {event}",
                sec=section, event=event)
        }
        /// List not starting with an identifier (inline code)
        NoIdent {
            description("No identifier in list mapping ident -> docs")
        }
        /// Invalid list formatting with identifier/docs
        WrongIdentDocsSeparator {
            description("List identifier and doc string must be written like this: `itend`: Docs")
        }
    }
}
