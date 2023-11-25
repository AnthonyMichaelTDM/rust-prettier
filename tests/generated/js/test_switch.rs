#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_comments_js_format_1_8180a21a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("switch (true) {\n  case true:\n  // Good luck getting here\n\n  case false:\n}\n\nswitch (true) {\n  case true:\n\n  // Good luck getting here\n  case false:\n}\n\nswitch(x) {\n  case x: {\n  }\n\n  // other\n\n  case y: {\n  }\n}\n\nswitch(x) {\n  default: // comment\n    break;\n}\n\nswitch(x) {\n  default: // comment\n    {break;}\n}\n\nswitch(x) {\n  default: {// comment\n    break;}\n}\n\nswitch(x) {\n  default: /* comment */\n    break;\n}\n\nswitch(x) {\n  default: /* comment */\n    {break;}\n}\n\nswitch(x) {\n  default: {/* comment */\n    break;}\n}\n\nswitch(x) {\n  default: /* comment */ {\n    break;}\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "switch (true) {\n  case true:\n  // Good luck getting here\n\n  case false:\n}\n\nswitch (true) {\n  case true:\n\n  // Good luck getting here\n  case false:\n}\n\nswitch (x) {\n  case x: {\n  }\n\n  // other\n\n  case y: {\n  }\n}\n\nswitch (x) {\n  default: // comment\n    break;\n}\n\nswitch (x) {\n  default: {\n    // comment\n    break;\n  }\n}\n\nswitch (x) {\n  default: {\n    // comment\n    break;\n  }\n}\n\nswitch (x) {\n  default: /* comment */\n    break;\n}\n\nswitch (x) {\n  default: /* comment */ {\n    break;\n  }\n}\n\nswitch (x) {\n  default: {\n    /* comment */\n    break;\n  }\n}\n\nswitch (x) {\n  default: /* comment */ {\n    break;\n  }\n}");
}
#[test]
fn test_comments_2_js_format_1_56c750f3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("switch(1){default: // comment1\n}\n\nswitch(2){default: // comment2\n//comment2a\n}\n\nswitch(3){default: // comment3\nbreak;// comment3a\n}\n\nswitch(4){default: // comment4\n// comment4a\nbreak;// comment4b\n}\n\nswitch(5){default: // comment5\n// comment5a\nfoo();bar();//comment5b\nbreak;// comment5c\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "switch (1) {\n  default: // comment1\n}\n\nswitch (2) {\n  default: // comment2\n  //comment2a\n}\n\nswitch (3) {\n  default: // comment3\n    break; // comment3a\n}\n\nswitch (4) {\n  default: // comment4\n    // comment4a\n    break; // comment4b\n}\n\nswitch (5) {\n  default: // comment5\n    // comment5a\n    foo();\n    bar(); //comment5b\n    break; // comment5c\n}");
}
#[test]
fn test_empty_lines_js_format_1_4bcc0658() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("switch (foo) {\n  case \"bar\":\n    doSomething();\n\n  case \"baz\":\n    doOtherThing();\n}\n\nswitch (foo) {\n\n  case \"bar\":\n    doSomething();\n\n  case \"baz\":\n    doOtherThing();\n}\n\nswitch (foo) {\n  case \"bar\":\n    doSomething();\n\n  case \"baz\":\n    doOtherThing();\n\n}\n\nswitch (foo) {\n  case \"bar\":\n    doSomething();\n\n\n\n  case \"baz\":\n    doOtherThing();\n}\n\nswitch (x) {\n  case y:\n    call();\n\n    break;\n\n  case z:\n    call();\n\n    break;\n}\n\nswitch (a) {\n  case b:\n    if (1) {};\n    c;\n}\n\nswitch (a) {\n  case x:\n  case y:\n    call();\n\n  case z:\n    call();\n}\n\nswitch (a) {\n  case x: case y:\n    call();\n\n  case z:\n    call();\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "switch (foo) {\n  case \"bar\":\n    doSomething();\n\n  case \"baz\":\n    doOtherThing();\n}\n\nswitch (foo) {\n  case \"bar\":\n    doSomething();\n\n  case \"baz\":\n    doOtherThing();\n}\n\nswitch (foo) {\n  case \"bar\":\n    doSomething();\n\n  case \"baz\":\n    doOtherThing();\n}\n\nswitch (foo) {\n  case \"bar\":\n    doSomething();\n\n  case \"baz\":\n    doOtherThing();\n}\n\nswitch (x) {\n  case y:\n    call();\n\n    break;\n\n  case z:\n    call();\n\n    break;\n}\n\nswitch (a) {\n  case b:\n    if (1) {\n    }\n    c;\n}\n\nswitch (a) {\n  case x:\n  case y:\n    call();\n\n  case z:\n    call();\n}\n\nswitch (a) {\n  case x:\n  case y:\n    call();\n\n  case z:\n    call();\n}");
}
#[test]
fn test_empty_statement_js_format_1_d05c9fa8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("switch (error.code) {\n\tcase ConfigurationEditingErrorCode.ERROR_INVALID_CONFIGURATION: {\n\t\tnls.localize('errorInvalidConfiguration', \"Unable to write into settings. Correct errors/warnings in the file and try again.\");\n\t};\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "switch (error.code) {\n  case ConfigurationEditingErrorCode.ERROR_INVALID_CONFIGURATION: {\n    nls.localize(\n      \"errorInvalidConfiguration\",\n      \"Unable to write into settings. Correct errors/warnings in the file and try again.\",\n    );\n  }\n}");
}
#[test]
fn test_empty_switch_js_format_1_a3c5267a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("switch (1) { default:; }\nswitch (1) {}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "switch (1) {\n  default:\n}\nswitch (1) {\n}");
}
#[test]
fn test_switch_js_format_1_24ff5f47() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("switch (a) {\n  case 3:\n    alert( '3' );\n    break;\n  case 4:\n    alert( '4' );\n    break;\n  case 5:\n    alert( '5' );\n    break;\n  default:\n    alert( 'default' );\n}\n\nswitch (veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLong) {\n  case 3:\n    alert( '3' );\n    break;\n  default:\n    alert( 'default' );\n}\n\nswitch (veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLong > veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLong) {\n  case 3:\n    alert( '3' );\n    break;\n  default:\n    alert( 'default' );\n}\n\nswitch ($veryLongAndVeryVerboseVariableName && $anotherVeryLongAndVeryVerboseVariableName) {\n}\n\nswitch ($longButSlightlyShorterVariableName && $anotherSlightlyShorterVariableName) {\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "switch (a) {\n  case 3:\n    alert(\"3\");\n    break;\n  case 4:\n    alert(\"4\");\n    break;\n  case 5:\n    alert(\"5\");\n    break;\n  default:\n    alert(\"default\");\n}\n\nswitch (\n  veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLong\n) {\n  case 3:\n    alert(\"3\");\n    break;\n  default:\n    alert(\"default\");\n}\n\nswitch (\n  veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLong >\n  veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLong\n) {\n  case 3:\n    alert(\"3\");\n    break;\n  default:\n    alert(\"default\");\n}\n\nswitch (\n  $veryLongAndVeryVerboseVariableName &&\n  $anotherVeryLongAndVeryVerboseVariableName\n) {\n}\n\nswitch (\n  $longButSlightlyShorterVariableName &&\n  $anotherSlightlyShorterVariableName\n) {\n}");
}
