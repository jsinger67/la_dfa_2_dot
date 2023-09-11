// ---------------------------------------------------------
// This file was generated by parol.
// It is not intended for manual editing and changes will be
// lost after next build.
// ---------------------------------------------------------

use parol_runtime::once_cell::sync::Lazy;
#[allow(unused_imports)]
use parol_runtime::parser::{LLKParser, LookaheadDFA, ParseTreeType, ParseType, Production, Trans};
use parol_runtime::{ParolError, ParseTree, TerminalIndex};
use parol_runtime::{TokenStream, Tokenizer};
use std::cell::RefCell;
use std::path::Path;

use crate::la_dfa_2_dot_grammar::LaDfa2DotGrammar;
use crate::la_dfa_2_dot_grammar_trait::LaDfa2DotGrammarAuto;

use parol_runtime::lexer::tokenizer::{
    ERROR_TOKEN, NEW_LINE_TOKEN, UNMATCHABLE_TOKEN, WHITESPACE_TOKEN,
};

pub const TERMINALS: &[&str; 28] = &[
    /*  0 */ UNMATCHABLE_TOKEN,
    /*  1 */ UNMATCHABLE_TOKEN,
    /*  2 */ UNMATCHABLE_TOKEN,
    /*  3 */ UNMATCHABLE_TOKEN,
    /*  4 */ UNMATCHABLE_TOKEN,
    /*  5 */ r"use",
    /*  6 */ r"const",
    /*  7 */ r"static",
    /*  8 */ r"pub",
    /*  9 */ r"(?s)&\[Production; \d+\] = &\[.*(?-s)",
    /* 10 */ r"=",
    /* 11 */ r"-?\d+",
    /* 12 */ r"\&",
    /* 13 */ r";",
    /* 14 */ r",",
    /* 15 */ r##"r#{0, 3}?".*?"#{0, 3}"##,
    /* 16 */ r#""(\\.|[^\\])*?""#,
    /* 17 */ r"[a-zA-Z_][a-zA-Z0-9_]*",
    /* 18 */ r"::",
    /* 19 */ r":",
    /* 20 */ r"\{",
    /* 21 */ r"\}",
    /* 22 */ r"\[",
    /* 23 */ r"\]",
    /* 24 */ r"\(",
    /* 25 */ r"\)",
    /* 26 */ r"#",
    /* 27 */ ERROR_TOKEN,
];

pub const TERMINAL_NAMES: &[&str; 28] = &[
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
    /* 12 */ "Ref",
    /* 13 */ "Semicolon",
    /* 14 */ "Comma",
    /* 15 */ "QuotedString",
    /* 16 */ "RawString",
    /* 17 */ "Ident",
    /* 18 */ "DoubleColon",
    /* 19 */ "Colon",
    /* 20 */ "LBrace",
    /* 21 */ "RBrace",
    /* 22 */ "LBracket",
    /* 23 */ "RBracket",
    /* 24 */ "LParen",
    /* 25 */ "RParen",
    /* 26 */ "Hash",
    /* 27 */ "Error",
];

/* SCANNER_0: "INITIAL" */
const SCANNER_0: (&[&str; 5], &[TerminalIndex; 22]) = (
    &[
        /*  0 */ UNMATCHABLE_TOKEN,
        /*  1 */ NEW_LINE_TOKEN,
        /*  2 */ WHITESPACE_TOKEN,
        /*  3 */ r"(//.*(\r\n|\r|\n|$))",
        /*  4 */ r"((?ms)/\*.*?\*/)",
    ],
    &[
        5,  /* Use */
        6,  /* Const */
        7,  /* Static */
        8,  /* Pub */
        9,  /* Skip */
        10, /* Assign */
        11, /* Number */
        12, /* Ref */
        13, /* Semicolon */
        14, /* Comma */
        15, /* QuotedString */
        16, /* RawString */
        17, /* Ident */
        18, /* DoubleColon */
        19, /* Colon */
        20, /* LBrace */
        21, /* RBrace */
        22, /* LBracket */
        23, /* RBracket */
        24, /* LParen */
        25, /* RParen */
        26, /* Hash */
    ],
);

const MAX_K: usize = 5;

