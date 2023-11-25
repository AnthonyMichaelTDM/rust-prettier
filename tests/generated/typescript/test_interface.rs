#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_comments_ts_semifalse_format_1_ed0df114() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("interface ScreenObject {\n\t// I make things weird.\n\tat(point: Point): Screen | undefined;\n}") ? ;
    assert_eq ! (formatted , "interface ScreenObject {\n  // I make things weird.\n  at(point: Point): Screen | undefined\n}");
    Ok(())
}
#[test]
fn test_comments_ts_format_1_ed0df114() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("interface ScreenObject {\n\t// I make things weird.\n\tat(point: Point): Screen | undefined;\n}") ? ;
    assert_eq ! (formatted , "interface ScreenObject {\n  // I make things weird.\n  at(point: Point): Screen | undefined;\n}");
    Ok(())
}
#[test]
fn test_comments_generic_ts_semifalse_format_1_a480cec7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("interface ReallyReallyLongName<\n  TypeArgumentNumberOne,\n  TypeArgumentNumberTwo,\n  TypeArgumentNumberThree\n> // 1\nextends BaseInterface {}\n\ninterface ReallyReallyLongName2<\n  TypeArgumentNumberOne,\n  TypeArgumentNumberTwo,\n  TypeArgumentNumberThree\n> // 1\n// 2\nextends BaseInterface {}\n\ninterface ReallyReallyLongName3<\n  TypeArgumentNumberOne,\n  TypeArgumentNumberTwo,\n  TypeArgumentNumberThree\n> // 1\n// 2\nextends BaseInterface // 3\n{}\n\ninterface Foo<\n  FOOOOOOOOOOOOOOOOOOOOOOOOOO,\n  FOOOOOOOOOOOOOOOOOOOOOOOOOO,\n  FOOOOOOOOOOOOOOOOOOOOOOOOOO\n> // comments\n  extends Foo {}") ? ;
    assert_eq ! (formatted , "interface ReallyReallyLongName<\n    TypeArgumentNumberOne,\n    TypeArgumentNumberTwo,\n    TypeArgumentNumberThree,\n  > // 1\n  extends BaseInterface {}\n\ninterface ReallyReallyLongName2<\n    TypeArgumentNumberOne,\n    TypeArgumentNumberTwo,\n    TypeArgumentNumberThree,\n  > // 1\n  // 2\n  extends BaseInterface {}\n\ninterface ReallyReallyLongName3<\n    TypeArgumentNumberOne,\n    TypeArgumentNumberTwo,\n    TypeArgumentNumberThree,\n  > // 1\n  // 2\n  extends BaseInterface {\n  // 3\n}\n\ninterface Foo<\n    FOOOOOOOOOOOOOOOOOOOOOOOOOO,\n    FOOOOOOOOOOOOOOOOOOOOOOOOOO,\n    FOOOOOOOOOOOOOOOOOOOOOOOOOO,\n  > // comments\n  extends Foo {}");
    Ok(())
}
#[test]
fn test_comments_generic_ts_format_1_a480cec7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("interface ReallyReallyLongName<\n  TypeArgumentNumberOne,\n  TypeArgumentNumberTwo,\n  TypeArgumentNumberThree\n> // 1\nextends BaseInterface {}\n\ninterface ReallyReallyLongName2<\n  TypeArgumentNumberOne,\n  TypeArgumentNumberTwo,\n  TypeArgumentNumberThree\n> // 1\n// 2\nextends BaseInterface {}\n\ninterface ReallyReallyLongName3<\n  TypeArgumentNumberOne,\n  TypeArgumentNumberTwo,\n  TypeArgumentNumberThree\n> // 1\n// 2\nextends BaseInterface // 3\n{}\n\ninterface Foo<\n  FOOOOOOOOOOOOOOOOOOOOOOOOOO,\n  FOOOOOOOOOOOOOOOOOOOOOOOOOO,\n  FOOOOOOOOOOOOOOOOOOOOOOOOOO\n> // comments\n  extends Foo {}") ? ;
    assert_eq ! (formatted , "interface ReallyReallyLongName<\n    TypeArgumentNumberOne,\n    TypeArgumentNumberTwo,\n    TypeArgumentNumberThree,\n  > // 1\n  extends BaseInterface {}\n\ninterface ReallyReallyLongName2<\n    TypeArgumentNumberOne,\n    TypeArgumentNumberTwo,\n    TypeArgumentNumberThree,\n  > // 1\n  // 2\n  extends BaseInterface {}\n\ninterface ReallyReallyLongName3<\n    TypeArgumentNumberOne,\n    TypeArgumentNumberTwo,\n    TypeArgumentNumberThree,\n  > // 1\n  // 2\n  extends BaseInterface {\n  // 3\n}\n\ninterface Foo<\n    FOOOOOOOOOOOOOOOOOOOOOOOOOO,\n    FOOOOOOOOOOOOOOOOOOOOOOOOOO,\n    FOOOOOOOOOOOOOOOOOOOOOOOOOO,\n  > // comments\n  extends Foo {}");
    Ok(())
}
#[test]
fn test_generic_ts_semifalse_format_1_b0342a9a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("interface Foo<FOOOOOOOOOOOOOOOOOOOOOOOOOO,FOOOOOOOOOOOOOOOOOOOOOOO>\n  extends Foo {}\n\ninterface Foo<\n  FOOOOOOOOOOOOOOOOOOOOOOOOOO,\n  FOOOOOOOOOOOOOOOOOOOOOOOOOO,\n  FOOOOOOOOOOOOOOOOOOOOOOOOOO\n> extends Foo {}") ? ;
    assert_eq ! (formatted , "interface Foo<FOOOOOOOOOOOOOOOOOOOOOOOOOO, FOOOOOOOOOOOOOOOOOOOOOOO>\n  extends Foo {}\n\ninterface Foo<\n  FOOOOOOOOOOOOOOOOOOOOOOOOOO,\n  FOOOOOOOOOOOOOOOOOOOOOOOOOO,\n  FOOOOOOOOOOOOOOOOOOOOOOOOOO,\n> extends Foo {}");
    Ok(())
}
#[test]
fn test_generic_ts_format_1_b0342a9a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("interface Foo<FOOOOOOOOOOOOOOOOOOOOOOOOOO,FOOOOOOOOOOOOOOOOOOOOOOO>\n  extends Foo {}\n\ninterface Foo<\n  FOOOOOOOOOOOOOOOOOOOOOOOOOO,\n  FOOOOOOOOOOOOOOOOOOOOOOOOOO,\n  FOOOOOOOOOOOOOOOOOOOOOOOOOO\n> extends Foo {}") ? ;
    assert_eq ! (formatted , "interface Foo<FOOOOOOOOOOOOOOOOOOOOOOOOOO, FOOOOOOOOOOOOOOOOOOOOOOO>\n  extends Foo {}\n\ninterface Foo<\n  FOOOOOOOOOOOOOOOOOOOOOOOOOO,\n  FOOOOOOOOOOOOOOOOOOOOOOOOOO,\n  FOOOOOOOOOOOOOOOOOOOOOOOOOO,\n> extends Foo {}");
    Ok(())
}
#[test]
fn test_ignore_ts_semifalse_format_1_b475b9ee() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("interface Interface {\n  // prettier-ignore\n  prop: type\n  // prettier-ignore\n  prop: type;\n  prop: type;\n}\n\n// Last element\ninterface Interface {\n  // prettier-ignore\n  prop: type\n  prop: type\n}\n\ninterface foo extends bar {\n  // prettier-ignore\n  f(): void;\n  // prettier-ignore\n  g(): void;\n  h(): void;\n}\n\ninterface T<T> {\n  // prettier-ignore\n  new<T>(): T<T>;\n  new<T>(): T<T>;\n}\n\ninterface I {\n  // prettier-ignore\n  x: y;\n}\n\ninterface I {\n  // prettier-ignore\n  x: y,\n}\n\ninterface I {\n  // prettier-ignore\n  x: y\n}\n\ninterface I {\n  // prettier-ignore\n  x: y;\n  y: x\n}\n\ninterface I {\n  // prettier-ignore\n  x: y,\n  y: x\n}\n\ninterface I {\n  // prettier-ignore\n  x: y\n  y: x\n}\n\ninterface I {\n  // prettier-ignore\n  (): void;\n}\n\ninterface I {\n  // prettier-ignore\n  (): void,\n}\n\ninterface I {\n  // prettier-ignore\n  (): void\n}\n\ninterface I {\n  // prettier-ignore\n  foo(): void;\n}\n\ninterface I {\n  // prettier-ignore\n  foo(): void,\n}\n\ninterface I {\n  // prettier-ignore\n  foo(): void\n}\n\ninterface I {\n  // prettier-ignore\n  new ();\n}\n\ninterface I {\n  // prettier-ignore\n  new (),\n}\n\ninterface I {\n  // prettier-ignore\n  new ()\n}") ? ;
    assert_eq ! (formatted , "interface Interface {\n  // prettier-ignore\n  prop: type\n  // prettier-ignore\n  prop: type;\n  prop: type\n}\n\n// Last element\ninterface Interface {\n  // prettier-ignore\n  prop: type\n  prop: type\n}\n\ninterface foo extends bar {\n  // prettier-ignore\n  f(): void;\n  // prettier-ignore\n  g(): void;\n  h(): void\n}\n\ninterface T<T> {\n  // prettier-ignore\n  new<T>(): T<T>;\n  new <T>(): T<T>\n}\n\ninterface I {\n  // prettier-ignore\n  x: y;\n}\n\ninterface I {\n  // prettier-ignore\n  x: y,\n}\n\ninterface I {\n  // prettier-ignore\n  x: y\n}\n\ninterface I {\n  // prettier-ignore\n  x: y;\n  y: x\n}\n\ninterface I {\n  // prettier-ignore\n  x: y,\n  y: x\n}\n\ninterface I {\n  // prettier-ignore\n  x: y\n  y: x\n}\n\ninterface I {\n  // prettier-ignore\n  (): void;\n}\n\ninterface I {\n  // prettier-ignore\n  (): void,\n}\n\ninterface I {\n  // prettier-ignore\n  (): void\n}\n\ninterface I {\n  // prettier-ignore\n  foo(): void;\n}\n\ninterface I {\n  // prettier-ignore\n  foo(): void,\n}\n\ninterface I {\n  // prettier-ignore\n  foo(): void\n}\n\ninterface I {\n  // prettier-ignore\n  new ();\n}\n\ninterface I {\n  // prettier-ignore\n  new (),\n}\n\ninterface I {\n  // prettier-ignore\n  new ()\n}");
    Ok(())
}
#[test]
fn test_ignore_ts_format_1_b475b9ee() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("interface Interface {\n  // prettier-ignore\n  prop: type\n  // prettier-ignore\n  prop: type;\n  prop: type;\n}\n\n// Last element\ninterface Interface {\n  // prettier-ignore\n  prop: type\n  prop: type\n}\n\ninterface foo extends bar {\n  // prettier-ignore\n  f(): void;\n  // prettier-ignore\n  g(): void;\n  h(): void;\n}\n\ninterface T<T> {\n  // prettier-ignore\n  new<T>(): T<T>;\n  new<T>(): T<T>;\n}\n\ninterface I {\n  // prettier-ignore\n  x: y;\n}\n\ninterface I {\n  // prettier-ignore\n  x: y,\n}\n\ninterface I {\n  // prettier-ignore\n  x: y\n}\n\ninterface I {\n  // prettier-ignore\n  x: y;\n  y: x\n}\n\ninterface I {\n  // prettier-ignore\n  x: y,\n  y: x\n}\n\ninterface I {\n  // prettier-ignore\n  x: y\n  y: x\n}\n\ninterface I {\n  // prettier-ignore\n  (): void;\n}\n\ninterface I {\n  // prettier-ignore\n  (): void,\n}\n\ninterface I {\n  // prettier-ignore\n  (): void\n}\n\ninterface I {\n  // prettier-ignore\n  foo(): void;\n}\n\ninterface I {\n  // prettier-ignore\n  foo(): void,\n}\n\ninterface I {\n  // prettier-ignore\n  foo(): void\n}\n\ninterface I {\n  // prettier-ignore\n  new ();\n}\n\ninterface I {\n  // prettier-ignore\n  new (),\n}\n\ninterface I {\n  // prettier-ignore\n  new ()\n}") ? ;
    assert_eq ! (formatted , "interface Interface {\n  // prettier-ignore\n  prop: type\n  // prettier-ignore\n  prop: type;\n  prop: type;\n}\n\n// Last element\ninterface Interface {\n  // prettier-ignore\n  prop: type\n  prop: type;\n}\n\ninterface foo extends bar {\n  // prettier-ignore\n  f(): void;\n  // prettier-ignore\n  g(): void;\n  h(): void;\n}\n\ninterface T<T> {\n  // prettier-ignore\n  new<T>(): T<T>;\n  new <T>(): T<T>;\n}\n\ninterface I {\n  // prettier-ignore\n  x: y;\n}\n\ninterface I {\n  // prettier-ignore\n  x: y,\n}\n\ninterface I {\n  // prettier-ignore\n  x: y\n}\n\ninterface I {\n  // prettier-ignore\n  x: y;\n  y: x;\n}\n\ninterface I {\n  // prettier-ignore\n  x: y,\n  y: x;\n}\n\ninterface I {\n  // prettier-ignore\n  x: y\n  y: x;\n}\n\ninterface I {\n  // prettier-ignore\n  (): void;\n}\n\ninterface I {\n  // prettier-ignore\n  (): void,\n}\n\ninterface I {\n  // prettier-ignore\n  (): void\n}\n\ninterface I {\n  // prettier-ignore\n  foo(): void;\n}\n\ninterface I {\n  // prettier-ignore\n  foo(): void,\n}\n\ninterface I {\n  // prettier-ignore\n  foo(): void\n}\n\ninterface I {\n  // prettier-ignore\n  new ();\n}\n\ninterface I {\n  // prettier-ignore\n  new (),\n}\n\ninterface I {\n  // prettier-ignore\n  new ()\n}");
    Ok(())
}
#[test]
fn test_long_extends_ts_semifalse_format_1_e441be6c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export interface I extends A, B, C {\n  c: string;\n}\n\nexport interface ThirdVeryLongAndBoringInterfaceName extends ALongAndBoringInterfaceName {\n  c: string;\n}\n\nexport interface ThirdVeryLongAndBoringInterfaceName extends ALongAndBoringInterfaceName, AnotherLongAndBoringInterfaceName {\n  c: string;\n}\n\nexport interface ThirdVeryLongAndBoringInterfaceName extends AVeryLongAndBoringInterfaceName, AnotherVeryLongAndBoringInterfaceName {\n  c: string;\n}\n\nexport interface ThirdVeryLongAndBoringInterfaceName extends A_AVeryLongAndBoringInterfaceName, B_AVeryLongAndBoringInterfaceName, C_AVeryLongAndBoringInterfaceName  {\n  c: string;\n}") ? ;
    assert_eq ! (formatted , "export interface I extends A, B, C {\n  c: string\n}\n\nexport interface ThirdVeryLongAndBoringInterfaceName\n  extends ALongAndBoringInterfaceName {\n  c: string\n}\n\nexport interface ThirdVeryLongAndBoringInterfaceName\n  extends ALongAndBoringInterfaceName,\n    AnotherLongAndBoringInterfaceName {\n  c: string\n}\n\nexport interface ThirdVeryLongAndBoringInterfaceName\n  extends AVeryLongAndBoringInterfaceName,\n    AnotherVeryLongAndBoringInterfaceName {\n  c: string\n}\n\nexport interface ThirdVeryLongAndBoringInterfaceName\n  extends A_AVeryLongAndBoringInterfaceName,\n    B_AVeryLongAndBoringInterfaceName,\n    C_AVeryLongAndBoringInterfaceName {\n  c: string\n}");
    Ok(())
}
#[test]
fn test_long_extends_ts_format_1_e441be6c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export interface I extends A, B, C {\n  c: string;\n}\n\nexport interface ThirdVeryLongAndBoringInterfaceName extends ALongAndBoringInterfaceName {\n  c: string;\n}\n\nexport interface ThirdVeryLongAndBoringInterfaceName extends ALongAndBoringInterfaceName, AnotherLongAndBoringInterfaceName {\n  c: string;\n}\n\nexport interface ThirdVeryLongAndBoringInterfaceName extends AVeryLongAndBoringInterfaceName, AnotherVeryLongAndBoringInterfaceName {\n  c: string;\n}\n\nexport interface ThirdVeryLongAndBoringInterfaceName extends A_AVeryLongAndBoringInterfaceName, B_AVeryLongAndBoringInterfaceName, C_AVeryLongAndBoringInterfaceName  {\n  c: string;\n}") ? ;
    assert_eq ! (formatted , "export interface I extends A, B, C {\n  c: string;\n}\n\nexport interface ThirdVeryLongAndBoringInterfaceName\n  extends ALongAndBoringInterfaceName {\n  c: string;\n}\n\nexport interface ThirdVeryLongAndBoringInterfaceName\n  extends ALongAndBoringInterfaceName,\n    AnotherLongAndBoringInterfaceName {\n  c: string;\n}\n\nexport interface ThirdVeryLongAndBoringInterfaceName\n  extends AVeryLongAndBoringInterfaceName,\n    AnotherVeryLongAndBoringInterfaceName {\n  c: string;\n}\n\nexport interface ThirdVeryLongAndBoringInterfaceName\n  extends A_AVeryLongAndBoringInterfaceName,\n    B_AVeryLongAndBoringInterfaceName,\n    C_AVeryLongAndBoringInterfaceName {\n  c: string;\n}");
    Ok(())
}
#[test]
fn test_pattern_parameters_ts_semifalse_format_1_b2376cee() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("interface B {\n  foo([]?): void;\n  bar({}, []?): any;\n  baz(a: string, b: number, []?): void;\n}") ? ;
    assert_eq ! (formatted , "interface B {\n  foo([]?): void\n  bar({}, []?): any\n  baz(a: string, b: number, []?): void\n}");
    Ok(())
}
#[test]
fn test_pattern_parameters_ts_format_1_b2376cee() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("interface B {\n  foo([]?): void;\n  bar({}, []?): any;\n  baz(a: string, b: number, []?): void;\n}") ? ;
    assert_eq ! (formatted , "interface B {\n  foo([]?): void;\n  bar({}, []?): any;\n  baz(a: string, b: number, []?): void;\n}");
    Ok(())
}
#[test]
fn test_separator_ts_semifalse_format_1_995f1dfa() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare module 'selenium-webdriver' {\n  export const until: {\n    ableToSwitchToFrame(frame: number | WebElement | By): Condition<boolean>;\n    alertIsPresent(): Condition<Alert>;\n  };\n}\n\nexport interface Edge {\n  cursor: {};\n  node: {\n    id: {};\n  };\n}\n\ninterface Test { one: string, two: any[] }") ? ;
    assert_eq ! (formatted , "declare module \"selenium-webdriver\" {\n  export const until: {\n    ableToSwitchToFrame(frame: number | WebElement | By): Condition<boolean>\n    alertIsPresent(): Condition<Alert>\n  }\n}\n\nexport interface Edge {\n  cursor: {}\n  node: {\n    id: {}\n  }\n}\n\ninterface Test {\n  one: string\n  two: any[]\n}");
    Ok(())
}
#[test]
fn test_separator_ts_format_1_995f1dfa() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare module 'selenium-webdriver' {\n  export const until: {\n    ableToSwitchToFrame(frame: number | WebElement | By): Condition<boolean>;\n    alertIsPresent(): Condition<Alert>;\n  };\n}\n\nexport interface Edge {\n  cursor: {};\n  node: {\n    id: {};\n  };\n}\n\ninterface Test { one: string, two: any[] }") ? ;
    assert_eq ! (formatted , "declare module \"selenium-webdriver\" {\n  export const until: {\n    ableToSwitchToFrame(frame: number | WebElement | By): Condition<boolean>;\n    alertIsPresent(): Condition<Alert>;\n  };\n}\n\nexport interface Edge {\n  cursor: {};\n  node: {\n    id: {};\n  };\n}\n\ninterface Test {\n  one: string;\n  two: any[];\n}");
    Ok(())
}
