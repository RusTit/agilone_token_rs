initSidebarItems({"attr":[["parser","The core attribute macro. Can only be applied to free functions with at least one parameter and a return value. To typecheck, the free function must meet the following typing requirements:"]],"macro":[["impl_show_with","Implements the `Show` trait for $($T)+ using the existing trait `$trait`."],["parse","Runs the parser with the given name and input, then [`parsers::eof()`]."],["parse_context","Returns the context from the current mark to the input position inclusive."],["parse_current_marker","Return the mark at the current parsing position."],["parse_error","Returns an `Err(ParseError::new($e))`. Can used like `format!` as well."],["parse_last_marker","Returns the last marker that was set."],["parse_mark","Sets the marker to the current position."],["parse_try","Runs a parser returning `Some` if it succeeds or `None` otherwise."],["switch","Invoked much like match, except each condition must be a parser, which is executed, and the corresponding arm is executed only if the parser succeeds. Once a condition succeeds, no other condition is executed."]]});