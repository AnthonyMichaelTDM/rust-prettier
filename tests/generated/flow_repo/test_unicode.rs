#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_unicode_utils_js_format_1_1b05ef6a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\n\n/**\n * @param {number} codeUnit   A Unicode code-unit, in range [0, 0x10FFFF]\n * @return {boolean}      Whether code-unit is in a surrogate (hi/low) range\n */\nfunction inSurrogateRange(codeUnit) {\n  return 0xD800 <= codeUnit && codeUnit <= 0xDFFF;\n}\n\n\n/**\n * Return the length of the original Unicode character at given position in the\n * String by looking into the UTF-16 code unit; that is equal to 1 for any\n * non-surrogate characters in BMP ([U+0000..U+D7FF] and [U+E000, U+FFFF]); and\n * returns 2 for the hi/low surrogates ([U+D800..U+DFFF]), which are in fact\n * representing non-BMP characters ([U+10000..U+10FFFF]).\n *\n * Examples:\n * - '\\\\u0020' => 1\n * - '\\\\u3020' => 1\n * - '\\\\uD835' => 2\n * - '\\\\uD835\\\\uDDEF' => 2\n * - '\\\\uDDEF' => 2\n *\n * @param {string} str  Non-empty string\n * @param {number} pos  Position in the string to look for one code unit\n * @return {number}      Number 1 or 2\n */\nfunction utf16Length(str, pos) {\n  return 1 + inSurrogateRange(str.charCodeAt(pos));\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @flow\n */\n\n/**\n * @param {number} codeUnit   A Unicode code-unit, in range [0, 0x10FFFF]\n * @return {boolean}      Whether code-unit is in a surrogate (hi/low) range\n */\nfunction inSurrogateRange(codeUnit) {\n  return 0xd800 <= codeUnit && codeUnit <= 0xdfff;\n}\n\n/**\n * Return the length of the original Unicode character at given position in the\n * String by looking into the UTF-16 code unit; that is equal to 1 for any\n * non-surrogate characters in BMP ([U+0000..U+D7FF] and [U+E000, U+FFFF]); and\n * returns 2 for the hi/low surrogates ([U+D800..U+DFFF]), which are in fact\n * representing non-BMP characters ([U+10000..U+10FFFF]).\n *\n * Examples:\n * - '\\\\u0020' => 1\n * - '\\\\u3020' => 1\n * - '\\\\uD835' => 2\n * - '\\\\uD835\\\\uDDEF' => 2\n * - '\\\\uDDEF' => 2\n *\n * @param {string} str  Non-empty string\n * @param {number} pos  Position in the string to look for one code unit\n * @return {number}      Number 1 or 2\n */\nfunction utf16Length(str, pos) {\n  return 1 + inSurrogateRange(str.charCodeAt(pos));\n}");
}
