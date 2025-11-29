use parol_runtime::parol_macros::{bail, parol};
#[allow(unused_imports)]
use parol_runtime::{ParolError, Result, Token};
use regex::Regex;
use std::{
    collections::HashSet,
    collections::VecDeque,
    fmt::{Debug, Display, Error, Formatter},
    fs,
    mem::swap,
    path::PathBuf,
};
use tera::{Context, Tera};

/// We generate the template into the binary to make it easier to install
const TEMPLATE: &str = include_str!("../templates/dfa.dot");

use crate::la_dfa_2_dot_grammar_trait::{
    ConstName, ConstVal, ConstValList, ConstValNumber, Ident, LaDfa2Dot, LaDfa2DotGrammarTrait,
    MemberValues, QualifiedVal, StructOrTupleVal, StructVal, TupleVal,
};

#[derive(Debug)]
struct Transition {
    id: usize,
    term: usize,
    to: usize,
    prod_num: Option<usize>,
}

#[derive(Debug)]
struct LaDFA {
    prod0: Option<usize>,
    non_terminal: String,
    transitions: Vec<Transition>,
    k: usize,
}

impl<'t> Ident<'t> {
    fn text(&'t self) -> &'t str {
        self.ident.text()
    }
}

impl<'t> MemberValues<'t> {
    fn get_member(&self, ident: &str) -> Option<&ConstVal<'t>> {
        if self.member_value.ident.text() == ident {
            Some(&self.member_value.const_val)
        } else {
            self.member_values_list.iter().find_map(|m| {
                if m.member_value.ident.text() == ident {
                    Some(&m.member_value.const_val)
                } else {
                    None
                }
            })
        }
    }
}

