#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_arrow_js_format_1_75edbbf0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// Error\nconst a = (data/*: Object */) => {}\n\n// OK\nconst b = (data/*: Object */, secondData/*: Object */) => {}\n\nconst c = (data/*: /* this is an object *-/ Object */) => {};\n\nconst run = (cmd /*: string */) /*: Promise<void> */ => {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// Error\nconst a = (data: Object) => {};\n\n// OK\nconst b = (data: Object, secondData: Object) => {};\n\nconst c = (data: /* this is an object */ Object) => {};\n\nconst run = (cmd: string): Promise<void> => {};");
}
#[test]
fn test_cast_js_format_1_fc48097d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("(a /* b */ /*: c */);\n(a /* b */   : c   );\n(a /* b */ /*  : c */);\n(a /* b */ /*:: : c */);\n(a /* b */ /*  :: : c */);\n\n(a : /* b */ c );\n(a : c /* b */ );") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "(a /* b */ : c);\n(a /* b */ : c);\n(a /* b */ : c);\n(a /* b */ : c);\n(a /* b */ : c);\n\n(a: /* b */ c);\n(a: c /* b */);");
}
#[test]
fn test_class_js_format_1_318517fd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("class A {\n  x /*: string */;\n\n  method(a /*: T */, b /*: T */) /*: T */ {}\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "class A {\n  x: string;\n\n  method(a: T, b: T): T {}\n}"
    );
}
#[test]
fn test_functions_js_format_1_58c9e67c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("function foo<T>(bar  /*: T[] */, baz  /*: T */) /*: S */ {}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "function foo<T>(bar: T[], baz: T): S {}");
}
#[test]
fn test_functions_2_js_format_1_aaed2574() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function f1(a /* b */ /*: c */){}\nfunction f2(a /* b */   : c   ){}\nfunction f3(a /* b */ /*  : c */){}\nfunction f4(a /* b */ /*:: : c */){}\n\nfunction f2(a : /* b */ c ){}\nfunction f2(a : c /* b */ ){}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function f1(a /* b */ : c) {}\nfunction f2(a /* b */ : c) {}\nfunction f3(a /* b */ : c) {}\nfunction f4(a /* b */ : c) {}\n\nfunction f2(a: /* b */ c) {}\nfunction f2(a: c /* b */) {}");
}
#[test]
fn test_generics_js_format_1_852b1843() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const Component = branch/*::     <Props, ExternalProps> */(\n  ({ src }) => !src,\n  // $FlowFixMe\n  renderNothing,\n)(BaseComponent);\n\nconst C = b/*:: <A> */(foo) + c/*:: <B> */(bar);\n\nfoo/*::<bar>*/(baz);\n\nfoo/*::<bar>*/();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const Component = branch<Props, ExternalProps>(\n  ({ src }) => !src,\n  // $FlowFixMe\n  renderNothing,\n)(BaseComponent);\n\nconst C = b<A>(foo) + c<B>(bar);\n\nfoo<bar>(baz);\n\nfoo<bar>();");
}
#[test]
fn test_interface_js_format_1_7a96c874() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("interface Foo {\n  bar(\n    currentRequest: {a: number},\n    // TODO this is a very very very very long comment that makes it go > 80 columns\n  ): number;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "interface Foo {\n  bar(\n    currentRequest: { a: number },\n    // TODO this is a very very very very long comment that makes it go > 80 columns\n  ): number;\n}");
}
#[test]
fn test_issues_js_format_1_008fcf60() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// Adding a comment stops the pretty printing process and everything is\n// squished in a single line afterward\nexport type BuckWebSocketMessage = {\n  // Not actually from Buck - this is to let the receiver know that the socket is connected.\n  type: 'SocketConnected',\n} | {\n  type: 'BuildProgressUpdated',\n  progressValue: number,\n} | {\n  type: 'BuildFinished',\n  exitCode: number,\n} | {\n  type: 'BuildStarted',\n} | {\n  type: 'ParseStarted',\n} | {\n  type: 'ParseFinished',\n} | {\n  type: 'RunStarted',\n} | {\n  type: 'RunComplete',\n};\n\n// Two extra levels of indentation because of the comment\nexport type AsyncExecuteOptions = child_process$execFileOpts & {\n  // The contents to write to stdin.\n  stdin?: ?string,\n  dontLogInNuclide?: ?boolean,\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// Adding a comment stops the pretty printing process and everything is\n// squished in a single line afterward\nexport type BuckWebSocketMessage =\n  | {\n      // Not actually from Buck - this is to let the receiver know that the socket is connected.\n      type: \"SocketConnected\",\n    }\n  | {\n      type: \"BuildProgressUpdated\",\n      progressValue: number,\n    }\n  | {\n      type: \"BuildFinished\",\n      exitCode: number,\n    }\n  | {\n      type: \"BuildStarted\",\n    }\n  | {\n      type: \"ParseStarted\",\n    }\n  | {\n      type: \"ParseFinished\",\n    }\n  | {\n      type: \"RunStarted\",\n    }\n  | {\n      type: \"RunComplete\",\n    };\n\n// Two extra levels of indentation because of the comment\nexport type AsyncExecuteOptions = child_process$execFileOpts & {\n  // The contents to write to stdin.\n  stdin?: ?string,\n  dontLogInNuclide?: ?boolean,\n};");
}
#[test]
fn test_let_js_format_1_f56e014e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("let foo /*: Groups<T> */;\nlet bar /*: string */ = 'a';");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "let foo: Groups<T>;\nlet bar: string = \"a\";");
}
#[test]
fn test_object_type_annotation_js_format_1_6e3afe7a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type Props1 = // (DispatchProps & StateProps); WHY DON'T YOU WORK FLOW!!!!!!!!!\n{\n  isPlaying: boolean,\n};\n\ntype Props2 = { // (DispatchProps & StateProps); WHY DON'T YOU WORK FLOW!!!!!!!!!\n  isPlaying: boolean\n};\n\ntype Props3 = {\n  // (DispatchProps & StateProps); WHY DON'T YOU WORK FLOW!!!!!!!!!\n  isPlaying: boolean\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type Props1 =\n  // (DispatchProps & StateProps); WHY DON'T YOU WORK FLOW!!!!!!!!!\n  {\n    isPlaying: boolean,\n  };\n\ntype Props2 = {\n  // (DispatchProps & StateProps); WHY DON'T YOU WORK FLOW!!!!!!!!!\n  isPlaying: boolean,\n};\n\ntype Props3 = {\n  // (DispatchProps & StateProps); WHY DON'T YOU WORK FLOW!!!!!!!!!\n  isPlaying: boolean,\n};");
}
#[test]
fn test_type_annotations_js_format_1_fd4bdcfd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("new (window.Request/*: Class<Request> */)(\"\");\n\n(this/*: any */).foo = 5;\n\n(this/*    : any */).foo = 5;\n\n// This next example illustrates that Prettier will not remove a line break\n// and unintentionally create a type annotation that was not there before.\n(this/*\n: any */).foo = 5;\n\nconst x = (input /*: string */) /*: string */ => {};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "new (window.Request: Class<Request>)(\"\");\n\n(this: any).foo = 5;\n\n(this: any).foo = 5;\n\n// This next example illustrates that Prettier will not remove a line break\n// and unintentionally create a type annotation that was not there before.\nthis /*\n: any */.foo = 5;\n\nconst x = (input: string): string => {};");
}
#[test]
fn test_type_annotations_2_js_format_1_08797b9e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("foo/*::<bar>*/(baz);\nclass Foo {\n  bar( data: Array<string>) {}\n");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "foo<bar>(baz);\nclass Foo {\n  bar(data: Array<string>) {}\n}"
    );
}
#[test]
fn test_type_annotations_3_js_format_1_78fd4f58() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import type { OneType } from './oneFile.js'\n/*::\nimport type { HelloType } from './otherFile.js'\n*/\n\ntype PropTypes = {|\n  TestComponent: React.AbstractComponent<any>,\n  hello: HelloType,\n|}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import type { OneType } from \"./oneFile.js\";\nimport type { HelloType } from \"./otherFile.js\";\ntype PropTypes = {|\n  TestComponent: React.AbstractComponent<any>,\n  hello: HelloType,\n|};");
}
#[test]
fn test_union_js_format_1_78e1e38f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type UploadState<E, EM, D>\n  // The upload hasnt begun yet\n  = {type: \"Not_begun\"}\n  // The upload timed out\n  | {type: \"Timed_out\"}\n  // Failed somewhere on the line\n  | {type: \"Failed\", error: E, errorMsg: EM}\n  // Uploading to aws3 and CreatePostMutation succeeded\n  | {type: \"Success\", data: D};\n\ntype UploadState2<E, EM, D>\n  // The upload hasnt begun yet\n  = A\n  // The upload timed out\n  | B\n  // Failed somewhere on the line\n  | C\n  // Uploading to aws3 and CreatePostMutation succeeded\n  | D;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type UploadState<E, EM, D> =\n  // The upload hasnt begun yet\n  | { type: \"Not_begun\" }\n  // The upload timed out\n  | { type: \"Timed_out\" }\n  // Failed somewhere on the line\n  | { type: \"Failed\", error: E, errorMsg: EM }\n  // Uploading to aws3 and CreatePostMutation succeeded\n  | { type: \"Success\", data: D };\n\ntype UploadState2<E, EM, D> =\n  // The upload hasnt begun yet\n  | A\n  // The upload timed out\n  | B\n  // Failed somewhere on the line\n  | C\n  // Uploading to aws3 and CreatePostMutation succeeded\n  | D;");
}
