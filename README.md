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
`treeedb-java.dl`. Create a Java file:

```java
class HelloWorld {
    public static void main(String[] args) {
        int x = 2 + 2;
    }
}
```

Let's try to find constant-valued variables in this code!

TODO(lb)!

[cargo]: https://doc.rust-lang.org/cargo/
[tree-sitter]: https://tree-sitter.github.io/tree-sitter/
[releases]: https://github.com/langston-barrett/treeedb/releases
[rustup]: https://rustup.rs/
[souffle]: https://souffle-lang.github.io/index.html
