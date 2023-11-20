#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_empty_js_format_1_d5511522() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const a = someVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeLong.Expression || [];\nconst b = someVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeLong.Expression || {};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const a =\n  someVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeLong.Expression || [];\nconst b =\n  someVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeLong.Expression || {};");
}
#[test]
fn test_holes_in_args_js_format_1_b7fae455() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("new Test()\n  .test()\n  .test([, 0])\n  .test();");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "new Test().test().test([, 0]).test();");
}
#[test]
fn test_issue_10159_js_format_1_157e187f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{for (const srcPath of [src, \\`\\${src}.js\\`, \\`\\${src}/index\\`, \\`\\${src}/index.js\\`]) {}}\n{for (const srcPath of [123, 123_123_123, 123_123_123_1, 13_123_3123_31_43]) {}}\n{for (const srcPath of [123, 123_123_123, 123_123_123_1, 13_123_3123_31_432]) {}}\n{for (const srcPath of [123, 123_123_123, 123_123_123_1, 13_123_3123_31_4321]) {}}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "{\n  for (const srcPath of [src, \\`\\${src}.js\\`, \\`\\${src}/index\\`, \\`\\${src}/index.js\\`]) {\n  }\n}\n{\n  for (const srcPath of [123, 123_123_123, 123_123_123_1, 13_123_3123_31_43]) {\n  }\n}\n{\n  for (const srcPath of [123, 123_123_123, 123_123_123_1, 13_123_3123_31_432]) {\n  }\n}\n{\n  for (const srcPath of [\n    123, 123_123_123, 123_123_123_1, 13_123_3123_31_4321,\n  ]) {\n  }\n}");
}
#[test]
fn test_last_js_format_1_f32ea45b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[,];\n[,,];\n[,,1,];\n[,,1,1];");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[,];\n[, ,];\n[, , 1];\n[, , 1, 1];");
}
#[test]
fn test_nested_js_format_1_08890127() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[[]];\n[[], []];\n[[], [], []];\n[[], [0], []];\n[[], [0], [0]];\n[[], [0, 1], [0]];\n[[], [0, 1], [0, 1]];\n[[0]];\n[[0], []];\n[[0], [], []];\n[[0], [0], []];\n[[0], [0], [0]];\n[[0], [0, 1], [0]];\n[[0], [0, 1], [0, 1]];\n[[0, 1]];\n[[0, 1], []];\n[[0, 1], [], []];\n[[0, 1], [0], []];\n[[0, 1], [0], [0]];\n[[0, 1], [0, 1], [0]];\n[[0, 1], [0, 1], [0, 1]];\n[[], [1, 2, 3]];\n[[1], [1]];\n[[1, 2], [1, 2, 3]];\n[[1, 0], [1, 0]];\n[{}];\n[{}, {}];\n[{}, {}, {}];\n[{}, { a }];\n[{}, { a, b }];\n[{}, { a, b, c }];\n[{ a }];\n[{ a }, { a }];\n[{ a }, { a }, { a }];\n[{ a }, { a, b }];\n[{ a }, { a, b, c}];\n[{ a, b }];\n[{ a, b }, { a }];\n[{ a, b }, { a }, { a }];\n[{ a, b }, { a, b }];\n[{ a, b }, { a, b, c }];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[[]];\n[[], []];\n[[], [], []];\n[[], [0], []];\n[[], [0], [0]];\n[[], [0, 1], [0]];\n[[], [0, 1], [0, 1]];\n[[0]];\n[[0], []];\n[[0], [], []];\n[[0], [0], []];\n[[0], [0], [0]];\n[[0], [0, 1], [0]];\n[[0], [0, 1], [0, 1]];\n[[0, 1]];\n[[0, 1], []];\n[[0, 1], [], []];\n[[0, 1], [0], []];\n[[0, 1], [0], [0]];\n[[0, 1], [0, 1], [0]];\n[\n  [0, 1],\n  [0, 1],\n  [0, 1],\n];\n[[], [1, 2, 3]];\n[[1], [1]];\n[\n  [1, 2],\n  [1, 2, 3],\n];\n[\n  [1, 0],\n  [1, 0],\n];\n[{}];\n[{}, {}];\n[{}, {}, {}];\n[{}, { a }];\n[{}, { a, b }];\n[{}, { a, b, c }];\n[{ a }];\n[{ a }, { a }];\n[{ a }, { a }, { a }];\n[{ a }, { a, b }];\n[{ a }, { a, b, c }];\n[{ a, b }];\n[{ a, b }, { a }];\n[{ a, b }, { a }, { a }];\n[\n  { a, b },\n  { a, b },\n];\n[\n  { a, b },\n  { a, b, c },\n];");
}
#[test]
fn test_numbers_in_args_js_format_1_386e2e93() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("expect(bifornCringerMoshedPerplexSawder.getArrayOfNumbers()).toEqual(\n  [1, 2, 3, 4, 5]\n);\n\nexpect(bifornCringerMoshedPerplexSawder.getLongArrayOfNumbers()).toEqual(\n  [\n\t\t66,57,45,47,33,53,82,81,76,78,10,78,15,98,24,29,32,27,28,76,41,65,84,35,\n\t\t97,90,75,24,88,45,23,75,63,86,24,39,9,51,33,40,58,17,49,86,63,59,97,91,\n\t\t98,99,5,69,51,44,34,69,17,91,27,83,26,34,93,29,66,88,49,33,49,73,9,81,4,\n\t\t36,5,14,43,31,86,27,39,75,98,99,55,19,39,21,85,86,46,82,11,44,48,77,35,\n\t\t48,78,97\n\t]\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "expect(bifornCringerMoshedPerplexSawder.getArrayOfNumbers()).toEqual([\n  1, 2, 3, 4, 5,\n]);\n\nexpect(bifornCringerMoshedPerplexSawder.getLongArrayOfNumbers()).toEqual([\n  66, 57, 45, 47, 33, 53, 82, 81, 76, 78, 10, 78, 15, 98, 24, 29, 32, 27, 28,\n  76, 41, 65, 84, 35, 97, 90, 75, 24, 88, 45, 23, 75, 63, 86, 24, 39, 9, 51, 33,\n  40, 58, 17, 49, 86, 63, 59, 97, 91, 98, 99, 5, 69, 51, 44, 34, 69, 17, 91, 27,\n  83, 26, 34, 93, 29, 66, 88, 49, 33, 49, 73, 9, 81, 4, 36, 5, 14, 43, 31, 86,\n  27, 39, 75, 98, 99, 55, 19, 39, 21, 85, 86, 46, 82, 11, 44, 48, 77, 35, 48,\n  78, 97,\n]);");
}
#[test]
fn test_numbers_in_assignment_js_format_1_8d45742c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("bifornCringerMoshedPerplex.bifornCringerMoshedPerplexSawder.arrayOfNumbers = [\n  1,\n  2,\n  3,\n  4,\n  5\n];\n\nbifornCringerMoshedPerplex.bifornCringerMoshedPerplexSawder.arrayOfNumbers2 = [\n  66,57,45,47,33,53,82,81,76,78,10,78,15,98,24,29,32,27,28,76,41,65,84,35,\n  97,90,75,24,88,45,23,75,63,86,24,39,9,51,33,40,58,17,49,86,63,59,97,91,\n  98,99,5,69,51,44,34,69,17,91,27,83,26,34,93,29,66,88,49,33,49,73,9,81,4,\n  36,5,14,43,31,86,27,39,75,98,99,55,19,39,21,85,86,46,82,11,44,48,77,35,\n  48,78,97\n];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "bifornCringerMoshedPerplex.bifornCringerMoshedPerplexSawder.arrayOfNumbers = [\n  1, 2, 3, 4, 5,\n];\n\nbifornCringerMoshedPerplex.bifornCringerMoshedPerplexSawder.arrayOfNumbers2 = [\n  66, 57, 45, 47, 33, 53, 82, 81, 76, 78, 10, 78, 15, 98, 24, 29, 32, 27, 28,\n  76, 41, 65, 84, 35, 97, 90, 75, 24, 88, 45, 23, 75, 63, 86, 24, 39, 9, 51, 33,\n  40, 58, 17, 49, 86, 63, 59, 97, 91, 98, 99, 5, 69, 51, 44, 34, 69, 17, 91, 27,\n  83, 26, 34, 93, 29, 66, 88, 49, 33, 49, 73, 9, 81, 4, 36, 5, 14, 43, 31, 86,\n  27, 39, 75, 98, 99, 55, 19, 39, 21, 85, 86, 46, 82, 11, 44, 48, 77, 35, 48,\n  78, 97,\n];");
}
#[test]
fn test_numbers_negative_js_format_1_baabc3ad() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const numbers1 = [-2017,-506252,-744011292,-7224,-70.4,-83353.6,-708.4,-174023963.52,-40385,\n// comment1\n-380014,\n-253951682,-728,-15.84,-2058467564.56,-43,-33,-85134845,-67092,-1,-78820379,-2371.6,-16,7,\n// comment2\n-62454,-4282239912,\n-10816495.36,0.88,-100622682,8.8,-67087.68000000001,-3758276,-25.5211,-54,-1184265243,-46073628,-280423.44,\n-41833463,-27961.12,-305.36,-199875.28];\n\nconst numbers2 = [-234, -342 // comment3\n, -223, -333333.33,12345]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const numbers1 = [\n  -2017, -506252, -744011292, -7224, -70.4, -83353.6, -708.4, -174023963.52,\n  -40385,\n  // comment1\n  -380014, -253951682, -728, -15.84, -2058467564.56, -43, -33, -85134845,\n  -67092, -1, -78820379, -2371.6, -16, 7,\n  // comment2\n  -62454, -4282239912, -10816495.36, 0.88, -100622682, 8.8, -67087.68000000001,\n  -3758276, -25.5211, -54, -1184265243, -46073628, -280423.44, -41833463,\n  -27961.12, -305.36, -199875.28,\n];\n\nconst numbers2 = [\n  -234,\n  -342, // comment3\n  -223,\n  -333333.33,\n  12345,\n];");
}
#[test]
fn test_numbers_negative_comment_after_minus_js_format_1_a7b066e5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const numbers = [-2017,-506252,-744011292,-7224,-70.4,-83353.6,-708.4,-174023963.52,-40385,-// comment1\n380014,\n-253951682,-728,-15.84,-2058467564.56,-43,-33,-85134845,-67092,-1,-78820379,-2371.6,-16,7,\n// comment2\n-62454,-4282239912,\n-10816495.36,0.88,-100622682,8.8,-67087.68000000001,-3758276,-25.5211,-54,-1184265243,-46073628,-280423.44,\n-41833463,-27961.12,-305.36,-199875.28];\n\nc = [\n  - /**/ 66, 66, 57, 45, 47, 33, 53, 82, 81, 76, 66, 57, 45, 47, 33, 53, 82, 81, 223323\n];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const numbers = [\n  -2017,\n  -506252,\n  -744011292,\n  -7224,\n  -70.4,\n  -83353.6,\n  -708.4,\n  -174023963.52,\n  -40385,\n  -(\n    // comment1\n    380014\n  ),\n  -253951682,\n  -728,\n  -15.84,\n  -2058467564.56,\n  -43,\n  -33,\n  -85134845,\n  -67092,\n  -1,\n  -78820379,\n  -2371.6,\n  -16,\n  7,\n  // comment2\n  -62454,\n  -4282239912,\n  -10816495.36,\n  0.88,\n  -100622682,\n  8.8,\n  -67087.68000000001,\n  -3758276,\n  -25.5211,\n  -54,\n  -1184265243,\n  -46073628,\n  -280423.44,\n  -41833463,\n  -27961.12,\n  -305.36,\n  -199875.28,\n];\n\nc = [\n  -(/**/ 66),\n  66,\n  57,\n  45,\n  47,\n  33,\n  53,\n  82,\n  81,\n  76,\n  66,\n  57,\n  45,\n  47,\n  33,\n  53,\n  82,\n  81,\n  223323,\n];");
}
#[test]
fn test_numbers_trailing_comma_js_format_1_ea88b84f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// --------------- print-width -------------------------------------------------\nc = [\n  66, 66, 57, 45, 47, 33, 53, 82, 81, 76, 66, 57, 45, 47, 33, 53, 82, 81, 223323,\n];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// --------------- print-width -------------------------------------------------\nc = [\n  66, 66, 57, 45, 47, 33, 53, 82, 81, 76, 66, 57, 45, 47, 33, 53, 82, 81,\n  223323,\n];");
}
#[test]
fn test_numbers_with_holes_js_format_1_701198b4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const numberWithHoles1 = [\n  7234932941,\n  7234932722,\n  7234932312,\n  // comment before a hole 1\n  ,\n  7234932841,\n  ,\n  7234932843,\n  ,\n  // comment after a hole 1\n  7234932436,\n];\n\nconst numberWithHoles2 = [\n  0x234932941,\n  0x234932722,\n  0x234932312,\n\n  // comment before a hole 2\n  ,\n  0x234932841,\n  ,\n  0x234932843,\n  ,\n\n  // comment after a hole 2\n  0x234932436,\n];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const numberWithHoles1 = [\n  7234932941,\n  7234932722,\n  7234932312,\n  ,\n  // comment before a hole 1\n  7234932841,\n  ,\n  7234932843,\n  ,\n  // comment after a hole 1\n  7234932436,\n];\n\nconst numberWithHoles2 = [\n  0x234932941,\n  0x234932722,\n  0x234932312,\n\n  ,\n  // comment before a hole 2\n  0x234932841,\n  ,\n  0x234932843,\n  ,\n  // comment after a hole 2\n  0x234932436,\n];");
}
#[test]
fn test_numbers_with_trailing_comments_js_format_1_a634f02a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function isUnusedDiagnostic(code) {\n  return [\n    6133, // '{0}' is declared but never used.\n    6138, // Property '{0}' is declared but its value is never read.\n    6192, // All imports in import declaration are unused.\n    6196, // '{0}' is declared but its value is never read.\n    6198,\n    6199,\n    6205, // All type parameters are unused.\n  ].includes(code);\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function isUnusedDiagnostic(code) {\n  return [\n    6133, // '{0}' is declared but never used.\n    6138, // Property '{0}' is declared but its value is never read.\n    6192, // All imports in import declaration are unused.\n    6196, // '{0}' is declared but its value is never read.\n    6198,\n    6199,\n    6205, // All type parameters are unused.\n  ].includes(code);\n}");
}
#[test]
fn test_numbers_with_tricky_comments_js_format_1_6f8b9da1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const lazyCatererNumbers =  [1, 2, 4, 7, 11, 16, 22, 29, 37, 46,\n56, 67, 79, 92, 106, 121, 137, 154, 172, 191, 211, 232, 254, 277, 301, 326, 352, 379, 407, 436, 466, /*block*/\n// line\n497, 529, 562, 596, 631, 667, 704, 742, 781,\n821, 862, 904, 947, 991, 1036, 1082, 1129, 1177, 1226,\n// line 2\n1276, 1327, 1379];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const lazyCatererNumbers = [\n  1, 2, 4, 7, 11, 16, 22, 29, 37, 46, 56, 67, 79, 92, 106, 121, 137, 154, 172,\n  191, 211, 232, 254, 277, 301, 326, 352, 379, 407, 436, 466 /*block*/,\n  // line\n  497, 529, 562, 596, 631, 667, 704, 742, 781, 821, 862, 904, 947, 991, 1036,\n  1082, 1129, 1177, 1226,\n  // line 2\n  1276, 1327, 1379,\n];");
}
#[test]
fn test_numbers_1_js_format_1_dc78deac() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const numbers1 = [\n  7234932941,\n  7234932722,\n  7234932312,\n  7234932933,\n  7234932841,\n  7234932166,\n  7234932843,\n  7234932978,\n  7234932436,\n  7234932687,\n  7234932269,\n  7234932573,\n  7234932913,\n  7234932873,\n  7234932748,\n  7234932354,\n  7234932153,\n  7234932181,\n  7234932947,\n  7234932563,\n  7234932324,\n  7234932952,\n  7234932885,\n  7234932911,\n  7234932698,\n  7234932248,\n  7234932764,\n  7234932431,\n  7234932811,\n  7234932344,\n  7234932855,\n  7234932430,\n  7234932396,\n  7234932981,\n  7234932594,\n  7234932131,\n  7234932489,\n  7234932552,\n  7234932116,\n  7234932833,\n  7234932521,\n  7234932252,\n  7234932503,\n  7234932540,\n  7234932893,\n  7234932736,\n  7234932969,\n  7234932145,\n  7234932925,\n  7234932417,\n  7234932344,\n  7234932108,\n  7234932161,\n  7234932777,\n  7234932971,\n  7234932159,\n  7234932158,\n  7234932908,\n  7234932511,\n  7234932876,\n  7234932768,\n  7234932284,\n  7234932640,\n  7234932309,\n  7234932651,\n  7234932292,\n  7234932898,\n  7234932284,\n  7234932201,\n  7234932506,\n  7234932654,\n  7234932840,\n  7234932334,\n  7234932246,\n  7234932376,\n  7234932398,\n  7234932714,\n  7234932134,\n  7234932435,\n  7234932181,\n  7234932980,\n  7234932594,\n  7234932396,\n  7234932100,\n  7234932743,\n  7234932812,\n  7234932583,\n  7234932622,\n  7234932800,\n  7234932310,\n  7234932111,\n  7234932537,\n  7234932751,\n  7234932920,\n  7234932872,\n  7234932700,\n  7234932702,\n  7234932655,\n  7234932515,\n  7234932298\n];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const numbers1 = [\n  7234932941, 7234932722, 7234932312, 7234932933, 7234932841, 7234932166,\n  7234932843, 7234932978, 7234932436, 7234932687, 7234932269, 7234932573,\n  7234932913, 7234932873, 7234932748, 7234932354, 7234932153, 7234932181,\n  7234932947, 7234932563, 7234932324, 7234932952, 7234932885, 7234932911,\n  7234932698, 7234932248, 7234932764, 7234932431, 7234932811, 7234932344,\n  7234932855, 7234932430, 7234932396, 7234932981, 7234932594, 7234932131,\n  7234932489, 7234932552, 7234932116, 7234932833, 7234932521, 7234932252,\n  7234932503, 7234932540, 7234932893, 7234932736, 7234932969, 7234932145,\n  7234932925, 7234932417, 7234932344, 7234932108, 7234932161, 7234932777,\n  7234932971, 7234932159, 7234932158, 7234932908, 7234932511, 7234932876,\n  7234932768, 7234932284, 7234932640, 7234932309, 7234932651, 7234932292,\n  7234932898, 7234932284, 7234932201, 7234932506, 7234932654, 7234932840,\n  7234932334, 7234932246, 7234932376, 7234932398, 7234932714, 7234932134,\n  7234932435, 7234932181, 7234932980, 7234932594, 7234932396, 7234932100,\n  7234932743, 7234932812, 7234932583, 7234932622, 7234932800, 7234932310,\n  7234932111, 7234932537, 7234932751, 7234932920, 7234932872, 7234932700,\n  7234932702, 7234932655, 7234932515, 7234932298,\n];");
}
#[test]
fn test_numbers_2_js_format_1_476dd0bf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const userIds1 = [\n  7234932941,\n  7234932722,\n  7234932312,\n  7234932933,\n];\n\nconst userIds2 = [\n  7234932941,\n  7234932722,\n  7234932312,\n  7234932933,\n  7234932841,\n  7234932166,\n  7234932843,\n  7234932978,\n  7234932436,\n];\n\nconst userIds3 = [\n  7234932941,\n  7234932722,\n  7234932312,\n  7234932933,\n  7234932841,\n  7234932166,\n  7234932843,\n\n  7234932978,\n  7234932436,\n];\n\nconst userIds4 = [\n  7234932941,\n  7234932722,\n  7234932312,\n  7234932933,\n  7234932841,\n  7234932166,\n  // comment 1\n  7234932843,\n\n  7234932978,\n\n  // comment 2\n  7234932436,\n  // comment 3\n];\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const userIds1 = [7234932941, 7234932722, 7234932312, 7234932933];\n\nconst userIds2 = [\n  7234932941, 7234932722, 7234932312, 7234932933, 7234932841, 7234932166,\n  7234932843, 7234932978, 7234932436,\n];\n\nconst userIds3 = [\n  7234932941, 7234932722, 7234932312, 7234932933, 7234932841, 7234932166,\n  7234932843,\n\n  7234932978, 7234932436,\n];\n\nconst userIds4 = [\n  7234932941, 7234932722, 7234932312, 7234932933, 7234932841, 7234932166,\n  // comment 1\n  7234932843,\n\n  7234932978,\n\n  // comment 2\n  7234932436,\n  // comment 3\n];");
}
#[test]
fn test_numbers_3_js_format_1_dc50360b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("let test_case = [\n\t[\n\t\t66,57,45,47,33,53,82,81,76,78,10,78,15,98,24,29,32,27,28,76,41,65,84,35,\n\t\t97,90,75,24,88,45,23,75,63,86,24,39,9,51,33,40,58,17,49,86,63,59,97,91,\n\t\t98,99,5,69,51,44,34,69,17,91,27,83,26,34,93,29,66,88,49,33,49,73,9,81,4,\n\t\t36,5,14,43,31,86,27,39,75,98,99,55,19,39,21,85,86,46,82,11,44,48,77,35,\n\t\t48,78,97\n\t],\n\t[\n\t\t41,83,31,62,15,70,10,90,/*21,*/48,39,76,14,48,63,62,16,17,61,97,86,80,34,27,\n\t\t39,53,90,80,56,71,31,22,29,7,71,90,65,17,48,85,14,94,16,32,4,96,49,97,\n\t\t53,87,54,2,78,37,21,3,97,62,93,62,11,27,14,29,64,44,11,5,39,43,94,52,0,\n\t\t4,86,58,63,42,97,54,2,1,53,17,92,79,52,47,81,93,34,17,93,20,61,68,58,49,\n\t\t27,45\n\t]\n];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "let test_case = [\n  [\n    66, 57, 45, 47, 33, 53, 82, 81, 76, 78, 10, 78, 15, 98, 24, 29, 32, 27, 28,\n    76, 41, 65, 84, 35, 97, 90, 75, 24, 88, 45, 23, 75, 63, 86, 24, 39, 9, 51,\n    33, 40, 58, 17, 49, 86, 63, 59, 97, 91, 98, 99, 5, 69, 51, 44, 34, 69, 17,\n    91, 27, 83, 26, 34, 93, 29, 66, 88, 49, 33, 49, 73, 9, 81, 4, 36, 5, 14, 43,\n    31, 86, 27, 39, 75, 98, 99, 55, 19, 39, 21, 85, 86, 46, 82, 11, 44, 48, 77,\n    35, 48, 78, 97,\n  ],\n  [\n    41, 83, 31, 62, 15, 70, 10, 90, /*21,*/ 48, 39, 76, 14, 48, 63, 62, 16, 17,\n    61, 97, 86, 80, 34, 27, 39, 53, 90, 80, 56, 71, 31, 22, 29, 7, 71, 90, 65,\n    17, 48, 85, 14, 94, 16, 32, 4, 96, 49, 97, 53, 87, 54, 2, 78, 37, 21, 3, 97,\n    62, 93, 62, 11, 27, 14, 29, 64, 44, 11, 5, 39, 43, 94, 52, 0, 4, 86, 58, 63,\n    42, 97, 54, 2, 1, 53, 17, 92, 79, 52, 47, 81, 93, 34, 17, 93, 20, 61, 68,\n    58, 49, 27, 45,\n  ],\n];");
}
#[test]
fn test_preserve_empty_lines_js_format_1_8e5732d5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("a = [\n\n  1,\n  2,\n\n  3,\n\n\n  4,\n\n\n]\n\nb = [\n  100,\n\n  (200)\n  ,\n\n  300\n  ,\n\n  1\n  ,\n  2, 3\n]\n\nc = [\n  \"apple\",\n  \"banana\",\n  \"blueberry\",\n\n  \"red\",\n  \"blue\"\n  ,\n  \"yellow\",\n\n  \"broccoli\",\n  \"celery\",\n  \"lettuce\"\n  ,\n\n  \"green\"\n  ,\n  \"green\",\n  \"green\",\n\n  //an egg\n  \"egg\",\n  //a bigger egg\n  \"big egg\"\n  //the biggest egg\n  ,\n  \"huge egg\"\n  ,\n\n  //not an egg\n  \"lasagna\"\n\n]\n\n_ = [\n  a,\n\n  b //\n]\n\n_ = [\n  (a),\n\n  b, //\n];\n\n_ = [\n  (((((\n    a = b/* comment */))/* comment */))),\n\n  c //\n]\n\n_ = [\n  (((((\n    (a = b)/* comment */))/* comment */))),\n\n  c //\n]\n\n_ = [\n  (a=b\n\n  ),\n  b, //\n];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "a = [\n  1, 2,\n\n  3,\n\n  4,\n];\n\nb = [\n  100,\n\n  200,\n\n  300,\n\n  1, 2, 3,\n];\n\nc = [\n  \"apple\",\n  \"banana\",\n  \"blueberry\",\n\n  \"red\",\n  \"blue\",\n  \"yellow\",\n\n  \"broccoli\",\n  \"celery\",\n  \"lettuce\",\n\n  \"green\",\n  \"green\",\n  \"green\",\n\n  //an egg\n  \"egg\",\n  //a bigger egg\n  \"big egg\",\n  //the biggest egg\n  \"huge egg\",\n\n  //not an egg\n  \"lasagna\",\n];\n\n_ = [\n  a,\n\n  b, //\n];\n\n_ = [\n  a,\n\n  b, //\n];\n\n_ = [\n  (a = b) /* comment */ /* comment */,\n\n  c, //\n];\n\n_ = [\n  (a = b) /* comment */ /* comment */,\n\n  c, //\n];\n\n_ = [\n  (a = b),\n  b, //\n];");
}
#[test]
fn test_tuple_and_record_js_acorn_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_tuple_and_record_js_espree_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_tuple_and_record_js_flow_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_tuple_and_record_js_meriyah_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_tuple_and_record_js_typescript_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_tuple_and_record_js_format_1_9907fda5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("a = [\n  [1,2],\n  [1,2],\n]\n\na = [\n  #[1,2],\n  #[1,2],\n]\n\na = [\n  {a, b},\n  {a, b},\n]\n\na = [\n  #{a, b},\n  #{a, b},\n]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "a = [\n  [1, 2],\n  [1, 2],\n];\n\na = [\n  #[1, 2],\n  #[1, 2],\n];\n\na = [\n  { a, b },\n  { a, b },\n];\n\na = [\n  #{ a, b },\n  #{ a, b },\n];");
}
