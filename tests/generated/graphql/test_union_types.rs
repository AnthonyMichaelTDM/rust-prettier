#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_union_types_graphql_format_1_9faabb23() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("graphql")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("union myUnion = ATypeName | ASecondTypeName\n\nunion tooLongNameeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee = A | B\n\nunion mySecondUnion = ATypeName | ASecondTypeName | AThirdTypeName\n\nunion myThirdUnion = AVeryVeryVeryLongNamedTypeName | ASecondVeryVeryVeryLongedNameTypeName \n\nunion longUnion = A | B | C | D | E | F | G | H | I | J | K | L |  A | B | C | D | E | F | G | H | I | J | K | L\n\n# comment\n# comment2\nunion union = B | C | ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "union myUnion = ATypeName | ASecondTypeName\n\nunion tooLongNameeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee =\n    A\n  | B\n\nunion mySecondUnion = ATypeName | ASecondTypeName | AThirdTypeName\n\nunion myThirdUnion =\n    AVeryVeryVeryLongNamedTypeName\n  | ASecondVeryVeryVeryLongedNameTypeName\n\nunion longUnion =\n    A\n  | B\n  | C\n  | D\n  | E\n  | F\n  | G\n  | H\n  | I\n  | J\n  | K\n  | L\n  | A\n  | B\n  | C\n  | D\n  | E\n  | F\n  | G\n  | H\n  | I\n  | J\n  | K\n  | L\n\n# comment\n# comment2\nunion union = B | C | D");
}
