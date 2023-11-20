# Prettier (Rust Port)

## Intro

Prettier is an opinionated code formatter. It enforces a consistent style by parsing your code and re-printing it with its own rules that take the maximum line length into account, wrapping code when necessary.

### Input

<!-- prettier-ignore -->
```rust
foo(reallyLongArg(), omgSoManyParameters(), IShouldRefactorThis(), isThereSeriouslyAnotherOne());
```

### Output

```rust
foo(
  reallyLongArg(),
  omgSoManyParameters(),
  IShouldRefactorThis(),
  isThereSeriouslyAnotherOne(),
);
```

## Roadmap

- [ ] >95% test complience
  - [x] port tests
    - [x] build script that parses `__snapshots__` from the Prettier format tests to generate rust tests of the same input, output, and options.
  - [ ] pass tests
  - As per the [challenge specs](https://console.algora.io/challenges/prettier), only need to be able to do this with JavaScript (ES6)
  - we can use [swc](https://swc.rs/) for this maybe
  - we definitely don't want to have to write our own parser, here are some other options:
    - [tree-sitter](https://crates.io/crates/tree-sitter-javascript)
      - pros: general, well maintained. There are tree-sitter grammars for a lot of languages, so we could reuse ideas and code for future parsers.
      - cons: not a rust library, so we have to use C FFI. Meaning we won't be able to compile to wasm.
    - [biome's parser](https://crates.io/crates/biome_js_parser)
      - pros: pure rust, so it should play nice with wasm; supports a lot of js file-types, including typescript; very well documented.
      - cons: not used much outside of the project it was made for, a little heavy
    - [swc's parser](https://crates.io/crates/swc_ecma_parser) (also supports typescript through a feature flag)
      - pros: pure rust, supports typescript, well maintained, good documentation, lightweight (>150KiB)
      - cons: really wants to parse a whole file, not snippets of code. This is quite problematic for testing but not so much for production use cases.
    - [deno's parser](https://crates.io/crates/deno_ast)
      - pros: similar to swc's parser
      - cons: also really wants to parse whole files, does not support parsing a string of code
  - [ ] parse a string of code into an AST for JavaScript
  - [ ] support embedded languages
  - [ ] support comments
  - [ ] support pragma
- [ ] parse a AST into a document
- [x] render a document
  - [x] define the Doc tree-like structure
  - [x] utilities for operating on Doc trees
  - [x] render a Doc tree into a string (with options)
