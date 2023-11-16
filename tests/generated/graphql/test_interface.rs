#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_interface_graphql_format_1_f0ebdc18() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("# https://github.com/graphql/graphql-spec/blob/master/spec/Section%203%20--%20Type%20System.md#interfaces\n\ninterface Actor {\n  id: ID\n  name: String\n}\n\ninterface Resource implements Node\n{\n  id: ID!\n  url: String\n}\n\ninterface Resource implements\nNode {\n  id: ID!\n  url: String\n}\n\ninterface Image implements Resource\n& Node {\n  id: ID!\n  url: String\n  thumbnail: String\n}\n\ninterface Node implements Named &\nNode {\n  id: ID!\n  name: String\n}\n\ninterface Named implements\nNode & Named {\n  id: ID!\n  name: String\n}\n\n# \\`InterfaceTypeExtension\\`\nextend interface Bar implements\nA& B & C {\n  two(argument: InputType!): Type\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "# https://github.com/graphql/graphql-spec/blob/master/spec/Section%203%20--%20Type%20System.md#interfaces\n\ninterface Actor {\n  id: ID\n  name: String\n}\n\ninterface Resource implements Node {\n  id: ID!\n  url: String\n}\n\ninterface Resource implements Node {\n  id: ID!\n  url: String\n}\n\ninterface Image implements Resource & Node {\n  id: ID!\n  url: String\n  thumbnail: String\n}\n\ninterface Node implements Named & Node {\n  id: ID!\n  name: String\n}\n\ninterface Named implements Node & Named {\n  id: ID!\n  name: String\n}\n\n# \\`InterfaceTypeExtension\\`\nextend interface Bar implements A & B & C {\n  two(argument: InputType!): Type\n}");
}
#[test]
fn test_object_type_def_graphql_format_1_bd3090a2() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("type Artist implements Node & Entity {\n  # The ID of an object\n  id: ID!\n\n  # The MBID of the entity.\n  mbid: MBID!\n\n  # A list of recordings linked to this entity.\n  recordings(after: String, first: Int): RecordingConnection\n\n  # A list of releases linked to this entity.\n  releases(\n    # Filter by one or more release group types.\n    type: [ReleaseGroupType]\n\n    # Filter by one or more release statuses.\n    status: [ReleaseStatus]\n    after: String\n    first: Int\n  ): ReleaseConnection\n\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type Artist implements Node & Entity {\n  # The ID of an object\n  id: ID!\n\n  # The MBID of the entity.\n  mbid: MBID!\n\n  # A list of recordings linked to this entity.\n  recordings(after: String, first: Int): RecordingConnection\n\n  # A list of releases linked to this entity.\n  releases(\n    # Filter by one or more release group types.\n    type: [ReleaseGroupType]\n\n    # Filter by one or more release statuses.\n    status: [ReleaseStatus]\n    after: String\n    first: Int\n  ): ReleaseConnection\n}");
}
#[test]
fn test_object_type_def_mixed_syntax_graphql_format_1_118b8080() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("type MixedArtist implements Node& Entity & Releasable {\n  # The ID of an object\n  id: ID!\n\n  # The MBID of the entity.\n  mbid: MBID!\n\n  # A list of recordings linked to this entity.\n  recordings(after: String, first: Int): RecordingConnection\n\n  # A list of releases linked to this entity.\n  releases(\n    # Filter by one or more release group types.\n    type: [ReleaseGroupType]\n\n    # Filter by one or more release statuses.\n    status: [ReleaseStatus]\n    after: String\n    first: Int\n  ): ReleaseConnection\n\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type MixedArtist implements Node & Entity & Releasable {\n  # The ID of an object\n  id: ID!\n\n  # The MBID of the entity.\n  mbid: MBID!\n\n  # A list of recordings linked to this entity.\n  recordings(after: String, first: Int): RecordingConnection\n\n  # A list of releases linked to this entity.\n  releases(\n    # Filter by one or more release group types.\n    type: [ReleaseGroupType]\n\n    # Filter by one or more release statuses.\n    status: [ReleaseStatus]\n    after: String\n    first: Int\n  ): ReleaseConnection\n}");
}
#[test]
fn test_object_type_def_old_syntax_graphql_format_1_32d6bbb7() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("type OldArtist implements Node& Entity {\n  # The ID of an object\n  id: ID!\n\n  # The MBID of the entity.\n  mbid: MBID!\n\n  # A list of recordings linked to this entity.\n  recordings(after: String, first: Int): RecordingConnection\n\n  # A list of releases linked to this entity.\n  releases(\n    # Filter by one or more release group types.\n    type: [ReleaseGroupType]\n\n    # Filter by one or more release statuses.\n    status: [ReleaseStatus]\n    after: String\n    first: Int\n  ): ReleaseConnection\n\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type OldArtist implements Node & Entity {\n  # The ID of an object\n  id: ID!\n\n  # The MBID of the entity.\n  mbid: MBID!\n\n  # A list of recordings linked to this entity.\n  recordings(after: String, first: Int): RecordingConnection\n\n  # A list of releases linked to this entity.\n  releases(\n    # Filter by one or more release group types.\n    type: [ReleaseGroupType]\n\n    # Filter by one or more release statuses.\n    status: [ReleaseStatus]\n    after: String\n    first: Int\n  ): ReleaseConnection\n}");
}
#[test]
fn test_separator_detection_graphql_format_1_e16c5aac() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("# This file used to test legacy syntax of interfaces that already removed in \\`graphql\\` v16\n# Some comments are outdated\ntype Type1 implements A& B& C& D\n# {&&&&&&&&\n# Should separate by \\`,\\` not \\`&\\`\n{a: a}\n\ntype Type2 implements A& B& C& D\n# &&&&{}&&&&\n# Should separate by \\`,\\` not \\`&\\`\n{a: a}\n\ntype Type3 implements A&\n# &&&&&&&& comment line 1\n   # &&&&&&&& comment line 2\nB& C& D\n{a: a}\n\ntype Type4 implements A\n# &&&&&&&& comment line 1\n&\n   # &&&&&&&& comment line 2\nB& C& D\n{a: a}\n\ntype Type5 implements A\n# &&&&&&&& comment line 1\n   # &&&&&&&& comment line 2\n&B& C& D\n{a: a}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "# This file used to test legacy syntax of interfaces that already removed in \\`graphql\\` v16\n# Some comments are outdated\ntype Type1 implements A & B & C & D {\n  # {&&&&&&&&\n  # Should separate by \\`,\\` not \\`&\\`\n  a: a\n}\n\ntype Type2 implements A & B & C & D {\n  # &&&&{}&&&&\n  # Should separate by \\`,\\` not \\`&\\`\n  a: a\n}\n\ntype Type3 implements A &\n# &&&&&&&& comment line 1\n# &&&&&&&& comment line 2\nB & C & D {\n  a: a\n}\n\ntype Type4 implements A &\n# &&&&&&&& comment line 1\n# &&&&&&&& comment line 2\nB & C & D {\n  a: a\n}\n\ntype Type5 implements A &\n# &&&&&&&& comment line 1\n# &&&&&&&& comment line 2\nB & C & D {\n  a: a\n}");
}
