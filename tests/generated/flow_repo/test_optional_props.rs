#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_f4ddb081() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var x: { } = { foo: 0 };\nvar y: { foo?: string } = x; // OK in TypeScript, not OK in Flow\n\nvar z: string = y.foo || \"\";\n\nvar o = { };\ny = o; // OK; we know that narrowing could not have happened\no.foo = 0; // future widening is constrained\n\nfunction bar(config: { foo?: number }) {}\nbar({});\nbar({foo: \"\"});") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var x: {} = { foo: 0 };\nvar y: { foo?: string } = x; // OK in TypeScript, not OK in Flow\n\nvar z: string = y.foo || \"\";\n\nvar o = {};\ny = o; // OK; we know that narrowing could not have happened\no.foo = 0; // future widening is constrained\n\nfunction bar(config: { foo?: number }) {}\nbar({});\nbar({ foo: \"\" });");
}
#[test]
fn test_test_2_js_format_1_a76541d0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var a: { foo?: string } = {};\na.foo = undefined; // This is not an error\na.foo = null; // But this is an error\n\nvar b: { foo?: ?string } = {};\nb.foo = undefined; // This is fine\nb.foo = null; // Also fine\n\nvar c: { foo?: string } = { foo: undefined }; // This is not an error\nvar d: { foo?: string } = { foo: null }; // But this is an error\n\nvar e: { foo?: ?string } = { foo: undefined }; // This is fine\nvar f: { foo?: ?string } = { foo: null }; // Also fine") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var a: { foo?: string } = {};\na.foo = undefined; // This is not an error\na.foo = null; // But this is an error\n\nvar b: { foo?: ?string } = {};\nb.foo = undefined; // This is fine\nb.foo = null; // Also fine\n\nvar c: { foo?: string } = { foo: undefined }; // This is not an error\nvar d: { foo?: string } = { foo: null }; // But this is an error\n\nvar e: { foo?: ?string } = { foo: undefined }; // This is fine\nvar f: { foo?: ?string } = { foo: null }; // Also fine");
}
#[test]
fn test_test_3_js_format_1_98e7ee2f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\n/*\n   object literals are sealed. this is simply a heuristic\n   decision: most of the time, the rule gives the 'right'\n   errors.\n\n   an exception is when a literal is used as an initializer\n   for an lvalue whose type specifies optional properties\n   missing from the literal, as below.\n\n   the problem becomes visible when a property assignment\n   is then used to (legitimately) extend the object with an\n   optional property - the variable's specific (path-\n   dependent) type has become that of the literal which.\n   without adjustment, will reject the property addition.\n\n   the solution in cases where a sealed object type (as from\n   an object literal) flows to an object type with optional\n   properties, is to have the sealed type acquire the optional\n   properties.\n */\n\n// x has optional property b.\n// (note that the initializer here does not play into\n// the problem, it's just a placeholder. initializers\n// do not narrow the types of annotated variables as do\n// subsequent assignments.)\n//\nvar x: { a: number, b?: number } = { a: 0 };\n\n// now assign an object literal lacking property b.\n// the literal's type is sealed and has only a at creation.\n// but it then flows, specific ~> general, to x's annotation\n// type. at that point, it acquires b as an optional property.\n//\nx = { a: 0 };\n\n// ...which allows this assignment to take place.\nx.b = 1;\n\n// T7810506\nclass A {\n    x: { a: number, b?: string };\n    foo() {\n        // Something similar should happen here, but doesn't: the problem is\n        // made explicit by adding generics (see test3_failure.js introduced by\n        // D2747512). There is a race between writing b on the object literal\n        // type and adding b as an optional property to it, since in general we\n        // cannot guarantee that the flow from the object literal to the\n        // annotation will be processed before the flow involving the\n        // access. Here we lose the race and get an error on the write.\n        this.x = { a: 123 };\n        this.x.b = 'hello';\n    }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\n/*\n   object literals are sealed. this is simply a heuristic\n   decision: most of the time, the rule gives the 'right'\n   errors.\n\n   an exception is when a literal is used as an initializer\n   for an lvalue whose type specifies optional properties\n   missing from the literal, as below.\n\n   the problem becomes visible when a property assignment\n   is then used to (legitimately) extend the object with an\n   optional property - the variable's specific (path-\n   dependent) type has become that of the literal which.\n   without adjustment, will reject the property addition.\n\n   the solution in cases where a sealed object type (as from\n   an object literal) flows to an object type with optional\n   properties, is to have the sealed type acquire the optional\n   properties.\n */\n\n// x has optional property b.\n// (note that the initializer here does not play into\n// the problem, it's just a placeholder. initializers\n// do not narrow the types of annotated variables as do\n// subsequent assignments.)\n//\nvar x: { a: number, b?: number } = { a: 0 };\n\n// now assign an object literal lacking property b.\n// the literal's type is sealed and has only a at creation.\n// but it then flows, specific ~> general, to x's annotation\n// type. at that point, it acquires b as an optional property.\n//\nx = { a: 0 };\n\n// ...which allows this assignment to take place.\nx.b = 1;\n\n// T7810506\nclass A {\n  x: { a: number, b?: string };\n  foo() {\n    // Something similar should happen here, but doesn't: the problem is\n    // made explicit by adding generics (see test3_failure.js introduced by\n    // D2747512). There is a race between writing b on the object literal\n    // type and adding b as an optional property to it, since in general we\n    // cannot guarantee that the flow from the object literal to the\n    // annotation will be processed before the flow involving the\n    // access. Here we lose the race and get an error on the write.\n    this.x = { a: 123 };\n    this.x.b = \"hello\";\n  }\n}");
}
#[test]
fn test_test_3_exact_annot_js_format_1_e143c9c4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* The logic that allows ({}: {p?:T}), described in test3.js, should _not_ also\n   fire for exact annotations. */\n\nconst a: {| a: number |} = { a: 1 };\nconst b: { a: number, b?: number } = a; // error: property \\`b\\` not found\nb.b = 0; // because subsequent writes would widen the exact object\n(a.b: number); // error: property \\`b\\` not found") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* The logic that allows ({}: {p?:T}), described in test3.js, should _not_ also\n   fire for exact annotations. */\n\nconst a: {| a: number |} = { a: 1 };\nconst b: { a: number, b?: number } = a; // error: property \\`b\\` not found\nb.b = 0; // because subsequent writes would widen the exact object\n(a.b: number); // error: property \\`b\\` not found");
}
#[test]
fn test_test_3_failure_js_format_1_09266ae1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// generalization of failure in test3.js\n\nclass A<O: {x: { a: number, b?: string}}> {\n  o: O;\n  foo() {\n    this.o.x = { a: 123 };\n    this.o.x.b = 'hello'; // this is a spurious error (see test3.js for details)\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// generalization of failure in test3.js\n\nclass A<O: { x: { a: number, b?: string } }> {\n  o: O;\n  foo() {\n    this.o.x = { a: 123 };\n    this.o.x.b = \"hello\"; // this is a spurious error (see test3.js for details)\n  }\n}");
}
