#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_fields_graphql_format_1_0b411f46() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("graphql")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{\n  posts {, title, votes, author {,   firstName,   posts {, author { firstName } }\n    }}\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "{\n  posts {\n    title\n    votes\n    author {\n      firstName\n      posts {\n        author {\n          firstName\n        }\n      }\n    }\n  }\n}");
}
