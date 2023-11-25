#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_abstract_class_ts_format_1_d53c510f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("abstract class AbstractRule {\n    /**\n     * @deprecated\n     * Failures will be filtered based on \\`tslint:disable\\` comments by tslint.\n     * This method now does nothing.\n     */\n    filterFailures() {}\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "abstract class AbstractRule {\n  /**\n   * @deprecated\n   * Failures will be filtered based on \\`tslint:disable\\` comments by tslint.\n   * This method now does nothing.\n   */\n  filterFailures() {}\n}");
}
#[test]
fn test_abstract_methods_ts_format_1_60fc9244() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("abstract class AbstractFoo {\n  abstract method1(/* comment */ arg: string);\n  abstract method2(\n    /* comment */\n    arg: string\n  );\n  abstract method3(\n    // comment\n    arg: string\n  );\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "abstract class AbstractFoo {\n  abstract method1(/* comment */ arg: string);\n  abstract method2(\n    /* comment */\n    arg: string,\n  );\n  abstract method3(\n    // comment\n    arg: string,\n  );\n}");
}
#[test]
fn test_after_jsx_generic_tsx_format_1_d46197ce() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("tsx")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("let comp = (\n  <>\n    <Component<number> /* comment1 */></Component>\n    <Component<number> foo /* comment2 */></Component>\n    <Component<number> /* comment3 */ bar></Component>\n    <Component<number> foo /* comment4 */ bar></Component>\n\n    <Component<number>\n      // comment5\n    ></Component>\n    <Component<number>\n      foo\n      // comment6\n    ></Component>\n    <Component<number>\n      // comment7\n      foo\n    ></Component>\n    <Component<number>\n      foo\n      // comment8\n      bar\n    ></Component>\n  </>\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "let comp = (\n  <>\n    <Component<number> /* comment1 */></Component>\n    <Component<number> foo /* comment2 */></Component>\n    <Component<number> /* comment3 */ bar></Component>\n    <Component<number> foo /* comment4 */ bar></Component>\n\n    <Component<number>\n    // comment5\n    ></Component>\n    <Component<number>\n      foo\n      // comment6\n    ></Component>\n    <Component<number>\n      // comment7\n      foo\n    ></Component>\n    <Component<number>\n      foo\n      // comment8\n      bar\n    ></Component>\n  </>\n);");
}
#[test]
fn test_declare_function_ts_format_1_213c8218() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare function fn(\n  currentRequest: {a: number},\n  // TODO this is a very very very very long comment that makes it go > 80 columns\n): number;\n\ndeclare function /* foo */ f( /* baz */ a /* taz */) /* bar */") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "declare function fn(\n  currentRequest: { a: number },\n  // TODO this is a very very very very long comment that makes it go > 80 columns\n): number;\n\ndeclare function /* foo */ f(/* baz */ a /* taz */); /* bar */");
}
#[test]
fn test_interface_ts_format_1_a046740f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("interface Foo {\n  bar(\n    currentRequest: {a: number},\n    // TODO this is a very very very very long comment that makes it go > 80 columns\n  ): number;\n\n  (\n    currentRequest: {a: number},\n    // TODO this is a very very very very long comment that makes it go > 80 columns\n  ): number;\n\n  new (\n    currentRequest: {a: number},\n    // TODO this is a very very very very long comment that makes it go > 80 columns\n  ): number;\n\n  foo: {\n    x(\n      currentRequest: {a: number},\n      // TODO this is a very very very very long comment that makes it go > 80 columns\n    ): number;\n\n    y: (\n      currentRequest: {a: number},\n      // TODO this is a very very very very long comment that makes it go > 80 columns\n    ) => number;\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "interface Foo {\n  bar(\n    currentRequest: { a: number },\n    // TODO this is a very very very very long comment that makes it go > 80 columns\n  ): number;\n\n  (\n    currentRequest: { a: number },\n    // TODO this is a very very very very long comment that makes it go > 80 columns\n  ): number;\n\n  new (\n    currentRequest: { a: number },\n    // TODO this is a very very very very long comment that makes it go > 80 columns\n  ): number;\n\n  foo: {\n    x(\n      currentRequest: { a: number },\n      // TODO this is a very very very very long comment that makes it go > 80 columns\n    ): number;\n\n    y: (\n      currentRequest: { a: number },\n      // TODO this is a very very very very long comment that makes it go > 80 columns\n    ) => number;\n  };\n}");
}
#[test]
fn test_issues_ts_format_1_a8ecf1c4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// Adding a comment stops the pretty printing process and everything is\n// squished in a single line afterward\nexport type BuckWebSocketMessage = {\n  // Not actually from Buck - this is to let the receiver know that the socket is connected.\n  type: 'SocketConnected',\n} | {\n  type: 'BuildProgressUpdated',\n  progressValue: number,\n} | {\n  type: 'BuildFinished',\n  exitCode: number,\n} | {\n  type: 'BuildStarted',\n} | {\n  type: 'ParseStarted',\n} | {\n  type: 'ParseFinished',\n} | {\n  type: 'RunStarted',\n} | {\n  type: 'RunComplete',\n};\n\n// Two extra levels of indentation because of the comment\nexport type AsyncExecuteOptions = child_process$execFileOpts & {\n  // The contents to write to stdin.\n  stdin?: string,\n  dontLogInNuclide?: boolean,\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// Adding a comment stops the pretty printing process and everything is\n// squished in a single line afterward\nexport type BuckWebSocketMessage =\n  | {\n      // Not actually from Buck - this is to let the receiver know that the socket is connected.\n      type: \"SocketConnected\";\n    }\n  | {\n      type: \"BuildProgressUpdated\";\n      progressValue: number;\n    }\n  | {\n      type: \"BuildFinished\";\n      exitCode: number;\n    }\n  | {\n      type: \"BuildStarted\";\n    }\n  | {\n      type: \"ParseStarted\";\n    }\n  | {\n      type: \"ParseFinished\";\n    }\n  | {\n      type: \"RunStarted\";\n    }\n  | {\n      type: \"RunComplete\";\n    };\n\n// Two extra levels of indentation because of the comment\nexport type AsyncExecuteOptions = child_process$execFileOpts & {\n  // The contents to write to stdin.\n  stdin?: string;\n  dontLogInNuclide?: boolean;\n};");
}
#[test]
fn test_jsx_tsx_format_1_05b16103() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("tsx")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var example1 = <div>\n\thttps://test\n</div>;\n\nvar example2 = <div>\n\t/*test*/\n</div>;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "var example1 = <div>https://test</div>;\n\nvar example2 = <div>/*test*/</div>;"
    );
}
#[test]
fn test_location_ts_format_1_d88a3da9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function x({\n  x,\n  y,\n}: {\n  // Hello world.\n  x: string,\n  // Yoyo.\n  y: string,\n}) {}\n\nexport interface ApplicationEventData {\n  registerBroadcastReceiver(onReceiveCallback: (\n    context: any /* android.content.Context */,\n    intent: any /* android.content.Intent */\n  ) => void): void;\n}\n\nexport type WrappedFormUtils = {\n  getFieldDecorator(id: string, options?: {\n    /** 子节点的值的属性，如 Checkbox 的是 'checked' */\n    valuePropName?: string;\n    /** 子节点的初始值，类型、可选值均由子节点决定 */\n    initialValue?: any;\n    /** 收集子节点的值的时机 */\n    trigger?: string;\n    /** 可以把 onChange 的参数转化为控件的值，例如 DatePicker 可设为：(date, dateString) => dateString */\n    getValueFromEvent?: (...args: any[]) => any;\n    /** 校验子节点值的时机 */\n    validateTrigger?: string | string[];\n    /** 校验规则，参见 [async-validator](https://github.com/yiminghe/async-validator) */\n    rules?: ValidationRule[];\n    /** 是否和其他控件互斥，特别用于 Radio 单选控件 */\n    exclusive?: boolean;\n  }): (node: React.ReactNode) => React.ReactNode;\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function x({\n  x,\n  y,\n}: {\n  // Hello world.\n  x: string;\n  // Yoyo.\n  y: string;\n}) {}\n\nexport interface ApplicationEventData {\n  registerBroadcastReceiver(\n    onReceiveCallback: (\n      context: any /* android.content.Context */,\n      intent: any /* android.content.Intent */,\n    ) => void,\n  ): void;\n}\n\nexport type WrappedFormUtils = {\n  getFieldDecorator(\n    id: string,\n    options?: {\n      /** 子节点的值的属性，如 Checkbox 的是 'checked' */\n      valuePropName?: string;\n      /** 子节点的初始值，类型、可选值均由子节点决定 */\n      initialValue?: any;\n      /** 收集子节点的值的时机 */\n      trigger?: string;\n      /** 可以把 onChange 的参数转化为控件的值，例如 DatePicker 可设为：(date, dateString) => dateString */\n      getValueFromEvent?: (...args: any[]) => any;\n      /** 校验子节点值的时机 */\n      validateTrigger?: string | string[];\n      /** 校验规则，参见 [async-validator](https://github.com/yiminghe/async-validator) */\n      rules?: ValidationRule[];\n      /** 是否和其他控件互斥，特别用于 Radio 单选控件 */\n      exclusive?: boolean;\n    },\n  ): (node: React.ReactNode) => React.ReactNode;\n};");
}
#[test]
fn test_mapped_types_ts_format_1_2708cb8b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type A = {\n  // commentA\n  [a in A]: string;\n}\n\ntype B = {\n  /* commentB */ [b in B]: string\n}\n\ntype C = {\n  [/* commentC */ c in C]: string\n}\n\ntype D = {\n  [d /* commentD */ in D]: string\n}\n\ntype E = {\n  [e in /* commentE */ E]: string\n}\n\ntype F = {\n  [f in F /* commentF */]: string\n}\n\ntype G = {\n  [g in G] /* commentG */: string\n}\n\ntype H = { /* commentH */ [h in H]: string }\n\ntype I = { [/* commentI */ i in I]: string }\n\ntype J = { [j /* commentJ */ in J]: string }\n\ntype K = { [k in /* commentK */ K]: string }\n\ntype L = { [l in L /* commentL */]: string }\n\ntype M = { [m in M] /* commentG */: string }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type A = {\n  // commentA\n  [a in A]: string;\n};\n\ntype B = {\n  /* commentB */ [b in B]: string;\n};\n\ntype C = {\n  [/* commentC */ c in C]: string;\n};\n\ntype D = {\n  [d /* commentD */ in D]: string;\n};\n\ntype E = {\n  [e in /* commentE */ E]: string;\n};\n\ntype F = {\n  [f in F /* commentF */]: string;\n};\n\ntype G = {\n  [g in G /* commentG */]: string;\n};\n\ntype H = { [/* commentH */ h in H]: string };\n\ntype I = { [/* commentI */ i in I]: string };\n\ntype J = { [j /* commentJ */ in J]: string };\n\ntype K = { [k in /* commentK */ K]: string };\n\ntype L = { [l in L /* commentL */]: string };\n\ntype M = { [m in M /* commentG */]: string };");
}
#[test]
fn test_method_types_ts_format_1_f8c37203() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("interface foo1 {\n  bar3/* foo */ (/* baz */) // bat\n  bar/* foo */ ? /* bar */ (/* baz */) /* bat */;\n  bar2/* foo */ (/* baz */) /* bat */\n}\n\ninterface foo2 {\n  bar/* foo */ ? /* bar */ (bar: /* baz */ string): /* bat */ string;\n}\n\ninterface foo3 {\n  /* foo */ (/* bar */): /* baz */ string;\n}\n\ninterface foo4 {\n  /* foo */ (bar: /* bar */ string): /* baz */ string;\n}\n\ninterface foo5 {\n  /* foo */ new /* bar */  (a: /* baz */ string): /* bat */ string\n}\n\ninterface foo6 {\n  /* foo */ new /* bar */ (/* baz */): /* bat */ string\n}\n\ntype foo7 = /* foo */ (/* bar */) /* baz */ => void\n\ntype foo8 = /* foo */ (a: /* bar */ string) /* baz */ => void\n\nlet foo9: new /* foo */ (/* bar */) /* baz */ => string;\n\nlet foo10: new /* foo */ (a: /* bar */ string) /* baz */ => string;\n\nabstract class Test {\n  abstract foo12 /* foo */ (a: /* bar */ string): /* baz */ void\n\n  abstract foo13 /* foo */ (/* bar */) /* baz */\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "interface foo1 {\n  bar3 /* foo */(/* baz */); // bat\n  bar /* foo */ /* bar */?(/* baz */) /* bat */;\n  bar2 /* foo */(/* baz */) /* bat */;\n}\n\ninterface foo2 {\n  bar /* foo */?(/* bar */ bar: /* baz */ string): /* bat */ string;\n}\n\ninterface foo3 {\n  /* foo */ (/* bar */): /* baz */ string;\n}\n\ninterface foo4 {\n  /* foo */ (bar: /* bar */ string): /* baz */ string;\n}\n\ninterface foo5 {\n  /* foo */ new (/* bar */ a: /* baz */ string): /* bat */ string;\n}\n\ninterface foo6 {\n  /* foo */ new (/* baz */) /* bar */ : /* bat */ string;\n}\n\ntype foo7 = /* foo */ (/* bar */) /* baz */ => void;\n\ntype foo8 = /* foo */ (a: /* bar */ string) /* baz */ => void;\n\nlet foo9: new (/* bar */) /* foo */ /* baz */ => string;\n\nlet foo10: new (/* foo */ a: /* bar */ string) /* baz */ => string;\n\nabstract class Test {\n  abstract foo12 /* foo */(a: /* bar */ string): /* baz */ void;\n\n  abstract foo13 /* foo */(/* bar */); /* baz */\n}");
}
#[test]
fn test_methods_ts_format_1_cdedc3fc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export class Point {\n/**\n * Does something.\n */\n  foo() {}\n\n    /**\n     * Does something else.\n     */\n  bar() {}\n\n                /**\n                 * Does\n                 * something\n                 * much\n                 * better\n                 * than\n                 * the\n                 * rest.\n                 */\n  baz() {}\n\n      /**\n       * Buzz-Fizz.\n       * Note: This is indented too far.\n       */\n      fizzBuzz() {}\n\n      /**\n       * Turns the given string into pig-latin.\n       */\n              pigLatinize(value: string) {\n/**\n * This is a block comment inside of a method.\n */\n              }\n\n  /**\n        * One\n * Two\n   * Three\n* Four \n   */\n  mismatchedIndentation() {}\n\n  inline /* foo*/ (/* bar */) /* baz */ {}\n\n  noBody(/* comment */ arg);\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export class Point {\n  /**\n   * Does something.\n   */\n  foo() {}\n\n  /**\n   * Does something else.\n   */\n  bar() {}\n\n  /**\n   * Does\n   * something\n   * much\n   * better\n   * than\n   * the\n   * rest.\n   */\n  baz() {}\n\n  /**\n   * Buzz-Fizz.\n   * Note: This is indented too far.\n   */\n  fizzBuzz() {}\n\n  /**\n   * Turns the given string into pig-latin.\n   */\n  pigLatinize(value: string) {\n    /**\n     * This is a block comment inside of a method.\n     */\n  }\n\n  /**\n   * One\n   * Two\n   * Three\n   * Four\n   */\n  mismatchedIndentation() {}\n\n  inline /* foo*/(/* bar */) /* baz */ {}\n\n  noBody(/* comment */ arg);\n}");
}
#[test]
fn test_ts_parameter_proerty_ts_format_1_289ee52f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class A {\n  constructor(\n      private readonly paramProp: Type,\n    // comment\n  ) {\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class A {\n  constructor(\n    private readonly paramProp: Type,\n    // comment\n  ) {}\n}");
}
#[test]
fn test_type_literals_ts_format_1_6e3afe7a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type Props1 = // (DispatchProps & StateProps); WHY DON'T YOU WORK FLOW!!!!!!!!!\n{\n  isPlaying: boolean,\n};\n\ntype Props2 = { // (DispatchProps & StateProps); WHY DON'T YOU WORK FLOW!!!!!!!!!\n  isPlaying: boolean\n};\n\ntype Props3 = {\n  // (DispatchProps & StateProps); WHY DON'T YOU WORK FLOW!!!!!!!!!\n  isPlaying: boolean\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type Props1 =\n  // (DispatchProps & StateProps); WHY DON'T YOU WORK FLOW!!!!!!!!!\n  {\n    isPlaying: boolean;\n  };\n\ntype Props2 = {\n  // (DispatchProps & StateProps); WHY DON'T YOU WORK FLOW!!!!!!!!!\n  isPlaying: boolean;\n};\n\ntype Props3 = {\n  // (DispatchProps & StateProps); WHY DON'T YOU WORK FLOW!!!!!!!!!\n  isPlaying: boolean;\n};");
}
#[test]
fn test_type_parameters_ts_format_1_c9a7ebb5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("functionName<A /* A comment */>();\nconst a: T</* comment */> = 1;\nfunctionName</* comment */>();\nfunction foo</* comment */>() {}\ninterface Foo {\n </* comment */>(arg): any;\n}\ntype T = </* comment */>(arg) => any;\n\nfunctionName<\n  A // comment\n>();\nconst a: T<\n  // comment\n> = 1;\nfunctionName<\n  // comment\n>();\nfunction foo<\n  // comment\n>() {}\ninterface Foo {\n <\n  A// comment\n >(arg): any;\n}\ntype T = <\n  // comment\n>(arg) => any;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "functionName<A /* A comment */>();\nconst a: T</* comment */> = 1;\nfunctionName</* comment */>();\nfunction foo</* comment */>() {}\ninterface Foo {\n  </* comment */>(arg): any;\n}\ntype T = </* comment */>(arg) => any;\n\nfunctionName<A>(); // comment\nconst a: T<\n  // comment\n> = 1;\nfunctionName<\n  // comment\n>();\nfunction foo<\n  // comment\n>() {}\ninterface Foo {\n  <\n    A, // comment\n  >(\n    arg,\n  ): any;\n}\ntype T = <\n  // comment\n>(\n  arg,\n) => any;");
}
#[test]
fn test_types_ts_format_1_af342ffc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "(() => {\n  // swallow error and fallback to using directory as path\n}) as string[];",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "(() => {\n  // swallow error and fallback to using directory as path\n}) as string[];"
    );
}
#[test]
fn test_union_ts_format_1_78e1e38f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type UploadState<E, EM, D>\n  // The upload hasnt begun yet\n  = {type: \"Not_begun\"}\n  // The upload timed out\n  | {type: \"Timed_out\"}\n  // Failed somewhere on the line\n  | {type: \"Failed\", error: E, errorMsg: EM}\n  // Uploading to aws3 and CreatePostMutation succeeded\n  | {type: \"Success\", data: D};\n\ntype UploadState2<E, EM, D>\n  // The upload hasnt begun yet\n  = A\n  // The upload timed out\n  | B\n  // Failed somewhere on the line\n  | C\n  // Uploading to aws3 and CreatePostMutation succeeded\n  | D;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type UploadState<E, EM, D> =\n  // The upload hasnt begun yet\n  | { type: \"Not_begun\" }\n  // The upload timed out\n  | { type: \"Timed_out\" }\n  // Failed somewhere on the line\n  | { type: \"Failed\"; error: E; errorMsg: EM }\n  // Uploading to aws3 and CreatePostMutation succeeded\n  | { type: \"Success\"; data: D };\n\ntype UploadState2<E, EM, D> =\n  // The upload hasnt begun yet\n  | A\n  // The upload timed out\n  | B\n  // Failed somewhere on the line\n  | C\n  // Uploading to aws3 and CreatePostMutation succeeded\n  | D;");
}