pub const NON_TERMINALS: &[&str; 64] = &[
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
    /* 39 */ "QuotedString",
    /* 40 */ "RBrace",
    /* 41 */ "RBracket",
    /* 42 */ "RParen",
    /* 43 */ "RawString",
    /* 44 */ "Ref",
    /* 45 */ "ScopedList",
    /* 46 */ "ScopedListItems",
    /* 47 */ "ScopedListItemsList",
    /* 48 */ "ScopedQualifiedIdent",
    /* 49 */ "ScopedQualifiedIdentOpt",
    /* 50 */ "Semicolon",
    /* 51 */ "Skip",
    /* 52 */ "String",
    /* 53 */ "StructOrTupleVal",
    /* 54 */ "StructVal",
    /* 55 */ "StructValOpt",
    /* 56 */ "TupleItems",
    /* 57 */ "TupleItemsList",
    /* 58 */ "TupleStructVal",
    /* 59 */ "TupleType",
    /* 60 */ "TupleVal",
    /* 61 */ "TupleValOpt",
    /* 62 */ "TypeSpec",
    /* 63 */ "UseStatement",
];

pub const LOOKAHEAD_AUTOMATA: &[LookaheadDFA; 64] = &[
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
        transitions: &[Trans(0, 12, 1, 62), Trans(0, 17, 2, 63)],
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
            Trans(0, 17, 1, 36),
            Trans(0, 23, 2, 37),
            Trans(0, 24, 1, 36),
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
        transitions: &[Trans(0, 23, 2, 8), Trans(0, 24, 1, 7)],
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
            Trans(0, 26, 1, 4),
        ],
        k: 1,
    },
    /* 10 - "Colon" */
    LookaheadDFA {
        prod0: 83,
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
            Trans(0, 21, 2, 71),
            Trans(0, 23, 2, 71),
            Trans(0, 25, 2, 71),
        ],
        k: 1,
    },
    /* 14 - "ConstDeclaration" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 6, 1, -1),
            Trans(0, 7, 1, -1),
            Trans(0, 8, 8, -1),
            Trans(1, 17, 2, -1),
            Trans(2, 19, 3, -1),
            Trans(3, 9, 12, -1),
            Trans(3, 12, 4, -1),
            Trans(3, 17, 6, -1),
            Trans(3, 24, 7, -1),
            Trans(4, 22, 5, 22),
            Trans(6, 10, 5, 22),
            Trans(6, 18, 5, 22),
            Trans(7, 12, 5, 22),
            Trans(7, 17, 5, 22),
            Trans(7, 24, 5, 22),
            Trans(8, 6, 9, -1),
            Trans(9, 17, 10, -1),
            Trans(10, 19, 11, -1),
            Trans(11, 9, 13, 23),
            Trans(11, 12, 5, 22),
            Trans(11, 17, 5, 22),
            Trans(11, 24, 5, 22),
            Trans(12, 0, 13, 23),
            Trans(12, 5, 13, 23),
            Trans(12, 6, 13, 23),
            Trans(12, 7, 13, 23),
            Trans(12, 8, 13, 23),
            Trans(12, 26, 13, 23),
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
            Trans(0, 16, 2, 31),
            Trans(0, 17, 3, 32),
            Trans(0, 24, 5, 34),
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
            Trans(0, 23, 3, -1),
            Trans(0, 25, 3, -1),
            Trans(1, 11, 2, 39),
            Trans(1, 12, 2, 39),
            Trans(1, 15, 2, 39),
            Trans(1, 16, 2, 39),
            Trans(1, 17, 2, 39),
            Trans(1, 23, 4, 40),
            Trans(1, 24, 2, 39),
            Trans(1, 25, 4, 40),
            Trans(3, 13, 4, 40),
            Trans(3, 14, 4, 40),
            Trans(3, 23, 4, 40),
            Trans(3, 25, 4, 40),
        ],
        k: 2,
    },
    /* 22 - "DoubleColon" */
    LookaheadDFA {
        prod0: 82,
        transitions: &[],
        k: 0,
    },
    /* 23 - "Hash" */
    LookaheadDFA {
        prod0: 90,
        transitions: &[],
        k: 0,
    },
    /* 24 - "Ident" */
    LookaheadDFA {
        prod0: 81,
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
        prod0: 84,
        transitions: &[],
        k: 0,
    },
    /* 27 - "LBracket" */
    LookaheadDFA {
        prod0: 86,
        transitions: &[],
        k: 0,
    },
    /* 28 - "LParen" */
    LookaheadDFA {
        prod0: 88,
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
            Trans(0, 26, 1, 1),
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
            Trans(1, 17, 2, 53),
            Trans(1, 21, 3, 54),
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
            Trans(0, 18, 1, -1),
            Trans(0, 20, 6, -1),
            Trans(0, 21, 7, -1),
            Trans(0, 23, 8, -1),
            Trans(0, 24, 9, -1),
            Trans(0, 25, 10, -1),
            Trans(1, 17, 2, 13),
            Trans(1, 20, 11, 14),
            Trans(3, 11, 11, 14),
            Trans(3, 12, 11, 14),
            Trans(3, 15, 11, 14),
            Trans(3, 16, 11, 14),
            Trans(3, 17, 11, 14),
            Trans(3, 24, 11, 14),
            Trans(4, 0, 11, 14),
            Trans(4, 5, 11, 14),
            Trans(4, 6, 11, 14),
            Trans(4, 7, 11, 14),
            Trans(4, 8, 11, 14),
            Trans(4, 26, 11, 14),
            Trans(5, 11, 11, 14),
            Trans(5, 12, 11, 14),
            Trans(5, 15, 11, 14),
            Trans(5, 16, 11, 14),
            Trans(5, 17, 11, 14),
            Trans(5, 21, 11, 14),
            Trans(5, 23, 11, 14),
            Trans(5, 24, 11, 14),
            Trans(5, 25, 11, 14),
            Trans(6, 17, 11, 14),
            Trans(6, 21, 11, 14),
            Trans(7, 13, 11, 14),
            Trans(7, 14, 11, 14),
            Trans(7, 21, 11, 14),
            Trans(8, 13, 11, 14),
            Trans(8, 14, 11, 14),
            Trans(8, 23, 11, 14),
            Trans(8, 25, 11, 14),
            Trans(9, 11, 11, 14),
            Trans(9, 12, 11, 14),
            Trans(9, 15, 11, 14),
            Trans(9, 16, 11, 14),
            Trans(9, 17, 11, 14),
            Trans(9, 24, 11, 14),
            Trans(9, 25, 11, 14),
            Trans(10, 10, 11, 14),
            Trans(10, 13, 11, 14),
            Trans(10, 14, 11, 14),
            Trans(10, 23, 11, 14),
            Trans(10, 25, 11, 14),
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
            Trans(0, 20, 1, 45),
            Trans(0, 23, 2, 46),
            Trans(0, 24, 1, 45),
            Trans(0, 25, 2, 46),
        ],
        k: 1,
    },
    /* 39 - "QuotedString" */
    LookaheadDFA {
        prod0: 79,
        transitions: &[],
        k: 0,
    },
    /* 40 - "RBrace" */
    LookaheadDFA {
        prod0: 85,
        transitions: &[],
        k: 0,
    },
    /* 41 - "RBracket" */
    LookaheadDFA {
        prod0: 87,
        transitions: &[],
        k: 0,
    },
    /* 42 - "RParen" */
    LookaheadDFA {
        prod0: 89,
        transitions: &[],
        k: 0,
    },
    /* 43 - "RawString" */
    LookaheadDFA {
        prod0: 80,
        transitions: &[],
        k: 0,
    },
    /* 44 - "Ref" */
    LookaheadDFA {
        prod0: 74,
        transitions: &[],
        k: 0,
    },
    /* 45 - "ScopedList" */
    LookaheadDFA {
        prod0: 18,
        transitions: &[],
        k: 0,
    },
    /* 46 - "ScopedListItems" */
    LookaheadDFA {
        prod0: 19,
        transitions: &[],
        k: 0,
    },
    /* 47 - "ScopedListItemsList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 14, 1, -1),
            Trans(0, 21, 3, -1),
            Trans(1, 17, 2, 20),
            Trans(1, 21, 4, 21),
            Trans(3, 13, 4, 21),
            Trans(3, 14, 4, 21),
            Trans(3, 21, 4, 21),
        ],
        k: 2,
    },
    /* 48 - "ScopedQualifiedIdent" */
    LookaheadDFA {
        prod0: 15,
        transitions: &[],
        k: 0,
    },
    /* 49 - "ScopedQualifiedIdentOpt" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 13, 2, 17),
            Trans(0, 14, 2, 17),
            Trans(0, 18, 1, 16),
            Trans(0, 21, 2, 17),
        ],
        k: 1,
    },
    /* 50 - "Semicolon" */
    LookaheadDFA {
        prod0: 75,
        transitions: &[],
        k: 0,
    },
    /* 51 - "Skip" */
    LookaheadDFA {
        prod0: 68,
        transitions: &[],
        k: 0,
    },
    /* 52 - "String" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 15, 1, 77), Trans(0, 16, 2, 78)],
        k: 1,
    },
    /* 53 - "StructOrTupleVal" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 20, 1, 47), Trans(0, 24, 2, 48)],
        k: 1,
    },
    /* 54 - "StructVal" */
    LookaheadDFA {
        prod0: 49,
        transitions: &[],
        k: 0,
    },
    /* 55 - "StructValOpt" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 17, 1, 50), Trans(0, 21, 2, 51)],
        k: 1,
    },
    /* 56 - "TupleItems" */
    LookaheadDFA {
        prod0: 65,
        transitions: &[],
        k: 0,
    },
    /* 57 - "TupleItemsList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 14, 1, -1),
            Trans(0, 25, 3, -1),
            Trans(1, 12, 2, 66),
            Trans(1, 17, 2, 66),
            Trans(1, 24, 2, 66),
            Trans(1, 25, 4, 67),
            Trans(3, 10, 4, 67),
            Trans(3, 14, 4, 67),
            Trans(3, 25, 4, 67),
        ],
        k: 2,
    },
    /* 58 - "TupleStructVal" */
    LookaheadDFA {
        prod0: 56,
        transitions: &[],
        k: 0,
    },
    /* 59 - "TupleType" */
    LookaheadDFA {
        prod0: 64,
        transitions: &[],
        k: 0,
    },
    /* 60 - "TupleVal" */
    LookaheadDFA {
        prod0: 41,
        transitions: &[],
        k: 0,
    },
    /* 61 - "TupleValOpt" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 11, 1, 42),
            Trans(0, 12, 1, 42),
            Trans(0, 15, 1, 42),
            Trans(0, 16, 1, 42),
            Trans(0, 17, 1, 42),
            Trans(0, 24, 1, 42),
            Trans(0, 25, 2, 43),
        ],
        k: 1,
    },
    /* 62 - "TypeSpec" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 12, 2, 58),
            Trans(0, 17, 1, 57),
            Trans(0, 24, 3, 59),
        ],
        k: 1,
    },
    /* 63 - "UseStatement" */
    LookaheadDFA {
        prod0: 11,
        transitions: &[],
        k: 0,
    },
];

