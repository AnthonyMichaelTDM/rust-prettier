#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_abstract_method_ts_babel_ts_format_1_96c8f469() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("class A {\n    @decorator()\n    abstract method(): Array<string>\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "class A {\n  @decorator()\n  abstract method(): Array<string>;\n}"
    );
}
#[test]
fn test_abstract_method_ts_typescript_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_accessor_ts_format_1_c6804d89() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("class A {\n    @foo()\n    get a() {return 1}\n    @bar()\n    set a(v) {}\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "class A {\n  @foo()\n  get a() {\n    return 1;\n  }\n  @bar()\n  set a(v) {}\n}"
    );
}
#[test]
fn test_argument_list_preserve_line_ts_format_1_21b3c0f7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class Foo {\n    constructor(\n        @inject(Bar)\n        private readonly bar: IBar,\n\n        @inject(MyProcessor)\n        private readonly myProcessor: IMyProcessor,\n\n        @inject(InjectionTypes.AnotherThing)\n\n        private readonly anotherThing: IAnotherThing | undefined,\n    ) { }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class Foo {\n  constructor(\n    @inject(Bar)\n    private readonly bar: IBar,\n\n    @inject(MyProcessor)\n    private readonly myProcessor: IMyProcessor,\n\n    @inject(InjectionTypes.AnotherThing)\n    private readonly anotherThing: IAnotherThing | undefined,\n  ) {}\n}");
}
#[test]
fn test_comments_ts_format_1_2485ed40() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class Something {\n    @Annotateme()\n    // comment\n    static property: Array<string>;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "class Something {\n  @Annotateme()\n  // comment\n  static property: Array<string>;\n}"
    );
}
#[test]
fn test_decorator_type_assertion_ts_format_1_0f7fd148() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("@(bind as ClassDecorator)\nclass Decorated {\n\n}\n\n@(<ClassDecorator>bind)\nclass Decorated {\n\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "@(bind as ClassDecorator)\nclass Decorated {}\n\n@(<ClassDecorator>bind)\nclass Decorated {}");
}
#[test]
fn test_decorators_ts_format_1_8f726109() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export class TestTextFileService {\n\tconstructor(\n\t\t@ILifecycleService lifecycleService,\n\t) {\n\t}\n}\n\n@commonEditorContribution\nexport class TabCompletionController {\n}\n\n@Component({\n  selector: 'angular-component',\n})\nclass AngularComponent {\n  @Input() myInput: string;\n}\n\nclass Class {\n  method(\n    @Decorator\n    { prop1, prop2 }: Type\n  ) {\n    doSomething();\n  }\n}\n\nclass Class2 {\n  method(\n    @Decorator1\n    @Decorator2\n    { prop1, prop2 }: Type\n  ) {\n    doSomething();\n  }\n}\n\nclass Class3 {\n  method(\n    @Decorator\n    { prop1_1, prop1_2 }: Type,\n    { prop2_1, prop2_2 }: Type\n  ) {\n    doSomething();\n  }\n}\n\nclass Class4 {\n  method(\n    param1,\n    @Decorator\n    { prop1, prop2 }: Type\n  ) {}\n}\n\nclass Class5 {\n  method(\n    @Decorator { prop1 }: Type\n  ) {}\n}\n\nclass Class6 {\n  method(\n    @Decorator({}) { prop1 }: Type\n  ) {}\n  method(\n    @Decorator(\n      {}) { prop1 }: Type\n  ) {}\n  method(\n    @Decorator([]) { prop1 }: Type\n  ) {}\n  method(\n    @Decorator(\n      []) { prop1 }: Type\n  ) {}\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export class TestTextFileService {\n  constructor(@ILifecycleService lifecycleService) {}\n}\n\n@commonEditorContribution\nexport class TabCompletionController {}\n\n@Component({\n  selector: \"angular-component\",\n})\nclass AngularComponent {\n  @Input() myInput: string;\n}\n\nclass Class {\n  method(\n    @Decorator\n    { prop1, prop2 }: Type,\n  ) {\n    doSomething();\n  }\n}\n\nclass Class2 {\n  method(\n    @Decorator1\n    @Decorator2\n    { prop1, prop2 }: Type,\n  ) {\n    doSomething();\n  }\n}\n\nclass Class3 {\n  method(\n    @Decorator\n    { prop1_1, prop1_2 }: Type,\n    { prop2_1, prop2_2 }: Type,\n  ) {\n    doSomething();\n  }\n}\n\nclass Class4 {\n  method(\n    param1,\n    @Decorator\n    { prop1, prop2 }: Type,\n  ) {}\n}\n\nclass Class5 {\n  method(@Decorator { prop1 }: Type) {}\n}\n\nclass Class6 {\n  method(@Decorator({}) { prop1 }: Type) {}\n  method(@Decorator({}) { prop1 }: Type) {}\n  method(@Decorator([]) { prop1 }: Type) {}\n  method(@Decorator([]) { prop1 }: Type) {}\n}");
}
#[test]
fn test_decorators_comments_ts_format_1_b04e0157() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\nclass Foo1 {\n    @foo\n    // comment\n    async method() {}\n}\n\nclass Foo2 {\n    @foo\n    // comment\n    private method() {}\n}\n\nclass Foo3 {\n    @foo\n    // comment\n    *method() {}\n}\n\nclass Foo4 {\n    @foo\n    // comment\n    async *method() {}\n}\n\nclass Something {\n    @foo()\n    // comment\n    readonly property: Array<string>\n}\n\nclass Something2 {\n    @foo()\n    // comment\n    abstract property: Array<string>\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class Foo1 {\n  @foo\n  // comment\n  async method() {}\n}\n\nclass Foo2 {\n  @foo\n  // comment\n  private method() {}\n}\n\nclass Foo3 {\n  @foo\n  // comment\n  *method() {}\n}\n\nclass Foo4 {\n  @foo\n  // comment\n  async *method() {}\n}\n\nclass Something {\n  @foo()\n  // comment\n  readonly property: Array<string>;\n}\n\nclass Something2 {\n  @foo()\n  // comment\n  abstract property: Array<string>;\n}");
}
#[test]
fn test_inline_decorators_ts_format_1_7332206d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\n@d1\n@d2(foo)\n@d3.bar\n@d4.baz()\nclass Class1 {}\n\nclass Class2 {\n    @d1\n    @d2(foo)\n    @d3.bar\n    @d4.baz()\n    method1() {}\n\n    @d1\n    method2() {}\n\n    @d2(foo)\n    method3() {}\n\n    @d3.bar\n    method4() {}\n}\n\nclass Class3 {\n    @d1 fieldA;\n    @d2(foo) fieldB;\n    @d3.bar fieldC;\n    @d4.baz() fieldD;\n\n    constructor (\n        @d1 private x: number,\n        @d2(foo) private y: number,\n        @d3('foo') private z: number,\n        @d4({\n            x: string\n        }) private a: string,\n    ) {}\n}\n\n@decorated class Foo {}\n\nclass Bar {\n    @decorated method() {}\n}\n\nclass MyContainerComponent {\n  @ContentChildren(MyComponent) components: QueryListSomeBigName<MyComponentThat>;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "@d1\n@d2(foo)\n@d3.bar\n@d4.baz()\nclass Class1 {}\n\nclass Class2 {\n  @d1\n  @d2(foo)\n  @d3.bar\n  @d4.baz()\n  method1() {}\n\n  @d1\n  method2() {}\n\n  @d2(foo)\n  method3() {}\n\n  @d3.bar\n  method4() {}\n}\n\nclass Class3 {\n  @d1 fieldA;\n  @d2(foo) fieldB;\n  @d3.bar fieldC;\n  @d4.baz() fieldD;\n\n  constructor(\n    @d1 private x: number,\n    @d2(foo) private y: number,\n    @d3(\"foo\") private z: number,\n    @d4({\n      x: string,\n    })\n    private a: string,\n  ) {}\n}\n\n@decorated\nclass Foo {}\n\nclass Bar {\n  @decorated method() {}\n}\n\nclass MyContainerComponent {\n  @ContentChildren(MyComponent)\n  components: QueryListSomeBigName<MyComponentThat>;\n}");
}
#[test]
fn test_legacy_ts_format_1_babce945() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[\n  @decorator() class {},\n  @decorator() class A {},\n];\n\nclass A {\n  @decorator() accessor #field;\n}\n\nclass B {\n  @decorator() #field () {}\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  @decorator()\n  class {},\n  @decorator()\n  class A {},\n];\n\nclass A {\n  @decorator() accessor #field;\n}\n\nclass B {\n  @decorator() #field() {}\n}");
}
#[test]
fn test_mobx_ts_format_1_12d611d0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import {observable} from \"mobx\";\n\n@observer class OrderLine {\n  @observable price:number = 0;\n  @observable amount:number = 1;\n\n  constructor(price) {\n    this.price = price;\n  }\n\n  @computed get total() {\n    return this.price * this.amount;\n  }\n\n  @action.bound setPrice(price) {\n    this.price = price;\n  }\n\n  @computed\n  get total2() {\n    return this.price * this.amount;\n  }\n\n  @action.bound\n  setPrice(price) {\n    this.price = price;\n  }\n\n  @computed @computed @computed @computed @computed @computed @computed get total3() {\n    return this.price * this.amount;\n  }\n\n  @action handleDecrease = (event: React.ChangeEvent<HTMLInputElement>) => this.count--;\n\n  @action handleSomething = (event: React.ChangeEvent<HTMLInputElement>) => doSomething();\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import { observable } from \"mobx\";\n\n@observer\nclass OrderLine {\n  @observable price: number = 0;\n  @observable amount: number = 1;\n\n  constructor(price) {\n    this.price = price;\n  }\n\n  @computed get total() {\n    return this.price * this.amount;\n  }\n\n  @action.bound setPrice(price) {\n    this.price = price;\n  }\n\n  @computed\n  get total2() {\n    return this.price * this.amount;\n  }\n\n  @action.bound\n  setPrice(price) {\n    this.price = price;\n  }\n\n  @computed\n  @computed\n  @computed\n  @computed\n  @computed\n  @computed\n  @computed\n  get total3() {\n    return this.price * this.amount;\n  }\n\n  @action handleDecrease = (event: React.ChangeEvent<HTMLInputElement>) =>\n    this.count--;\n\n  @action handleSomething = (event: React.ChangeEvent<HTMLInputElement>) =>\n    doSomething();\n}");
}
