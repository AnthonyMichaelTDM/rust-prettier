#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_comments_1_js_format_1_e22b89e1() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted =
        pretty_printer.format("// hi l<|>ol\nfunction ehllooo () {\n  const hi = \"hi\"\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// hi l<|>ol\nfunction ehllooo() {\n  const hi = \"hi\";\n}"
    );
}
#[test]
fn test_comments_2_js_format_1_ef247da8() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("<|>\n// howdy\n// hi lol\nconst y = 5");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<|>// howdy\n// hi lol\nconst y = 5;");
}
#[test]
fn test_comments_3_js_format_1_c3fbcfa9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("/<|>/ howdy\n// hi lol\nconst y = 5");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "/<|>/ howdy\n// hi lol\nconst y = 5;");
}
#[test]
fn test_comments_4_js_format_1_d8edf93e() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("// howdy\n// hi lol\nconst y = 5\n//  traling! <|>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// howdy\n// hi lol\nconst y = 5;\n//  traling!<|>"
    );
}
#[test]
fn test_cursor_0_js_format_1_9e948623() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("(function() {return        <|> 15})()");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "(function () {\n  return <|>15;\n})();");
}
#[test]
fn test_cursor_1_js_format_1_6f1058bc() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("(function(){return        <|>15})()");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "(function () {\n  return <|>15;\n})();");
}
#[test]
fn test_cursor_2_js_format_1_3169d8f8() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("\nfoo  <|>  (bar);");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo<|>(bar);");
}
#[test]
fn test_cursor_3_js_format_1_cefcebce() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("\n\n  <|>\n\n  const y = 5");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<|>const y = 5;");
}
#[test]
fn test_cursor_4_js_format_1_db95304a() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("\n\n  const y = 5\n\n  <|>\n\n  const z = 9");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "const y = 5;\n\n<|>const z = 9;");
}
#[test]
fn test_cursor_5_js_format_1_50b89fc0() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("const    /* h<|>i */    y = 5");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "const /* h<|>i */ y = 5;");
}
#[test]
fn test_cursor_6_js_format_1_7a9b2791() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("const      y    /* h<|>i */   = 5");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "const y /* h<|>i */ = 5;");
}
#[test]
fn test_cursor_7_js_format_1_c864d521() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("const y = 5\n<|>\n\nconst z = 9");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "const y = 5;\n<|>\nconst z = 9;");
}
#[test]
fn test_cursor_8_js_format_1_8e8d4604() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("  func<|>tion banana(){");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "func<|>tion banana() {}");
}
#[test]
fn test_cursor_9_js_format_1_4057d6a3() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("     thisWillBeFormatted  <|>  (2  ,3,   ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "thisWillBeFormatted<|>(2, 3);");
}
#[test]
fn test_cursor_10_js_format_1_700a4ba9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("const y = 5\n\n\n\n\n<|>\n ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "const y = 5;\n<|");
}
#[test]
fn test_cursor_emoji_js_format_1_9375a5da() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("\"😀😀😀😀<|>\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\"😀😀😀😀<|>\";");
}
#[test]
fn test_file_start_with_comment_1_js_format_1_707cc1b7() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("// hi<|> lol\nhaha()");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "// hi<|> lol\nhaha();");
}
#[test]
fn test_file_start_with_comment_2_js_format_1_3d4e0f7f() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("// hi lol\nhaha()<|>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "// hi lol\nhaha()<|>;");
}
#[test]
fn test_file_start_with_comment_3_js_format_1_11d8b01b() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// All messages are represented in JSON.\n// So, the prettier.py controls a subprocess which spawns \"node {this_file}\".\nimport {<|>  } from \"fs\"") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// All messages are represented in JSON.\n// So, the prettier.py controls a subprocess which spawns \"node {this_file}\".\nimport {<|>} from \"fs\";");
}
#[test]
fn test_range_0_js_format_1_616a96c8() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("  1 | thisWontBeFormatted  ( 1  ,3)\n  2 |\n> 3 |     thisWillBeFormatted  <|>  (2  ,3,   )\n    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n> 4 |\n    | ^\n> 5 |     thisWontBeFormatted  (2, 90  ,)\n    | ^\n  6 |    ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "thisWontBeFormatted  ( 1  ,3)\n\n    thisWillBeFormatted<|>(2, 3);\n\n    thisWontBeFormatted  (2, 90  ,)\n   ");
}
#[test]
fn test_range_1_js_format_1_355fdf96() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("  1 | thisWontBeFormatted  ( 1  ,3)\n  2 |\n> 3 |     thisWillBeFormatted    (2  ,3<|>,   )\n    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n> 4 |\n    | ^\n> 5 |     thisWontBeFormatted  (2, 90  ,)\n    | ^\n  6 |    ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "thisWontBeFormatted  ( 1  ,3)\n\n    thisWillBeFormatted(2, 3<|>);\n\n    thisWontBeFormatted  (2, 90  ,)\n   ");
}
#[test]
fn test_range_2_js_format_1_bdabf752() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("  1 | thisWontBeFormatted  ( 1  ,3)\n  2 |\n> 3 |     thisWillBeFormatted    (2  ,3,  <|> )\n    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n> 4 |\n    | ^\n> 5 |     thisWontBeFormatted  (2, 90  ,)\n    | ^\n  6 |    ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "thisWontBeFormatted  ( 1  ,3)\n\n    thisWillBeFormatted(2, 3<|>);\n\n    thisWontBeFormatted  (2, 90  ,)\n   ");
}
#[test]
fn test_range_3_js_format_1_dd670a12() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("  1 | thisWontBeFormatted <|> ( 1  ,3)\n> 2 |\n    | ^\n> 3 |     thisWillBeFormatted    (2  ,3,   )\n    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n> 4 |\n    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n> 5 |     thisWontBeFormatted  (2, 90  ,)\n    | ^\n  6 |    ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "thisWontBeFormatted <|> ( 1  ,3)\n\n    thisWillBeFormatted(2, 3);\n\n    thisWontBeFormatted  (2, 90  ,)\n   ");
}
#[test]
fn test_range_4_js_format_1_b6c8447a() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("  1 | thisWontBeFormatted  ( 1  ,3)\n  2 |\n> 3 |     thisWillBeFormatted    (2  ,3,   )\n    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n> 4 |\n    | ^\n> 5 |     thisWontBeFormatted  (2, 9<|>0  ,)\n    | ^^^^\n  6 |    ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "thisWontBeFormatted  ( 1  ,3)\n\n    thisWillBeFormatted(2, 3);\n\n    thisWontBeFormatted  (2, 9<|>0  ,)\n   ");
}
#[test]
fn test_range_5_js_format_1_c2f12f3d() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer
        .format("> 1 | const myVar = aFunction<|>\n    |               ^^^^^^^^^\n  2 ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "const myVar = aFunction<|>;");
}
#[test]
fn test_range_6_js_format_1_39375b8c() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer
        .format("> 1 | const myVar = aFunction;<|>\n    |               ^^^^^^^^^^\n  2 ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "const myVar = aFunction;<|>");
}
#[test]
fn test_range_7_js_format_1_91bb87be() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer
        .format("> 1 | const myVar = aFunction<|>;\n    |               ^^^^^^^^^^^^^\n  2 ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "const myVar = aFunction<|>;");
}
#[test]
fn test_range_8_js_format_1_f3e9fe41() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("  1 | thisWontBeFormatted  ( 1  ,3)\n> 2 |\n    | ^\n> 3 |     thisWillBeFormatted    (2  ,3,   )<|>\n    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n> 4 |\n    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n> 5 |     thisWontBeFormatted  (2, 90  ,)\n    | ^\n  6 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "thisWontBeFormatted  ( 1  ,3)\n\n    thisWillBeFormatted(2, 3)<|>;\n\n    thisWontBeFormatted  (2, 90  ,)");
}
