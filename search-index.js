var searchIndex = {};
searchIndex["bitflags"] = {"doc":"A typesafe bitmask flag generator.","items":[[6,"__BitFlagsWidth","bitflags","",null,null],[14,"bitflags","","The `bitflags!` macro generates a `struct` that holds a set of C-style bitmask flags. It is useful for creating typesafe wrappers for C APIs.",null,null]],"paths":[]};
searchIndex["docstrings"] = {"doc":"Extract data from documentation strings.","items":[[3,"DocBlock","docstrings","Information extracted from a doc comment",null,null],[12,"teaser","","First line",0,null],[12,"description","","Paragraphs after first line",0,null],[12,"sections","","Sections",0,null],[4,"ParseError","","Errors while parsing documention",null,null],[13,"NoTeaser","","Missing teaser",1,null],[13,"UnexpectedMarkdown","","Unexpected markdown text",1,null],[13,"NoIdent","","List not starting with an identifier (inline code)",1,null],[13,"WrongIdentDocsSeparator","","Invalid list formatting with identifier/docs",1,null],[4,"DocSection","","Documentation sections",null,null],[13,"Parameters","","Function parameters, mapping param name to docs",2,null],[13,"TypeParameters","","Type parameters (generics), mapping ident of generic to docs",2,null],[13,"LifetimeParameters","","Lifetime parameters, documenting the life and death of your times",2,null],[13,"Returns","","Return value documentation with optional list of enum variants.",2,null],[13,"Custom","","Custom/unknown sections, mapping headlines to docs",2,null],[5,"parse_md_docblock","","Parse documentation and extract data",null,{"inputs":[{"name":"str"}],"output":{"name":"result"}}],[5,"parse_md_docblock_events","","Parse documentation and extract data",null,{"inputs":[{"name":"peekable"}],"output":{"name":"result"}}],[11,"fmt","","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",0,{"inputs":[{"name":"self"},{"name":"docblock"}],"output":{"name":"bool"}}],[11,"ne","","",0,{"inputs":[{"name":"self"},{"name":"docblock"}],"output":{"name":"bool"}}],[11,"clone","","",0,{"inputs":[{"name":"self"}],"output":{"name":"docblock"}}],[11,"hash","","",0,null],[11,"fmt","","",2,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",2,{"inputs":[{"name":"self"},{"name":"docsection"}],"output":{"name":"bool"}}],[11,"ne","","",2,{"inputs":[{"name":"self"},{"name":"docsection"}],"output":{"name":"bool"}}],[11,"clone","","",2,{"inputs":[{"name":"self"}],"output":{"name":"docsection"}}],[11,"hash","","",2,null],[11,"fmt","","",1,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",1,{"inputs":[{"name":"self"},{"name":"parseerror"}],"output":{"name":"bool"}}],[11,"ne","","",1,{"inputs":[{"name":"self"},{"name":"parseerror"}],"output":{"name":"bool"}}],[11,"clone","","",1,{"inputs":[{"name":"self"}],"output":{"name":"parseerror"}}],[11,"hash","","",1,null],[11,"fmt","","",1,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"description","","",1,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[11,"cause","","",1,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[6,"Identifier","","A Rust identifier",null,null],[6,"Pattern","","A Rust pattern, as used in pattern matching.",null,null],[6,"Documentation","","Documentation, a Markdown `String`",null,null],[6,"SectionHeadline","","The headline of a section",null,null]],"paths":[[3,"DocBlock"],[4,"ParseError"],[4,"DocSection"]]};
searchIndex["getopts"] = {"doc":"Simple getopt alternative.","items":[[3,"Options","getopts","A description of the options that a program can handle.",null,null],[3,"Matches","","The result of checking command line arguments. Contains a vector of matches and a vector of free strings.",null,null],[12,"free","","Free string fragments",0,null],[4,"ParsingStyle","","What parsing style to use when parsing arguments.",null,null],[13,"FloatingFrees","","Flags and \"free\" arguments can be freely inter-mixed.",1,null],[13,"StopAtFirstFree","","As soon as a \"free\" argument (i.e. non-flag) is encountered, stop considering any remaining arguments as flags.",1,null],[4,"HasArg","","Describes whether an option has an argument.",null,null],[13,"Yes","","The option requires an argument.",2,null],[13,"No","","The option takes no argument.",2,null],[13,"Maybe","","The option argument is optional.",2,null],[4,"Occur","","Describes how often an option may occur.",null,null],[13,"Req","","The option occurs once.",3,null],[13,"Optional","","The option occurs at most once.",3,null],[13,"Multi","","The option occurs zero or more times.",3,null],[4,"Fail","","The type returned when the command line does not conform to the expected format. Use the `Debug` implementation to output detailed information.",null,null],[13,"ArgumentMissing","","The option requires an argument but none was passed.",4,null],[13,"UnrecognizedOption","","The passed option is not declared among the possible options.",4,null],[13,"OptionMissing","","A required option is not present.",4,null],[13,"OptionDuplicated","","A single occurrence option is being used multiple times.",4,null],[13,"UnexpectedArgument","","There's an argument being passed to a non-argument option.",4,null],[4,"FailType","","The type of failure that occurred.",null,null],[13,"ArgumentMissing_","","",5,null],[13,"UnrecognizedOption_","","",5,null],[13,"OptionMissing_","","",5,null],[13,"OptionDuplicated_","","",5,null],[13,"UnexpectedArgument_","","",5,null],[6,"Result","","The result of parsing a command line with a set of options.",null,null],[11,"new","","Create a blank set of options.",6,{"inputs":[],"output":{"name":"options"}}],[11,"parsing_style","","Set the parsing style.",6,{"inputs":[{"name":"self"},{"name":"parsingstyle"}],"output":{"name":"options"}}],[11,"opt","","Create a generic option group, stating all parameters explicitly.",6,{"inputs":[{"name":"self"},{"name":"str"},{"name":"str"},{"name":"str"},{"name":"str"},{"name":"hasarg"},{"name":"occur"}],"output":{"name":"options"}}],[11,"optflag","","Create a long option that is optional and does not take an argument.",6,{"inputs":[{"name":"self"},{"name":"str"},{"name":"str"},{"name":"str"}],"output":{"name":"options"}}],[11,"optflagmulti","","Create a long option that can occur more than once and does not take an argument.",6,{"inputs":[{"name":"self"},{"name":"str"},{"name":"str"},{"name":"str"}],"output":{"name":"options"}}],[11,"optflagopt","","Create a long option that is optional and takes an optional argument.",6,{"inputs":[{"name":"self"},{"name":"str"},{"name":"str"},{"name":"str"},{"name":"str"}],"output":{"name":"options"}}],[11,"optmulti","","Create a long option that is optional, takes an argument, and may occur multiple times.",6,{"inputs":[{"name":"self"},{"name":"str"},{"name":"str"},{"name":"str"},{"name":"str"}],"output":{"name":"options"}}],[11,"optopt","","Create a long option that is optional and takes an argument.",6,{"inputs":[{"name":"self"},{"name":"str"},{"name":"str"},{"name":"str"},{"name":"str"}],"output":{"name":"options"}}],[11,"reqopt","","Create a long option that is required and takes an argument.",6,{"inputs":[{"name":"self"},{"name":"str"},{"name":"str"},{"name":"str"},{"name":"str"}],"output":{"name":"options"}}],[11,"parse","","Parse command line arguments according to the provided options.",6,{"inputs":[{"name":"self"},{"name":"c"}],"output":{"name":"result"}}],[11,"short_usage","","Derive a short one-line usage summary from a set of long options.",6,{"inputs":[{"name":"self"},{"name":"str"}],"output":{"name":"string"}}],[11,"usage","","Derive a usage message from a set of options.",6,{"inputs":[{"name":"self"},{"name":"str"}],"output":{"name":"string"}}],[11,"clone","","",1,{"inputs":[{"name":"self"}],"output":{"name":"parsingstyle"}}],[11,"eq","","",1,{"inputs":[{"name":"self"},{"name":"parsingstyle"}],"output":{"name":"bool"}}],[11,"clone","","",2,{"inputs":[{"name":"self"}],"output":{"name":"hasarg"}}],[11,"eq","","",2,{"inputs":[{"name":"self"},{"name":"hasarg"}],"output":{"name":"bool"}}],[11,"clone","","",3,{"inputs":[{"name":"self"}],"output":{"name":"occur"}}],[11,"eq","","",3,{"inputs":[{"name":"self"},{"name":"occur"}],"output":{"name":"bool"}}],[11,"clone","","",0,{"inputs":[{"name":"self"}],"output":{"name":"matches"}}],[11,"eq","","",0,{"inputs":[{"name":"self"},{"name":"matches"}],"output":{"name":"bool"}}],[11,"ne","","",0,{"inputs":[{"name":"self"},{"name":"matches"}],"output":{"name":"bool"}}],[11,"clone","","",4,{"inputs":[{"name":"self"}],"output":{"name":"fail"}}],[11,"fmt","","",4,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",4,{"inputs":[{"name":"self"},{"name":"fail"}],"output":{"name":"bool"}}],[11,"ne","","",4,{"inputs":[{"name":"self"},{"name":"fail"}],"output":{"name":"bool"}}],[11,"description","","",4,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[11,"clone","","",5,{"inputs":[{"name":"self"}],"output":{"name":"failtype"}}],[11,"eq","","",5,{"inputs":[{"name":"self"},{"name":"failtype"}],"output":{"name":"bool"}}],[11,"opt_present","","Returns true if an option was matched.",0,{"inputs":[{"name":"self"},{"name":"str"}],"output":{"name":"bool"}}],[11,"opt_count","","Returns the number of times an option was matched.",0,{"inputs":[{"name":"self"},{"name":"str"}],"output":{"name":"usize"}}],[11,"opts_present","","Returns true if any of several options were matched.",0,null],[11,"opts_str","","Returns the string argument supplied to one of several matching options or `None`.",0,null],[11,"opt_strs","","Returns a vector of the arguments provided to all matches of the given option.",0,{"inputs":[{"name":"self"},{"name":"str"}],"output":{"name":"vec"}}],[11,"opt_str","","Returns the string argument supplied to a matching option or `None`.",0,{"inputs":[{"name":"self"},{"name":"str"}],"output":{"name":"option"}}],[11,"opt_default","","Returns the matching string, a default, or `None`.",0,{"inputs":[{"name":"self"},{"name":"str"},{"name":"str"}],"output":{"name":"option"}}],[11,"fmt","","",4,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}]],"paths":[[3,"Matches"],[4,"ParsingStyle"],[4,"HasArg"],[4,"Occur"],[4,"Fail"],[4,"FailType"],[3,"Options"]]};
searchIndex["pulldown_cmark"] = {"doc":"Pull parser for commonmark.","items":[[3,"Parser","pulldown_cmark","",null,null],[3,"Options","","",null,null],[4,"Event","","",null,null],[13,"Start","","",0,null],[13,"End","","",0,null],[13,"Text","","",0,null],[13,"Html","","",0,null],[13,"InlineHtml","","",0,null],[13,"FootnoteReference","","",0,null],[13,"SoftBreak","","",0,null],[13,"HardBreak","","",0,null],[4,"Tag","","",null,null],[13,"Paragraph","","",1,null],[13,"Rule","","",1,null],[13,"Header","","",1,null],[13,"BlockQuote","","",1,null],[13,"CodeBlock","","",1,null],[13,"List","","",1,null],[13,"Item","","",1,null],[13,"FootnoteDefinition","","",1,null],[13,"Table","","",1,null],[13,"TableHead","","",1,null],[13,"TableRow","","",1,null],[13,"TableCell","","",1,null],[13,"Emphasis","","",1,null],[13,"Strong","","",1,null],[13,"Code","","",1,null],[13,"Link","","",1,null],[13,"Image","","",1,null],[0,"html","","HTML renderer that takes an iterator of events as input.",null,null],[5,"push_html","pulldown_cmark::html","",null,{"inputs":[{"name":"string"},{"name":"i"}],"output":null}],[11,"new","pulldown_cmark","",2,{"inputs":[{"name":"str"}],"output":{"name":"parser"}}],[11,"new_ext","","",2,{"inputs":[{"name":"str"},{"name":"options"}],"output":{"name":"parser"}}],[11,"get_offset","","",2,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"next","","",2,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"clone","","",1,{"inputs":[{"name":"self"}],"output":{"name":"tag"}}],[11,"fmt","","",1,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",0,{"inputs":[{"name":"self"}],"output":{"name":"event"}}],[11,"fmt","","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",3,{"inputs":[{"name":"self"},{"name":"options"}],"output":{"name":"bool"}}],[11,"ne","","",3,{"inputs":[{"name":"self"},{"name":"options"}],"output":{"name":"bool"}}],[11,"clone","","",3,{"inputs":[{"name":"self"}],"output":{"name":"options"}}],[11,"partial_cmp","","",3,{"inputs":[{"name":"self"},{"name":"options"}],"output":{"name":"option"}}],[11,"lt","","",3,{"inputs":[{"name":"self"},{"name":"options"}],"output":{"name":"bool"}}],[11,"le","","",3,{"inputs":[{"name":"self"},{"name":"options"}],"output":{"name":"bool"}}],[11,"gt","","",3,{"inputs":[{"name":"self"},{"name":"options"}],"output":{"name":"bool"}}],[11,"ge","","",3,{"inputs":[{"name":"self"},{"name":"options"}],"output":{"name":"bool"}}],[11,"cmp","","",3,{"inputs":[{"name":"self"},{"name":"options"}],"output":{"name":"ordering"}}],[11,"hash","","",3,null],[11,"fmt","","",3,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"empty","","Returns an empty set of flags.",3,{"inputs":[],"output":{"name":"options"}}],[11,"all","","Returns the set containing all flags.",3,{"inputs":[],"output":{"name":"options"}}],[11,"bits","","Returns the raw value of the flags currently stored.",3,{"inputs":[{"name":"self"}],"output":{"name":"u32"}}],[11,"from_bits","","Convert from underlying bit representation, unless that representation contains bits that do not correspond to a flag.",3,{"inputs":[{"name":"u32"}],"output":{"name":"option"}}],[11,"from_bits_truncate","","Convert from underlying bit representation, dropping any bits that do not correspond to flags.",3,{"inputs":[{"name":"u32"}],"output":{"name":"options"}}],[11,"is_empty","","Returns `true` if no flags are currently stored.",3,{"inputs":[{"name":"self"}],"output":{"name":"bool"}}],[11,"is_all","","Returns `true` if all flags are currently set.",3,{"inputs":[{"name":"self"}],"output":{"name":"bool"}}],[11,"intersects","","Returns `true` if there are flags common to both `self` and `other`.",3,{"inputs":[{"name":"self"},{"name":"options"}],"output":{"name":"bool"}}],[11,"contains","","Returns `true` all of the flags in `other` are contained within `self`.",3,{"inputs":[{"name":"self"},{"name":"options"}],"output":{"name":"bool"}}],[11,"insert","","Inserts the specified flags in-place.",3,{"inputs":[{"name":"self"},{"name":"options"}],"output":null}],[11,"remove","","Removes the specified flags in-place.",3,{"inputs":[{"name":"self"},{"name":"options"}],"output":null}],[11,"toggle","","Toggles the specified flags in-place.",3,{"inputs":[{"name":"self"},{"name":"options"}],"output":null}],[11,"set","","Inserts or removes the specified flags depending on the passed value.",3,{"inputs":[{"name":"self"},{"name":"options"},{"name":"bool"}],"output":null}],[11,"bitor","","Returns the union of the two sets of flags.",3,{"inputs":[{"name":"self"},{"name":"options"}],"output":{"name":"options"}}],[11,"bitor_assign","","Adds the set of flags.",3,{"inputs":[{"name":"self"},{"name":"options"}],"output":null}],[11,"bitxor","","Returns the left flags, but with all the right flags toggled.",3,{"inputs":[{"name":"self"},{"name":"options"}],"output":{"name":"options"}}],[11,"bitxor_assign","","Toggles the set of flags.",3,{"inputs":[{"name":"self"},{"name":"options"}],"output":null}],[11,"bitand","","Returns the intersection between the two sets of flags.",3,{"inputs":[{"name":"self"},{"name":"options"}],"output":{"name":"options"}}],[11,"bitand_assign","","Disables all flags disabled in the set.",3,{"inputs":[{"name":"self"},{"name":"options"}],"output":null}],[11,"sub","","Returns the set difference of the two sets of flags.",3,{"inputs":[{"name":"self"},{"name":"options"}],"output":{"name":"options"}}],[11,"sub_assign","","Disables all flags enabled in the set.",3,{"inputs":[{"name":"self"},{"name":"options"}],"output":null}],[11,"not","","Returns the complement of this set of flags.",3,{"inputs":[{"name":"self"}],"output":{"name":"options"}}],[11,"extend","","",3,{"inputs":[{"name":"self"},{"name":"t"}],"output":null}],[11,"from_iter","","",3,{"inputs":[{"name":"t"}],"output":{"name":"options"}}],[17,"OPTION_ENABLE_TABLES","","",null,null],[17,"OPTION_ENABLE_FOOTNOTES","","",null,null]],"paths":[[4,"Event"],[4,"Tag"],[3,"Parser"],[3,"Options"]]};
searchIndex["quick_error"] = {"doc":"A macro which makes errors easy to write","items":[[3,"Context","quick_error","",null,null],[12,"0","","",0,null],[12,"1","","",0,null],[8,"ResultExt","","",null,null],[10,"context","","",1,{"inputs":[{"name":"self"},{"name":"x"}],"output":{"name":"result"}}],[11,"fmt","","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[14,"quick_error","","Main macro that does all the work",null,null]],"paths":[[3,"Context"],[8,"ResultExt"]]};
initSearch(searchIndex);