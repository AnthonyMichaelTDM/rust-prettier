#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_test_component_ts_trailing_commaes_5_format_1_e5c56993() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("@Component({\n       selector: 'app-test',\n  template: \\`<ul>   <li>test</li>\n  </ul>\n  \\`,\n  styles: [   \\`\n  \n :host {\n   color: red;\n } \n div { background: blue\n }\n\\`\n\n]\n})\nclass     TestComponent {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "@Component({\n  selector: \"app-test\",\n  template: \\`<ul>\n    <li>test</li>\n  </ul> \\`,\n  styles: [\n    \\`\n      :host {\n        color: red;\n      }\n      div {\n        background: blue;\n      }\n    \\`,\n  ],\n})\nclass TestComponent {}");
}
#[test]
fn test_test_component_ts_trailing_commanone_format_1_e5c56993() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("@Component({\n       selector: 'app-test',\n  template: \\`<ul>   <li>test</li>\n  </ul>\n  \\`,\n  styles: [   \\`\n  \n :host {\n   color: red;\n } \n div { background: blue\n }\n\\`\n\n]\n})\nclass     TestComponent {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "@Component({\n  selector: \"app-test\",\n  template: \\`<ul>\n    <li>test</li>\n  </ul> \\`,\n  styles: [\n    \\`\n      :host {\n        color: red;\n      }\n      div {\n        background: blue;\n      }\n    \\`\n  ]\n})\nclass TestComponent {}");
}
