use parol_runtime::parol_macros::{bail, parol};
#[allow(unused_imports)]
use parol_runtime::{ParolError, Result, Token};
use regex::Regex;
use std::{
    collections::VecDeque,
    // collections::HashSet,
    fmt::{Debug, Display, Error, Formatter},
    mem::swap,
    path::PathBuf,
};
// use tera::{Context, Tera};

use crate::la_dfa_2_dot_grammar_trait::{
    ConstName, ConstVal, ConstValList, ConstValNumber, Ident, LaDfa2Dot, LaDfa2DotGrammarTrait,
    MemberValues, QualifiedVal, StructOrTupleVal, StructVal, TupleVal,
};

#[allow(dead_code)]
#[derive(Debug)]
struct Transition {
    id: usize,
    term: usize,
    to: usize,
    prod_num: Option<usize>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct LaDFA {
    prod0: Option<usize>,
    non_terminal: String,
    transitions: Vec<Transition>,
}

impl<'t> Ident<'t> {
    fn text(&'t self) -> &'t str {
        self.ident.text()
    }
}

impl<'t> MemberValues<'t> {
    fn get_member(&self, ident: &str) -> Option<&ConstVal<'t>> {
        if self.member_value.ident.text() == ident {
            Some(self.member_value.const_val.as_ref())
        } else {
            self.member_values_list.iter().find_map(|m| {
                if m.member_value.ident.text() == ident {
                    Some(m.member_value.const_val.as_ref())
                } else {
                    None
                }
            })
        }
    }
}

///
/// Data structure that implements the semantic actions for our LaDfa2Dot grammar
///
#[derive(Debug)]
pub struct LaDfa2DotGrammar<'t> {
    pub la_dfa_2_dot: Option<LaDfa2Dot<'t>>,
    terminal_names: Vec<String>,
    non_terminal_names: VecDeque<String>,
    naming_comment_matcher: Regex,
    _out_folder: Option<PathBuf>,
    parsed_const_names: Vec<String>,
    current_transitions: Vec<Transition>,
    dfas: Vec<LaDFA>,
}

impl LaDfa2DotGrammar<'_> {
    pub fn new(out_folder: Option<PathBuf>) -> Self {
        LaDfa2DotGrammar {
            la_dfa_2_dot: None,
            terminal_names: Vec::new(),
            non_terminal_names: VecDeque::new(),
            naming_comment_matcher: Regex::new(r#"/\* \d+ - ("\w+") \*/"#).unwrap(),
            _out_folder: out_folder,
            parsed_const_names: Vec::new(),
            current_transitions: Vec::new(),
            dfas: Vec::new(),
        }
    }

    fn generate_dots(&self) -> Result<()> {
        self.dfas.iter().for_each(|d| println!("DFA: {:?}", d));
        Ok(())
    }

    fn extract_transition(cv_list: &ConstValList<'_>) -> Result<Transition> {
        if let (
            ConstVal::Number(id),
            ConstVal::Number(term),
            ConstVal::Number(to),
            ConstVal::Number(prod),
        ) = (
            cv_list.const_val.as_ref(),
            cv_list.const_val_list_list[0].const_val.as_ref(),
            cv_list.const_val_list_list[1].const_val.as_ref(),
            cv_list.const_val_list_list[2].const_val.as_ref(),
        ) {
            let id = Self::extract_value(id);
            let term = Self::extract_value(term);
            let to = Self::extract_value(to);
            let p = Self::extract_signed_value(prod);
            let prod_num = if p > 0 { Some(p as usize) } else { None };
            let transition = Transition {
                id,
                term,
                to,
                prod_num,
            };
            Ok(transition)
        } else {
            Err(parol!("Invalid constant values"))
        }
    }

    fn process_lookahead_struct(&mut self, strct: &StructVal<'_>) -> Result<()> {
        if let Some(non_terminal) = self.non_terminal_names.pop_front() {
            let prod0 = Self::extract_prod0(strct);
            let mut transitions = Vec::new();
            swap(&mut transitions, &mut self.current_transitions);
            let dfa = LaDFA {
                prod0,
                non_terminal,
                transitions,
            };
            self.dfas.push(dfa);
        } else {
            bail!("Inconsistent non-terminal name count");
        }
        Ok(())
    }

    /// Semantic action for non-terminal 'TupleVal'
    fn process_trans_tuple(&mut self, tuple: &TupleVal<'_>) -> Result<()> {
        if let Some(tuple) = tuple.tuple_val_opt.as_ref() {
            if tuple.const_val_list.const_val_list_list.len() == 3 {
                let transition = Self::extract_transition(&tuple.const_val_list)?;
                self.current_transitions.push(transition);
            }
        }
        Ok(())
    }

    fn extract_value(n: &ConstValNumber) -> usize {
        n.number.number.text().parse::<usize>().unwrap()
    }

    fn extract_signed_value(n: &ConstValNumber) -> isize {
        n.number.number.text().parse::<isize>().unwrap()
    }

    fn extract_prod0(strct: &StructVal<'_>) -> Option<usize> {
        if let Some(struct_val) = strct.struct_val_opt.as_ref() {
            if let Some(ConstVal::Number(number)) = struct_val.member_values.get_member("prod0") {
                let prod0 = Self::extract_signed_value(number);
                if prod0 > 0 {
                    Some(prod0 as usize)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

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
    /// Semantic action for non-terminal 'LaDfa2Dot'
    fn la_dfa2_dot(&mut self, arg: &LaDfa2Dot<'t>) -> Result<()> {
        self.la_dfa_2_dot = Some(arg.clone());
        self.generate_dots()?;
        Ok(())
    }

    /// Semantic action for non-terminal 'ConstName'
    fn const_name(&mut self, const_name: &ConstName<'t>) -> Result<()> {
        self.parsed_const_names
            .push(const_name.ident.ident.text().to_owned());
        Ok(())
    }

    /// Semantic action for non-terminal 'QualifiedVal'
    fn qualified_val(&mut self, q: &QualifiedVal<'t>) -> Result<()> {
        let ident = q.qualified_ident.ident.ident.text();
        if ident == "LookaheadDFA" {
            if let Some(qualified_val) = q.qualified_val_opt.as_ref() {
                if let StructOrTupleVal::StructVal(struct_val) = &*qualified_val.struct_or_tuple_val
                {
                    self.process_lookahead_struct(&struct_val.struct_val)?;
                }
            }
        } else if ident == "Trans" {
            if let Some(qualified_val) = q.qualified_val_opt.as_ref() {
                if let StructOrTupleVal::TupleStructVal(tuple_val) =
                    &*qualified_val.struct_or_tuple_val
                {
                    self.process_trans_tuple(&tuple_val.tuple_struct_val.tuple_val)?;
                }
            }
        }
        Ok(())
    }

    /// Semantic action for non-terminal 'String'
    fn string(&mut self, str: &crate::la_dfa_2_dot_grammar_trait::String<'t>) -> Result<()> {
        if self.parsed_const_names.last().as_ref().unwrap().as_str() == "TERMINAL_NAMES" {
            self.terminal_names.push(str.string.text().to_owned());
        }
        Ok(())
    }

    fn on_comment_parsed(&mut self, token: Token<'t>) {
        if let Some(caps) = self.naming_comment_matcher.captures(token.text()) {
            let nt_name = caps.get(1).unwrap().as_str().to_owned();
            self.non_terminal_names.push_back(nt_name);
        }
    }
}
