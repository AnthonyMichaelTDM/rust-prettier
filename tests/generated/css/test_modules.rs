#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_modules_css_format_1_c8bb69c9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("@value colors: './colors.css';\n@value first, second, third from colors;\n\n.title {\n  @nest :global(h1)& {\n    background: red;\n  }\n}\n\n.className {\n  color: green;\n  background: red;\n}\n\n.otherClassName {\n  composes: className;\n  color: yellow;\n}\n\n.otherClassName {\n  composes: className from \"./style.css\";\n}\n\n.otherClassName {\n  composes: globalClassName from global;\n}\n\n:global {\n  .global-class-name {\n    color: green;\n  }\n}\n\n.root :global .text {\n  color: brown;\n  font-size: 24px;\n  font-family: helvetica, arial, sans-serif;\n  font-weight: 600;\n}\n\n:import(\"path/to/dep.css\") {\n  localAlias: keyFromDep;\n}\n:export {\n  exportedKey: exportedValue;\n}\n\n@keyframes :global(spin) {\n  from {\n    transform: rotate(0deg);\n  }\n  to {\n    transform: rotate(360deg);\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "@value colors: './colors.css';\n@value first, second, third from colors;\n\n.title {\n  @nest :global(h1)& {\n    background: red;\n  }\n}\n\n.className {\n  color: green;\n  background: red;\n}\n\n.otherClassName {\n  composes: className;\n  color: yellow;\n}\n\n.otherClassName {\n  composes: className from \"./style.css\";\n}\n\n.otherClassName {\n  composes: globalClassName from global;\n}\n\n:global {\n  .global-class-name {\n    color: green;\n  }\n}\n\n.root :global .text {\n  color: brown;\n  font-size: 24px;\n  font-family: helvetica, arial, sans-serif;\n  font-weight: 600;\n}\n\n:import(\"path/to/dep.css\") {\n  localAlias: keyFromDep;\n}\n:export {\n  exportedKey: exportedValue;\n}\n\n@keyframes :global(spin) {\n  from {\n    transform: rotate(0deg);\n  }\n  to {\n    transform: rotate(360deg);\n  }\n}");
}
