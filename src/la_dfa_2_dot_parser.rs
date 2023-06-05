// ---------------------------------------------------------
// This file was generated by parol.
// It is not intended for manual editing and changes will be
// lost after next build.
// ---------------------------------------------------------

use parol_runtime::once_cell::sync::Lazy;
#[allow(unused_imports)]
use parol_runtime::parser::{LLKParser, LookaheadDFA, ParseTreeType, ParseType, Production, Trans};
use parol_runtime::{ParolError, ParseTree};
use parol_runtime::{TokenStream, Tokenizer};
use std::cell::RefCell;
use std::path::Path;

use crate::la_dfa_2_dot_grammar::LaDfa2DotGrammar;
use crate::la_dfa_2_dot_grammar_trait::LaDfa2DotGrammarAuto;

use parol_runtime::lexer::tokenizer::{
    ERROR_TOKEN, NEW_LINE_TOKEN, UNMATCHABLE_TOKEN, WHITESPACE_TOKEN,
};

pub const TERMINALS: &[&str; 27] = &[
    /*  0 */ UNMATCHABLE_TOKEN,
    /*  1 */ UNMATCHABLE_TOKEN,
    /*  2 */ UNMATCHABLE_TOKEN,
    /*  3 */ UNMATCHABLE_TOKEN,
    /*  4 */ UNMATCHABLE_TOKEN,
    /*  5 */ r###"use"###,
    /*  6 */ r###"const"###,
    /*  7 */ r###"static"###,
    /*  8 */ r###"pub"###,
    /*  9 */ r###"(?s)&\[Production; \d+\] = &\[.*(?-s)"###,
    /* 10 */ r###"="###,
    /* 11 */ r###"-?\d+"###,
    /* 12 */ r###"\&"###,
    /* 13 */ r###";"###,
    /* 14 */ r###","###,
    /* 15 */ r###"(r#*)?"(\\.|[^\\])*?"#*"###,
    /* 16 */ r###"[a-zA-Z_][a-zA-Z0-9_]*"###,
    /* 17 */ r###"::"###,
    /* 18 */ r###":"###,
    /* 19 */ r###"\{"###,
    /* 20 */ r###"\}"###,
    /* 21 */ r###"\["###,
    /* 22 */ r###"\]"###,
    /* 23 */ r###"\("###,
    /* 24 */ r###"\)"###,
    /* 25 */ r###"#"###,
    /* 26 */ ERROR_TOKEN,
];

pub const TERMINAL_NAMES: &[&str; 27] = &[
    /*  0 */ "EndOfInput",
    /*  1 */ "Newline",
    /*  2 */ "Whitespace",
    /*  3 */ "LineComment",
    /*  4 */ "BlockComment",
    /*  5 */ "Use",
    /*  6 */ "Const",
    /*  7 */ "Static",
    /*  8 */ "Pub",
    /*  9 */ "Skip",
    /* 10 */ "Assign",
    /* 11 */ "Number",
    /* 12 */ "Amp",
    /* 13 */ "Semicolon",
    /* 14 */ "Comma",
    /* 15 */ "String",
    /* 16 */ "Ident",
    /* 17 */ "DoubleColon",
    /* 18 */ "Colon",
    /* 19 */ "LBrace",
    /* 20 */ "RBrace",
    /* 21 */ "LBracket",
    /* 22 */ "RBracket",
    /* 23 */ "LParen",
    /* 24 */ "RParen",
    /* 25 */ "Hash",
    /* 26 */ "Error",
];

/* SCANNER_0: "INITIAL" */
const SCANNER_0: (&[&str; 5], &[usize; 21]) = (
    &[
        /*  0 */ UNMATCHABLE_TOKEN,
        /*  1 */ NEW_LINE_TOKEN,
        /*  2 */ WHITESPACE_TOKEN,
        /*  3 */ r###"(//.*(\r\n|\r|\n|$))"###,
        /*  4 */ r###"((?ms)/\*.*?\*/)"###,
    ],
    &[
        5,  /* Use */
        6,  /* Const */
        7,  /* Static */
        8,  /* Pub */
        9,  /* Skip */
        10, /* Assign */
        11, /* Number */
        12, /* Amp */
        13, /* Semicolon */
        14, /* Comma */
        15, /* String */
        16, /* Ident */
        17, /* DoubleColon */
        18, /* Colon */
        19, /* LBrace */
        20, /* RBrace */
        21, /* LBracket */
        22, /* RBracket */
        23, /* LParen */
        24, /* RParen */
        25, /* Hash */
    ],
);

const MAX_K: usize = 5;

