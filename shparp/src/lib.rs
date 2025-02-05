pub use pest::Parser;
pub use pest::iterators::Pair;
pub use pest::iterators::Pairs;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "shell.pest"]
pub struct ShellParser;
