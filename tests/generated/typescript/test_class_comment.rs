#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_class_implements_ts_format_1_f019f137() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class a1 extends b // comment\nimplements z\n{\n  constructor() {}\n}\n\nclass a2 extends b implements z { // comment\n  constructor() {}\n}\n\nclass a3 extends b\nimplements\n// comment\nz,\ny {\n  constructor() {}\n}\n\nclass a4 extends b\nimplements z, // comment\ny {\n  constructor() {}\n}\n\nclass a5 extends b  implements\n    z, // comment-z\n    y // comment-y\n    {\n  constructor() {}\n}\n\nclass a6 extends b  implements\n// comment-z1\n    z, // comment-z2\n    // comment-y1\n    y // comment-y2\n    {\n  constructor() {}\n}\n\nclass a7 extends b  implements\n// comment-z1\n    z, // comment-z2\n    // comment-y1\n    y // comment-y2\n         // comment-y3\n    {\n      //comment-body\n  constructor() {}\n}\n\nclass a8 extends b // comment-b\n implements\n// comment-z1\n    z, // comment-z2\n    // comment-y1\n    y // comment-y2\n    {\n  constructor() {}\n}\n\nclass a9 extends\n// comment-b1\nb // comment-b2\n// comment-b3\n implements\n// comment-z1\n    z, // comment-z2\n    // comment-y1\n    y // comment-y2\n    {\n  constructor() {}\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class a1\n  extends b // comment\n  implements z\n{\n  constructor() {}\n}\n\nclass a2 extends b implements z {\n  // comment\n  constructor() {}\n}\n\nclass a3\n  extends b\n  // comment\n  implements z, y\n{\n  constructor() {}\n}\n\nclass a4\n  extends b\n  implements\n    z, // comment\n    y\n{\n  constructor() {}\n}\n\nclass a5\n  extends b\n  implements\n    z, // comment-z\n    y\n{\n  // comment-y\n  constructor() {}\n}\n\nclass a6\n  extends b\n  // comment-z1\n  implements\n    z, // comment-z2\n    // comment-y1\n    y\n{\n  // comment-y2\n  constructor() {}\n}\n\nclass a7\n  extends b\n  // comment-z1\n  implements\n    z, // comment-z2\n    // comment-y1\n    y\n{\n  // comment-y2\n  // comment-y3\n  //comment-body\n  constructor() {}\n}\n\nclass a8\n  extends b // comment-b\n  // comment-z1\n  implements\n    z, // comment-z2\n    // comment-y1\n    y\n{\n  // comment-y2\n  constructor() {}\n}\n\nclass a9\n  // comment-b1\n  extends b // comment-b2\n  // comment-b3\n  // comment-z1\n  implements\n    z, // comment-z2\n    // comment-y1\n    y\n{\n  // comment-y2\n  constructor() {}\n}");
}
#[test]
fn test_declare_ts_format_1_58a58e5b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare class a  // 1\n  // extends b   // 2\n  implements z,x // 3\n{\n  doo:boolean\n}\n\ndeclare class A1<T> // 1\n// 2\nextends B<T> // 3\n{}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "declare class a // 1\n  // extends b   // 2\n  implements z, x\n{\n  // 3\n  doo: boolean;\n}\n\ndeclare class A1<T> // 1\n  // 2\n  extends B<T> {\n  // 3\n}");
}
#[test]
fn test_generic_ts_format_1_c630a59d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class G1<T> implements IPoly<T> {\n  x: T;\n}\n\nclass G2 // g2\n<T> implements IPoly<T> {\n  x: T;\n}\n\nclass G3 // g3\n<T> extends U implements IPoly<T> {\n  x: T;\n}\n\nclass G4<T // g4\n> extends U implements IPoly<T> {\n  x: T;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class G1<T> implements IPoly<T> {\n  x: T;\n}\n\nclass G2<T> // g2\n  implements IPoly<T>\n{\n  x: T;\n}\n\nclass G3<T> // g3\n  extends U\n  implements IPoly<T>\n{\n  x: T;\n}\n\nclass G4<\n    T, // g4\n  >\n  extends U\n  implements IPoly<T>\n{\n  x: T;\n}");
}
#[test]
fn test_misc_ts_format_1_43265726() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export class SnapshotLogger {\n  constructor(\n    retentionPeriod: number = 5 * 60 * 1000, // retain past five minutes\n    snapshotInterval: number = 30 * 1000, // snapshot no more than every 30s\n  ) {\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export class SnapshotLogger {\n  constructor(\n    retentionPeriod: number = 5 * 60 * 1000, // retain past five minutes\n    snapshotInterval: number = 30 * 1000, // snapshot no more than every 30s\n  ) {}\n}");
}