pub const NON_TERMINALS: &[&str; 62] = &[
    /*  0 */ "ArrayType",
    /*  1 */ "ArrayTypeSpec",
    /*  2 */ "ArrayTypeSpecOpt",
    /*  3 */ "ArrayVal",
    /*  4 */ "ArrayValOpt",
    /*  5 */ "Assign",
    /*  6 */ "AttributeArgOpt",
    /*  7 */ "AttributeArgOpt0",
    /*  8 */ "AttributeOpt",
    /*  9 */ "AttributeOpt0",
    /* 10 */ "Colon",
    /* 11 */ "Comma",
    /* 12 */ "CommaOpt",
    /* 13 */ "CommaOpt0",
    /* 14 */ "ConstDeclaration",
    /* 15 */ "ConstName",
    /* 16 */ "ConstPreamble",
    /* 17 */ "ConstQualifier",
    /* 18 */ "ConstQualifierOpt",
    /* 19 */ "ConstVal",
    /* 20 */ "ConstValList",
    /* 21 */ "ConstValListList",
    /* 22 */ "DoubleColon",
    /* 23 */ "Hash",
    /* 24 */ "Ident",
    /* 25 */ "Item",
    /* 26 */ "LBrace",
    /* 27 */ "LBracket",
    /* 28 */ "LParen",
    /* 29 */ "LaDfa2Dot",
    /* 30 */ "LaDfa2DotList",
    /* 31 */ "MemberValue",
    /* 32 */ "MemberValues",
    /* 33 */ "MemberValuesList",
    /* 34 */ "Number",
    /* 35 */ "QualifiedIdent",
    /* 36 */ "QualifiedIdentList",
    /* 37 */ "QualifiedVal",
    /* 38 */ "QualifiedValOpt",
    /* 39 */ "RBrace",
    /* 40 */ "RBracket",
    /* 41 */ "RParen",
    /* 42 */ "Ref",
    /* 43 */ "ScopedList",
    /* 44 */ "ScopedListItems",
    /* 45 */ "ScopedListItemsList",
    /* 46 */ "ScopedQualifiedIdent",
    /* 47 */ "ScopedQualifiedIdentOpt",
    /* 48 */ "Semicolon",
    /* 49 */ "Skip",
    /* 50 */ "String",
    /* 51 */ "StructOrTupleVal",
    /* 52 */ "StructVal",
    /* 53 */ "StructValOpt",
    /* 54 */ "TupleItems",
    /* 55 */ "TupleItemsList",
    /* 56 */ "TupleStructVal",
    /* 57 */ "TupleType",
    /* 58 */ "TupleVal",
    /* 59 */ "TupleValOpt",
    /* 60 */ "TypeSpec",
    /* 61 */ "UseStatement",
];

