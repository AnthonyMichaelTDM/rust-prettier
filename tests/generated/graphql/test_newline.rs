#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_directive_decl_graphql_format_1_73bf5a9b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("graphql")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("directive @dir(\n  # comment\n  arg1: String\n\n\n  # comment\n  arg2: String\n  arg3: String\n\n) on QUERY") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "directive @dir(\n  # comment\n  arg1: String\n\n  # comment\n  arg2: String\n  arg3: String\n) on QUERY");
}
#[test]
fn test_directives_graphql_format_1_790f3011() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("graphql")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("query MyQuery @directive(\n  arg: 5\n\n  # comment\n  arg2: 10\n\n) {\n  field @skip(\n    if: true\n\n    # comment\n    cursor: 10\n\n  ) @nope\n  otherField\n  ...fragmentSpread, @include(if: [\"this isn't even a boolean\", \"wow, that's really odd\",,,,,])\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "query MyQuery\n@directive(\n  arg: 5\n\n  # comment\n  arg2: 10\n) {\n  field\n    @skip(\n      if: true\n\n      # comment\n      cursor: 10\n    )\n    @nope\n  otherField\n  ...fragmentSpread\n    @include(if: [\"this isn't even a boolean\", \"wow, that's really odd\"])\n}");
}
#[test]
fn test_enum_graphql_format_1_126f3173() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("graphql")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("enum State {\n  # pending state\n  PENDING\n\n  # visible states\n  VISIBLE\n  INVISIBLE\n\n  # archive state\n  ARCHIVED\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "enum State {\n  # pending state\n  PENDING\n\n  # visible states\n  VISIBLE\n  INVISIBLE\n\n  # archive state\n  ARCHIVED\n}");
}
#[test]
fn test_fields_graphql_format_1_8e66c31d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("graphql")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("query MyFirstQuery {\n  # comment\n  field {\n\n    subfield\n\n    # comment\n    subfield\n\n  }\n\n  field\n  #comment\n  field\n\n}\n\nmutation MyFirstMutation {\n\n  # comment\n  name\n\n  comment # comment\n  kind\n\n}\n\nsubscription MySubscription {\n\n  name\n\n  comment\n  kind\n\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "query MyFirstQuery {\n  # comment\n  field {\n    subfield\n\n    # comment\n    subfield\n  }\n\n  field\n  #comment\n  field\n}\n\nmutation MyFirstMutation {\n  # comment\n  name\n\n  comment # comment\n  kind\n}\n\nsubscription MySubscription {\n  name\n\n  comment\n  kind\n}");
}
#[test]
fn test_input_graphql_format_1_f4861a8e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("graphql")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("input Params {\n  # Id\n  id: ID\n\n  # Name\n  name: String\n\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "input Params {\n  # Id\n  id: ID\n\n  # Name\n  name: String\n}"
    );
}
#[test]
fn test_interface_graphql_format_1_2bfd3eb7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("graphql")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("interface Actor {\n  # Id\n  id: ID\n\n  # Actor fields\n  name: String\n  kind: String\n\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "interface Actor {\n  # Id\n  id: ID\n\n  # Actor fields\n  name: String\n  kind: String\n}");
}
#[test]
fn test_object_type_def_graphql_format_1_9b3925fe() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("graphql")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type Artist implements Node& Entity {\n  # The ID of an object\n  id: ID!\n\n  # The MBID of the entity.\n  mbid: MBID!\n\n  # A list of recordings linked to this entity.\n  recordings(after: String, first: Int): RecordingConnection\n\n  # A list of releases linked to this entity.\n  releases(\n    # Filter by one or more release group types.\n    type: [ReleaseGroupType]\n\n    # Filter by one or more release statuses.\n    status: [ReleaseStatus]\n    after: String\n    first: Int\n  ): ReleaseConnection\n\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type Artist implements Node & Entity {\n  # The ID of an object\n  id: ID!\n\n  # The MBID of the entity.\n  mbid: MBID!\n\n  # A list of recordings linked to this entity.\n  recordings(after: String, first: Int): RecordingConnection\n\n  # A list of releases linked to this entity.\n  releases(\n    # Filter by one or more release group types.\n    type: [ReleaseGroupType]\n\n    # Filter by one or more release statuses.\n    status: [ReleaseStatus]\n    after: String\n    first: Int\n  ): ReleaseConnection\n}");
}
#[test]
fn test_schema_graphql_format_1_3d9f3c6b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("graphql")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("# Schema\nschema {\n  # Query and Mutation\n  query: Root\n  mutation: Mutation\n\n  # Subscription\n  subscription: Subscription\n\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "# Schema\nschema {\n  # Query and Mutation\n  query: Root\n  mutation: Mutation\n\n  # Subscription\n  subscription: Subscription\n}");
}
