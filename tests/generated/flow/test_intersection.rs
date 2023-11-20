#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_intersection_js_format_1_656629ee() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export type ReallyBigSocketServer = ReallyBigSocketServerInterface & ReallyBigSocketServerStatics;\n\ntype Props = {\n  propA: X\n} & {\n  propB: X\n} & {\n  propC: X\n} & {\n  propD: X\n};\n\ntype Props = {\n  focusedChildren?: React.Children,\n  onClick: () => void,\n  overlayChildren?: React.Children,\n  style?: Object,\n  thumbnail: ImageSource,\n} & FooterProps;\n\ntype DuplexStreamOptions = ReadableStreamOptions & {\n  allowHalfOpen?: boolean,\n  readableObjectMode?: boolean\n};\n\ntype DuplexStreamOptions = {\n  allowHalfOpen?: boolean,\n  readableObjectMode?: boolean\n} & {\n  allowHalfOpen?: boolean,\n  readableObjectMode?: boolean\n};\n\ntype DuplexStreamOptions = ReadableStreamOptions &\n  WritableStreamOptions & {\n    allowHalfOpen?: boolean,\n    readableObjectMode?: boolean\n  };\n\ntype DuplexStreamOptions = ReadableStreamOptions &\n  WritableStreamOptions & {\n    allowHalfOpen?: boolean,\n    readableObjectMode?: boolean\n  } & {\n    allowHalfOpen?: boolean,\n    readableObjectMode?: boolean\n  };") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export type ReallyBigSocketServer = ReallyBigSocketServerInterface &\n  ReallyBigSocketServerStatics;\n\ntype Props = {\n  propA: X,\n} & {\n  propB: X,\n} & {\n  propC: X,\n} & {\n  propD: X,\n};\n\ntype Props = {\n  focusedChildren?: React.Children,\n  onClick: () => void,\n  overlayChildren?: React.Children,\n  style?: Object,\n  thumbnail: ImageSource,\n} & FooterProps;\n\ntype DuplexStreamOptions = ReadableStreamOptions & {\n  allowHalfOpen?: boolean,\n  readableObjectMode?: boolean,\n};\n\ntype DuplexStreamOptions = {\n  allowHalfOpen?: boolean,\n  readableObjectMode?: boolean,\n} & {\n  allowHalfOpen?: boolean,\n  readableObjectMode?: boolean,\n};\n\ntype DuplexStreamOptions = ReadableStreamOptions &\n  WritableStreamOptions & {\n    allowHalfOpen?: boolean,\n    readableObjectMode?: boolean,\n  };\n\ntype DuplexStreamOptions = ReadableStreamOptions &\n  WritableStreamOptions & {\n    allowHalfOpen?: boolean,\n    readableObjectMode?: boolean,\n  } & {\n    allowHalfOpen?: boolean,\n    readableObjectMode?: boolean,\n  };");
}
