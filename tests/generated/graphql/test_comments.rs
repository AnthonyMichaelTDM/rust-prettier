#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_argument_comment_graphql_format_1_ba2670e4() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("graphql")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\nquery (\n  $string: String, # Some variable comment\n  $bool: Boolean # Some comment\n ) {\n   someField\n }") ? ;
    assert_eq ! (formatted , "query (\n  $string: String # Some variable comment\n  $bool: Boolean # Some comment\n) {\n  someField\n}");
    Ok(())
}
#[test]
fn test_fields_graphql_format_1_16ad3a17() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("graphql")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("query { \n  someField # Trailing comment\n}\n\n")?;
    assert_eq!(formatted, "query {\n  someField # Trailing comment\n}");
    Ok(())
}
#[test]
fn test_interfaces_graphql_format_1_64422827() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("graphql")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type Type1\nimplements\nA & B &\n# comment 1\n                 C & D &\n# comment 2\n E {a: a}") ? ;
    assert_eq!(
        formatted,
        "type Type1 implements A & B &\n# comment 1\nC & D &\n# comment 2\nE {\n  a: a\n}"
    );
    Ok(())
}
#[test]
fn test_tokens_graphql_format_1_1f9f1e0f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("graphql")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("# import \"./claimsFragment.gql\"\n\nquery claimsByBookingReferenceAndLastName($bookingReference: String!, $lastName: String!) {\n  claimsByBookingReferenceAndLastName(bookingReference: $bookingReference, lastName: $lastName) {\n    ... claim\n  }\n}") ? ;
    assert_eq ! (formatted , "# import \"./claimsFragment.gql\"\n\nquery claimsByBookingReferenceAndLastName(\n  $bookingReference: String!\n  $lastName: String!\n) {\n  claimsByBookingReferenceAndLastName(\n    bookingReference: $bookingReference\n    lastName: $lastName\n  ) {\n    ...claim\n  }\n}");
    Ok(())
}
