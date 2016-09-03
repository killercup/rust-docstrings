/// A Rust identifier
pub type Identifier = String;

/// A Rust pattern, as used in pattern matching.
pub type Pattern = String;

/// Documentation, a Markdown `String`
pub type Documentation = String;

/// The headline of a section
pub type SectionHeadline = String;

/// Information extracted from a doc comment
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct DocBlock {
    /// First line
    pub teaser: String,
    /// Paragraphs after first line
    pub description: Option<Documentation>,
    /// Sections
    pub sections: Vec<DocSection>,
}

/// Documentation sections
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum DocSection {
    /// Function parameters, mapping param name to docs
    Parameters(Vec<(Identifier, Documentation)>),
    /// Type parameters (generics), mapping ident of generic to docs
    TypeParameters(Vec<(Identifier, Documentation)>),
    /// Lifetime parameters, documenting the life and death of your times
    LifetimeParameters(Vec<(Identifier, Documentation)>),
    /// Return value documentation with optional list of enum variants.
    Returns(Documentation, Vec<(Pattern, Documentation)>),
    /// Custom/unknown sections, mapping headlines to docs
    ///
    /// In the future, some of the sections currently treated as 'custom' may
    /// be added as new variants, e.g. 'Examples', or 'Panics'.
    Custom(SectionHeadline, Documentation),
}