impl<'t> crate::la_dfa_2_dot_grammar_trait::String<'t> {
    fn text(&'t self) -> String {
        match self {
            crate::la_dfa_2_dot_grammar_trait::String::QuotedString(string_quoted_string) => {
                string_quoted_string.quoted_string.quoted_string.text().to_string()
            }
            crate::la_dfa_2_dot_grammar_trait::String::RawString(string_raw_string) => {
                string_raw_string.raw_string.strng.raw_string_content.text().to_string()
            }
            crate::la_dfa_2_dot_grammar_trait::String::RawString1(string_raw_string1) => {
                string_raw_string1
                    .raw_string1
                    .strng
                    .raw_string1_content_list
                    .iter()
                    .fold(String::new(), |acc, x| {
                        acc + match &x.raw_string1_content_list_group {
                            crate::la_dfa_2_dot_grammar_trait::RawString1ContentListGroup::RawStringContentNoQuotes(raw_string1_content_list_group_raw_string_content_no_quotes) =>
                                raw_string1_content_list_group_raw_string_content_no_quotes.raw_string_content_no_quotes.raw_string_content_no_quotes.text(),
                            crate::la_dfa_2_dot_grammar_trait::RawString1ContentListGroup::RawString1ContentQuotes(raw_string1_content_list_group_raw_string_content_quotes) =>
                                raw_string1_content_list_group_raw_string_content_quotes.raw_string1_content_quotes.raw_string_end.text(),
                        }
                    })
            }
            crate::la_dfa_2_dot_grammar_trait::String::RawString2(string_raw_string2) => {
                string_raw_string2.raw_string2.strng.raw_string2_content_list.iter().fold(String::new(), |acc, x| {
                    acc + match &x.raw_string2_content_list_group {
                        crate::la_dfa_2_dot_grammar_trait::RawString2ContentListGroup::RawStringContentNoQuotes(raw_string2_content_list_group_raw_string_content_no_quotes) =>
                            raw_string2_content_list_group_raw_string_content_no_quotes.raw_string_content_no_quotes.raw_string_content_no_quotes.text(),
                        crate::la_dfa_2_dot_grammar_trait::RawString2ContentListGroup::RawString2ContentQuotes(raw_string2_content_list_group_raw_string_content_quotes) =>
                            raw_string2_content_list_group_raw_string_content_quotes.raw_string2_content_quotes.raw_string_end.text(),
                    }
                })
            }
            crate::la_dfa_2_dot_grammar_trait::String::RawString3(string_raw_string3) => {
                string_raw_string3.raw_string3.strng.raw_string3_content_list.iter().fold(String::new(), |acc, x| {
                    acc + match &x.raw_string3_content_list_group {
                        crate::la_dfa_2_dot_grammar_trait::RawString3ContentListGroup::RawStringContentNoQuotes(raw_string3_content_list_group_raw_string_content_no_quotes) =>
                            raw_string3_content_list_group_raw_string_content_no_quotes.raw_string_content_no_quotes.raw_string_content_no_quotes.text(),
                        crate::la_dfa_2_dot_grammar_trait::RawString3ContentListGroup::RawString3ContentQuotes(raw_string3_content_list_group_raw_string_content_quotes) =>
                            raw_string3_content_list_group_raw_string_content_quotes.raw_string3_content_quotes.raw_string_end.text(),
                    }
                })
            }
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
    out_folder: PathBuf,
    parsed_const_names: Vec<String>,
    current_transitions: Vec<Transition>,
    dfas: Vec<LaDFA>,
}

impl LaDfa2DotGrammar<'_> {
    pub fn new(out_folder: PathBuf) -> Self {
        LaDfa2DotGrammar {
            la_dfa_2_dot: None,
            terminal_names: Vec::new(),
            non_terminal_names: VecDeque::new(),
            naming_comment_matcher: Regex::new(r#"/\* \d+ - "(\w+)" \*/"#).unwrap(),
            out_folder,
            parsed_const_names: Vec::new(),
            current_transitions: Vec::new(),
            dfas: Vec::new(),
        }
    }

    fn generate_dots(&self) -> Result<()> {
        for d in &self.dfas {
            // println!("DFA: {:?}", d);
            self.generate_dot(d)?;
        }
        Ok(())
    }

    fn extract_transition(cv_list: &ConstValList<'_>) -> Result<Transition> {
        if let (
            ConstVal::Number(id),
            ConstVal::Number(term),
            ConstVal::Number(to),
            ConstVal::Number(prod),
        ) = (
            &cv_list.const_val,
            &cv_list.const_val_list_list[0].const_val,
            &cv_list.const_val_list_list[1].const_val,
            &cv_list.const_val_list_list[2].const_val,
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
            let k = Self::extract_k(strct).unwrap_or_default();
            let mut transitions = Vec::new();
            swap(&mut transitions, &mut self.current_transitions);
            let dfa = LaDFA {
                prod0,
                non_terminal,
                transitions,
                k,
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

    fn extract_k(strct: &StructVal<'_>) -> Option<usize> {
        if let Some(struct_val) = strct.struct_val_opt.as_ref() {
            if let Some(ConstVal::Number(number)) = struct_val.member_values.get_member("k") {
                let prod0 = Self::extract_value(number);
                Some(prod0)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn generate_dot(&self, dfa: &LaDFA) -> Result<()> {
        let mut tera = Tera::default();
        let mut context = Context::new();
        let title = format!("{} (k={})", dfa.non_terminal, dfa.k);
        context.insert("title", &title);
        let state0 = if let Some(prod0) = dfa.prod0 {
            format!(
                "0 [label=\"Id(0) , accepting, Pr({}))\", color=red];",
                prod0
            )
        } else {
            "0 [label = \"Id(0)\"];".to_owned()
        };
        let states = dfa
            .transitions
            .iter()
            .fold(
                (vec![state0], HashSet::<usize>::new()),
                |(mut acc, mut printed_states), t| {
                    if !printed_states.contains(&t.to) {
                        printed_states.insert(t.to);
                        if let Some(p) = t.prod_num {
                            acc.push(format!(
                                "{} [label = \"Id({}, accepting, Pr({}))\", color=red];",
                                t.to, t.to, p
                            ));
                        } else {
                            acc.push(format!("{} [label = \"Id({})\"];", t.to, t.to));
                        }
                    }
                    (acc, printed_states)
                },
            )
            .0;
        context.insert("states", &states);
        let transitions = dfa.transitions.iter().fold(vec![], |mut acc, t| {
            acc.push(format!(
                "{} -> {} [label = {}];",
                t.id, t.to, self.terminal_names[t.term]
            ));
            acc
        });
        context.insert("transitions", &transitions);

        let mut file_name = self.out_folder.clone();
        file_name.push(&dfa.non_terminal);
        file_name.set_extension("dot");
        fs::write(
            file_name,
            tera.render_str(TEMPLATE, &context)
                .map_err(|e| ParolError::UserError(e.into()))?,
        )
        .map_err(|e| ParolError::UserError(e.into()))?;
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
                if let StructOrTupleVal::StructVal(struct_val) = &qualified_val.struct_or_tuple_val
                {
                    self.process_lookahead_struct(&struct_val.struct_val)?;
                }
            }
        } else if ident == "Trans" {
            if let Some(qualified_val) = q.qualified_val_opt.as_ref() {
                if let StructOrTupleVal::TupleStructVal(tuple_val) =
                    &qualified_val.struct_or_tuple_val
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
            self.terminal_names.push(str.text().to_owned());
        }
        Ok(())
    }

    fn on_comment(&mut self, token: Token<'t>) {
        if let Some(caps) = self.naming_comment_matcher.captures(token.text()) {
            let nt_name = caps.get(1).unwrap().as_str().to_owned();
            self.non_terminal_names.push_back(nt_name);
        }
    }
}
