use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_main_js_format_1_e49dbe53() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nimport declare_module_exports from \"declare_module_exports\";\n(declare_module_exports: number);\n(declare_module_exports: string); // Error: number ~> string\n\n// Error: Has no named export \"str\"!\nimport {str} from \"declare_m_e_with_other_value_declares\";\n\nimport type {str2} from \"declare_m_e_with_other_type_declares\";\n(\"asdf\": str2);\n(42: str2); // Error: number ~> string\n\n/**\n * \\`declare var exports\\` is deprecated, so we have a grace period where both\n * syntaxes will work.\n */\n\nimport DEPRECATED__declare_var_exports from \"DEPRECATED__declare_var_exports\";\n(DEPRECATED__declare_var_exports: number);\n(DEPRECATED__declare_var_exports: string); // Error: number ~> string\n\nimport declare_m_e_with_declare_var_e from \"declare_m_e_with_declare_var_e\";\n(declare_m_e_with_declare_var_e: number);\n(declare_m_e_with_declare_var_e: string); // Error: number ~> string") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nimport declare_module_exports from \"declare_module_exports\";\n(declare_module_exports: number);\n(declare_module_exports: string); // Error: number ~> string\n\n// Error: Has no named export \"str\"!\nimport { str } from \"declare_m_e_with_other_value_declares\";\n\nimport type { str2 } from \"declare_m_e_with_other_type_declares\";\n(\"asdf\": str2);\n(42: str2); // Error: number ~> string\n\n/**\n * \\`declare var exports\\` is deprecated, so we have a grace period where both\n * syntaxes will work.\n */\n\nimport DEPRECATED__declare_var_exports from \"DEPRECATED__declare_var_exports\";\n(DEPRECATED__declare_var_exports: number);\n(DEPRECATED__declare_var_exports: string); // Error: number ~> string\n\nimport declare_m_e_with_declare_var_e from \"declare_m_e_with_declare_var_e\";\n(declare_m_e_with_declare_var_e: number);\n(declare_m_e_with_declare_var_e: string); // Error: number ~> string");
}
