use crate::la_dfa_2_dot_grammar_trait::{LaDfa2Dot, LaDfa2DotGrammarTrait};
#[allow(unused_imports)]
use parol_runtime::Result;
use std::fmt::{Debug, Display, Error, Formatter};

///
/// Data structure that implements the semantic actions for our LaDfa2Dot grammar
/// !Change this type as needed!
///
#[derive(Debug, Default)]
pub struct LaDfa2DotGrammar<'t> {
    pub la_dfa_2_dot: Option<LaDfa2Dot<'t>>,
}

impl LaDfa2DotGrammar<'_> {
    pub fn new() -> Self {
        LaDfa2DotGrammar::default()
    }
}

impl Display for LaDfa2Dot<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        write!(f, "{:?}", self)
    }
}

impl Display for LaDfa2DotGrammar<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        match &self.la_dfa_2_dot {
            Some(la_dfa_2_dot) => writeln!(f, "{}", la_dfa_2_dot),
            None => write!(f, "No parse result"),
        }
    }
}

impl<'t> LaDfa2DotGrammarTrait<'t> for LaDfa2DotGrammar<'t> {
    // !Adjust your implementation as needed!

    /// Semantic action for non-terminal 'LaDfa2Dot'
    fn la_dfa2_dot(&mut self, arg: &LaDfa2Dot<'t>) -> Result<()> {
        self.la_dfa_2_dot = Some(arg.clone());
        Ok(())
    }
}
