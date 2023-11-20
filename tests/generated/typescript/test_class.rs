#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_abstract_method_ts_format_1_956abd4c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class Foo {\n  abstract foo();\n}\n\nabstract class Bar {\n  method() {\n    return class {\n      abstract m();\n    }\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class Foo {\n  abstract foo();\n}\n\nabstract class Bar {\n  method() {\n    return class {\n      abstract m();\n    };\n  }\n}");
}
#[test]
fn test_constructor_ts_format_1_857b14d8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class C {constructor(override a: number) {}}\nclass D {constructor(private a: number) {}}\nclass E {constructor(protected a: number) {}}\nclass F {constructor(public a: number) {}}\nclass G {constructor(readonly a: number) {}}\n\nclass A {\n    'constructor': typeof A\n    static Foo() {\n        return new A()\n    }\n}\n\nclass B {\n  constructor<>() {}\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class C {\n  constructor(override a: number) {}\n}\nclass D {\n  constructor(private a: number) {}\n}\nclass E {\n  constructor(protected a: number) {}\n}\nclass F {\n  constructor(public a: number) {}\n}\nclass G {\n  constructor(readonly a: number) {}\n}\n\nclass A {\n  \"constructor\": typeof A;\n  static Foo() {\n    return new A();\n  }\n}\n\nclass B {\n  constructor<>() {}\n}");
}
#[test]
fn test_declare_readonly_field_initializer_ts_format_1_34fb4eda() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class A {\n  declare readonly bar = \"test\";\n  declare readonly foo = 1;\n  declare readonly baz = a.b;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class A {\n  declare readonly bar = \"test\";\n  declare readonly foo = 1;\n  declare readonly baz = a.b;\n}");
}
#[test]
fn test_declare_readonly_field_initializer_w_annotation_ts_format_1_0047ab51() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class A {\n  declare readonly bar: string = \"test\";\n  declare baz: string = \"test\";\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class A {\n  declare readonly bar: string = \"test\";\n  declare baz: string = \"test\";\n}");
}
#[test]
fn test_dunder_ts_format_1_59621879() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("class F<__T> {}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "class F<__T> {}");
}
#[test]
fn test_duplicates_access_modifier_ts_format_1_632ae58f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["typescript"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class Foo {\n  public public a;\n  private public b;\n  protected private c;\n  public protected d;\n  public protected private e;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "class Foo {\n  public a;\n  private b;\n  protected c;\n  public d;\n  public e;\n}"
    );
}
#[test]
fn test_empty_method_body_ts_format_1_69c2f004() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// #9324\n\nclass foo1 {\n  bar() /* bat */;\n}\n\n// #9367\nclass Test {\n  foo (/* 2 */) /* 3 */;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// #9324\n\nclass foo1 {\n  bar /* bat */();\n}\n\n// #9367\nclass Test {\n  foo /* 3 */(/* 2 */);\n}");
}
#[test]
fn test_extends_implements_ts_format_1_172acf4d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class Class extends AbstractClass implements Interface1, Interface2, Interface3, Interface4 {}\n\nclass ExtendsAbstractClassAndImplementsInterfaces1 extends AbstractClass\n  implements Interface1, Interface2, Interface3, Interface4 {}\n\nclass ExtendsAbstractClassAndImplementsInterfaces2\n  extends AAAAAAAAAAAAAAbstractClass\n  implements Interface1, Interface2, Interface3, Interface4 {}\n\nclass ExtendsAbstractClassAndImplementsInterfaces3\n  extends AAAAAAAAAAAAAAbstractClass\n  implements\n    Interface1,\n    Interface2,\n    Interface3,\n    Interface4,\n    Interface5,\n    Interface6,\n    Interface7,\n    Interface8 {}\n\nclass ExtendsAbstractClassAndImplementsInterfaces4\n  extends AAAAAAAAAAAAAAbstractClass<Type1, Type2, Type3, Type4, Type5, Type6, Type7> {}\n\nclass ExtendsAbstractClassAndImplementsInterfaces5\n  extends AAAAAAAAAAAAAAbstractClass<Type1, Type2, Type3, Type4, Type5, Type6, Type7>\n  implements\n    Interface1,\n    Interface2,\n    Interface3,\n    Interface4,\n    Interface5,\n    Interface6,\n    Interface7,\n    Interface8 {}\n\nclass ImplementsInterfaceAndExtendsAbstractClass1<Foo>\n  extends FOOOOOOOOOOOOOOOOO\n  implements FOOOOOOOOOOOOOOOOO, BARRRRRRRRRR {}\n\nclass Foo<FOOOOOOOOOOOOOOOOOOOOOOOOOOO, FOOOOOOOOOOOOOOOOOOOOOOOOOOO> implements Foo {}\n\nclass ImplementsInterfaceAndExtendsAbstractClass2<\n    TypeArgumentNumberOne,\n    TypeArgumentNumberTwo,\n    TypeArgumentNumberThree\n  >\n   extends FOOOOOOOOOOOOOOOOOO implements BaseInterface {}\n\nclass ImplementsInterfaceClass1<\n    TypeArgumentNumberOne,\n    TypeArgumentNumberTwo,\n    TypeArgumentNumberThree\n  >\n    implements BaseInterface {}\n\nclass ImplementsInterfaceClassWithComments1<\n    TypeArgumentNumberOne,\n    TypeArgumentNumberTwo,\n    TypeArgumentNumberThree\n  > // comments\n    implements BaseInterface {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class Class\n  extends AbstractClass\n  implements Interface1, Interface2, Interface3, Interface4 {}\n\nclass ExtendsAbstractClassAndImplementsInterfaces1\n  extends AbstractClass\n  implements Interface1, Interface2, Interface3, Interface4 {}\n\nclass ExtendsAbstractClassAndImplementsInterfaces2\n  extends AAAAAAAAAAAAAAbstractClass\n  implements Interface1, Interface2, Interface3, Interface4 {}\n\nclass ExtendsAbstractClassAndImplementsInterfaces3\n  extends AAAAAAAAAAAAAAbstractClass\n  implements\n    Interface1,\n    Interface2,\n    Interface3,\n    Interface4,\n    Interface5,\n    Interface6,\n    Interface7,\n    Interface8 {}\n\nclass ExtendsAbstractClassAndImplementsInterfaces4 extends AAAAAAAAAAAAAAbstractClass<\n  Type1,\n  Type2,\n  Type3,\n  Type4,\n  Type5,\n  Type6,\n  Type7\n> {}\n\nclass ExtendsAbstractClassAndImplementsInterfaces5\n  extends AAAAAAAAAAAAAAbstractClass<\n    Type1,\n    Type2,\n    Type3,\n    Type4,\n    Type5,\n    Type6,\n    Type7\n  >\n  implements\n    Interface1,\n    Interface2,\n    Interface3,\n    Interface4,\n    Interface5,\n    Interface6,\n    Interface7,\n    Interface8 {}\n\nclass ImplementsInterfaceAndExtendsAbstractClass1<Foo>\n  extends FOOOOOOOOOOOOOOOOO\n  implements FOOOOOOOOOOOOOOOOO, BARRRRRRRRRR {}\n\nclass Foo<FOOOOOOOOOOOOOOOOOOOOOOOOOOO, FOOOOOOOOOOOOOOOOOOOOOOOOOOO>\n  implements Foo {}\n\nclass ImplementsInterfaceAndExtendsAbstractClass2<\n    TypeArgumentNumberOne,\n    TypeArgumentNumberTwo,\n    TypeArgumentNumberThree,\n  >\n  extends FOOOOOOOOOOOOOOOOOO\n  implements BaseInterface {}\n\nclass ImplementsInterfaceClass1<\n  TypeArgumentNumberOne,\n  TypeArgumentNumberTwo,\n  TypeArgumentNumberThree,\n> implements BaseInterface {}\n\nclass ImplementsInterfaceClassWithComments1<\n    TypeArgumentNumberOne,\n    TypeArgumentNumberTwo,\n    TypeArgumentNumberThree,\n  > // comments\n  implements BaseInterface {}");
}
#[test]
fn test_generics_ts_format_1_4e0ce41f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class A<T> implements Map<T> {}\n\ninterface AudioBufferList {\n\tmBuffers: interop.Reference<any /*AudioBuffer*/>;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class A<T> implements Map<T> {}\n\ninterface AudioBufferList {\n  mBuffers: interop.Reference<any /*AudioBuffer*/>;\n}");
}
#[test]
fn test_methods_ts_format_1_3b5d8142() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class X {\n    optionalMethod?() {}\n}\n\ninterface Iterable<T> {\n  [Symbol.iterator](): Iterator<T>;\n}\n\nexport class Check {\n  private static property = 'test';\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class X {\n  optionalMethod?() {}\n}\n\ninterface Iterable<T> {\n  [Symbol.iterator](): Iterator<T>;\n}\n\nexport class Check {\n  private static property = \"test\";\n}");
}
#[test]
fn test_optional_ts_format_1_98b0b724() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class X {\n  private foo? = undefined;\n  \"a-prop\"?: boolean;\n}\n\nclass A {\n  protected [s]?() {}\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class X {\n  private foo? = undefined;\n  \"a-prop\"?: boolean;\n}\n\nclass A {\n  protected [s]?() {}\n}");
}
#[test]
fn test_parameter_properties_ts_format_1_6d9de4f0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class MyClass {\n  constructor(protected x: number, private y: string) {\n  }\n}\n\n[\n  class {\n    \"constructor\"(protected x: number, private y: string) {\n    }\n  },\n]\n\nclass Mixed {\n  constructor(public a: number, b: unknown) {\n  }\n}\n\nclass OneParameterProperty {\n  constructor(public foobar: boolean) {\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class MyClass {\n  constructor(\n    protected x: number,\n    private y: string,\n  ) {}\n}\n\n[\n  class {\n    constructor(\n      protected x: number,\n      private y: string,\n    ) {}\n  },\n];\n\nclass Mixed {\n  constructor(\n    public a: number,\n    b: unknown,\n  ) {}\n}\n\nclass OneParameterProperty {\n  constructor(public foobar: boolean) {}\n}");
}
#[test]
fn test_quoted_property_ts_format_1_eaff5941() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["typescript"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("class User {\n    \"username\": string;\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "class User {\n  \"username\": string;\n}");
}
#[test]
fn test_standard_private_fields_ts_format_1_9ba3022a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["typescript"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class Square {\n    #sideLength: number;\n    readonly #area: number;\n    #unit?: string;\n\n    constructor(sideLength: number, unit?: string) {\n        this.#sideLength = sideLength;\n        this.#area = this.#sideLength ** 2;\n        if (unit) {\n          this.#unit = unit;\n        }\n    }\n\n    equals(other: any) {\n        return this.#sideLength === other.#sideLength;\n    }\n\n    getArea() {\n      return this.#area + (this.#unit ?? 'px') + '²';\n    }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class Square {\n  #sideLength: number;\n  readonly #area: number;\n  #unit?: string;\n\n  constructor(sideLength: number, unit?: string) {\n    this.#sideLength = sideLength;\n    this.#area = this.#sideLength ** 2;\n    if (unit) {\n      this.#unit = unit;\n    }\n  }\n\n  equals(other: any) {\n    return this.#sideLength === other.#sideLength;\n  }\n\n  getArea() {\n    return this.#area + (this.#unit ?? \"px\") + \"²\";\n  }\n}");
}
