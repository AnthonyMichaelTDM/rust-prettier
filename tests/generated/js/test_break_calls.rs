#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_break_js_format_1_a77fb8c0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["babel", "flow", "typescript"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("h(f(g(() => {\n  a\n})))\n\ndeepCopyAndAsyncMapLeavesA(\n  { source: sourceValue, destination: destination[sourceKey] },\n  { valueMapper, overwriteExistingKeys }\n)\n\ndeepCopyAndAsyncMapLeavesB(\n  1337,\n  { source: sourceValue, destination: destination[sourceKey] },\n  { valueMapper, overwriteExistingKeys }\n)\n\ndeepCopyAndAsyncMapLeavesC(\n  { source: sourceValue, destination: destination[sourceKey] },\n  1337,\n  { valueMapper, overwriteExistingKeys }\n)\n\nfunction someFunction(url) {\n  return get(url)\n    .then(\n      json => dispatch(success(json)),\n      error => dispatch(failed(error))\n    );\n}\n\nconst mapChargeItems = fp.flow(\n  l => l < 10 ? l: 1,\n  l => Immutable.Range(l).toMap()\n);\n\nexpect(new LongLongLongLongLongRange([0, 0], [0, 0])).toEqualAtomLongLongLongLongRange(new LongLongLongRange([0, 0], [0, 0]));\n\n[\"red\", \"white\", \"blue\", \"black\", \"hotpink\", \"rebeccapurple\"].reduce(\n  (allColors, color) => {\n    return allColors.concat(color);\n  },\n  []\n);\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "h(\n  f(\n    g(() => {\n      a;\n    }),\n  ),\n);\n\ndeepCopyAndAsyncMapLeavesA(\n  { source: sourceValue, destination: destination[sourceKey] },\n  { valueMapper, overwriteExistingKeys },\n);\n\ndeepCopyAndAsyncMapLeavesB(\n  1337,\n  { source: sourceValue, destination: destination[sourceKey] },\n  { valueMapper, overwriteExistingKeys },\n);\n\ndeepCopyAndAsyncMapLeavesC(\n  { source: sourceValue, destination: destination[sourceKey] },\n  1337,\n  { valueMapper, overwriteExistingKeys },\n);\n\nfunction someFunction(url) {\n  return get(url).then(\n    (json) => dispatch(success(json)),\n    (error) => dispatch(failed(error)),\n  );\n}\n\nconst mapChargeItems = fp.flow(\n  (l) => (l < 10 ? l : 1),\n  (l) => Immutable.Range(l).toMap(),\n);\n\nexpect(\n  new LongLongLongLongLongRange([0, 0], [0, 0]),\n).toEqualAtomLongLongLongLongRange(new LongLongLongRange([0, 0], [0, 0]));\n\n[\"red\", \"white\", \"blue\", \"black\", \"hotpink\", \"rebeccapurple\"].reduce(\n  (allColors, color) => {\n    return allColors.concat(color);\n  },\n  [],\n);");
}
#[test]
fn test_parent_js_format_1_3fab6729() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("runtimeAgent.getProperties(\n  objectId,\n  false, // ownProperties\n  false, // accessorPropertiesOnly\n  false, // generatePreview\n  (error, properties, internalProperties) => {\n    return 1\n  },\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "runtimeAgent.getProperties(\n  objectId,\n  false, // ownProperties\n  false, // accessorPropertiesOnly\n  false, // generatePreview\n  (error, properties, internalProperties) => {\n    return 1;\n  },\n);");
}
#[test]
fn test_react_js_format_1_29ab3dbe() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function helloWorld() {\n  useEffect(() => {\n    // do something\n  }, [props.value])\n  useEffect(() => {\n    // do something\n  }, [props.value, props.value, props.value, props.value, props.value, props.value, props.value, props.value, props.value, props.value, props.value])\n}\n\nfunction helloWorldWithReact() {\n  React.useEffect(() => {\n    // do something\n  }, [props.value])\n  React.useEffect(() => {\n    // do something\n  }, [props.value, props.value, props.value, props.value, props.value, props.value, props.value, props.value, props.value, props.value, props.value])\n}\n\nfunction MyComponent(props) {\n  useEffect(\n    () => {\n      console.log(\"some code\", props.foo);\n    },\n\n    // We need to disable the eslint warning here,\n    // because of some complicated reason.\n    // eslint-disable line react-hooks/exhaustive-deps\n    []\n  );\n\n  return null;\n}\n\nfunction Comp1() {\n  const { firstName, lastName } = useMemo(\n    () => parseFullName(fullName),\n    [fullName],\n  );\n}\n\nfunction Comp2() {\n  const { firstName, lastName } = useMemo(\n    () => func(),\n    [props.value, props.value, props.value, props.value, props.value, props.value, props.value, props.value, props.value, props.value, props.value]\n  )\n}\n\nfunction Comp3() {\n  const { firstName, lastName } = useMemo(\n    (aaa, bbb, ccc, ddd, eee, fff, ggg, hhh, iii, jjj, kkk) => func(aaa, bbb, ccc, ddd, eee, fff, ggg, hhh, iii, jjj, kkk),\n    [foo, bar, baz]\n  );\n}\n\nfunction Comp4() {\n  const { firstName, lastName } = useMemo(\n    () => foo && bar && baz || baz || foo && baz(foo) + bar(foo) + foo && bar && baz || baz || foo && baz(foo) + bar(foo),\n    [foo, bar, baz]\n  )\n}\n\nfunction Comp5() {\n  const { firstName, lastName } = useMemo(() => func(), [foo]);\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function helloWorld() {\n  useEffect(() => {\n    // do something\n  }, [props.value]);\n  useEffect(() => {\n    // do something\n  }, [\n    props.value,\n    props.value,\n    props.value,\n    props.value,\n    props.value,\n    props.value,\n    props.value,\n    props.value,\n    props.value,\n    props.value,\n    props.value,\n  ]);\n}\n\nfunction helloWorldWithReact() {\n  React.useEffect(() => {\n    // do something\n  }, [props.value]);\n  React.useEffect(() => {\n    // do something\n  }, [\n    props.value,\n    props.value,\n    props.value,\n    props.value,\n    props.value,\n    props.value,\n    props.value,\n    props.value,\n    props.value,\n    props.value,\n    props.value,\n  ]);\n}\n\nfunction MyComponent(props) {\n  useEffect(\n    () => {\n      console.log(\"some code\", props.foo);\n    },\n\n    // We need to disable the eslint warning here,\n    // because of some complicated reason.\n    // eslint-disable line react-hooks/exhaustive-deps\n    [],\n  );\n\n  return null;\n}\n\nfunction Comp1() {\n  const { firstName, lastName } = useMemo(\n    () => parseFullName(fullName),\n    [fullName],\n  );\n}\n\nfunction Comp2() {\n  const { firstName, lastName } = useMemo(\n    () => func(),\n    [\n      props.value,\n      props.value,\n      props.value,\n      props.value,\n      props.value,\n      props.value,\n      props.value,\n      props.value,\n      props.value,\n      props.value,\n      props.value,\n    ],\n  );\n}\n\nfunction Comp3() {\n  const { firstName, lastName } = useMemo(\n    (aaa, bbb, ccc, ddd, eee, fff, ggg, hhh, iii, jjj, kkk) =>\n      func(aaa, bbb, ccc, ddd, eee, fff, ggg, hhh, iii, jjj, kkk),\n    [foo, bar, baz],\n  );\n}\n\nfunction Comp4() {\n  const { firstName, lastName } = useMemo(\n    () =>\n      (foo && bar && baz) ||\n      baz ||\n      (foo && baz(foo) + bar(foo) + foo && bar && baz) ||\n      baz ||\n      (foo && baz(foo) + bar(foo)),\n    [foo, bar, baz],\n  );\n}\n\nfunction Comp5() {\n  const { firstName, lastName } = useMemo(() => func(), [foo]);\n}");
}
#[test]
fn test_reduce_js_format_1_f46932d1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const [ first1 ] = array.reduce(\n  () => [accumulator, element, accumulator, element],\n  [fullName]\n);\n\nconst [ first2 ] = array.reduce(\n  (accumulator, element, ) => [accumulator, element],\n  [fullName]\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const [first1] = array.reduce(\n  () => [accumulator, element, accumulator, element],\n  [fullName],\n);\n\nconst [first2] = array.reduce(\n  (accumulator, element) => [accumulator, element],\n  [fullName],\n);");
}
