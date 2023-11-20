#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_comments_js_format_1_2485ed40() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel-flow"])
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
fn test_mobx_js_format_1_c8aa5db6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel-flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import {observable} from \"mobx\";\n\n@observer class OrderLine {\n  @observable price:number = 0;\n  @observable amount:number = 1;\n\n  constructor(price) {\n    this.price = price;\n  }\n\n  @computed get total() {\n    return this.price * this.amount;\n  }\n\n  @action.bound setPrice(price) {\n    this.price = price;\n  }\n\n  @computed\n  get total() {\n    return this.price * this.amount;\n  }\n\n  @action.bound\n  setPrice(price) {\n    this.price = price;\n  }\n\n  @computed @computed @computed @computed @computed @computed @computed get total() {\n    return this.price * this.amount;\n  }\n\n  @action handleDecrease = (event: React.ChangeEvent<HTMLInputElement>) => this.count--;\n\n  @action handleSomething = (event: React.ChangeEvent<HTMLInputElement>) => doSomething();\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import { observable } from \"mobx\";\n\n@observer\nclass OrderLine {\n  @observable price: number = 0;\n  @observable amount: number = 1;\n\n  constructor(price) {\n    this.price = price;\n  }\n\n  @computed get total() {\n    return this.price * this.amount;\n  }\n\n  @action.bound setPrice(price) {\n    this.price = price;\n  }\n\n  @computed\n  get total() {\n    return this.price * this.amount;\n  }\n\n  @action.bound\n  setPrice(price) {\n    this.price = price;\n  }\n\n  @computed\n  @computed\n  @computed\n  @computed\n  @computed\n  @computed\n  @computed\n  get total() {\n    return this.price * this.amount;\n  }\n\n  @action handleDecrease = (event: React.ChangeEvent<HTMLInputElement>) =>\n    this.count--;\n\n  @action handleSomething = (event: React.ChangeEvent<HTMLInputElement>) =>\n    doSomething();\n}");
}
