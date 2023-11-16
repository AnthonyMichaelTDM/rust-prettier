#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_shebang_js_format_1_f0d14fbc() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("#!/usr/bin/env node\nfunction a() {}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "#!/usr/bin/env node\nfunction a() {}");
}
#[test]
fn test_shebang_newline_js_format_1_7bc29edc() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("#!/usr/bin/env node\n\nfunction a() {}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "#!/usr/bin/env node\n\nfunction a() {}");
}
#[test]
fn test_snippet_empty_file_with_shebang_format_1_38ea2608() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("#!/usr/bin/env nod");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "#!/usr/bin/env node");
}
#[test]
fn test_snippet_empty_file_with_shebang_format_2_3e2dca7d() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("#!/usr/bin/env node");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "#!/usr/bin/env node");
}
