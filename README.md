# treeedb

`treeedb` makes it easier to start writing a source-level program analysis in
[Soufflé Datalog][souffle]. First, `treeedb` generates Soufflé types and
relations that represent a program's AST. Then, `treeedb` parses source code
and emits facts that populate those relations.

`treeedb` currently supports analysis of these languages:

- C
- Java
- Soufflé

`treeedb`'s parsers and ASTs are based on [tree-sitter][tree-sitter] grammars,
and it's very easy to [add support](#adding-a-language) for any [language with a
tree-sitter grammar][tree-sitter-langs].

The name `treeedb` is a portmanteau of "tree-sitter" with "EDB", where EDB
stands for "extensional database" and refers to the set of facts in a Datalog
program.

## Installation

You'll need two artifacts for each programming language you want to analyze:

1. A Soufflé file with the types and relations defining the AST
2. The executable that parses that language and emits facts

For instance, for Java these are called `treeedb-java.dl` and `treeedb-java`,
respectively.

To actually analyze some code, you'll also need to [install
Soufflé][souffle-install].

### Install From a Release

Navigate to the most recent release on the [releases page][releases] and
download the artifacts related to the language you want to analyze. The
pre-built executables are statically linked, but are [currently][#3] only
available for Linux.

### Build From Source

To build from source, you'll need the Rust compiler and the [Cargo][cargo] build
tool. [rustup][rustup] makes it very easy to obtain these. Then, run:

```bash
cargo build --release
```

You can find the `treeedb-<LANG>` binaries in `target/release`. To generate
the Datalog file, run the corresponding `treeedbgen-souffle-<LANG>` binary.

## Example: Analyzing Java Code

To follow along with this example, follow [the installation
instructions](#installation) for Java. Then, create a Java file named
`Main.java`:

```java
class Main {
    public static void main(String[] args) {
        int x = 2 + 2;
    }
}
```

(The files shown in this section are also available in
[`examples/java/`](./examples/java/).)

Create a Datalog file named `const-binop.dl` that includes `treeedb-java.dl` and
has a rule to find constant-valued binary expressions:

```souffle
#include "treeedb-java.dl"

.decl const_binop(expr: JavaBinaryExpression)

const_binop(expr) :-
  java_binary_expression(expr),
  java_binary_expression_left_f(expr, l),
  java_binary_expression_right_f(expr, r),
  java_decimal_integer_literal(l),
  java_decimal_integer_literal(r).

.decl show_const_binop(text: JavaNodeText)

show_const_binop(text) :-
  const_binop(expr),
  java_node_text(expr, text).

.output const_binop(IO=stdout)
.output show_const_binop(IO=stdout)
```

Generate the input files (`node.csv` and `field.csv`):

```bash
treeedb-java Main.java
```

Finally, run the analysis with Soufflé:

```bash
souffle const-binop.dl
```

You'll see something like this:

```
---------------
const_binop
===============
94001952741472
===============
---------------
show_const_binop
===============
2 + 2
===============
```

### Digging Deeper

To see what type and relation names are available, look at
`treeedb-<LANGUAGE>.dl`. If it's not evident which part of the language a given
type or relation corresponds to, take a look at the tree-sitter grammar (e.g.
[grammar.js in the tree-sitter-java repo][java-grammar] for Java).

## Motivation and Comparison to Other Tools

Before writing a program analysis in Datalog, you need to figure out (1) how to
represent the program as relations, and (2) how to ingest programs into that
representation. State-of-the-art Datalog projects do all this "by hand":

- [cclyzer++][cclyzerpp] has a ["schema" directory][cclyzerpp-schema] (1) and
  the [FactGenerator][cclyzerpp-fact-generator] (2).
- [Doop][doop] has a big [imports.dl][doop-imports] file (1) and [a variety
  of generators][doop-gen] (2).
- [ddisasm][ddisasm] has the [gtirb-decoder][ddisasm-gtirb-decoder] (2).
- [securify][securify] has [`analysis-input.dl`][securify-input] (1).

Writing these representations and ingestion tools takes up valuable time and
distracts from the work of writing analyses. `treeedb` aims to automate it,
fitting in the same niche as these tools.

## Repository Structure

<!-- for f in **/Cargo.toml; do printf "- [\`%s\`](%s): %s\n" "$(dirname ${f})" "./$(dirname ${f})"  "$(grep descript "${f}" | grep -oP 'description = "\K[^"]+')"; done -->

- [`treeedb`](./treeedb): Generate Datalog facts from tree-sitter parse trees
- [`treeedb-c`](./treeedb-c): Generate Datalog facts from C source code
- [`treeedbgen`](./treeedbgen): Parse node-types.json from a tree-sitter grammar
- [`treeedbgen-souffle`](./treeedbgen-souffle): Generate Soufflé types and relations from tree-sitter grammars
- [`treeedbgen-souffle-c`](./treeedbgen-souffle-c): Generate Soufflé types and relations from the C tree-sitter grammar
- [`treeedbgen-souffle-java`](./treeedbgen-souffle-java): Generate Soufflé types and relations from the Java tree-sitter grammar
- [`treeedbgen-souffle-souffle`](./treeedbgen-souffle-souffle): Generate Soufflé types and relations from the Soufflé tree-sitter grammar
- [`treeedb-java`](./treeedb-java): Generate Datalog facts from Java source code
- [`treeedb-souffle`](./treeedb-souffle): Generate Datalog facts from Soufflé source code

## Contributing

Thank you for your interest in `treeedb`! We welcome and appreciate all kinds of
contributions. Please feel free to file and issue or open a pull request.

### Adding a Language

As explained in [Installation](#installation), there are two tools involved in
supporting analysis of each programming language: One to generate Soufflé types
and relations (e.g., `treeedbgen-souffle-c`), and another to parse the language
being analyzed and emit facts (e.g., `treeedb-c`).

To add a new language:

- Create new directories `treeedb-<LANG>` and `treeedbgen-souffle-<LANG>`
  with the same structure as an existing one (it might be easiest to just
  recursively copy existing ones).
- Add the new directories to the top-level [`Cargo.toml`](Cargo.toml).
- Add the language to `.github/workflows/release.yml` by copying and modifying
  existing lines for other languages.

[#3]: https://github.com/langston-barrett/treeedb/issues/3
[cargo]: https://doc.rust-lang.org/cargo/
[cclyzerpp-fact-generator]: https://galoisinc.github.io/cclyzerpp/architecture.html#the-fact-generator
[cclyzerpp-schema]: https://github.com/GaloisInc/cclyzerpp/tree/746e30ac4579da68e06d49faac27f1f88d8edc72/datalog/schema
[cclyzerpp]: https://galoisinc.github.io/cclyzerpp/index.html
[ddisasm]: https://github.com/GrammaTech/ddisasm
[ddisasm-gtirb-decoder]: https://github.com/GrammaTech/ddisasm/tree/c56be069dc9565e4267f3cbb6ca02fb6b97bca2e/src/gtirb-decoder
[doop-gen]: https://bitbucket.org/yanniss/doop/src/master/generators/
[doop-imports]: https://bitbucket.org/yanniss/doop/src/55d39516653efb634f833fccb5b3d30ae472badb/souffle-logic/facts/imports.dl?at=master
[doop]: https://bitbucket.org/yanniss/doop/src/master/
[java-grammar]: https://github.com/tree-sitter/tree-sitter-java/blob/master/grammar.js
[releases]: https://github.com/langston-barrett/treeedb/releases
[rustup]: https://rustup.rs/
[securify]: https://github.com/eth-sri/securify2
[securify-input]: https://github.com/eth-sri/securify2/blob/71c22dd3d6fc74fb87ed4c4118710642a0d6707e/securify/staticanalysis/souffle_analysis/analysis-input.dl
[souffle-install]: https://souffle-lang.github.io/install
[souffle]: https://souffle-lang.github.io/index.html
[tree-sitter-langs]: https://tree-sitter.github.io/tree-sitter/#available-parsers
[tree-sitter]: https://tree-sitter.github.io/tree-sitter/
