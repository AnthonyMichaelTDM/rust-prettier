#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_comments_1_js_format_1_e22b89e1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(7)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("// hi l<|>ol\nfunction ehllooo () {\n  const hi = \"hi\"\n}")?;
    assert_eq!(
        formatted,
        "// hi l<|>ol\nfunction ehllooo() {\n  const hi = \"hi\";\n}"
    );
    Ok(())
}
#[test]
fn test_comments_2_js_format_1_ef247da8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(0)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<|>\n// howdy\n// hi lol\nconst y = 5")?;
    assert_eq!(formatted, "<|>// howdy\n// hi lol\nconst y = 5;");
    Ok(())
}
#[test]
fn test_comments_3_js_format_1_c3fbcfa9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(1)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/<|>/ howdy\n// hi lol\nconst y = 5")?;
    assert_eq!(formatted, "/<|>/ howdy\n// hi lol\nconst y = 5;");
    Ok(())
}
#[test]
fn test_comments_4_js_format_1_d8edf93e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(44)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// howdy\n// hi lol\nconst y = 5\n//  traling! <|>")?;
    assert_eq!(
        formatted,
        "// howdy\n// hi lol\nconst y = 5;\n//  traling!<|>"
    );
    Ok(())
}
#[test]
fn test_cursor_0_js_format_1_9e948623() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(27)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("(function() {return        <|> 15})()")?;
    assert_eq!(formatted, "(function () {\n  return <|>15;\n})();");
    Ok(())
}
#[test]
fn test_cursor_1_js_format_1_6f1058bc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(26)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("(function(){return        <|>15})()")?;
    assert_eq!(formatted, "(function () {\n  return <|>15;\n})();");
    Ok(())
}
#[test]
fn test_cursor_2_js_format_1_3169d8f8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(6)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\nfoo  <|>  (bar);")?;
    assert_eq!(formatted, "foo<|>(bar);");
    Ok(())
}
#[test]
fn test_cursor_3_js_format_1_cefcebce() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(4)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\n\n  <|>\n\n  const y = 5")?;
    assert_eq!(formatted, "<|>const y = 5;");
    Ok(())
}
#[test]
fn test_cursor_4_js_format_1_db95304a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(19)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\n\n  const y = 5\n\n  <|>\n\n  const z = 9")?;
    assert_eq!(formatted, "const y = 5;\n\n<|>const z = 9;");
    Ok(())
}
#[test]
fn test_cursor_5_js_format_1_50b89fc0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(13)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("const    /* h<|>i */    y = 5")?;
    assert_eq!(formatted, "const /* h<|>i */ y = 5;");
    Ok(())
}
#[test]
fn test_cursor_6_js_format_1_7a9b2791() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(20)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("const      y    /* h<|>i */   = 5")?;
    assert_eq!(formatted, "const y /* h<|>i */ = 5;");
    Ok(())
}
#[test]
fn test_cursor_7_js_format_1_c864d521() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(12)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("const y = 5\n<|>\n\nconst z = 9")?;
    assert_eq!(formatted, "const y = 5;\n<|>\nconst z = 9;");
    Ok(())
}
#[test]
fn test_cursor_8_js_format_1_8e8d4604() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(6)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("  func<|>tion banana(){")?;
    assert_eq!(formatted, "func<|>tion banana() {}");
    Ok(())
}
#[test]
fn test_cursor_9_js_format_1_4057d6a3() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(26)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("     thisWillBeFormatted  <|>  (2  ,3,   ")?;
    assert_eq!(formatted, "thisWillBeFormatted<|>(2, 3);");
    Ok(())
}
#[test]
fn test_cursor_10_js_format_1_700a4ba9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(16)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("const y = 5\n\n\n\n\n<|>\n ")?;
    assert_eq!(formatted, "const y = 5;\n<|");
    Ok(())
}
#[test]
fn test_cursor_emoji_js_format_1_9375a5da() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(9)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\"😀😀😀😀<|>\"")?;
    assert_eq!(formatted, "\"😀😀😀😀<|>\";");
    Ok(())
}
#[test]
fn test_file_start_with_comment_1_js_format_1_707cc1b7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(5)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// hi<|> lol\nhaha()")?;
    assert_eq!(formatted, "// hi<|> lol\nhaha();");
    Ok(())
}
#[test]
fn test_file_start_with_comment_2_js_format_1_3d4e0f7f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(16)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// hi lol\nhaha()<|>")?;
    assert_eq!(formatted, "// hi lol\nhaha()<|>;");
    Ok(())
}
#[test]
fn test_file_start_with_comment_3_js_format_1_11d8b01b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(127)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// All messages are represented in JSON.\n// So, the prettier.py controls a subprocess which spawns \"node {this_file}\".\nimport {<|>  } from \"fs\"") ? ;
    assert_eq ! (formatted , "// All messages are represented in JSON.\n// So, the prettier.py controls a subprocess which spawns \"node {this_file}\".\nimport {<|>} from \"fs\";");
    Ok(())
}
#[test]
fn test_range_0_js_format_1_616a96c8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(56)
        .parser("js")
        .print_width(80)
        .range_end(72)
        .range_start(31)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 | thisWontBeFormatted  ( 1  ,3)\n  2 |\n> 3 |     thisWillBeFormatted  <|>  (2  ,3,   )\n    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n> 4 |\n    | ^\n> 5 |     thisWontBeFormatted  (2, 90  ,)\n    | ^\n  6 |    ") ? ;
    assert_eq ! (formatted , "thisWontBeFormatted  ( 1  ,3)\n\n    thisWillBeFormatted<|>(2, 3);\n\n    thisWontBeFormatted  (2, 90  ,)\n   ");
    Ok(())
}
#[test]
fn test_range_1_js_format_1_355fdf96() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(64)
        .parser("js")
        .print_width(80)
        .range_end(72)
        .range_start(31)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 | thisWontBeFormatted  ( 1  ,3)\n  2 |\n> 3 |     thisWillBeFormatted    (2  ,3<|>,   )\n    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n> 4 |\n    | ^\n> 5 |     thisWontBeFormatted  (2, 90  ,)\n    | ^\n  6 |    ") ? ;
    assert_eq ! (formatted , "thisWontBeFormatted  ( 1  ,3)\n\n    thisWillBeFormatted(2, 3<|>);\n\n    thisWontBeFormatted  (2, 90  ,)\n   ");
    Ok(())
}
#[test]
fn test_range_2_js_format_1_bdabf752() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(67)
        .parser("js")
        .print_width(80)
        .range_end(72)
        .range_start(31)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 | thisWontBeFormatted  ( 1  ,3)\n  2 |\n> 3 |     thisWillBeFormatted    (2  ,3,  <|> )\n    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n> 4 |\n    | ^\n> 5 |     thisWontBeFormatted  (2, 90  ,)\n    | ^\n  6 |    ") ? ;
    assert_eq ! (formatted , "thisWontBeFormatted  ( 1  ,3)\n\n    thisWillBeFormatted(2, 3<|>);\n\n    thisWontBeFormatted  (2, 90  ,)\n   ");
    Ok(())
}
#[test]
fn test_range_3_js_format_1_dd670a12() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(20)
        .parser("js")
        .print_width(80)
        .range_end(72)
        .range_start(30)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 | thisWontBeFormatted <|> ( 1  ,3)\n> 2 |\n    | ^\n> 3 |     thisWillBeFormatted    (2  ,3,   )\n    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n> 4 |\n    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n> 5 |     thisWontBeFormatted  (2, 90  ,)\n    | ^\n  6 |    ") ? ;
    assert_eq ! (formatted , "thisWontBeFormatted <|> ( 1  ,3)\n\n    thisWillBeFormatted(2, 3);\n\n    thisWontBeFormatted  (2, 90  ,)\n   ");
    Ok(())
}
#[test]
fn test_range_4_js_format_1_b6c8447a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(101)
        .parser("js")
        .print_width(80)
        .range_end(75)
        .range_start(31)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 | thisWontBeFormatted  ( 1  ,3)\n  2 |\n> 3 |     thisWillBeFormatted    (2  ,3,   )\n    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n> 4 |\n    | ^\n> 5 |     thisWontBeFormatted  (2, 9<|>0  ,)\n    | ^^^^\n  6 |    ") ? ;
    assert_eq ! (formatted , "thisWontBeFormatted  ( 1  ,3)\n\n    thisWillBeFormatted(2, 3);\n\n    thisWontBeFormatted  (2, 9<|>0  ,)\n   ");
    Ok(())
}
#[test]
fn test_range_5_js_format_1_c2f12f3d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(23)
        .parser("js")
        .print_width(80)
        .range_end(23)
        .range_start(14)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("> 1 | const myVar = aFunction<|>\n    |               ^^^^^^^^^\n  2 ")?;
    assert_eq!(formatted, "const myVar = aFunction<|>;");
    Ok(())
}
#[test]
fn test_range_6_js_format_1_39375b8c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(24)
        .parser("js")
        .print_width(80)
        .range_end(24)
        .range_start(14)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("> 1 | const myVar = aFunction;<|>\n    |               ^^^^^^^^^^\n  2 ")?;
    assert_eq!(formatted, "const myVar = aFunction;<|>");
    Ok(())
}
#[test]
fn test_range_7_js_format_1_91bb87be() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(23)
        .parser("js")
        .print_width(80)
        .range_end(24)
        .range_start(14)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("> 1 | const myVar = aFunction<|>;\n    |               ^^^^^^^^^^^^^\n  2 ")?;
    assert_eq!(formatted, "const myVar = aFunction<|>;");
    Ok(())
}
#[test]
fn test_range_8_js_format_1_f3e9fe41() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(69)
        .parser("js")
        .print_width(80)
        .range_end(72)
        .range_start(30)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 | thisWontBeFormatted  ( 1  ,3)\n> 2 |\n    | ^\n> 3 |     thisWillBeFormatted    (2  ,3,   )<|>\n    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n> 4 |\n    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n> 5 |     thisWontBeFormatted  (2, 90  ,)\n    | ^\n  6 ") ? ;
    assert_eq ! (formatted , "thisWontBeFormatted  ( 1  ,3)\n\n    thisWillBeFormatted(2, 3)<|>;\n\n    thisWontBeFormatted  (2, 90  ,)");
    Ok(())
}
