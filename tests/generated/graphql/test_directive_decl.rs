#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_directive_decl_graphql_format_1_b9347be9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("graphql")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("directive @a on QUERY\ndirective @a(as: String) on QUERY\ndirective @a(as: String = 1) on QUERY\ndirective @a(as: String, b: Int!) on QUERY\ndirective @a(as: String! = 1 @deprecated) on QUERY\ndirective @a(as: String! = 1 @deprecated) on QUERY | MUTATION\ndirective @a(as: String! = 1 @deprecated) repeatable on QUERY | MUTATION\n\ndirective @a on QUERY\ndirective @a repeatable on QUERY\n\n\ndirective @a(as: String) on QUERY\n\n\n\n\ndirective @a(as: String = 1) on QUERY") ? ;
    assert_eq ! (formatted , "directive @a on QUERY\ndirective @a(as: String) on QUERY\ndirective @a(as: String = 1) on QUERY\ndirective @a(as: String, b: Int!) on QUERY\ndirective @a(as: String! = 1 @deprecated) on QUERY\ndirective @a(as: String! = 1 @deprecated) on QUERY | MUTATION\ndirective @a(as: String! = 1 @deprecated) repeatable on QUERY | MUTATION\n\ndirective @a on QUERY\ndirective @a repeatable on QUERY\n\ndirective @a(as: String) on QUERY\n\ndirective @a(as: String = 1) on QUERY");
    Ok(())
}
