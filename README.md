# LaDfa2Dot

This is a tool to support the development of [parol](https://github.com/jsinger67/parol.git).

It creates Graphviz representations of `parol`'s LookaheadDFAs.

The syntax is derived from the generated parser source (Rust). The tool transforms a generated
parser file into DOT files which are put into the given output folder.

It also inserts the correct terminal name in lieu of the rather cryptic terminal index into the
generated DOT files to increase readability.

This way changes in the resulting DFA could be evaluated much easier.

Try

```shell
la_dfa_2_dot --help
```

to get help.
