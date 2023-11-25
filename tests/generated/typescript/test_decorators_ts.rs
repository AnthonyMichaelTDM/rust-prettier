#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_accessor_decorator_ts_format_1_3fb672a3() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class Point {\n  private _x: number;\n  private _y: number;\n  constructor(x: number, y: number) {\n    this._x = x;\n    this._y = y;\n  }\n\n  @configurable(false)\n  get x() {\n    return this._x;\n  }\n\n  @configurable(false)\n  get y() {\n    return this._y;\n  }\n}") ? ;
    assert_eq ! (formatted , "class Point {\n  private _x: number;\n  private _y: number;\n  constructor(x: number, y: number) {\n    this._x = x;\n    this._y = y;\n  }\n\n  @configurable(false)\n  get x() {\n    return this._x;\n  }\n\n  @configurable(false)\n  get y() {\n    return this._y;\n  }\n}");
    Ok(())
}
#[test]
fn test_angular_ts_format_1_c6068b21() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("@Component({\n  selector: 'toh-hero-button',\n  template: `<button>{{label}}</button>`\n})\nexport class HeroButtonComponent {\n  @Output() change = new EventEmitter<any>();\n  @Input() label: string;\n}") ? ;
    assert_eq ! (formatted , "@Component({\n  selector: \"toh-hero-button\",\n  template: `<button>{{ label }}</button>`,\n})\nexport class HeroButtonComponent {\n  @Output() change = new EventEmitter<any>();\n  @Input() label: string;\n}");
    Ok(())
}
#[test]
fn test_class_decorator_ts_format_1_b49ec913() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("@sealed\nclass Greeter {\n  greeting: string;\n  constructor(message: string) {\n    this.greeting = message;\n  }\n  greet() {\n    return \"Hello, \" + this.greeting;\n  }\n}") ? ;
    assert_eq ! (formatted , "@sealed\nclass Greeter {\n  greeting: string;\n  constructor(message: string) {\n    this.greeting = message;\n  }\n  greet() {\n    return \"Hello, \" + this.greeting;\n  }\n}");
    Ok(())
}
#[test]
fn test_method_decorator_ts_format_1_bfc65b8d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class Greeter {\n  greeting: string;\n  constructor(message: string) {\n    this.greeting = message;\n  }\n\n  @enumerable(false)\n  greet() {\n    return \"Hello, \" + this.greeting;\n  }\n}") ? ;
    assert_eq ! (formatted , "class Greeter {\n  greeting: string;\n  constructor(message: string) {\n    this.greeting = message;\n  }\n\n  @enumerable(false)\n  greet() {\n    return \"Hello, \" + this.greeting;\n  }\n}");
    Ok(())
}
#[test]
fn test_mobx_ts_format_1_c0e32ab7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class X {\n\t@deco x() {\n      return this.count * 2;\n\t}\n\t@deco get x() {\n      return this.count * 2;\n\t}\n}") ? ;
    assert_eq ! (formatted , "class X {\n  @deco x() {\n    return this.count * 2;\n  }\n  @deco get x() {\n    return this.count * 2;\n  }\n}");
    Ok(())
}
#[test]
fn test_multiple_ts_format_1_afc7b74b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("class C {\n  @f()\n  @g()\n  method() {}\n}")?;
    assert_eq!(formatted, "class C {\n  @f()\n  @g()\n  method() {}\n}");
    Ok(())
}
#[test]
fn test_parameter_decorator_ts_format_1_7807b4c1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class Greeter {\n  greeting: string;\n\n  constructor(message: string) {\n    this.greeting = message;\n  }\n\n  @validate\n  greet(@required name: string) {\n    return \"Hello \" + name + \", \" + this.greeting;\n  }\n\n  @validate\n  destructured(@required { toString }: Object) {\n    return Function.prototype.toString.apply(toString);\n  }\n}") ? ;
    assert_eq ! (formatted , "class Greeter {\n  greeting: string;\n\n  constructor(message: string) {\n    this.greeting = message;\n  }\n\n  @validate\n  greet(@required name: string) {\n    return \"Hello \" + name + \", \" + this.greeting;\n  }\n\n  @validate\n  destructured(@required { toString }: Object) {\n    return Function.prototype.toString.apply(toString);\n  }\n}");
    Ok(())
}
#[test]
fn test_property_decorator_ts_format_1_5fdeca76() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class Greeter {\n  @format(\"Hello, %s\") greeting: string;\n\n  constructor(message: string) {\n    this.greeting = message;\n  }\n  greet() {\n    let formatString = getFormat(this, \"greeting\");\n    return formatString.replace(\"%s\", this.greeting);\n  }\n}") ? ;
    assert_eq ! (formatted , "class Greeter {\n  @format(\"Hello, %s\") greeting: string;\n\n  constructor(message: string) {\n    this.greeting = message;\n  }\n  greet() {\n    let formatString = getFormat(this, \"greeting\");\n    return formatString.replace(\"%s\", this.greeting);\n  }\n}");
    Ok(())
}
#[test]
fn test_typeorm_ts_format_1_8840be56() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("@Entity()\nexport class Board {\n\n    @PrimaryGeneratedColumn()\n    id: number;\n\n    @Column()\n    slug: string;\n\n    @Column()\n    name: string;\n\n    @Column()\n    theme: string;\n\n    @Column()\n    description: string;\n\n    @OneToMany(type => Topic, topic => topic.board)\n    topics: Topic[]\n\n}") ? ;
    assert_eq ! (formatted , "@Entity()\nexport class Board {\n  @PrimaryGeneratedColumn()\n  id: number;\n\n  @Column()\n  slug: string;\n\n  @Column()\n  name: string;\n\n  @Column()\n  theme: string;\n\n  @Column()\n  description: string;\n\n  @OneToMany((type) => Topic, (topic) => topic.board)\n  topics: Topic[];\n}");
    Ok(())
}
