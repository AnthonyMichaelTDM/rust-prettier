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
  - we definitely don't want to have to write our own parser, here are some other options:
    - [biome's parser](https://crates.io/crates/biome_js_parser)
      - pros: pure rust, so it should play nice with wasm; supports a lot of js file-types, including typescript; very well documented.
      - cons: not used much outside of the project it was made for, a little heavy
    - [swc's parser](https://crates.io/crates/swc_ecma_parser) (also supports typescript through a feature flag)
      - pros: pure rust, supports typescript, well maintained, good documentation, lightweight (>150KiB)
      - cons: really wants to parse a whole file, not snippets of code. This is quite problematic for testing but not so much for production use cases.
  - [ ] parse a string of code into an AST for JavaScript
  - [ ] support embedded languages
  - [ ] support comments
  - [ ] support pragma
- [ ] parse a AST into a document
- [x] render a document
  - [x] define the Doc tree-like structure
  - [x] utilities for operating on Doc trees
  - [x] render a Doc tree into a string (with options)

### TODO list

- [x] build script to generate rust tests from the Prettier repo's format test snapshots
- [x] port the Document Internal Representation
- [x] render a Doc to a string
- [x] organization: flesh out the Parser trait, move things around, separate module for config, maybe rethink configs as a whole (remember, It will eventually need to support plugins).
- [ ] experiment with the various parsers until I find the one I want to use (likely going to be biome's parser)
  - eventually will probably support multiple through feature flags or something
- [ ] use the chosen parser to implement the "text to AST" part of formatting
- [ ] port the comments api/algorithm from the JS version
- [ ] implement the "AST to Doc" part of formatting
- [ ] workspace organization
- [ ] cli tool (really simple, just take a file path as input, and output the formatted test of said input)
- [ ] wasm bundle, just expose the api as wasm bindings
