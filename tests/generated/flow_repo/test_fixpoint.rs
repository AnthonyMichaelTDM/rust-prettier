#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_fun_js_format_1_ec9623a7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\n/* @providesModule Fun */\n\nfunction eq(x:number,y:number) { return true };\nfunction sub(x:number,y:number) { return 0; }\nfunction mul(x:number,y:number) { return 0; }\n\nfunction fix(fold) {\n  var delta = function(delta) {\n    return fold(\n      function(x) { var eta = delta(delta); return eta(x); }\n    );\n  };\n  return delta(delta);\n}\n\nfunction mk_factorial() {\n  return fix(function(factorial) {\n    return function(n) {\n      if (eq (n, 1)) { return 1; }\n      return mul (factorial (sub (n, 1)), n);\n    };\n  });\n}\n\n\nvar factorial = mk_factorial();\nfactorial(\"...\");\n\nmodule.exports = {fn: fix};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @providesModule Fun */\n\nfunction eq(x: number, y: number) {\n  return true;\n}\nfunction sub(x: number, y: number) {\n  return 0;\n}\nfunction mul(x: number, y: number) {\n  return 0;\n}\n\nfunction fix(fold) {\n  var delta = function (delta) {\n    return fold(function (x) {\n      var eta = delta(delta);\n      return eta(x);\n    });\n  };\n  return delta(delta);\n}\n\nfunction mk_factorial() {\n  return fix(function (factorial) {\n    return function (n) {\n      if (eq(n, 1)) {\n        return 1;\n      }\n      return mul(factorial(sub(n, 1)), n);\n    };\n  });\n}\n\nvar factorial = mk_factorial();\nfactorial(\"...\");\n\nmodule.exports = { fn: fix };");
}
#[test]
fn test_ycombinator_js_format_1_fd37ef31() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\n/* @providesModule Ycombinator */\n\nfunction Y(f) {\n  function g(x) { return f(x(x)); }\n  g(g);\n}\n\nfunction func1(f) {\n  function fix_f(x:number):number { return f(x); }\n  return fix_f;\n}\nfunction func2(f) {\n  function fix_f(x:string):string { return f(x); }\n  return fix_f;\n}\n\nY(func1);\nY(func2);\n\nmodule.exports = Y;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @providesModule Ycombinator */\n\nfunction Y(f) {\n  function g(x) {\n    return f(x(x));\n  }\n  g(g);\n}\n\nfunction func1(f) {\n  function fix_f(x: number): number {\n    return f(x);\n  }\n  return fix_f;\n}\nfunction func2(f) {\n  function fix_f(x: string): string {\n    return f(x);\n  }\n  return fix_f;\n}\n\nY(func1);\nY(func2);\n\nmodule.exports = Y;");
}
