use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_brackets_js_bracket_spacingfalse_format_1_47430aa8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import {\n  runTaskForChanged,\n  description,\n  someOtherLabel,\n  thatMakes,\n  itGo,\n  multiLine,\n  andMore,\n  soWeCanGetItTo80Columns\n} from '.';\nimport {fitsIn, oneLine} from '.';") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import {\n  runTaskForChanged,\n  description,\n  someOtherLabel,\n  thatMakes,\n  itGo,\n  multiLine,\n  andMore,\n  soWeCanGetItTo80Columns,\n} from \".\";\nimport {fitsIn, oneLine} from \".\";");
}
#[test]
fn test_brackets_js_format_1_47430aa8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import {\n  runTaskForChanged,\n  description,\n  someOtherLabel,\n  thatMakes,\n  itGo,\n  multiLine,\n  andMore,\n  soWeCanGetItTo80Columns\n} from '.';\nimport {fitsIn, oneLine} from '.';") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import {\n  runTaskForChanged,\n  description,\n  someOtherLabel,\n  thatMakes,\n  itGo,\n  multiLine,\n  andMore,\n  soWeCanGetItTo80Columns,\n} from \".\";\nimport { fitsIn, oneLine } from \".\";");
}
#[test]
fn test_comments_js_bracket_spacingfalse_format_1_a55289af() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import { a //comment1\n//comment2\n//comment3\nas b} from \"\";\n\nimport {\n  a as //comment1\n  //comment2\n  //comment3\n  b1\n} from \"\";\n\nimport {\n  a as //comment2 //comment1\n  //comment3\n  b2\n} from \"\";\n\nimport {\n  a as //comment3 //comment2 //comment1\n  b3\n} from \"\";\n\nimport {\n  // comment 1\n  FN1, // comment 2\n  /* comment 3 */ FN2,\n  // FN3,\n  FN4 /* comment 4 */\n  // FN4,\n  // FN5\n} from \"./module\";\n\nimport {\n  ExecutionResult,\n  DocumentNode,\n  /* tslint:disable */\n  SelectionSetNode,\n  /* tslint:enable */\n} from 'graphql';\n\nimport x, {\n  // comment\n  y\n} from 'z';") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import {\n  //comment1\n  //comment2\n  //comment3\n  a as b,\n} from \"\";\n\nimport {\n  //comment1\n  //comment2\n  //comment3\n  a as b1,\n} from \"\";\n\nimport {\n  //comment2 //comment1\n  //comment3\n  a as b2,\n} from \"\";\n\nimport {\n  //comment3 //comment2 //comment1\n  a as b3,\n} from \"\";\n\nimport {\n  // comment 1\n  FN1, // comment 2\n  /* comment 3 */ FN2,\n  // FN3,\n  FN4 /* comment 4 */,\n  // FN4,\n  // FN5\n} from \"./module\";\n\nimport {\n  ExecutionResult,\n  DocumentNode,\n  /* tslint:disable */\n  SelectionSetNode,\n  /* tslint:enable */\n} from \"graphql\";\n\nimport x, {\n  // comment\n  y,\n} from \"z\";");
}
#[test]
fn test_comments_js_format_1_a55289af() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import { a //comment1\n//comment2\n//comment3\nas b} from \"\";\n\nimport {\n  a as //comment1\n  //comment2\n  //comment3\n  b1\n} from \"\";\n\nimport {\n  a as //comment2 //comment1\n  //comment3\n  b2\n} from \"\";\n\nimport {\n  a as //comment3 //comment2 //comment1\n  b3\n} from \"\";\n\nimport {\n  // comment 1\n  FN1, // comment 2\n  /* comment 3 */ FN2,\n  // FN3,\n  FN4 /* comment 4 */\n  // FN4,\n  // FN5\n} from \"./module\";\n\nimport {\n  ExecutionResult,\n  DocumentNode,\n  /* tslint:disable */\n  SelectionSetNode,\n  /* tslint:enable */\n} from 'graphql';\n\nimport x, {\n  // comment\n  y\n} from 'z';") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import {\n  //comment1\n  //comment2\n  //comment3\n  a as b,\n} from \"\";\n\nimport {\n  //comment1\n  //comment2\n  //comment3\n  a as b1,\n} from \"\";\n\nimport {\n  //comment2 //comment1\n  //comment3\n  a as b2,\n} from \"\";\n\nimport {\n  //comment3 //comment2 //comment1\n  a as b3,\n} from \"\";\n\nimport {\n  // comment 1\n  FN1, // comment 2\n  /* comment 3 */ FN2,\n  // FN3,\n  FN4 /* comment 4 */,\n  // FN4,\n  // FN5\n} from \"./module\";\n\nimport {\n  ExecutionResult,\n  DocumentNode,\n  /* tslint:disable */\n  SelectionSetNode,\n  /* tslint:enable */\n} from \"graphql\";\n\nimport x, {\n  // comment\n  y,\n} from \"z\";");
}
#[test]
fn test_empty_import_js_bracket_spacingfalse_format_1_88610474() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("import { } from '@types/googlemaps';");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "import {} from \"@types/googlemaps\";");
}
#[test]
fn test_empty_import_js_format_1_88610474() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("import { } from '@types/googlemaps';");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "import {} from \"@types/googlemaps\";");
}
#[test]
fn test_inline_js_bracket_spacingfalse_format_1_e33f2b77() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import somethingSuperLongsomethingSuperLong from 'somethingSuperLongsomethingSuperLongsomethingSuperLong'\nimport {somethingSuperLongsomethingSuperLong1} from 'somethingSuperLongsomethingSuperLongsomethingSuperLong'\nimport a, {somethingSuperLongsomethingSuperLong2} from 'somethingSuperLongsomethingSuperLongsomethingSuperLong'\nimport {a2, somethingSuperLongsomethingSuperLong3} from 'somethingSuperLongsomethingSuperLongsomethingSuperLong'") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import somethingSuperLongsomethingSuperLong from \"somethingSuperLongsomethingSuperLongsomethingSuperLong\";\nimport {somethingSuperLongsomethingSuperLong1} from \"somethingSuperLongsomethingSuperLongsomethingSuperLong\";\nimport a, {\n  somethingSuperLongsomethingSuperLong2,\n} from \"somethingSuperLongsomethingSuperLongsomethingSuperLong\";\nimport {\n  a2,\n  somethingSuperLongsomethingSuperLong3,\n} from \"somethingSuperLongsomethingSuperLongsomethingSuperLong\";");
}
#[test]
fn test_inline_js_format_1_e33f2b77() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import somethingSuperLongsomethingSuperLong from 'somethingSuperLongsomethingSuperLongsomethingSuperLong'\nimport {somethingSuperLongsomethingSuperLong1} from 'somethingSuperLongsomethingSuperLongsomethingSuperLong'\nimport a, {somethingSuperLongsomethingSuperLong2} from 'somethingSuperLongsomethingSuperLongsomethingSuperLong'\nimport {a2, somethingSuperLongsomethingSuperLong3} from 'somethingSuperLongsomethingSuperLongsomethingSuperLong'") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import somethingSuperLongsomethingSuperLong from \"somethingSuperLongsomethingSuperLongsomethingSuperLong\";\nimport { somethingSuperLongsomethingSuperLong1 } from \"somethingSuperLongsomethingSuperLongsomethingSuperLong\";\nimport a, {\n  somethingSuperLongsomethingSuperLong2,\n} from \"somethingSuperLongsomethingSuperLongsomethingSuperLong\";\nimport {\n  a2,\n  somethingSuperLongsomethingSuperLong3,\n} from \"somethingSuperLongsomethingSuperLongsomethingSuperLong\";");
}
#[test]
fn test_long_line_js_bracket_spacingfalse_format_1_041136ae() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import someCoolUtilWithARatherLongName from '../../../../utils/someCoolUtilWithARatherLongName';") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import someCoolUtilWithARatherLongName from \"../../../../utils/someCoolUtilWithARatherLongName\";");
}
#[test]
fn test_long_line_js_format_1_041136ae() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import someCoolUtilWithARatherLongName from '../../../../utils/someCoolUtilWithARatherLongName';") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import someCoolUtilWithARatherLongName from \"../../../../utils/someCoolUtilWithARatherLongName\";");
}
#[test]
fn test_multiple_standalones_js_bracket_spacingfalse_format_1_9ccab025() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("import a, * as b from 'a';");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "import a, * as b from \"a\";");
}
#[test]
fn test_multiple_standalones_js_format_1_9ccab025() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("import a, * as b from 'a';");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "import a, * as b from \"a\";");
}
#[test]
fn test_same_local_and_imported_js_bracket_spacingfalse_format_1_1c047fdd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "import {a} from 'a';\nimport {b as b} from 'b';\nimport {c as /* comment */c} from 'c';",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import {a} from \"a\";\nimport {b as b} from \"b\";\nimport {c as /* comment */ c} from \"c\";");
}
#[test]
fn test_same_local_and_imported_js_format_1_1c047fdd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "import {a} from 'a';\nimport {b as b} from 'b';\nimport {c as /* comment */c} from 'c';",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import { a } from \"a\";\nimport { b as b } from \"b\";\nimport { c as /* comment */ c } from \"c\";");
}