pub const LOOKAHEAD_AUTOMATA: &[LookaheadDFA; 62] = &[
    /* 0 - "ArrayType" */
    LookaheadDFA {
        prod0: 60,
        transitions: &[],
        k: 0,
    },
    /* 1 - "ArrayTypeSpec" */
    LookaheadDFA {
        prod0: 61,
        transitions: &[],
        k: 0,
    },
    /* 2 - "ArrayTypeSpecOpt" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 12, 1, 62), Trans(0, 16, 2, 63)],
        k: 1,
    },
    /* 3 - "ArrayVal" */
    LookaheadDFA {
        prod0: 35,
        transitions: &[],
        k: 0,
    },
    /* 4 - "ArrayValOpt" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 11, 1, 36),
            Trans(0, 12, 1, 36),
            Trans(0, 15, 1, 36),
            Trans(0, 16, 1, 36),
            Trans(0, 22, 2, 37),
            Trans(0, 23, 1, 36),
        ],
        k: 1,
    },
    /* 5 - "Assign" */
    LookaheadDFA {
        prod0: 72,
        transitions: &[],
        k: 0,
    },
    /* 6 - "AttributeArgOpt" */
    LookaheadDFA {
        prod0: 6,
        transitions: &[],
        k: 0,
    },
    /* 7 - "AttributeArgOpt0" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 22, 2, 8), Trans(0, 23, 1, 7)],
        k: 1,
    },
    /* 8 - "AttributeOpt" */
    LookaheadDFA {
        prod0: 3,
        transitions: &[],
        k: 0,
    },
    /* 9 - "AttributeOpt0" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 5, 2, 5),
            Trans(0, 6, 2, 5),
            Trans(0, 7, 2, 5),
            Trans(0, 8, 2, 5),
            Trans(0, 25, 1, 4),
        ],
        k: 1,
    },
    /* 10 - "Colon" */
    LookaheadDFA {
        prod0: 80,
        transitions: &[],
        k: 0,
    },
    /* 11 - "Comma" */
    LookaheadDFA {
        prod0: 76,
        transitions: &[],
        k: 0,
    },
    /* 12 - "CommaOpt" */
    LookaheadDFA {
        prod0: 69,
        transitions: &[],
        k: 0,
    },
    /* 13 - "CommaOpt0" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 14, 1, 70),
            Trans(0, 20, 2, 71),
            Trans(0, 22, 2, 71),
            Trans(0, 24, 2, 71),
        ],
        k: 1,
    },
    /* 14 - "ConstDeclaration" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 6, 2, -1),
            Trans(0, 7, 2, -1),
            Trans(0, 8, 8, -1),
            Trans(2, 16, 3, -1),
            Trans(3, 18, 4, -1),
            Trans(4, 9, 13, -1),
            Trans(4, 12, 5, -1),
            Trans(4, 16, 6, -1),
            Trans(4, 23, 7, -1),
            Trans(5, 21, 1, 22),
            Trans(6, 10, 1, 22),
            Trans(6, 17, 1, 22),
            Trans(7, 12, 1, 22),
            Trans(7, 16, 1, 22),
            Trans(7, 23, 1, 22),
            Trans(8, 6, 9, -1),
            Trans(9, 16, 10, -1),
            Trans(10, 18, 11, -1),
            Trans(11, 9, 12, 23),
            Trans(11, 12, 1, 22),
            Trans(11, 16, 1, 22),
            Trans(11, 23, 1, 22),
            Trans(13, 0, 12, 23),
            Trans(13, 5, 12, 23),
            Trans(13, 6, 12, 23),
            Trans(13, 7, 12, 23),
            Trans(13, 8, 12, 23),
            Trans(13, 25, 12, 23),
        ],
        k: 5,
    },
    /* 15 - "ConstName" */
    LookaheadDFA {
        prod0: 29,
        transitions: &[],
        k: 0,
    },
    /* 16 - "ConstPreamble" */
    LookaheadDFA {
        prod0: 24,
        transitions: &[],
        k: 0,
    },
    /* 17 - "ConstQualifier" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 6, 1, 25), Trans(0, 7, 2, 26), Trans(0, 8, 1, 25)],
        k: 1,
    },
    /* 18 - "ConstQualifierOpt" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 6, 2, 28), Trans(0, 8, 1, 27)],
        k: 1,
    },
    /* 19 - "ConstVal" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 11, 1, 30),
            Trans(0, 12, 4, 33),
            Trans(0, 15, 2, 31),
            Trans(0, 16, 3, 32),
            Trans(0, 23, 5, 34),
        ],
        k: 1,
    },
    /* 20 - "ConstValList" */
    LookaheadDFA {
        prod0: 38,
        transitions: &[],
        k: 0,
    },
    /* 21 - "ConstValListList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 14, 1, -1),
            Trans(0, 22, 3, -1),
            Trans(0, 24, 3, -1),
            Trans(1, 11, 2, 39),
            Trans(1, 12, 2, 39),
            Trans(1, 15, 2, 39),
            Trans(1, 16, 2, 39),
            Trans(1, 22, 4, 40),
            Trans(1, 23, 2, 39),
            Trans(1, 24, 4, 40),
            Trans(3, 13, 4, 40),
            Trans(3, 14, 4, 40),
            Trans(3, 22, 4, 40),
            Trans(3, 24, 4, 40),
        ],
        k: 2,
    },
    /* 22 - "DoubleColon" */
    LookaheadDFA {
        prod0: 79,
        transitions: &[],
        k: 0,
    },
    /* 23 - "Hash" */
    LookaheadDFA {
        prod0: 87,
        transitions: &[],
        k: 0,
    },
    /* 24 - "Ident" */
    LookaheadDFA {
        prod0: 78,
        transitions: &[],
        k: 0,
    },
    /* 25 - "Item" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 5, 1, 9),
            Trans(0, 6, 2, 10),
            Trans(0, 7, 2, 10),
            Trans(0, 8, 2, 10),
        ],
        k: 1,
    },
    /* 26 - "LBrace" */
    LookaheadDFA {
        prod0: 81,
        transitions: &[],
        k: 0,
    },
    /* 27 - "LBracket" */
    LookaheadDFA {
        prod0: 83,
        transitions: &[],
        k: 0,
    },
    /* 28 - "LParen" */
    LookaheadDFA {
        prod0: 85,
        transitions: &[],
        k: 0,
    },
    /* 29 - "LaDfa2Dot" */
    LookaheadDFA {
        prod0: 0,
        transitions: &[],
        k: 0,
    },
    /* 30 - "LaDfa2DotList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 0, 2, 2),
            Trans(0, 5, 1, 1),
            Trans(0, 6, 1, 1),
            Trans(0, 7, 1, 1),
            Trans(0, 8, 1, 1),
            Trans(0, 25, 1, 1),
        ],
        k: 1,
    },
    /* 31 - "MemberValue" */
    LookaheadDFA {
        prod0: 55,
        transitions: &[],
        k: 0,
    },
    /* 32 - "MemberValues" */
    LookaheadDFA {
        prod0: 52,
        transitions: &[],
        k: 0,
    },
    /* 33 - "MemberValuesList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 14, 1, -1),
            Trans(1, 16, 2, 53),
            Trans(1, 20, 3, 54),
        ],
        k: 2,
    },
    /* 34 - "Number" */
    LookaheadDFA {
        prod0: 73,
        transitions: &[],
        k: 0,
    },
    /* 35 - "QualifiedIdent" */
    LookaheadDFA {
        prod0: 12,
        transitions: &[],
        k: 0,
    },
    /* 36 - "QualifiedIdentList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 10, 3, -1),
            Trans(0, 13, 4, -1),
            Trans(0, 14, 5, -1),
            Trans(0, 17, 1, -1),
            Trans(0, 19, 6, -1),
            Trans(0, 20, 7, -1),
            Trans(0, 22, 8, -1),
            Trans(0, 23, 9, -1),
            Trans(0, 24, 10, -1),
            Trans(1, 16, 2, 13),
            Trans(1, 19, 11, 14),
            Trans(3, 11, 11, 14),
            Trans(3, 12, 11, 14),
            Trans(3, 15, 11, 14),
            Trans(3, 16, 11, 14),
            Trans(3, 23, 11, 14),
            Trans(4, 0, 11, 14),
            Trans(4, 5, 11, 14),
            Trans(4, 6, 11, 14),
            Trans(4, 7, 11, 14),
            Trans(4, 8, 11, 14),
            Trans(4, 25, 11, 14),
            Trans(5, 11, 11, 14),
            Trans(5, 12, 11, 14),
            Trans(5, 15, 11, 14),
            Trans(5, 16, 11, 14),
            Trans(5, 20, 11, 14),
            Trans(5, 22, 11, 14),
            Trans(5, 23, 11, 14),
            Trans(5, 24, 11, 14),
            Trans(6, 16, 11, 14),
            Trans(6, 20, 11, 14),
            Trans(7, 13, 11, 14),
            Trans(7, 14, 11, 14),
            Trans(7, 20, 11, 14),
            Trans(8, 13, 11, 14),
            Trans(8, 14, 11, 14),
            Trans(8, 22, 11, 14),
            Trans(8, 24, 11, 14),
            Trans(9, 11, 11, 14),
            Trans(9, 12, 11, 14),
            Trans(9, 15, 11, 14),
            Trans(9, 16, 11, 14),
            Trans(9, 23, 11, 14),
            Trans(9, 24, 11, 14),
            Trans(10, 10, 11, 14),
            Trans(10, 13, 11, 14),
            Trans(10, 14, 11, 14),
            Trans(10, 22, 11, 14),
            Trans(10, 24, 11, 14),
        ],
        k: 2,
    },
    /* 37 - "QualifiedVal" */
    LookaheadDFA {
        prod0: 44,
        transitions: &[],
        k: 0,
    },
    /* 38 - "QualifiedValOpt" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 13, 2, 46),
            Trans(0, 14, 2, 46),
            Trans(0, 19, 1, 45),
            Trans(0, 22, 2, 46),
            Trans(0, 23, 1, 45),
            Trans(0, 24, 2, 46),
        ],
        k: 1,
    },
    /* 39 - "RBrace" */
    LookaheadDFA {
        prod0: 82,
        transitions: &[],
        k: 0,
    },
    /* 40 - "RBracket" */
    LookaheadDFA {
        prod0: 84,
        transitions: &[],
        k: 0,
    },
    /* 41 - "RParen" */
    LookaheadDFA {
        prod0: 86,
        transitions: &[],
        k: 0,
    },
    /* 42 - "Ref" */
    LookaheadDFA {
        prod0: 74,
        transitions: &[],
        k: 0,
    },
    /* 43 - "ScopedList" */
    LookaheadDFA {
        prod0: 18,
        transitions: &[],
        k: 0,
    },
    /* 44 - "ScopedListItems" */
    LookaheadDFA {
        prod0: 19,
        transitions: &[],
        k: 0,
    },
    /* 45 - "ScopedListItemsList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 14, 1, -1),
            Trans(0, 20, 3, -1),
            Trans(1, 16, 2, 20),
            Trans(1, 20, 4, 21),
            Trans(3, 13, 4, 21),
            Trans(3, 14, 4, 21),
            Trans(3, 20, 4, 21),
        ],
        k: 2,
    },
    /* 46 - "ScopedQualifiedIdent" */
    LookaheadDFA {
        prod0: 15,
        transitions: &[],
        k: 0,
    },
    /* 47 - "ScopedQualifiedIdentOpt" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 13, 2, 17),
            Trans(0, 14, 2, 17),
            Trans(0, 17, 1, 16),
            Trans(0, 20, 2, 17),
        ],
        k: 1,
    },
    /* 48 - "Semicolon" */
    LookaheadDFA {
        prod0: 75,
        transitions: &[],
        k: 0,
    },
    /* 49 - "Skip" */
    LookaheadDFA {
        prod0: 68,
        transitions: &[],
        k: 0,
    },
    /* 50 - "String" */
    LookaheadDFA {
        prod0: 77,
        transitions: &[],
        k: 0,
    },
    /* 51 - "StructOrTupleVal" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 19, 1, 47), Trans(0, 23, 2, 48)],
        k: 1,
    },
    /* 52 - "StructVal" */
    LookaheadDFA {
        prod0: 49,
        transitions: &[],
        k: 0,
    },
    /* 53 - "StructValOpt" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 16, 1, 50), Trans(0, 20, 2, 51)],
        k: 1,
    },
    /* 54 - "TupleItems" */
    LookaheadDFA {
        prod0: 65,
        transitions: &[],
        k: 0,
    },
    /* 55 - "TupleItemsList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 14, 1, -1),
            Trans(0, 24, 3, -1),
            Trans(1, 12, 2, 66),
            Trans(1, 16, 2, 66),
            Trans(1, 23, 2, 66),
            Trans(1, 24, 4, 67),
            Trans(3, 10, 4, 67),
            Trans(3, 14, 4, 67),
            Trans(3, 24, 4, 67),
        ],
        k: 2,
    },
    /* 56 - "TupleStructVal" */
    LookaheadDFA {
        prod0: 56,
        transitions: &[],
        k: 0,
    },
    /* 57 - "TupleType" */
    LookaheadDFA {
        prod0: 64,
        transitions: &[],
        k: 0,
    },
    /* 58 - "TupleVal" */
    LookaheadDFA {
        prod0: 41,
        transitions: &[],
        k: 0,
    },
    /* 59 - "TupleValOpt" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 11, 1, 42),
            Trans(0, 12, 1, 42),
            Trans(0, 15, 1, 42),
            Trans(0, 16, 1, 42),
            Trans(0, 23, 1, 42),
            Trans(0, 24, 2, 43),
        ],
        k: 1,
    },
    /* 60 - "TypeSpec" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 12, 2, 58),
            Trans(0, 16, 1, 57),
            Trans(0, 23, 3, 59),
        ],
        k: 1,
    },
    /* 61 - "UseStatement" */
    LookaheadDFA {
        prod0: 11,
        transitions: &[],
        k: 0,
    },
];

