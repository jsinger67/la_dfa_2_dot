#[allow(unused_imports)]
use parol_runtime::Result;

use crate::la_dfa_2_dot_grammar_trait::LaDfa2DotGrammarTrait;
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
    //     pub la_dfa_2_dot: Option<LaDfa2Dot<'t>>,
    _x: &'t str,
}

impl LaDfa2DotGrammar<'_> {
    pub fn new() -> Self {
        LaDfa2DotGrammar::default()
    }

    // fn extract_value(t: &TransEntryValue) -> usize {
    //     t.integer.integer.text().parse::<usize>().unwrap()
    // }

    // fn extract_signed_value(t: &TransEntryValue) -> isize {
    //     t.integer.integer.text().parse::<isize>().unwrap()
    // }

    // fn get_transitions(data: &LaDfa2Dot) -> Vec<Transition> {
    //     data.parts
    //         .transitions
    //         .trans_list
    //         .trans_list_list
    //         .iter()
    //         .fold(vec![], |mut acc, t| {
    //             let t = &t.trans_entry.trans_values;
    //             let id = LaDfa2DotGrammar::extract_value(&t.trans_entry_value);
    //             let term = LaDfa2DotGrammar::extract_value(&t.trans_entry_value0);
    //             let to = LaDfa2DotGrammar::extract_value(&t.trans_entry_value1);
    //             let p = LaDfa2DotGrammar::extract_signed_value(&t.trans_entry_value2);
    //             let prod_num = if p > 0 { Some(p as usize) } else { None };
    //             acc.push(Transition {
    //                 id,
    //                 term,
    //                 to,
    //                 prod_num,
    //             });
    //             acc
    //         })
    // }

    // fn generate_dot(&self) -> Result<()> {
    //     if let Some(data) = &self.la_dfa_2_dot {
    //         let tera = Tera::new("templates/*.dot").map_err(|e| ParolError::UserError(e.into()))?;
    //         let mut context = Context::new();
    //         context.insert("title", data.naming_comment.nt_name.nt_name.text());
    //         let states = LaDfa2DotGrammar::get_transitions(data)
    //             .iter()
    //             .fold(
    //                 (vec![], HashSet::<usize>::new()),
    //                 |(mut acc, mut printed_states), t| {
    //                     if !printed_states.contains(&t.to) {
    //                         printed_states.insert(t.to);
    //                         if let Some(p) = t.prod_num {
    //                             acc.push(format!(
    //                                 "{} [label = \"Id({}, accepting), Pr({}))\"];",
    //                                 t.to, t.to, p
    //                             ));
    //                         } else {
    //                             acc.push(format!("{} [label = \"Id({})\"];", t.to, t.to));
    //                         }
    //                     }
    //                     (acc, printed_states)
    //                 },
    //             )
    //             .0;
    //         context.insert("states", &states);
    //         let transitions =
    //             LaDfa2DotGrammar::get_transitions(data)
    //                 .iter()
    //                 .fold(vec![], |mut acc, t| {
    //                     acc.push(format!("{} -> {} [label = \"{}\"];", t.id, t.to, t.term));
    //                     acc
    //                 });
    //         context.insert("transitions", &transitions);

    //         print!(
    //             "{}",
    //             tera.render("dfa.dot", &context)
    //                 .map_err(|e| ParolError::UserError(e.into()))?
    //         );
    //     }
    //     Ok(())
    // }
}

// impl Display for LaDfa2Dot<'_> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
//         write!(f, "{:?}", self)
//     }
// }

// impl Display for LaDfa2DotGrammar<'_> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
//         match &self.la_dfa_2_dot {
//             Some(la_dfa_2_dot) => writeln!(f, "{}", la_dfa_2_dot),
//             None => write!(f, "No parse result"),
//         }
//     }
// }

impl<'t> LaDfa2DotGrammarTrait<'t> for LaDfa2DotGrammar<'t> {
    // !Adjust your implementation as needed!

    // / Semantic action for non-terminal 'LaDfa2Dot'
    // fn la_dfa2_dot(&mut self, arg: &LaDfa2Dot<'t>) -> Result<()> {
    //     self.la_dfa_2_dot = Some(arg.clone());
    //     // self.generate_dot()?;
    //     Ok(())
    // }
}
