#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_issue_2322_ts_format_1_c135436b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export const listAuthorizedSitesForDefaultHandler: ListAuthorizedSitesForHandler = aListAuthorizedSitesForResponse;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export const listAuthorizedSitesForDefaultHandler: ListAuthorizedSitesForHandler =\n  aListAuthorizedSitesForResponse;");
}
#[test]
fn test_issue_2482_ts_format_1_8f4fef30() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export function countriesReceived(countries: Array<Country>): CountryActionType {\n  return {\n    type: ActionTypes.COUNTRIES_RECEIVED,\n    countries: countries,\n  };\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export function countriesReceived(\n  countries: Array<Country>,\n): CountryActionType {\n  return {\n    type: ActionTypes.COUNTRIES_RECEIVED,\n    countries: countries,\n  };\n}");
}
#[test]
fn test_issue_2485_ts_format_1_ae3d3d5a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class x {\n  private readonly rawConfigFromFile$: BehaviorSubject<\n    any\n  > = new BehaviorSubject(notRead);\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class x {\n  private readonly rawConfigFromFile$: BehaviorSubject<any> =\n    new BehaviorSubject(notRead);\n}");
}
#[test]
fn test_issue_3122_ts_format_1_c39b29f1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export const findByDate: Resolver<void, Recipe[], { date: Date }> =\n  (_, { date }, { req } ) => {\n    const repo = req.getRepository(Recipe);\n    return repo.find({ createDate: date });\n  }\n\nexport const findByDate: Resolver<void, Recipe[], { date: Date }> =\n  (_, { date }, { req } ) => Recipe.find({ createDate: date });") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export const findByDate: Resolver<void, Recipe[], { date: Date }> = (\n  _,\n  { date },\n  { req },\n) => {\n  const repo = req.getRepository(Recipe);\n  return repo.find({ createDate: date });\n};\n\nexport const findByDate: Resolver<void, Recipe[], { date: Date }> = (\n  _,\n  { date },\n  { req },\n) => Recipe.find({ createDate: date });");
}
#[test]
fn test_issue_5370_ts_format_1_7c47efc2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const durabilityMetricsSelectable: Immutable.OrderedSet<\n  SomeReportingMetric,\n> = myExperienceSelectable.concat(otherDurabilityMetricsSelectable);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const durabilityMetricsSelectable: Immutable.OrderedSet<SomeReportingMetric> =\n  myExperienceSelectable.concat(otherDurabilityMetricsSelectable);");
}
#[test]
fn test_issue_6783_ts_format_1_919f7389() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export const enviromentProdValues: EnvironmentValues = assign<EnvironmentValues>(\n  {\n    apiURL: '/api',\n  },\n  enviromentBaseValues\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export const enviromentProdValues: EnvironmentValues =\n  assign<EnvironmentValues>(\n    {\n      apiURL: \"/api\",\n    },\n    enviromentBaseValues,\n  );");
}
#[test]
fn test_issue_8619_ts_format_1_39af8bab() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{\n    {\n        {\n            const myLongVariableName: MyLongTypeName | null = myLongFunctionCallHere();\n        }\n    }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "{\n  {\n    {\n      const myLongVariableName: MyLongTypeName | null =\n        myLongFunctionCallHere();\n    }\n  }\n}");
}
#[test]
fn test_issue_9172_ts_format_1_edee03e2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "const firestorePersonallyIdentifiablePaths: Array<Collections.Users.Entity> = somefunc();",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const firestorePersonallyIdentifiablePaths: Array<Collections.Users.Entity> =\n  somefunc();");
}
#[test]
fn test_issue_10846_ts_format_1_e0e40c20() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const foo = call<{\n  prop1: string;\n  prop2: string;\n  prop3: string;\n}>();\n\nexport const CallRecorderContext =\n  createContext<{\n    deleteRecording: (id: string) => void;\n    deleteAll: () => void;\n  } | null>(null);\n\nexport const CallRecorderContext =\n  createContext<{\n    deleteRecording: (id: string) => void;\n    deleteAll: () => void;\n  } | null>(null, \"useless\");\n\nconst foo =\n  call<Foooooo, Foooooo, Foooooo, Foooooo, Foooooo, Foooooo, Foooooo>();\n\nconst foo =\n  call<\n    | Foooooooooooo\n    | Foooooooooooo\n    | Foooooooooooo\n    | Foooooooooooo\n    | Foooooooooooo\n  >();\n\nconst foo =\n  call<\n    Foooooooooooo &\n      Foooooooooooo &\n      Foooooooooooo &\n      Foooooooooooo &\n      Foooooooooooo\n  >();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const foo = call<{\n  prop1: string;\n  prop2: string;\n  prop3: string;\n}>();\n\nexport const CallRecorderContext = createContext<{\n  deleteRecording: (id: string) => void;\n  deleteAll: () => void;\n} | null>(null);\n\nexport const CallRecorderContext = createContext<{\n  deleteRecording: (id: string) => void;\n  deleteAll: () => void;\n} | null>(null, \"useless\");\n\nconst foo = call<\n  Foooooo,\n  Foooooo,\n  Foooooo,\n  Foooooo,\n  Foooooo,\n  Foooooo,\n  Foooooo\n>();\n\nconst foo = call<\n  Foooooooooooo | Foooooooooooo | Foooooooooooo | Foooooooooooo | Foooooooooooo\n>();\n\nconst foo = call<\n  Foooooooooooo & Foooooooooooo & Foooooooooooo & Foooooooooooo & Foooooooooooo\n>();");
}
#[test]
fn test_issue_10848_tsx_format_1_9521f999() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("tsx")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const MyComponent: React.VoidFunctionComponent<MyComponentProps> = ({ x }) => {\n  const a = useA()\n  return <div>x = {x}; a = {a}</div>\n}\n\nconst MyComponent2: React.VoidFunctionComponent<MyComponent2Props> = ({ x, y }) => {\n  const a = useA()\n  return <div>x = {x}; y = {y}; a = {a}</div>\n}\n\nconst MyComponentWithLongName1: React.VoidFunctionComponent<MyComponentWithLongNameProps> = ({ x, y }) => {\n  const a = useA()\n  return <div>x = {x}; y = {y}; a = {a}</div>\n}\n\nconst MyComponentWithLongName2: React.VoidFunctionComponent<MyComponentWithLongNameProps> = ({ x, y, anotherPropWithLongName1, anotherPropWithLongName2, anotherPropWithLongName3, anotherPropWithLongName4 }) => {\n  const a = useA()\n  return <div>x = {x}; y = {y}; a = {a}</div>\n}\n\nconst MyGenericComponent: React.VoidFunctionComponent<MyGenericComponentProps<number>> = ({ x, y }) => {\n  const a = useA()\n  return <div>x = {x}; y = {y}; a = {a}</div>\n}\n\nexport const ExportToExcalidrawPlus: React.FC<{\n  elements: readonly NonDeletedExcalidrawElement[];\n  appState: AppState;\n  onError: (error: Error) => void;\n}> = ({ elements, appState, onError }) => {\n  return null;\n}\n\nconst Query: FunctionComponent<QueryProps> = ({\n    children,\n    type,\n    resource,\n    payload,\n    // Provides an undefined onSuccess just so the key \\`onSuccess\\` is defined\n    // This is used to detect options in useDataProvider\n    options = { onSuccess: undefined },\n}) =>\n    children(\n        useQuery(\n            { type, resource, payload },\n            { ...options, withDeclarativeSideEffectsSupport: true }\n        )\n    );") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const MyComponent: React.VoidFunctionComponent<MyComponentProps> = ({ x }) => {\n  const a = useA();\n  return (\n    <div>\n      x = {x}; a = {a}\n    </div>\n  );\n};\n\nconst MyComponent2: React.VoidFunctionComponent<MyComponent2Props> = ({\n  x,\n  y,\n}) => {\n  const a = useA();\n  return (\n    <div>\n      x = {x}; y = {y}; a = {a}\n    </div>\n  );\n};\n\nconst MyComponentWithLongName1: React.VoidFunctionComponent<\n  MyComponentWithLongNameProps\n> = ({ x, y }) => {\n  const a = useA();\n  return (\n    <div>\n      x = {x}; y = {y}; a = {a}\n    </div>\n  );\n};\n\nconst MyComponentWithLongName2: React.VoidFunctionComponent<\n  MyComponentWithLongNameProps\n> = ({\n  x,\n  y,\n  anotherPropWithLongName1,\n  anotherPropWithLongName2,\n  anotherPropWithLongName3,\n  anotherPropWithLongName4,\n}) => {\n  const a = useA();\n  return (\n    <div>\n      x = {x}; y = {y}; a = {a}\n    </div>\n  );\n};\n\nconst MyGenericComponent: React.VoidFunctionComponent<\n  MyGenericComponentProps<number>\n> = ({ x, y }) => {\n  const a = useA();\n  return (\n    <div>\n      x = {x}; y = {y}; a = {a}\n    </div>\n  );\n};\n\nexport const ExportToExcalidrawPlus: React.FC<{\n  elements: readonly NonDeletedExcalidrawElement[];\n  appState: AppState;\n  onError: (error: Error) => void;\n}> = ({ elements, appState, onError }) => {\n  return null;\n};\n\nconst Query: FunctionComponent<QueryProps> = ({\n  children,\n  type,\n  resource,\n  payload,\n  // Provides an undefined onSuccess just so the key \\`onSuccess\\` is defined\n  // This is used to detect options in useDataProvider\n  options = { onSuccess: undefined },\n}) =>\n  children(\n    useQuery(\n      { type, resource, payload },\n      { ...options, withDeclarativeSideEffectsSupport: true },\n    ),\n  );");
}
#[test]
fn test_issue_10850_ts_format_1_f43d9368() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const map: Map<Function, Map<string | void, { value: UnloadedDescriptor }>> =\n  new Map();\n\nconst map: Map<Function, Condition extends Foo ? FooFooFoo : BarBarBar> =\n  new Map();\n\nconst map: Map<Function, FunctionFunctionFunctionFunctionffFunction> =\n  new Map();\n\nconst map: Map<Function, Foo<S>> = new Map();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const map: Map<\n  Function,\n  Map<string | void, { value: UnloadedDescriptor }>\n> = new Map();\n\nconst map: Map<\n  Function,\n  Condition extends Foo ? FooFooFoo : BarBarBar\n> = new Map();\n\nconst map: Map<Function, FunctionFunctionFunctionFunctionffFunction> =\n  new Map();\n\nconst map: Map<Function, Foo<S>> = new Map();");
}
#[test]
fn test_issue_12413_ts_format_1_f73b95a2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("let emit =\n  defineEmits<{ (event: \"ready\", canvas: HTMLCanvasElement): void; (event:\"resize\",canvas:HTMLCanvasElement):void; }>();\n\nlet abc =\n  func<{a:2,b:3,d:78,e:9,f:8,g:7,h:6,i:5,j:4,k:3,l:2,m:1,n:0,o:9,p:8,q:7,r:6,s:5,t:4,u:3,v:2,w:1,x:0,y:9,z:8}>();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "let emit = defineEmits<{\n  (event: \"ready\", canvas: HTMLCanvasElement): void;\n  (event: \"resize\", canvas: HTMLCanvasElement): void;\n}>();\n\nlet abc = func<{\n  a: 2;\n  b: 3;\n  d: 78;\n  e: 9;\n  f: 8;\n  g: 7;\n  h: 6;\n  i: 5;\n  j: 4;\n  k: 3;\n  l: 2;\n  m: 1;\n  n: 0;\n  o: 9;\n  p: 8;\n  q: 7;\n  r: 6;\n  s: 5;\n  t: 4;\n  u: 3;\n  v: 2;\n  w: 1;\n  x: 0;\n  y: 9;\n  z: 8;\n}>();");
}
#[test]
fn test_lone_arg_ts_format_1_605b0db6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("if (true) {\n  if (condition) {\n    const secondType = sourceCode.getNodeByRangeIndex1234(second.range[0])!\n      .type;\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "if (true) {\n  if (condition) {\n    const secondType = sourceCode.getNodeByRangeIndex1234(\n      second.range[0],\n    )!.type;\n  }\n}");
}
#[test]
fn test_parenthesized_ts_format_1_6adf8032() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://github.com/babel/babel/pull/12933/files\n(<number>x) = null;\n(x!) = null;\n(a as any) = null;\n(a as number) = 42;\n((a as any) as string) = null;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// https://github.com/babel/babel/pull/12933/files\n(<number>x) = null;\nx! = null;\n(a as any) = null;\n(a as number) = 42;\n(a as any as string) = null;");
}
