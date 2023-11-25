#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_arrow_regression_ts_semifalse_format_1_f353599e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const bar = (...varargs:any[]) => {\n  console.log(varargs);\n};\n\nconst foo = (x:string):void => (\n  bar(\n    x,\n    () => {},\n    () => {}\n  )\n);\n\napp.get(\"/\", (req, res): void => {\n  res.send(\"Hello world\");\n});") ? ;
    assert_eq ! (formatted , "const bar = (...varargs: any[]) => {\n  console.log(varargs)\n}\n\nconst foo = (x: string): void =>\n  bar(\n    x,\n    () => {},\n    () => {},\n  )\n\napp.get(\"/\", (req, res): void => {\n  res.send(\"Hello world\")\n})");
    Ok(())
}
#[test]
fn test_arrow_regression_ts_format_1_f353599e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const bar = (...varargs:any[]) => {\n  console.log(varargs);\n};\n\nconst foo = (x:string):void => (\n  bar(\n    x,\n    () => {},\n    () => {}\n  )\n);\n\napp.get(\"/\", (req, res): void => {\n  res.send(\"Hello world\");\n});") ? ;
    assert_eq ! (formatted , "const bar = (...varargs: any[]) => {\n  console.log(varargs);\n};\n\nconst foo = (x: string): void =>\n  bar(\n    x,\n    () => {},\n    () => {},\n  );\n\napp.get(\"/\", (req, res): void => {\n  res.send(\"Hello world\");\n});");
    Ok(())
}
#[test]
fn test_comments_ts_semifalse_format_1_2f46ee2e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const fn1 = () => {\n  return;\n} /* foo */;\n\nconst fn2 = () => {\n  return;\n}\n\n// foo\n;") ? ;
    assert_eq!(
        formatted,
        "const fn1 = () => {\n  return\n} /* foo */\n\nconst fn2 = () => {\n  return\n}\n\n// foo"
    );
    Ok(())
}
#[test]
fn test_comments_ts_format_1_2f46ee2e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const fn1 = () => {\n  return;\n} /* foo */;\n\nconst fn2 = () => {\n  return;\n}\n\n// foo\n;") ? ;
    assert_eq ! (formatted , "const fn1 = () => {\n  return;\n}; /* foo */\n\nconst fn2 = () => {\n  return;\n};\n\n// foo");
    Ok(())
}
#[test]
fn test_issue_6107_curry_ts_semifalse_format_1_b64e2abf() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const getIconEngagementTypeFrom = (engagementTypes: Array<EngagementType>) =>\n  iconEngagementType => engagementTypes.includes(iconEngagementType);\n\nconst getIconEngagementTypeFrom2 =\n  (\n    engagementTypes: Array<EngagementType>,\n    secondArg: Something\n  ) =>\n  iconEngagementType =>\n  engagementTypes.includes(iconEngagementType);\n\nconst getIconEngagementTypeFrom2 =\n  (\n    engagementTypes: Array<EngagementType>,\n    secondArg: Something,\n    thirArg: SomethingElse\n  ) =>\n  iconEngagementType =>\n  engagementTypes.includes(iconEngagementType);") ? ;
    assert_eq ! (formatted , "const getIconEngagementTypeFrom =\n  (engagementTypes: Array<EngagementType>) => (iconEngagementType) =>\n    engagementTypes.includes(iconEngagementType)\n\nconst getIconEngagementTypeFrom2 =\n  (engagementTypes: Array<EngagementType>, secondArg: Something) =>\n  (iconEngagementType) =>\n    engagementTypes.includes(iconEngagementType)\n\nconst getIconEngagementTypeFrom2 =\n  (\n    engagementTypes: Array<EngagementType>,\n    secondArg: Something,\n    thirArg: SomethingElse,\n  ) =>\n  (iconEngagementType) =>\n    engagementTypes.includes(iconEngagementType)");
    Ok(())
}
#[test]
fn test_issue_6107_curry_ts_format_1_b64e2abf() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const getIconEngagementTypeFrom = (engagementTypes: Array<EngagementType>) =>\n  iconEngagementType => engagementTypes.includes(iconEngagementType);\n\nconst getIconEngagementTypeFrom2 =\n  (\n    engagementTypes: Array<EngagementType>,\n    secondArg: Something\n  ) =>\n  iconEngagementType =>\n  engagementTypes.includes(iconEngagementType);\n\nconst getIconEngagementTypeFrom2 =\n  (\n    engagementTypes: Array<EngagementType>,\n    secondArg: Something,\n    thirArg: SomethingElse\n  ) =>\n  iconEngagementType =>\n  engagementTypes.includes(iconEngagementType);") ? ;
    assert_eq ! (formatted , "const getIconEngagementTypeFrom =\n  (engagementTypes: Array<EngagementType>) => (iconEngagementType) =>\n    engagementTypes.includes(iconEngagementType);\n\nconst getIconEngagementTypeFrom2 =\n  (engagementTypes: Array<EngagementType>, secondArg: Something) =>\n  (iconEngagementType) =>\n    engagementTypes.includes(iconEngagementType);\n\nconst getIconEngagementTypeFrom2 =\n  (\n    engagementTypes: Array<EngagementType>,\n    secondArg: Something,\n    thirArg: SomethingElse,\n  ) =>\n  (iconEngagementType) =>\n    engagementTypes.includes(iconEngagementType);");
    Ok(())
}
