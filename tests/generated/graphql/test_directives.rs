#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_directives_graphql_format_1_7d86c7c1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["graphql"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("query MyQuery @directive(\n  arg: 5\n) {\n  field\n  @skip(if: true) @nope\n  otherField\n  ...fragmentSpread, @include(if: [\"this isn't even a boolean\", \"wow, that's really odd\",,,,,])\n}\n\nfragment YouCanHaveDirectivesHereToo on SomeType @yesReally(what: \"yes\") {\n  fields\n  ... on AType @what(sup: \"yo\") @otherDirective { goodbye}\n  ... @notEvenATypeHere(args: [1, 2, 3]) {\n    hello\n  }\n\n  thisFieldHasALotOfDirectives @thisIsthefirst @thisIsTheSecond @thisIsTheThird @thisIstheFourthWillBeTooLongForSure (and: \"it has arguments as well\")\n}\n\nquery QueryWVars($x: String) @directive { hey }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "query MyQuery @directive(arg: 5) {\n  field @skip(if: true) @nope\n  otherField\n  ...fragmentSpread\n    @include(if: [\"this isn't even a boolean\", \"wow, that's really odd\"])\n}\n\nfragment YouCanHaveDirectivesHereToo on SomeType @yesReally(what: \"yes\") {\n  fields\n  ... on AType @what(sup: \"yo\") @otherDirective {\n    goodbye\n  }\n  ... @notEvenATypeHere(args: [1, 2, 3]) {\n    hello\n  }\n\n  thisFieldHasALotOfDirectives\n    @thisIsthefirst\n    @thisIsTheSecond\n    @thisIsTheThird\n    @thisIstheFourthWillBeTooLongForSure(and: \"it has arguments as well\")\n}\n\nquery QueryWVars($x: String) @directive {\n  hey\n}");
}
