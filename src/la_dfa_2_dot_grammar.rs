use crate::la_dfa_2_dot_grammar_trait::{LaDfa2Dot, LaDfa2DotGrammarTrait};
#[allow(unused_imports)]
use parol_runtime::Result;
use std::{
    collections::HashSet,
    fmt::{Debug, Display, Error, Formatter},
};

#[derive(Debug)]
struct Transition {
    id: usize,
    term: usize,
    to: usize,
    prod_num: Option<usize>,
}

///
/// Data structure that implements the semantic actions for our LaDfa2Dot grammar
///
#[derive(Debug, Default)]
pub struct LaDfa2DotGrammar<'t> {
    pub la_dfa_2_dot: Option<LaDfa2Dot<'t>>,
}

impl LaDfa2DotGrammar<'_> {
    pub fn new() -> Self {
        LaDfa2DotGrammar::default()
    }

    fn get_transitions(data: &LaDfa2Dot) -> Vec<Transition> {
        data.parts
            .transitions
            .trans_list
            .trans_list_list
            .iter()
            .fold(vec![], |mut acc, t| {
                let id = t
                    .trans_entry
                    .integer
                    .integer
                    .text()
                    .parse::<usize>()
                    .unwrap();
                let term = t
                    .trans_entry
                    .integer0
                    .integer
                    .text()
                    .parse::<usize>()
                    .unwrap();
                let to = t
                    .trans_entry
                    .integer1
                    .integer
                    .text()
                    .parse::<usize>()
                    .unwrap();
                let p = t
                    .trans_entry
                    .integer2
                    .integer
                    .text()
                    .parse::<isize>()
                    .unwrap();
                let prod_num = if p > 0 { Some(p as usize) } else { None };
                acc.push(Transition {
                    id,
                    term,
                    to,
                    prod_num,
                });
                acc
            })
    }

    fn generate_dot(&self) -> Result<()> {
        if let Some(data) = &self.la_dfa_2_dot {
            println!(
                r#"
digraph G {{
    rankdir=LR;"#
            );
            println!("    label={};", data.naming_comment.nt_name.nt_name.text());
            println!(
                r#"    node [shape=point, style=invis]; ""
    node [shape=ellipse, color=cyan, style=solid];
    "" -> 0;

    node [shape=ellipse, color=cyan];
            "#
            );
            let mut printed_states = HashSet::<usize>::new();
            for t in LaDfa2DotGrammar::get_transitions(data) {
                if printed_states.contains(&t.to) {
                    continue;
                }
                printed_states.insert(t.to);
                if let Some(p) = t.prod_num {
                    println!(
                        "    {} [label = \"Id({}, accepting), Pr({}))\"];",
                        t.to, t.to, p
                    );
                } else {
                    println!("    {} [label = \"Id({})\"];", t.to, t.to);
                }
            }

            println!();

            for t in LaDfa2DotGrammar::get_transitions(data) {
                println!("    {} -> {} [label = \"{}\"];", t.id, t.to, t.term);
            }

            println!("}}");
        }
        Ok(())
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
        self.generate_dot()?;
        Ok(())
    }
}