pub const PRODUCTIONS: &[Production; 88] = &[
    // 0 - LaDfa2Dot: LaDfa2DotList /* Vec */;
    Production {
        lhs: 29,
        production: &[ParseType::N(30)],
    },
    // 1 - LaDfa2DotList: AttributeOpt^ /* Clipped */ Item LaDfa2DotList;
    Production {
        lhs: 30,
        production: &[ParseType::N(30), ParseType::N(25), ParseType::N(8)],
    },
    // 2 - LaDfa2DotList: ;
    Production {
        lhs: 30,
        production: &[],
    },
    // 3 - AttributeOpt: AttributeOpt0 /* Option */;
    Production {
        lhs: 8,
        production: &[ParseType::N(9)],
    },
    // 4 - AttributeOpt0: Hash LBracket Ident AttributeArgOpt RBracket;
    Production {
        lhs: 9,
        production: &[
            ParseType::N(40),
            ParseType::N(6),
            ParseType::N(24),
            ParseType::N(27),
            ParseType::N(23),
        ],
    },
    // 5 - AttributeOpt0: ;
    Production {
        lhs: 9,
        production: &[],
    },
    // 6 - AttributeArgOpt: AttributeArgOpt0 /* Option */;
    Production {
        lhs: 6,
        production: &[ParseType::N(7)],
    },
    // 7 - AttributeArgOpt0: LParen Ident RParen;
    Production {
        lhs: 7,
        production: &[ParseType::N(41), ParseType::N(24), ParseType::N(28)],
    },
    // 8 - AttributeArgOpt0: ;
    Production {
        lhs: 7,
        production: &[],
    },
    // 9 - Item: UseStatement^ /* Clipped */;
    Production {
        lhs: 25,
        production: &[ParseType::N(61)],
    },
    // 10 - Item: ConstDeclaration;
    Production {
        lhs: 25,
        production: &[ParseType::N(14)],
    },
    // 11 - UseStatement: 'use' ScopedQualifiedIdent Semicolon;
    Production {
        lhs: 61,
        production: &[ParseType::N(48), ParseType::N(46), ParseType::T(5)],
    },
    // 12 - QualifiedIdent: Ident QualifiedIdentList /* Vec */;
    Production {
        lhs: 35,
        production: &[ParseType::N(36), ParseType::N(24)],
    },
    // 13 - QualifiedIdentList: DoubleColon Ident QualifiedIdentList;
    Production {
        lhs: 36,
        production: &[ParseType::N(36), ParseType::N(24), ParseType::N(22)],
    },
    // 14 - QualifiedIdentList: ;
    Production {
        lhs: 36,
        production: &[],
    },
    // 15 - ScopedQualifiedIdent: QualifiedIdent ScopedQualifiedIdentOpt /* Option */;
    Production {
        lhs: 46,
        production: &[ParseType::N(47), ParseType::N(35)],
    },
    // 16 - ScopedQualifiedIdentOpt: DoubleColon ScopedList;
    Production {
        lhs: 47,
        production: &[ParseType::N(43), ParseType::N(22)],
    },
    // 17 - ScopedQualifiedIdentOpt: ;
    Production {
        lhs: 47,
        production: &[],
    },
    // 18 - ScopedList: LBrace ScopedListItems CommaOpt RBrace;
    Production {
        lhs: 43,
        production: &[
            ParseType::N(39),
            ParseType::N(12),
            ParseType::N(44),
            ParseType::N(26),
        ],
    },
    // 19 - ScopedListItems: ScopedQualifiedIdent ScopedListItemsList /* Vec */;
    Production {
        lhs: 44,
        production: &[ParseType::N(45), ParseType::N(46)],
    },
    // 20 - ScopedListItemsList: Comma^ /* Clipped */ ScopedQualifiedIdent ScopedListItemsList;
    Production {
        lhs: 45,
        production: &[ParseType::N(45), ParseType::N(46), ParseType::N(11)],
    },
    // 21 - ScopedListItemsList: ;
    Production {
        lhs: 45,
        production: &[],
    },
    // 22 - ConstDeclaration: ConstPreamble TypeSpec Assign ConstVal Semicolon;
    Production {
        lhs: 14,
        production: &[
            ParseType::N(48),
            ParseType::N(19),
            ParseType::N(5),
            ParseType::N(60),
            ParseType::N(16),
        ],
    },
    // 23 - ConstDeclaration: ConstPreamble^ /* Clipped */ Skip^ /* Clipped */;
    Production {
        lhs: 14,
        production: &[ParseType::N(49), ParseType::N(16)],
    },
    // 24 - ConstPreamble: ConstQualifier ConstName Colon;
    Production {
        lhs: 16,
        production: &[ParseType::N(10), ParseType::N(15), ParseType::N(17)],
    },
    // 25 - ConstQualifier: ConstQualifierOpt /* Option */ 'const';
    Production {
        lhs: 17,
        production: &[ParseType::T(6), ParseType::N(18)],
    },
    // 26 - ConstQualifier: 'static';
    Production {
        lhs: 17,
        production: &[ParseType::T(7)],
    },
    // 27 - ConstQualifierOpt: 'pub';
    Production {
        lhs: 18,
        production: &[ParseType::T(8)],
    },
    // 28 - ConstQualifierOpt: ;
    Production {
        lhs: 18,
        production: &[],
    },
    // 29 - ConstName: Ident;
    Production {
        lhs: 15,
        production: &[ParseType::N(24)],
    },
    // 30 - ConstVal: Number;
    Production {
        lhs: 19,
        production: &[ParseType::N(34)],
    },
    // 31 - ConstVal: String;
    Production {
        lhs: 19,
        production: &[ParseType::N(50)],
    },
    // 32 - ConstVal: QualifiedVal;
    Production {
        lhs: 19,
        production: &[ParseType::N(37)],
    },
    // 33 - ConstVal: ArrayVal;
    Production {
        lhs: 19,
        production: &[ParseType::N(3)],
    },
    // 34 - ConstVal: TupleVal;
    Production {
        lhs: 19,
        production: &[ParseType::N(58)],
    },
    // 35 - ArrayVal: Ref LBracket ArrayValOpt /* Option */ RBracket;
    Production {
        lhs: 3,
        production: &[
            ParseType::N(40),
            ParseType::N(4),
            ParseType::N(27),
            ParseType::N(42),
        ],
    },
    // 36 - ArrayValOpt: ConstValList CommaOpt;
    Production {
        lhs: 4,
        production: &[ParseType::N(12), ParseType::N(20)],
    },
    // 37 - ArrayValOpt: ;
    Production {
        lhs: 4,
        production: &[],
    },
    // 38 - ConstValList: ConstVal ConstValListList /* Vec */;
    Production {
        lhs: 20,
        production: &[ParseType::N(21), ParseType::N(19)],
    },
    // 39 - ConstValListList: Comma ConstVal ConstValListList;
    Production {
        lhs: 21,
        production: &[ParseType::N(21), ParseType::N(19), ParseType::N(11)],
    },
    // 40 - ConstValListList: ;
    Production {
        lhs: 21,
        production: &[],
    },
    // 41 - TupleVal: LParen TupleValOpt /* Option */ RParen;
    Production {
        lhs: 58,
        production: &[ParseType::N(41), ParseType::N(59), ParseType::N(28)],
    },
    // 42 - TupleValOpt: ConstValList CommaOpt;
    Production {
        lhs: 59,
        production: &[ParseType::N(12), ParseType::N(20)],
    },
    // 43 - TupleValOpt: ;
    Production {
        lhs: 59,
        production: &[],
    },
    // 44 - QualifiedVal: QualifiedIdent QualifiedValOpt /* Option */;
    Production {
        lhs: 37,
        production: &[ParseType::N(38), ParseType::N(35)],
    },
    // 45 - QualifiedValOpt: StructOrTupleVal;
    Production {
        lhs: 38,
        production: &[ParseType::N(51)],
    },
    // 46 - QualifiedValOpt: ;
    Production {
        lhs: 38,
        production: &[],
    },
    // 47 - StructOrTupleVal: StructVal;
    Production {
        lhs: 51,
        production: &[ParseType::N(52)],
    },
    // 48 - StructOrTupleVal: TupleStructVal;
    Production {
        lhs: 51,
        production: &[ParseType::N(56)],
    },
    // 49 - StructVal: LBrace StructValOpt /* Option */ RBrace;
    Production {
        lhs: 52,
        production: &[ParseType::N(39), ParseType::N(53), ParseType::N(26)],
    },
    // 50 - StructValOpt: MemberValues Comma;
    Production {
        lhs: 53,
        production: &[ParseType::N(11), ParseType::N(32)],
    },
    // 51 - StructValOpt: ;
    Production {
        lhs: 53,
        production: &[],
    },
    // 52 - MemberValues: MemberValue MemberValuesList /* Vec */;
    Production {
        lhs: 32,
        production: &[ParseType::N(33), ParseType::N(31)],
    },
    // 53 - MemberValuesList: Comma MemberValue MemberValuesList;
    Production {
        lhs: 33,
        production: &[ParseType::N(33), ParseType::N(31), ParseType::N(11)],
    },
    // 54 - MemberValuesList: ;
    Production {
        lhs: 33,
        production: &[],
    },
    // 55 - MemberValue: Ident Colon ConstVal;
    Production {
        lhs: 31,
        production: &[ParseType::N(19), ParseType::N(10), ParseType::N(24)],
    },
    // 56 - TupleStructVal: TupleVal;
    Production {
        lhs: 56,
        production: &[ParseType::N(58)],
    },
    // 57 - TypeSpec: QualifiedIdent;
    Production {
        lhs: 60,
        production: &[ParseType::N(35)],
    },
    // 58 - TypeSpec: ArrayType;
    Production {
        lhs: 60,
        production: &[ParseType::N(0)],
    },
    // 59 - TypeSpec: TupleType;
    Production {
        lhs: 60,
        production: &[ParseType::N(57)],
    },
    // 60 - ArrayType: Ref LBracket ArrayTypeSpec RBracket;
    Production {
        lhs: 0,
        production: &[
            ParseType::N(40),
            ParseType::N(1),
            ParseType::N(27),
            ParseType::N(42),
        ],
    },
    // 61 - ArrayTypeSpec: ArrayTypeSpecOpt /* Option */ Ident Semicolon Number;
    Production {
        lhs: 1,
        production: &[
            ParseType::N(34),
            ParseType::N(48),
            ParseType::N(24),
            ParseType::N(2),
        ],
    },
    // 62 - ArrayTypeSpecOpt: Ref;
    Production {
        lhs: 2,
        production: &[ParseType::N(42)],
    },
    // 63 - ArrayTypeSpecOpt: ;
    Production {
        lhs: 2,
        production: &[],
    },
    // 64 - TupleType: LParen TupleItems CommaOpt RParen;
    Production {
        lhs: 57,
        production: &[
            ParseType::N(41),
            ParseType::N(12),
            ParseType::N(54),
            ParseType::N(28),
        ],
    },
    // 65 - TupleItems: TypeSpec TupleItemsList /* Vec */;
    Production {
        lhs: 54,
        production: &[ParseType::N(55), ParseType::N(60)],
    },
    // 66 - TupleItemsList: Comma TypeSpec TupleItemsList;
    Production {
        lhs: 55,
        production: &[ParseType::N(55), ParseType::N(60), ParseType::N(11)],
    },
    // 67 - TupleItemsList: ;
    Production {
        lhs: 55,
        production: &[],
    },
    // 68 - Skip: /(?s)&\[Production; \d+\] = &\[.*(?-s)/;
    Production {
        lhs: 49,
        production: &[ParseType::T(9)],
    },
    // 69 - CommaOpt: CommaOpt0 /* Option */;
    Production {
        lhs: 12,
        production: &[ParseType::N(13)],
    },
    // 70 - CommaOpt0: Comma;
    Production {
        lhs: 13,
        production: &[ParseType::N(11)],
    },
    // 71 - CommaOpt0: ;
    Production {
        lhs: 13,
        production: &[],
    },
    // 72 - Assign: '=';
    Production {
        lhs: 5,
        production: &[ParseType::T(10)],
    },
    // 73 - Number: /-?\d+/;
    Production {
        lhs: 34,
        production: &[ParseType::T(11)],
    },
    // 74 - Ref: '&';
    Production {
        lhs: 42,
        production: &[ParseType::T(12)],
    },
    // 75 - Semicolon: ';';
    Production {
        lhs: 48,
        production: &[ParseType::T(13)],
    },
    // 76 - Comma: ',';
    Production {
        lhs: 11,
        production: &[ParseType::T(14)],
    },
    // 77 - String: /(r#*)?"(\\.|[^\\])*?"#*/;
    Production {
        lhs: 50,
        production: &[ParseType::T(15)],
    },
    // 78 - Ident: /[a-zA-Z_][a-zA-Z0-9_]*/;
    Production {
        lhs: 24,
        production: &[ParseType::T(16)],
    },
    // 79 - DoubleColon: '::';
    Production {
        lhs: 22,
        production: &[ParseType::T(17)],
    },
    // 80 - Colon: ':';
    Production {
        lhs: 10,
        production: &[ParseType::T(18)],
    },
    // 81 - LBrace: '{';
    Production {
        lhs: 26,
        production: &[ParseType::T(19)],
    },
    // 82 - RBrace: '}';
    Production {
        lhs: 39,
        production: &[ParseType::T(20)],
    },
    // 83 - LBracket: '[';
    Production {
        lhs: 27,
        production: &[ParseType::T(21)],
    },
    // 84 - RBracket: ']';
    Production {
        lhs: 40,
        production: &[ParseType::T(22)],
    },
    // 85 - LParen: '(';
    Production {
        lhs: 28,
        production: &[ParseType::T(23)],
    },
    // 86 - RParen: ')';
    Production {
        lhs: 41,
        production: &[ParseType::T(24)],
    },
    // 87 - Hash: /#/;
    Production {
        lhs: 23,
        production: &[ParseType::T(25)],
    },
];

static TOKENIZERS: Lazy<Vec<(&'static str, Tokenizer)>> = Lazy::new(|| {
    vec![(
        "INITIAL",
        Tokenizer::build(TERMINALS, SCANNER_0.0, SCANNER_0.1).unwrap(),
    )]
});

pub fn parse<'t, T>(
    input: &'t str,
    file_name: T,
    user_actions: &mut LaDfa2DotGrammar<'t>,
) -> Result<ParseTree<'t>, ParolError>
where
    T: AsRef<Path>,
{
    let mut llk_parser = LLKParser::new(
        29,
        LOOKAHEAD_AUTOMATA,
        PRODUCTIONS,
        TERMINAL_NAMES,
        NON_TERMINALS,
    );
    llk_parser.trim_parse_tree();
    let token_stream =
        RefCell::new(TokenStream::new(input, file_name, &TOKENIZERS, MAX_K).unwrap());
    // Initialize wrapper
    let mut user_actions = LaDfa2DotGrammarAuto::new(user_actions);
    llk_parser.parse(token_stream, &mut user_actions)
}
