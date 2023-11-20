#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_parens_js_format_1_aea11354() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const result = (a + b) >>> 1;\nvar sizeIndex = ((index - 1) >>> level) & MASK;\nvar from = offset > left ? 0 : (left - offset) >> level;\nvar to = ((right - offset) >> level) + 1;\nif (rawIndex < 1 << (list._level + SHIFT)) {}\nvar res = size < SIZE ? 0 : (((size - 1) >>> SHIFT) << SHIFT);\nsign = 1 - (2 * (b[3] >> 7));\nexponent = (((b[3] << 1) & 0xff) | (b[2] >> 7)) - 127;\nmantissa = ((b[2] & 0x7f) << 16) | (b[1] << 8) | b[0];\n\n2 / 3 * 10 / 2 + 2;\nconst rotateX = ((RANGE / rect.height) * refY - RANGE / 2) * getXMultiplication(rect.width);\nconst rotateY = ((RANGE / rect.width) * refX - RANGE / 2) * getYMultiplication(rect.width);\n\na % 10 - 5;\na * b % 10;\na % 10 > 5;\na % 10 == 0;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const result = (a + b) >>> 1;\nvar sizeIndex = ((index - 1) >>> level) & MASK;\nvar from = offset > left ? 0 : (left - offset) >> level;\nvar to = ((right - offset) >> level) + 1;\nif (rawIndex < 1 << (list._level + SHIFT)) {\n}\nvar res = size < SIZE ? 0 : ((size - 1) >>> SHIFT) << SHIFT;\nsign = 1 - 2 * (b[3] >> 7);\nexponent = (((b[3] << 1) & 0xff) | (b[2] >> 7)) - 127;\nmantissa = ((b[2] & 0x7f) << 16) | (b[1] << 8) | b[0];\n\n((2 / 3) * 10) / 2 + 2;\nconst rotateX =\n  ((RANGE / rect.height) * refY - RANGE / 2) * getXMultiplication(rect.width);\nconst rotateY =\n  ((RANGE / rect.width) * refX - RANGE / 2) * getYMultiplication(rect.width);\n\n(a % 10) - 5;\n(a * b) % 10;\na % 10 > 5;\na % 10 == 0;");
}
