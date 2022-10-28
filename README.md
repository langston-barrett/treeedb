# treeedb

`treeedb` generates [Soufflé Datalog][souffle] types, relations, and facts that
represent ASTs from a variety of programming languages. The parsers are based on
[tree-sitter][tree-sitter] grammars.

<!-- for f in **/Cargo.toml; do printf "- [\`%s\`](%s): %s\n" "$(dirname ${f})" "./$(dirname ${f})"  "$(grep descript "${f}" | grep -oP 'description = "\K[^"]+')"; done -->

- [`treeedb`](./treeedb): Generate Datalog facts from tree-sitter parse trees
- [`treeedbgen`](./treeedbgen): Parse node-types.json from a tree-sitter grammar
- [`treeedbgen-souffle`](./treeedbgen-souffle): Generate Soufflé types and relations from tree-sitter grammars
- [`treeedbgen-souffle-java`](./treeedbgen-souffle-java): Generate Soufflé types and relations from the Java tree-sitter grammar
- [`treeedbgen-souffle-souffle`](./treeedbgen-souffle-souffle): Generate Soufflé types and relations from the Soufflé tree-sitter grammar
- [`treeedb-java`](./treeedb-java): Generate Datalog facts from Java source code
- [`treeedb-souffle`](./treeedb-souffle): Generate Datalog facts from Soufflé source code

Don't see your favorite language? Adding a new one is *very* simple, see any of
the existing languages. Or file an issue!

The name is a portmanteau of "tree-sitter" with "EDB", where EDB stands for
"extensional database" and refers to the set of facts in a Datalog program.

## Usage

### Example: Analyzing Java Code with Soufflé

Navigate to the most recent release on the [releases page][releases]. Download
the Java-related artifacts, namely the `treeedb-java` executable and
`treeedb-java.dl`. Create a Java file named `Main.java` (the files shown in this
example are also available in [`examples/java/`](./examples/java/)):

```java
class Main {
    public static void main(String[] args) {
        int x = 2 + 2;
    }
}
```

Let's try to find constant-valued binary operations in this code! Create a
Datalog file named `const-binop.dl` that includes `treeedb-java.dl` and has a
rule to find constant-valued binary expressions:

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

Generate the input files:

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

## Contributing

Thank you for your interest in `treeedb`! We welcome and appreciate all kinds of
contributions. Please feel free to file and issue or open a pull request.

[cargo]: https://doc.rust-lang.org/cargo/
[java-grammar]: https://github.com/tree-sitter/tree-sitter-java/blob/master/grammar.js
[tree-sitter]: https://tree-sitter.github.io/tree-sitter/
[releases]: https://github.com/langston-barrett/treeedb/releases
[rustup]: https://rustup.rs/
[souffle]: https://souffle-lang.github.io/index.html
