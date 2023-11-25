#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_at_rules_scss_trailing_commaes_5_format_1_9208d4ff() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("scss")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("a {\n@include section-type-1(\n$header:\n(\nmargin: 0 0 $margin-base,\ntext-align: left,\n),\n$decoration:\n(\ntype: base,\nmargin: 0 auto -1px 0,\nprimary-color: $brand-primary,\nsecondary-color: $gray-light,\n),\n$title:\n(\nmargin: 0 0 $margin-small,\ncolor: false,\nfont-size: $font-size-h3,\nfont-weight: false,\nline-height: $line-height-h3,\n)\n);\n}\n\na {\n@include item-spotlight-properties-transition(\n\"-title\",\n(\nbox-shadow: 0 3px 10px rgba(0, 0, 0, 0.15),\n)\n);\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "a {\n  @include section-type-1(\n    $header: (\n      margin: 0 0 $margin-base,\n      text-align: left,\n    ),\n    $decoration: (\n      type: base,\n      margin: 0 auto -1px 0,\n      primary-color: $brand-primary,\n      secondary-color: $gray-light,\n    ),\n    $title: (\n      margin: 0 0 $margin-small,\n      color: false,\n      font-size: $font-size-h3,\n      font-weight: false,\n      line-height: $line-height-h3,\n    )\n  );\n}\n\na {\n  @include item-spotlight-properties-transition(\n    \"-title\",\n    (\n      box-shadow: 0 3px 10px rgba(0, 0, 0, 0.15),\n    )\n  );\n}");
}
#[test]
fn test_at_rules_scss_trailing_commanone_format_1_9208d4ff() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("scss")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("a {\n@include section-type-1(\n$header:\n(\nmargin: 0 0 $margin-base,\ntext-align: left,\n),\n$decoration:\n(\ntype: base,\nmargin: 0 auto -1px 0,\nprimary-color: $brand-primary,\nsecondary-color: $gray-light,\n),\n$title:\n(\nmargin: 0 0 $margin-small,\ncolor: false,\nfont-size: $font-size-h3,\nfont-weight: false,\nline-height: $line-height-h3,\n)\n);\n}\n\na {\n@include item-spotlight-properties-transition(\n\"-title\",\n(\nbox-shadow: 0 3px 10px rgba(0, 0, 0, 0.15),\n)\n);\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "a {\n  @include section-type-1(\n    $header: (\n      margin: 0 0 $margin-base,\n      text-align: left\n    ),\n    $decoration: (\n      type: base,\n      margin: 0 auto -1px 0,\n      primary-color: $brand-primary,\n      secondary-color: $gray-light\n    ),\n    $title: (\n      margin: 0 0 $margin-small,\n      color: false,\n      font-size: $font-size-h3,\n      font-weight: false,\n      line-height: $line-height-h3\n    )\n  );\n}\n\na {\n  @include item-spotlight-properties-transition(\n    \"-title\",\n    (\n      box-shadow: 0 3px 10px rgba(0, 0, 0, 0.15)\n    )\n  );\n}");
}
#[test]
fn test_declaration_scss_trailing_commaes_5_format_1_351a4dcc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("scss")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a {\n    margin: $bar,;\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "a {\n  margin: $bar;\n}");
}
#[test]
fn test_declaration_scss_trailing_commanone_format_1_351a4dcc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("scss")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a {\n    margin: $bar,;\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "a {\n  margin: $bar;\n}");
}
#[test]
fn test_list_scss_trailing_commaes_5_format_1_d1a3021a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("scss")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("$colors: hotpink deepskyblue firebrick,;\n$list: (a,);\n$list: ('Helvetica', 'Arial', sans-serif,);\n$colors: (\n  \"red\",\n  \"blue\"\n);\n$config: (\n  themes: (\n    mist: (\n      header: #dcfac0,\n      content: #00968b,\n      footer: #85c79c\n    ),\n    $spring: (\n      header: #f4fac7,\n      content: #c2454e,\n      footer: #ffb158\n    )\n  )\n);\n\n$breakpoint-map: (\n  small: (\n    min-width: null,\n    max-width: 479px,\n    base-font: 16px,\n    vertical-rhythm: 1.3\n  ),\n  medium: (\n    min-width: 480px,\n    max-width: 959px,\n    base-font: 18px,\n    vertical-rhythm: 1.414\n  ),\n  large: (\n    min-width: 960px,\n    max-width: 1099px,\n    base-font: 18px,\n    vertical-rhythm: 1.5\n  ),\n  xlarge: (\n    min-width: 1100px,\n    max-width: null,\n    base-font: 21px,\n    vertical-rhythm: 1.618\n  )\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "$colors: hotpink deepskyblue firebrick;\n$list: (a);\n$list: (\"Helvetica\", \"Arial\", sans-serif);\n$colors: (\"red\", \"blue\");\n$config: (\n  themes: (\n    mist: (\n      header: #dcfac0,\n      content: #00968b,\n      footer: #85c79c,\n    ),\n    $spring: (\n      header: #f4fac7,\n      content: #c2454e,\n      footer: #ffb158,\n    ),\n  ),\n);\n\n$breakpoint-map: (\n  small: (\n    min-width: null,\n    max-width: 479px,\n    base-font: 16px,\n    vertical-rhythm: 1.3,\n  ),\n  medium: (\n    min-width: 480px,\n    max-width: 959px,\n    base-font: 18px,\n    vertical-rhythm: 1.414,\n  ),\n  large: (\n    min-width: 960px,\n    max-width: 1099px,\n    base-font: 18px,\n    vertical-rhythm: 1.5,\n  ),\n  xlarge: (\n    min-width: 1100px,\n    max-width: null,\n    base-font: 21px,\n    vertical-rhythm: 1.618,\n  ),\n);");
}
#[test]
fn test_list_scss_trailing_commanone_format_1_d1a3021a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("scss")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("$colors: hotpink deepskyblue firebrick,;\n$list: (a,);\n$list: ('Helvetica', 'Arial', sans-serif,);\n$colors: (\n  \"red\",\n  \"blue\"\n);\n$config: (\n  themes: (\n    mist: (\n      header: #dcfac0,\n      content: #00968b,\n      footer: #85c79c\n    ),\n    $spring: (\n      header: #f4fac7,\n      content: #c2454e,\n      footer: #ffb158\n    )\n  )\n);\n\n$breakpoint-map: (\n  small: (\n    min-width: null,\n    max-width: 479px,\n    base-font: 16px,\n    vertical-rhythm: 1.3\n  ),\n  medium: (\n    min-width: 480px,\n    max-width: 959px,\n    base-font: 18px,\n    vertical-rhythm: 1.414\n  ),\n  large: (\n    min-width: 960px,\n    max-width: 1099px,\n    base-font: 18px,\n    vertical-rhythm: 1.5\n  ),\n  xlarge: (\n    min-width: 1100px,\n    max-width: null,\n    base-font: 21px,\n    vertical-rhythm: 1.618\n  )\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "$colors: hotpink deepskyblue firebrick;\n$list: (a);\n$list: (\"Helvetica\", \"Arial\", sans-serif);\n$colors: (\"red\", \"blue\");\n$config: (\n  themes: (\n    mist: (\n      header: #dcfac0,\n      content: #00968b,\n      footer: #85c79c\n    ),\n    $spring: (\n      header: #f4fac7,\n      content: #c2454e,\n      footer: #ffb158\n    )\n  )\n);\n\n$breakpoint-map: (\n  small: (\n    min-width: null,\n    max-width: 479px,\n    base-font: 16px,\n    vertical-rhythm: 1.3\n  ),\n  medium: (\n    min-width: 480px,\n    max-width: 959px,\n    base-font: 18px,\n    vertical-rhythm: 1.414\n  ),\n  large: (\n    min-width: 960px,\n    max-width: 1099px,\n    base-font: 18px,\n    vertical-rhythm: 1.5\n  ),\n  xlarge: (\n    min-width: 1100px,\n    max-width: null,\n    base-font: 21px,\n    vertical-rhythm: 1.618\n  )\n);");
}
#[test]
fn test_map_scss_trailing_commaes_5_format_1_305b355c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("scss")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("$map: (\n    small: 767px,\n    medium: 992px,\n    large: 1200px,\n);\n$map: (\n    'medium': (min-width: 800px),\n    'large': (min-width: 1000px),\n    'huge': (min-width: 1200px),\n);\n$map: ( small: 767px, medium: 992px, large: 1200px );") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "$map: (\n  small: 767px,\n  medium: 992px,\n  large: 1200px,\n);\n$map: (\n  \"medium\": (\n    min-width: 800px,\n  ),\n  \"large\": (\n    min-width: 1000px,\n  ),\n  \"huge\": (\n    min-width: 1200px,\n  ),\n);\n$map: (\n  small: 767px,\n  medium: 992px,\n  large: 1200px,\n);");
}
#[test]
fn test_map_scss_trailing_commanone_format_1_305b355c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("scss")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("$map: (\n    small: 767px,\n    medium: 992px,\n    large: 1200px,\n);\n$map: (\n    'medium': (min-width: 800px),\n    'large': (min-width: 1000px),\n    'huge': (min-width: 1200px),\n);\n$map: ( small: 767px, medium: 992px, large: 1200px );") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "$map: (\n  small: 767px,\n  medium: 992px,\n  large: 1200px\n);\n$map: (\n  \"medium\": (\n    min-width: 800px\n  ),\n  \"large\": (\n    min-width: 1000px\n  ),\n  \"huge\": (\n    min-width: 1200px\n  )\n);\n$map: (\n  small: 767px,\n  medium: 992px,\n  large: 1200px\n);");
}
#[test]
fn test_selector_list_scss_trailing_commaes_5_format_1_d1e9c00d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("scss")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("asdasldaskdhjkashdahsdkjahskdjhakjsdkjahsdhkasdhkajsdhakjsdhkajsdhjkahskjdkjahsjkdjkakjsdm,\nasdasldaskdhjkashdahsdkjahskdjhakjsdkjahsdhkasdhkajsdhakjsdhkajsdhjkahskjdkjahsjkdjkakjsdm, {\n\n}\n\n.some-class, {\n&.another-class, {\n     color: red;\n }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "asdasldaskdhjkashdahsdkjahskdjhakjsdkjahsdhkasdhkajsdhakjsdhkajsdhjkahskjdkjahsjkdjkakjsdm,\nasdasldaskdhjkashdahsdkjahskdjhakjsdkjahsdhkasdhkajsdhakjsdhkajsdhjkahskjdkjahsjkdjkakjsdm {\n}\n\n.some-class {\n  &.another-class {\n    color: red;\n  }\n}");
}
#[test]
fn test_selector_list_scss_trailing_commanone_format_1_d1e9c00d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("scss")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("asdasldaskdhjkashdahsdkjahskdjhakjsdkjahsdhkasdhkajsdhakjsdhkajsdhjkahskjdkjahsjkdjkakjsdm,\nasdasldaskdhjkashdahsdkjahskdjhakjsdkjahsdhkasdhkajsdhakjsdhkajsdhjkahskjdkjahsjkdjkakjsdm, {\n\n}\n\n.some-class, {\n&.another-class, {\n     color: red;\n }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "asdasldaskdhjkashdahsdkjahskdjhakjsdkjahsdhkasdhkajsdhakjsdhkajsdhjkahskjdkjahsjkdjkakjsdm,\nasdasldaskdhjkashdahsdkjahskdjhakjsdkjahsdhkasdhkajsdhakjsdhkajsdhjkahskjdkjahsjkdjkakjsdm {\n}\n\n.some-class {\n  &.another-class {\n    color: red;\n  }\n}");
}
#[test]
fn test_variable_scss_trailing_commaes_5_format_1_257a4bf8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("scss")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("$test: 1,;\n$margin: 0, 2em, 0, 1.5em,;\n");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "$test: 1;\n$margin: 0, 2em, 0, 1.5em;");
}
#[test]
fn test_variable_scss_trailing_commanone_format_1_257a4bf8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("scss")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("$test: 1,;\n$margin: 0, 2em, 0, 1.5em,;\n");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "$test: 1;\n$margin: 0, 2em, 0, 1.5em;");
}