pub const PRODUCTIONS: &[Production; 91] = &[
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
            ParseType::N(41),
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
        production: &[ParseType::N(42), ParseType::N(24), ParseType::N(28)],
    },
    // 8 - AttributeArgOpt0: ;
    Production {
        lhs: 7,
        production: &[],
    },
    // 9 - Item: UseStatement^ /* Clipped */;
    Production {
        lhs: 25,
        production: &[ParseType::N(63)],
    },
    // 10 - Item: ConstDeclaration;
    Production {
        lhs: 25,
        production: &[ParseType::N(14)],
    },
    // 11 - UseStatement: 'use' ScopedQualifiedIdent Semicolon;
    Production {
        lhs: 63,
        production: &[ParseType::N(50), ParseType::N(48), ParseType::T(5)],
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
        lhs: 48,
        production: &[ParseType::N(49), ParseType::N(35)],
    },
    // 16 - ScopedQualifiedIdentOpt: DoubleColon ScopedList;
    Production {
        lhs: 49,
        production: &[ParseType::N(45), ParseType::N(22)],
    },
    // 17 - ScopedQualifiedIdentOpt: ;
    Production {
        lhs: 49,
        production: &[],
    },
    // 18 - ScopedList: LBrace ScopedListItems CommaOpt RBrace;
    Production {
        lhs: 45,
        production: &[
            ParseType::N(40),
            ParseType::N(12),
            ParseType::N(46),
            ParseType::N(26),
        ],
    },
    // 19 - ScopedListItems: ScopedQualifiedIdent ScopedListItemsList /* Vec */;
    Production {
        lhs: 46,
        production: &[ParseType::N(47), ParseType::N(48)],
    },
    // 20 - ScopedListItemsList: Comma^ /* Clipped */ ScopedQualifiedIdent ScopedListItemsList;
    Production {
        lhs: 47,
        production: &[ParseType::N(47), ParseType::N(48), ParseType::N(11)],
    },
    // 21 - ScopedListItemsList: ;
    Production {
        lhs: 47,
        production: &[],
    },
    // 22 - ConstDeclaration: ConstPreamble TypeSpec^ /* Clipped */ Assign^ /* Clipped */ ConstVal Semicolon^ /* Clipped */;
    Production {
        lhs: 14,
        production: &[
            ParseType::N(50),
            ParseType::N(19),
            ParseType::N(5),
            ParseType::N(62),
            ParseType::N(16),
        ],
    },
    // 23 - ConstDeclaration: ConstPreamble^ /* Clipped */ Skip^ /* Clipped */;
    Production {
        lhs: 14,
        production: &[ParseType::N(51), ParseType::N(16)],
    },
    // 24 - ConstPreamble: ConstQualifier^ /* Clipped */ ConstName Colon^ /* Clipped */;
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
        production: &[ParseType::N(52)],
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
        production: &[ParseType::N(60)],
    },
    // 35 - ArrayVal: Ref^ /* Clipped */ LBracket^ /* Clipped */ ArrayValOpt /* Option */ RBracket^ /* Clipped */;
    Production {
        lhs: 3,
        production: &[
            ParseType::N(41),
            ParseType::N(4),
            ParseType::N(27),
            ParseType::N(44),
        ],
    },
    // 36 - ArrayValOpt: ConstValList CommaOpt^ /* Clipped */;
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
    // 39 - ConstValListList: Comma^ /* Clipped */ ConstVal ConstValListList;
    Production {
        lhs: 21,
        production: &[ParseType::N(21), ParseType::N(19), ParseType::N(11)],
    },
    // 40 - ConstValListList: ;
    Production {
        lhs: 21,
        production: &[],
    },
    // 41 - TupleVal: LParen^ /* Clipped */ TupleValOpt /* Option */ RParen^ /* Clipped */;
    Production {
        lhs: 60,
        production: &[ParseType::N(42), ParseType::N(61), ParseType::N(28)],
    },
    // 42 - TupleValOpt: ConstValList CommaOpt^ /* Clipped */;
    Production {
        lhs: 61,
        production: &[ParseType::N(12), ParseType::N(20)],
    },
    // 43 - TupleValOpt: ;
    Production {
        lhs: 61,
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
        production: &[ParseType::N(53)],
    },
    // 46 - QualifiedValOpt: ;
    Production {
        lhs: 38,
        production: &[],
    },
    // 47 - StructOrTupleVal: StructVal;
    Production {
        lhs: 53,
        production: &[ParseType::N(54)],
    },
    // 48 - StructOrTupleVal: TupleStructVal;
    Production {
        lhs: 53,
        production: &[ParseType::N(58)],
    },
    // 49 - StructVal: LBrace^ /* Clipped */ StructValOpt /* Option */ RBrace^ /* Clipped */;
    Production {
        lhs: 54,
        production: &[ParseType::N(40), ParseType::N(55), ParseType::N(26)],
    },
    // 50 - StructValOpt: MemberValues Comma^ /* Clipped */;
    Production {
        lhs: 55,
        production: &[ParseType::N(11), ParseType::N(32)],
    },
    // 51 - StructValOpt: ;
    Production {
        lhs: 55,
        production: &[],
    },
    // 52 - MemberValues: MemberValue MemberValuesList /* Vec */;
    Production {
        lhs: 32,
        production: &[ParseType::N(33), ParseType::N(31)],
    },
    // 53 - MemberValuesList: Comma^ /* Clipped */ MemberValue MemberValuesList;
    Production {
        lhs: 33,
        production: &[ParseType::N(33), ParseType::N(31), ParseType::N(11)],
    },
    // 54 - MemberValuesList: ;
    Production {
        lhs: 33,
        production: &[],
    },
    // 55 - MemberValue: Ident Colon^ /* Clipped */ ConstVal;
    Production {
        lhs: 31,
        production: &[ParseType::N(19), ParseType::N(10), ParseType::N(24)],
    },
    // 56 - TupleStructVal: TupleVal;
    Production {
        lhs: 58,
        production: &[ParseType::N(60)],
    },
    // 57 - TypeSpec: QualifiedIdent;
    Production {
        lhs: 62,
        production: &[ParseType::N(35)],
    },
    // 58 - TypeSpec: ArrayType;
    Production {
        lhs: 62,
        production: &[ParseType::N(0)],
    },
    // 59 - TypeSpec: TupleType;
    Production {
        lhs: 62,
        production: &[ParseType::N(59)],
    },
    // 60 - ArrayType: Ref LBracket ArrayTypeSpec RBracket;
    Production {
        lhs: 0,
        production: &[
            ParseType::N(41),
            ParseType::N(1),
            ParseType::N(27),
            ParseType::N(44),
        ],
    },
    // 61 - ArrayTypeSpec: ArrayTypeSpecOpt /* Option */ Ident Semicolon Number;
    Production {
        lhs: 1,
        production: &[
            ParseType::N(34),
            ParseType::N(50),
            ParseType::N(24),
            ParseType::N(2),
        ],
    },
    // 62 - ArrayTypeSpecOpt: Ref;
    Production {
        lhs: 2,
        production: &[ParseType::N(44)],
    },
    // 63 - ArrayTypeSpecOpt: ;
    Production {
        lhs: 2,
        production: &[],
    },
    // 64 - TupleType: LParen TupleItems CommaOpt RParen;
    Production {
        lhs: 59,
        production: &[
            ParseType::N(42),
            ParseType::N(12),
            ParseType::N(56),
            ParseType::N(28),
        ],
    },
    // 65 - TupleItems: TypeSpec TupleItemsList /* Vec */;
    Production {
        lhs: 56,
        production: &[ParseType::N(57), ParseType::N(62)],
    },
    // 66 - TupleItemsList: Comma TypeSpec TupleItemsList;
    Production {
        lhs: 57,
        production: &[ParseType::N(57), ParseType::N(62), ParseType::N(11)],
    },
    // 67 - TupleItemsList: ;
    Production {
        lhs: 57,
        production: &[],
    },
    // 68 - Skip: /(?s)&\[Production; \d+\] = &\[.*(?-s)/;
    Production {
        lhs: 51,
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
        lhs: 44,
        production: &[ParseType::T(12)],
    },
    // 75 - Semicolon: ';';
    Production {
        lhs: 50,
        production: &[ParseType::T(13)],
    },
    // 76 - Comma: ',';
    Production {
        lhs: 11,
        production: &[ParseType::T(14)],
    },
    // 77 - String: QuotedString;
    Production {
        lhs: 52,
        production: &[ParseType::N(39)],
    },
    // 78 - String: RawString;
    Production {
        lhs: 52,
        production: &[ParseType::N(43)],
    },
    // 79 - QuotedString: /r#{0, 3}?".*?"#{0, 3}/;
    Production {
        lhs: 39,
        production: &[ParseType::T(15)],
    },
    // 80 - RawString: /"(\\.|[^\\])*?"/;
    Production {
        lhs: 43,
        production: &[ParseType::T(16)],
    },
    // 81 - Ident: /[a-zA-Z_][a-zA-Z0-9_]*/;
    Production {
        lhs: 24,
        production: &[ParseType::T(17)],
    },
    // 82 - DoubleColon: '::';
    Production {
        lhs: 22,
        production: &[ParseType::T(18)],
    },
    // 83 - Colon: ':';
    Production {
        lhs: 10,
        production: &[ParseType::T(19)],
    },
    // 84 - LBrace: '{';
    Production {
        lhs: 26,
        production: &[ParseType::T(20)],
    },
    // 85 - RBrace: '}';
    Production {
        lhs: 40,
        production: &[ParseType::T(21)],
    },
    // 86 - LBracket: '[';
    Production {
        lhs: 27,
        production: &[ParseType::T(22)],
    },
    // 87 - RBracket: ']';
    Production {
        lhs: 41,
        production: &[ParseType::T(23)],
    },
    // 88 - LParen: '(';
    Production {
        lhs: 28,
        production: &[ParseType::T(24)],
    },
    // 89 - RParen: ')';
    Production {
        lhs: 42,
        production: &[ParseType::T(25)],
    },
    // 90 - Hash: /#/;
    Production {
        lhs: 23,
        production: &[ParseType::T(26)],
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
