#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_binaryish_js_format_1_74bdbd65() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const computedDescriptionLines = (showConfirm &&\n  descriptionLinesConfirming) ||\n  (focused && !loading && descriptionLinesFocused) ||\n  descriptionLines;\n\ncomputedDescriptionLines = (focused &&\n  !loading &&\n  descriptionLinesFocused) ||\n  descriptionLines;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const computedDescriptionLines =\n  (showConfirm && descriptionLinesConfirming) ||\n  (focused && !loading && descriptionLinesFocused) ||\n  descriptionLines;\n\ncomputedDescriptionLines =\n  (focused && !loading && descriptionLinesFocused) || descriptionLines;");
}
#[test]
fn test_call_with_template_js_format_1_81cc5902() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const result = template(\\`\n  if (SOME_VAR === \"\") {}\n\\`)({\n  SOME_VAR: value,\n});\n\nconst output =\n template(\\`function f() %%A%%\\`)({\n   A: t.blockStatement([]),\n });") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const result = template(\\`\n  if (SOME_VAR === \"\") {}\n\\`)({\n  SOME_VAR: value,\n});\n\nconst output = template(\\`function f() %%A%%\\`)({\n  A: t.blockStatement([]),\n});");
}
#[test]
fn test_chain_js_format_1_c1b46d45() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("let bifornCringerMoshedPerplexSawder=\naskTrovenaBeenaDependsRowans=\nglimseGlyphsHazardNoopsTieTie=\naverredBathersBoxroomBuggyNurl=\nanodyneCondosMalateOverateRetinol=\nannularCooeedSplicesWalksWayWay=\nkochabCooieGameOnOboleUnweave;\n\nbifornCringerMoshedPerplexSawder =\n  askTrovenaBeenaDependsRowans =\n  glimseGlyphsHazardNoopsTieTie =\n  x =\n  averredBathersBoxroomBuggyNurl =\n  anodyneCondosMal(sdsadsa,dasdas,asd(()=>sdf)).ateOverateRetinol =\n  annularCooeedSplicesWalksWayWay =\n    kochabCooieGameOnOboleUnweave;\n\nbifornCringerMoshedPerplexSawder =\n  askTrovenaBeenaDependsRowans =\n  glimseGlyphsHazardNoopsTieTie =\n  x =\n  averredBathersBoxroomBuggyNurl =\n  anodyneCondosMal(sdsadsa,dasdas,asd(()=>sdf)).ateOverateRetinol =\n  annularCooeedSplicesWalksWayWay =\n    kochabCooieGameOnOboleUnweave+kochabCooieGameOnOboleUnweave;\n\na=b=c;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "let bifornCringerMoshedPerplexSawder =\n  (askTrovenaBeenaDependsRowans =\n  glimseGlyphsHazardNoopsTieTie =\n  averredBathersBoxroomBuggyNurl =\n  anodyneCondosMalateOverateRetinol =\n  annularCooeedSplicesWalksWayWay =\n    kochabCooieGameOnOboleUnweave);\n\nbifornCringerMoshedPerplexSawder =\n  askTrovenaBeenaDependsRowans =\n  glimseGlyphsHazardNoopsTieTie =\n  x =\n  averredBathersBoxroomBuggyNurl =\n  anodyneCondosMal(\n    sdsadsa,\n    dasdas,\n    asd(() => sdf),\n  ).ateOverateRetinol =\n  annularCooeedSplicesWalksWayWay =\n    kochabCooieGameOnOboleUnweave;\n\nbifornCringerMoshedPerplexSawder =\n  askTrovenaBeenaDependsRowans =\n  glimseGlyphsHazardNoopsTieTie =\n  x =\n  averredBathersBoxroomBuggyNurl =\n  anodyneCondosMal(\n    sdsadsa,\n    dasdas,\n    asd(() => sdf),\n  ).ateOverateRetinol =\n  annularCooeedSplicesWalksWayWay =\n    kochabCooieGameOnOboleUnweave + kochabCooieGameOnOboleUnweave;\n\na = b = c;");
}
#[test]
fn test_chain_two_segments_js_format_1_0ff83fd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("tt.parenR.updateContext = tt.braceR.updateContext = function () {\n  if (this.state.context.length === 1) {\n    return;\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "tt.parenR.updateContext = tt.braceR.updateContext = function () {\n  if (this.state.context.length === 1) {\n    return;\n  }\n};");
}
#[test]
fn test_destructuring_js_format_1_b61abff9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("let {\n  bottom: offsetBottom,\n  left: offsetLeft,\n  right: offsetRight,\n  top: offsetTop,\n} = getPressRectOffset == null ? DEFAULT_PRESS_RECT : getPressRectOffset();\n\nconst { accessibilityModule: FooAccessibilityModule, accessibilityModule: FooAccessibilityModule2, accessibilityModule: FooAccessibilityModule3, accessibilityModule: FooAccessibilityModule4,\n      } = foo || {};\n\n({ prop: toAssign = \"default\" } = { prop: \"propval\" });") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "let {\n  bottom: offsetBottom,\n  left: offsetLeft,\n  right: offsetRight,\n  top: offsetTop,\n} = getPressRectOffset == null ? DEFAULT_PRESS_RECT : getPressRectOffset();\n\nconst {\n  accessibilityModule: FooAccessibilityModule,\n  accessibilityModule: FooAccessibilityModule2,\n  accessibilityModule: FooAccessibilityModule3,\n  accessibilityModule: FooAccessibilityModule4,\n} = foo || {};\n\n({ prop: toAssign = \"default\" } = { prop: \"propval\" });");
}
#[test]
fn test_destructuring_array_js_format_1_4dee8726() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const [\n  width= nextWidth,\n  height= nextHeight,\n  baseline= nextBaseline,\n] = measureText(nextText, getFontString(element));") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const [width = nextWidth, height = nextHeight, baseline = nextBaseline] =\n  measureText(nextText, getFontString(element));");
}
#[test]
fn test_destructuring_heuristic_js_format_1_3da8afd5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{{\n  const {\n    id,\n    static: isStatic,\n    method: isMethod,\n    methodId,\n    getId,\n    setId,\n  } = privateNamesMap.get(name);\n\n  const {\n    id1,\n    method: isMethod1,\n    methodId1\n  } = privateNamesMap.get(name);\n\n  const {\n    id2,\n    method: isMethod2,\n    methodId2\n  } = privateNamesMap.get(bifornCringerMoshedPerplexSawder);\n\n  const {\n    id3,\n    method: isMethod3,\n    methodId3\n  } = anodyneCondosMalateOverateRetinol.get(bifornCringerMoshedPerplexSawder);\n}}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "{\n  {\n    const {\n      id,\n      static: isStatic,\n      method: isMethod,\n      methodId,\n      getId,\n      setId,\n    } = privateNamesMap.get(name);\n\n    const { id1, method: isMethod1, methodId1 } = privateNamesMap.get(name);\n\n    const {\n      id2,\n      method: isMethod2,\n      methodId2,\n    } = privateNamesMap.get(bifornCringerMoshedPerplexSawder);\n\n    const {\n      id3,\n      method: isMethod3,\n      methodId3,\n    } = anodyneCondosMalateOverateRetinol.get(bifornCringerMoshedPerplexSawder);\n  }\n}");
}
#[test]
fn test_discussion_15196_js_format_1_f816fa2f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("async function f() {\n  const { section, rubric, authors, tags } = await utils.upsertCommonData(mainData);\n\n  const loooooooooooooooooooooooooong1 = await looooooooooooooong.looooooooooooooong.loooooong;\n  const loooooooooooooooooooooooooong2 = await looooooooooooooong.looooooooooooooong.loooooong();\n  const loooooooooooooooooooooooooong3 = await looooooooooooooooooooooooooooooooooooooooooooog();\n  const loooooooooooooooooooooooooong4 = !await looooooooooooooong.looooooooooooooong.loooooong;\n  const loooooooooooooooooooooooooong5 = void !!await looooooooooooooong.looooooooooooooong.loooooong;\n\n  const longlonglonglonglonglonglong1 = await new Promise((resolve, reject) => { setTimeout(() => { resolve('foo'); }, 300); })\n  const longlonglonglonglonglonglong2 = await { then(onFulfilled, onRejected) { onFulfilled(1234567890) } };\n}\n\nfunction* g() {\n  const { section, rubric, authors, tags } = yield utils.upsertCommonData(mainData);\n\n  const loooooooooooooooooooooooooong1 = yield looooooooooooooong.looooooooooooooong.loooooong;\n  const loooooooooooooooooooooooooong2 = yield looooooooooooooong.looooooooooooooong.loooooong();\n  const loooooooooooooooooooooooooong3 = yield looooooooooooooooooooooooooooooooooooooooooooog();\n  const loooooooooooooooooooooooooong4 = !(yield looooooooooooooong.looooooooooooooong.loooooong);\n  const loooooooooooooooooooooooooong5 = void !!(yield looooooooooooooong.looooooooooooooong.loooooong);\n  const loooooooooooooooooooooooooong6 = yield* looooooooooooooong.looooooooooooooong.loooooong;\n\n  const longlonglonglonglonglonglong1 = yield qwertyuiop(asdfghjkl, zxcvbnm, qwertyuiop, asdfghjkl);\n  const longlonglonglonglonglonglong2 = yield { qwertyuiop: 1234567890, asdfghjkl: 1234567890, zxcvbnm: 123456789 };\n\n  const x = yield;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "async function f() {\n  const { section, rubric, authors, tags } =\n    await utils.upsertCommonData(mainData);\n\n  const loooooooooooooooooooooooooong1 =\n    await looooooooooooooong.looooooooooooooong.loooooong;\n  const loooooooooooooooooooooooooong2 =\n    await looooooooooooooong.looooooooooooooong.loooooong();\n  const loooooooooooooooooooooooooong3 =\n    await looooooooooooooooooooooooooooooooooooooooooooog();\n  const loooooooooooooooooooooooooong4 =\n    !(await looooooooooooooong.looooooooooooooong.loooooong);\n  const loooooooooooooooooooooooooong5 =\n    void !!(await looooooooooooooong.looooooooooooooong.loooooong);\n\n  const longlonglonglonglonglonglong1 = await new Promise((resolve, reject) => {\n    setTimeout(() => {\n      resolve(\"foo\");\n    }, 300);\n  });\n  const longlonglonglonglonglonglong2 = await {\n    then(onFulfilled, onRejected) {\n      onFulfilled(1234567890);\n    },\n  };\n}\n\nfunction* g() {\n  const { section, rubric, authors, tags } =\n    yield utils.upsertCommonData(mainData);\n\n  const loooooooooooooooooooooooooong1 =\n    yield looooooooooooooong.looooooooooooooong.loooooong;\n  const loooooooooooooooooooooooooong2 =\n    yield looooooooooooooong.looooooooooooooong.loooooong();\n  const loooooooooooooooooooooooooong3 =\n    yield looooooooooooooooooooooooooooooooooooooooooooog();\n  const loooooooooooooooooooooooooong4 =\n    !(yield looooooooooooooong.looooooooooooooong.loooooong);\n  const loooooooooooooooooooooooooong5 =\n    void !!(yield looooooooooooooong.looooooooooooooong.loooooong);\n  const loooooooooooooooooooooooooong6 =\n    yield* looooooooooooooong.looooooooooooooong.loooooong;\n\n  const longlonglonglonglonglonglong1 = yield qwertyuiop(\n    asdfghjkl,\n    zxcvbnm,\n    qwertyuiop,\n    asdfghjkl,\n  );\n  const longlonglonglonglonglonglong2 = yield {\n    qwertyuiop: 1234567890,\n    asdfghjkl: 1234567890,\n    zxcvbnm: 123456789,\n  };\n\n  const x = yield;\n}");
}
#[test]
fn test_issue_1419_js_format_1_3df9f1e9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("someReallyLongThingStoredInAMapWithAReallyBigName[pageletID] =\n  _someVariableThatWeAreCheckingForFalsiness\n    ? Date.now() - _someVariableThatWeAreCheckingForFalsiness\n    : 0;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "someReallyLongThingStoredInAMapWithAReallyBigName[pageletID] =\n  _someVariableThatWeAreCheckingForFalsiness\n    ? Date.now() - _someVariableThatWeAreCheckingForFalsiness\n    : 0;");
}
#[test]
fn test_issue_1966_js_format_1_6ffcd05d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const aVeryLongNameThatGoesOnAndOn = this.someOtherObject.someOtherNestedObject.someLongFunctionName();\n\nthis.someObject.someOtherNestedObject = this.someOtherObject.whyNotNestAnotherOne.someLongFunctionName();\n\nthis.isaverylongmethodexpression.withmultiplelevels = this.isanotherverylongexpression.thatisalsoassigned = 0;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const aVeryLongNameThatGoesOnAndOn =\n  this.someOtherObject.someOtherNestedObject.someLongFunctionName();\n\nthis.someObject.someOtherNestedObject =\n  this.someOtherObject.whyNotNestAnotherOne.someLongFunctionName();\n\nthis.isaverylongmethodexpression.withmultiplelevels =\n  this.isanotherverylongexpression.thatisalsoassigned = 0;");
}
#[test]
fn test_issue_2184_js_format_1_aeded06a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const areaPercentageDiff = (\n    topRankedZoneFit.areaPercentageRemaining\n  - previousZoneFitNow.areaPercentageRemaining\n).toFixed(2)") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const areaPercentageDiff = (\n  topRankedZoneFit.areaPercentageRemaining -\n  previousZoneFitNow.areaPercentageRemaining\n).toFixed(2);");
}
#[test]
fn test_issue_2482_1_js_format_1_f32856a3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("aParticularlyLongAndObnoxiousNameForIllustrativePurposes =\n  anotherVeryLongNameForIllustrativePurposes;\n\naParticularlyLongAndObnoxiousNameForIllustrativePurposes =\n  'a very long string for illustrative purposes'.length;\n\naParticularlyLongAndObnoxiousNameForIllustrativePurposes =\n  anotherVeryLongNameForIllustrativePurposes();\n\naParticularlyLongAndObnoxiousNameForIllustrativePurposes =\n  anotherVeryLongNameForIllustrativePurposes.length;\n\naParticularlyLongAndObnoxiousNameForIllustrativePurposes =\n  anotherVeryLongNameForIllustrativePurposes + 1;\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "aParticularlyLongAndObnoxiousNameForIllustrativePurposes =\n  anotherVeryLongNameForIllustrativePurposes;\n\naParticularlyLongAndObnoxiousNameForIllustrativePurposes =\n  \"a very long string for illustrative purposes\".length;\n\naParticularlyLongAndObnoxiousNameForIllustrativePurposes =\n  anotherVeryLongNameForIllustrativePurposes();\n\naParticularlyLongAndObnoxiousNameForIllustrativePurposes =\n  anotherVeryLongNameForIllustrativePurposes.length;\n\naParticularlyLongAndObnoxiousNameForIllustrativePurposes =\n  anotherVeryLongNameForIllustrativePurposes + 1;");
}
#[test]
fn test_issue_2482_2_js_format_1_2383b5fb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class foo {\n    bar() {\n        const median = dates.length % 2\n            ? dates[half].getTime()\n            : (dates[half - 1].getTime() + dates[half].getTime()) / 2.0;\n    }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class foo {\n  bar() {\n    const median =\n      dates.length % 2\n        ? dates[half].getTime()\n        : (dates[half - 1].getTime() + dates[half].getTime()) / 2.0;\n  }\n}");
}
#[test]
fn test_issue_2540_js_format_1_3d7eb43e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("manifestCache[templateId] = readFileSync(\\`\\${MANIFESTS_PATH}/\\${templateId}.json\\`, { encoding: 'utf-8' });") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "manifestCache[templateId] = readFileSync(\n  \\`\\${MANIFESTS_PATH}/\\${templateId}.json\\`,\n  { encoding: \"utf-8\" },\n);");
}
#[test]
fn test_issue_3819_js_format_1_bad674a1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("this.dummy.type1.dummyPropertyFunction\n      = this.dummy.type2.dummyPropertyFunction\n      = this.dummy.type3.dummyPropertyFunction\n      = this.dummy.type4.dummyPropertyFunction\n      = this.dummy.type5.dummyPropertyFunction\n      = this.dummy.type6.dummyPropertyFunction\n      = this.dummy.type7.dummyPropertyFunction\n      = this.dummy.type8.dummyPropertyFunction\n      = () => {\n        return 'dummy';\n      };") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "this.dummy.type1.dummyPropertyFunction =\n  this.dummy.type2.dummyPropertyFunction =\n  this.dummy.type3.dummyPropertyFunction =\n  this.dummy.type4.dummyPropertyFunction =\n  this.dummy.type5.dummyPropertyFunction =\n  this.dummy.type6.dummyPropertyFunction =\n  this.dummy.type7.dummyPropertyFunction =\n  this.dummy.type8.dummyPropertyFunction =\n    () => {\n      return \"dummy\";\n    };");
}
#[test]
fn test_issue_4094_js_format_1_bcc065fd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("if (something) {\n  const otherBrandsWithThisAdjacencyCount123 = Object.values(edge.to.edges).length\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "if (something) {\n  const otherBrandsWithThisAdjacencyCount123 = Object.values(\n    edge.to.edges,\n  ).length;\n}");
}
#[test]
fn test_issue_5610_js_format_1_10fd0cfd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// Function call wrapping is not optimal for readability:\n// Function names tend to get pushed to the right, whereas arguments end up on the left,\n// creating a wide gap that the eyes have to cross in order to read the call.\nconst {qfwvfkwjdqgz, bctsyljqucgz, xuodxhmgwwpw} =\n  qbhtcuzxwedz(yrwimwkjeeiu, njwvozigdkfi, alvvjgkmnmhd);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// Function call wrapping is not optimal for readability:\n// Function names tend to get pushed to the right, whereas arguments end up on the left,\n// creating a wide gap that the eyes have to cross in order to read the call.\nconst { qfwvfkwjdqgz, bctsyljqucgz, xuodxhmgwwpw } = qbhtcuzxwedz(\n  yrwimwkjeeiu,\n  njwvozigdkfi,\n  alvvjgkmnmhd,\n);");
}
#[test]
fn test_issue_6922_js_format_1_530552df() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("async function f() {\n  const { data, status } = await request.delete(\n    \\`/account/\\${accountId}/documents/\\${type}/\\${documentNumber}\\`,\n    { validateStatus: () => true }\n  );\n  return { data, status };\n}\n\nconst data1 = request.delete(\n  '----------------------------------------------',\n  { validateStatus: () => true }\n);\n\nconst data2 = request.delete(\n  '----------------------------------------------x',\n  { validateStatus: () => true }\n);\n\nconst data3 = request.delete(\n  '----------------------------------------------xx',\n  { validateStatus: () => true }\n);\n\nconst data4 = request.delete(\n  '----------------------------------------------xxx',\n  { validateStatus: () => true }\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "async function f() {\n  const { data, status } = await request.delete(\n    \\`/account/\\${accountId}/documents/\\${type}/\\${documentNumber}\\`,\n    { validateStatus: () => true },\n  );\n  return { data, status };\n}\n\nconst data1 = request.delete(\"----------------------------------------------\", {\n  validateStatus: () => true,\n});\n\nconst data2 = request.delete(\n  \"----------------------------------------------x\",\n  { validateStatus: () => true },\n);\n\nconst data3 = request.delete(\n  \"----------------------------------------------xx\",\n  { validateStatus: () => true },\n);\n\nconst data4 = request.delete(\n  \"----------------------------------------------xxx\",\n  { validateStatus: () => true },\n);");
}
#[test]
fn test_issue_7091_js_format_1_5684637b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "const {\n  imStore, showChat, customerServiceAccount\n} = store[config.reduxStoreName]",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "const { imStore, showChat, customerServiceAccount } =\n  store[config.reduxStoreName];"
    );
}
#[test]
fn test_issue_7572_js_format_1_372aa1a0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const t = {\n  \"hello\": world(),\n  'this-is-a-very-long-key-and-the-assignment-should-be-put-on-the-next-line':\n  \torMaybeIAmMisunderstandingAndIHaveSetSomethingWrongInMyConfig(),\n  \"can-someone-explain\": this()\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const t = {\n  hello: world(),\n  \"this-is-a-very-long-key-and-the-assignment-should-be-put-on-the-next-line\":\n    orMaybeIAmMisunderstandingAndIHaveSetSomethingWrongInMyConfig(),\n  \"can-someone-explain\": this(),\n};");
}
#[test]
fn test_issue_7961_js_format_1_81bcea49() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// works as expected\nsomething.veeeeeery.looooooooooooooooooooooooooong = some.other.rather.long.chain;\n\n// does not work if it ends with a function call\nsomething.veeeeeery.looooooooooooooooooooooooooong = some.other.rather.long.chain.functionCall();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// works as expected\nsomething.veeeeeery.looooooooooooooooooooooooooong =\n  some.other.rather.long.chain;\n\n// does not work if it ends with a function call\nsomething.veeeeeery.looooooooooooooooooooooooooong =\n  some.other.rather.long.chain.functionCall();");
}
#[test]
fn test_issue_8218_js_format_1_2b099be3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const pendingIndicators = shield.alarmGeneratorConfiguration.getPendingVersionColumnValues;\n\nconst pendingIndicatorz =\n  shield.alarmGeneratorConfiguration.getPendingVersionColumnValues();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const pendingIndicators =\n  shield.alarmGeneratorConfiguration.getPendingVersionColumnValues;\n\nconst pendingIndicatorz =\n  shield.alarmGeneratorConfiguration.getPendingVersionColumnValues();");
}
#[test]
fn test_issue_10218_js_format_1_a49dee7c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const _id1 = data.createTestMessageWithAReallyLongName.someVeryLongProperty.thisIsAlsoALongProperty._id;\n\nconst {_id2} = data.createTestMessageWithAReallyLongName.someVeryLongProperty.thisIsAlsoALongProperty;\n\nconst {_id:id3} = data.createTestMessageWithAReallyLongName.someVeryLongProperty.thisIsAlsoALongProperty;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const _id1 =\n  data.createTestMessageWithAReallyLongName.someVeryLongProperty\n    .thisIsAlsoALongProperty._id;\n\nconst { _id2 } =\n  data.createTestMessageWithAReallyLongName.someVeryLongProperty\n    .thisIsAlsoALongProperty;\n\nconst { _id: id3 } =\n  data.createTestMessageWithAReallyLongName.someVeryLongProperty\n    .thisIsAlsoALongProperty;");
}
#[test]
fn test_issue_15534_js_format_1_33f190cf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("params[\"redirectTo\"] = \\`\\${window.location.pathname}\\${window.location.search}\\${window.location.hash}\\`;\n\nparams[\"redirectTo\"][\"codePointAt\"][\"name\"] =\n  \\`\\${window.location.pathname}\\${window.location.search}\\${window.location.hash}\\`;\n\nparams.redirectTo.bar.bar.ba.barab[\"foo\"].abr =\n  \\`\\${window.location.pathname}\\${window.location.search}\\${window.location.hash}\\`;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "params[\"redirectTo\"] =\n  \\`\\${window.location.pathname}\\${window.location.search}\\${window.location.hash}\\`;\n\nparams[\"redirectTo\"][\"codePointAt\"][\"name\"] =\n  \\`\\${window.location.pathname}\\${window.location.search}\\${window.location.hash}\\`;\n\nparams.redirectTo.bar.bar.ba.barab[\"foo\"].abr =\n  \\`\\${window.location.pathname}\\${window.location.search}\\${window.location.hash}\\`;");
}
#[test]
fn test_lone_arg_js_format_1_e822fdaf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("let vgChannel = pointPositionDefaultRef({\n  model,\n  defaultPos,\n  channel,\n})()\n\nlet vgChannel2 = pointPositionDefaultRef({ model,\n  defaultPos,\n  channel,\n})()\n\nconst bifornCringerMoshedPerplexSawderGlyphsHa =\n  someBigFunctionName(\"foo\")(\"bar\");\n\nif (true) {\n  node.id = this.flowParseTypeAnnotatableIdentifier(/*allowPrimitiveOverride*/ true);\n}\n\nconst bifornCringerMoshedPerplexSawderGlyphsHb = someBigFunctionName(\\`foo\n\\`)(\"bar\");") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "let vgChannel = pointPositionDefaultRef({\n  model,\n  defaultPos,\n  channel,\n})();\n\nlet vgChannel2 = pointPositionDefaultRef({ model, defaultPos, channel })();\n\nconst bifornCringerMoshedPerplexSawderGlyphsHa =\n  someBigFunctionName(\"foo\")(\"bar\");\n\nif (true) {\n  node.id = this.flowParseTypeAnnotatableIdentifier(\n    /*allowPrimitiveOverride*/ true,\n  );\n}\n\nconst bifornCringerMoshedPerplexSawderGlyphsHb = someBigFunctionName(\\`foo\n\\`)(\"bar\");");
}
#[test]
fn test_sequence_js_format_1_3d8377c4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("for ((i = 0), (len = arr.length); i < len; i++) {\n  console.log(arr[i])\n}\n\nfor (i = 0, len = arr.length; i < len; i++) {\n  console.log(arr[i])\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "for (i = 0, len = arr.length; i < len; i++) {\n  console.log(arr[i]);\n}\n\nfor (i = 0, len = arr.length; i < len; i++) {\n  console.log(arr[i]);\n}");
}
#[test]
fn test_unary_js_format_1_43fd8ad1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const loooooooooooooooooooooooooong1 = void looooooooooooooong.looooooooooooooong.loooooong;\nconst loooooooooooooooooooooooooong2 = void \"looooooooooooooooooooooooooooooooooooooooooog\";\nconst loooooooooooooooooooooooooong3 = !looooooooooooooong.looooooooooooooong.loooooong;\nconst loooooooooooooooooooooooooong4 = !\"looooooooooooooooooooooooooooooooooooooooooog\";\nconst loooooooooooooooooooooooooong5 = void void looooooooooooooong.looooooooooooooong.loooooong;\nconst loooooooooooooooooooooooooong6 = void void \"looooooooooooooooooooooooooooooooooooooooooog\";\nconst loooooooooooooooooooooooooong7 = !!looooooooooooooong.looooooooooooooong.loooooong;\nconst loooooooooooooooooooooooooong8 = !!\"looooooooooooooooooooooooooooooooooooooooooog\";") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const loooooooooooooooooooooooooong1 =\n  void looooooooooooooong.looooooooooooooong.loooooong;\nconst loooooooooooooooooooooooooong2 =\n  void \"looooooooooooooooooooooooooooooooooooooooooog\";\nconst loooooooooooooooooooooooooong3 =\n  !looooooooooooooong.looooooooooooooong.loooooong;\nconst loooooooooooooooooooooooooong4 =\n  !\"looooooooooooooooooooooooooooooooooooooooooog\";\nconst loooooooooooooooooooooooooong5 =\n  void void looooooooooooooong.looooooooooooooong.loooooong;\nconst loooooooooooooooooooooooooong6 =\n  void void \"looooooooooooooooooooooooooooooooooooooooooog\";\nconst loooooooooooooooooooooooooong7 =\n  !!looooooooooooooong.looooooooooooooong.loooooong;\nconst loooooooooooooooooooooooooong8 =\n  !!\"looooooooooooooooooooooooooooooooooooooooooog\";");
}
