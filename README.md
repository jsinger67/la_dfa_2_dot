# LaDfa2Dot

This is a tool to support the development of [parol](https://github.com/jsinger67/parol.git).

It creates Graphviz representations of `parol`'s LookaheadDFAs.

The syntax is derived from the generated parser source (Rust). The tool transforms a single DFA
copied from the generated parser source into a DOT format.
This way changes in the resulting DFA could be evaluated much easier.
