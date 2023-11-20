#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_classes_js_acorn_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_classes_js_espree_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_classes_js_format_1_5fec4324() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("@deco class Foo {}\n\n@deco export class Bar {}\n\n@deco export default class Baz {}\n\nconst foo = @deco class {\n  //\n};\n\nconst bar =\n  @deco\n  class {\n    //\n  };") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "@deco\nclass Foo {}\n\n@deco\nexport class Bar {}\n\n@deco\nexport default class Baz {}\n\nconst foo =\n  @deco\n  class {\n    //\n  };\n\nconst bar =\n  @deco\n  class {\n    //\n  };");
}
#[test]
fn test_comments_js_acorn_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_comments_js_espree_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_comments_js_format_1_2da35880() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["babel"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var x = 100\n\n@Hello({\n  a: 'a', // Comment is in the wrong place\n  // test\n  b: '2'\n})\nclass X {\n\n}\n\n\n@NgModule({\n  // Imports.\n  imports: [\n    // Angular modules.\n    BrowserModule,\n\n    // App modules.\n    CoreModule,\n    SharedModule,\n  ],\n})\nexport class AppModule {}\n\n// A\n@Foo()\n// B\n@Bar()\n// C\nexport class Bar{}\n\nclass Something {\n    @Annotateme()\n    property;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var x = 100;\n\n@Hello({\n  a: \"a\", // Comment is in the wrong place\n  // test\n  b: \"2\",\n})\nclass X {}\n\n@NgModule({\n  // Imports.\n  imports: [\n    // Angular modules.\n    BrowserModule,\n\n    // App modules.\n    CoreModule,\n    SharedModule,\n  ],\n})\nexport class AppModule {}\n\n// A\n@Foo()\n// B\n@Bar()\n// C\nexport class Bar {}\n\nclass Something {\n  @Annotateme()\n  property;\n}");
}
#[test]
fn test_member_expression_js_acorn_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_member_expression_js_espree_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_member_expression_js_format_1_892927bd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[\n  class {\n    @(decorators[0])\n    method() {}\n  },\n  class {\n    @decorators[0]\n    method() {}\n  },\n  class {\n    @(decorators?.[0])\n    method() {}\n  },\n  class {\n    @(decorators.at(0))\n    method() {}\n  },\n  class {\n    @(decorators?.at(0))\n    method() {}\n  },\n  class {\n    @(decorators.first)\n    method() {}\n  },\n  class {\n    @(decorators?.first)\n    method() {}\n  },\n  class {\n    @(decorators[first])\n    method() {}\n  },\n  class {\n    @decorators[first]\n    method() {}\n  },\n  class {\n    @(decorators[\"first\"])\n    method() {}\n  },\n  @(decorators[first])\n  class {\n    method() {}\n  },\n  @(decorators[0])\n  class {\n    method() {}\n  },\n]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  class {\n    @(decorators[0])\n    method() {}\n  },\n  class {\n    @decorators [0];\n    method() {}\n  },\n  class {\n    @(decorators?.[0])\n    method() {}\n  },\n  class {\n    @decorators.at(0)\n    method() {}\n  },\n  class {\n    @(decorators?.at(0))\n    method() {}\n  },\n  class {\n    @decorators.first\n    method() {}\n  },\n  class {\n    @(decorators?.first)\n    method() {}\n  },\n  class {\n    @(decorators[first])\n    method() {}\n  },\n  class {\n    @decorators [first];\n    method() {}\n  },\n  class {\n    @(decorators[\"first\"])\n    method() {}\n  },\n  @(decorators[first])\n  class {\n    method() {}\n  },\n  @(decorators[0])\n  class {\n    method() {}\n  },\n];");
}
#[test]
fn test_methods_js_acorn_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_methods_js_espree_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_methods_js_format_1_59dddbd8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["babel"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\nclass Yo {\n  @foo(\"hello\")\n  async plop() {}\n\n  @anotherDecoratorWithALongName(\"and a very long string as a first argument\")\n  async plip() {}\n\n  @anotherDecoratorWithALongName(\"another very long string, but now inline\") async plip() {}\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class Yo {\n  @foo(\"hello\")\n  async plop() {}\n\n  @anotherDecoratorWithALongName(\"and a very long string as a first argument\")\n  async plip() {}\n\n  @anotherDecoratorWithALongName(\"another very long string, but now inline\")\n  async plip() {}\n}");
}
#[test]
fn test_mixed_js_acorn_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_mixed_js_espree_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_mixed_js_format_1_9aae137a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://github.com/prettier/prettier/issues/6747\n\n@foo\nexport default class MyComponent {\n  @task\n  *foo() {\n  }\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// https://github.com/prettier/prettier/issues/6747\n\n@foo\nexport default class MyComponent {\n  @task\n  *foo() {}\n}");
}
#[test]
fn test_mobx_js_acorn_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_mobx_js_espree_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_mobx_js_format_1_90ddb0eb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["babel"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import {observable} from \"mobx\";\n\n@observer class OrderLine {\n  @observable price = 0;\n  @observable amount = 1;\n\n  constructor(price) {\n    this.price = price;\n  }\n\n  @computed get total() {\n    return this.price * this.amount;\n  }\n\n  @action.bound setPrice(price) {\n    this.price = price;\n  }\n\n  @computed\n  get total() {\n    return this.price * this.amount;\n  }\n\n  @action.bound\n  setPrice(price) {\n    this.price = price;\n  }\n\n  @computed @computed @computed @computed @computed @computed @computed get total() {\n    return this.price * this.amount;\n  }\n\n  @action handleDecrease = (event) => this.count--;\n\n  @action handleSomething = (event) => doSomething();\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import { observable } from \"mobx\";\n\n@observer\nclass OrderLine {\n  @observable price = 0;\n  @observable amount = 1;\n\n  constructor(price) {\n    this.price = price;\n  }\n\n  @computed get total() {\n    return this.price * this.amount;\n  }\n\n  @action.bound setPrice(price) {\n    this.price = price;\n  }\n\n  @computed\n  get total() {\n    return this.price * this.amount;\n  }\n\n  @action.bound\n  setPrice(price) {\n    this.price = price;\n  }\n\n  @computed\n  @computed\n  @computed\n  @computed\n  @computed\n  @computed\n  @computed\n  get total() {\n    return this.price * this.amount;\n  }\n\n  @action handleDecrease = (event) => this.count--;\n\n  @action handleSomething = (event) => doSomething();\n}");
}
#[test]
fn test_multiline_js_acorn_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_multiline_js_espree_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_multiline_js_format_1_8a7115e1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class Foo {\n  @deco([\n    foo,\n    bar\n  ]) prop = value;\n\n  @decorator([]) method() {}\n\n  @decorator([\n  ]) method() {}\n\n  @decorator({}) method() {}\n\n  @decorator({\n  }) method() {}\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class Foo {\n  @deco([foo, bar]) prop = value;\n\n  @decorator([]) method() {}\n\n  @decorator([]) method() {}\n\n  @decorator({}) method() {}\n\n  @decorator({}) method() {}\n}");
}
#[test]
fn test_multiple_js_acorn_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_multiple_js_espree_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_multiple_js_format_1_bb739bba() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const dog = class {\n  @readonly\n  @nonenumerable\n  @doubledValue\n  legs = 4;\n\n  @readonly\n  @nonenumerable\n  @doubledValue\n  eyes() {return 2}\n};\n\nconst foo = class {\n  @multipleDecorators @inline @theyWontAllFitInOneline aVeryLongPropName = \"A very long string as value\"\n  @multipleDecorators @inline @theyWontAllFitInOneline aVeryLongPropName() { \"A very long string as value\"}\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const dog = class {\n  @readonly\n  @nonenumerable\n  @doubledValue\n  legs = 4;\n\n  @readonly\n  @nonenumerable\n  @doubledValue\n  eyes() {\n    return 2;\n  }\n};\n\nconst foo = class {\n  @multipleDecorators @inline @theyWontAllFitInOneline aVeryLongPropName =\n    \"A very long string as value\";\n  @multipleDecorators @inline @theyWontAllFitInOneline aVeryLongPropName() {\n    \"A very long string as value\";\n  }\n};");
}
#[test]
fn test_parens_js_acorn_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_parens_js_espree_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_parens_js_format_1_ced2ac89() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("class X {\n  @(computed().volatile())\n  x\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "class X {\n  @(computed().volatile())\n  x;\n}");
}
#[test]
fn test_redux_js_acorn_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_redux_js_espree_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_redux_js_format_1_3de2e328() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["babel"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("@connect(mapStateToProps, mapDispatchToProps)\nexport class MyApp extends React.Component {}\n\n@connect(state => ({ todos: state.todos }))\nexport class Home extends React.Component {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "@connect(mapStateToProps, mapDispatchToProps)\nexport class MyApp extends React.Component {}\n\n@connect((state) => ({ todos: state.todos }))\nexport class Home extends React.Component {}");
}
