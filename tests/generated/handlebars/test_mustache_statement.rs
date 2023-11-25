use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_basics_hbs_single_quotetrue_format_1_20a2b98f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("hbs")
        .print_width(80)
        .single_quote(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<GlimmerComponent @progress={{aPropertyEngdingAfterEightiethColumnToHighlightAWeirdClosingParenIssue}}\n/>\n\n<GlimmerComponent\n  class=\"medium\"\n  @autocomplete=\"off\"\n  @errors={{or this.aLProp (and this.aProperty (v-get bike \"number\" \"message\"))}}\n  data-test-beneficiary-account-number\n/>\n\n<GlimmerComponent\n  class=\"medium\"\n  @autocomplete=\"off\"\n  @errors={{or this.aVeryLongProperty (and this.aProperty (v-get bike \"number\" \"message\"))}}\n  data-test-beneficiary-account-number\n/>\n\n<div class=\"a long list of classes more than 80 chars with a complex mustache statement {{if (or this.a this.b) this.c this.d}}\">\n</div>\n\n<div class=\"a multi line list of classes with a complex mustache statement\n           {{if (or this.a this.b) this.c this.d}}\">\n</div>\n\n<div class=\"a-first-class {{if this.optionOne \"optionOne\"}} {{if this.optionTwo \"optionTwo\"}} {{if this.optionThree \"optionThree\"}} {{if this.optionFour \"optionFour\"}} {{if this.optionFive \"optionFive\"}} {{this.class}}\" ...attributes >\n</div>\n\n<div\n  class=\"a-first-class\n    {{if this.optionOne \"optionOne\"}}\n    \n    {{if this.optionTwo \"optionTwo\"}}\n    \n    {{if this.optionThree \"optionThree\"}}\n    \n    {{if this.optionFour \"optionFour\"}}\n    \n    {{if this.optionFive \"optionFive\"}}\n    \n    {{this.class}}\"\n  ...attributes >\n</div>\n\n<div>\n  {{classic-component-with-many-properties\n    class=\"hello\"\n    param=this.someValue\n    secondParam=this.someValue\n    thirdParam=this.someValue\n  }}\n</div>\n\n<div>\n  {{yield (hash\n    title=(component title)\n    header=(component header)\n    language=(component language)\n  )}}\n</div>\n\n<div>\n  {{a-helper \"that takes some arguments\" this.anotherOne \"why not\" @aLastOneLongEnoughToBreak}}\n</div>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<GlimmerComponent\n  @progress={{aPropertyEngdingAfterEightiethColumnToHighlightAWeirdClosingParenIssue}}\n/>\n\n<GlimmerComponent\n  class='medium'\n  @autocomplete='off'\n  @errors={{or\n    this.aLProp\n    (and this.aProperty (v-get bike 'number' 'message'))\n  }}\n  data-test-beneficiary-account-number\n/>\n\n<GlimmerComponent\n  class='medium'\n  @autocomplete='off'\n  @errors={{or\n    this.aVeryLongProperty\n    (and this.aProperty (v-get bike 'number' 'message'))\n  }}\n  data-test-beneficiary-account-number\n/>\n\n<div\n  class='a long list of classes more than 80 chars with a complex mustache statement\n    {{if (or this.a this.b) this.c this.d}}'\n>\n</div>\n\n<div\n  class='a multi line list of classes with a complex mustache statement\n    {{if (or this.a this.b) this.c this.d}}'\n>\n</div>\n\n<div\n  class='a-first-class\n    {{if this.optionOne \"optionOne\"}}\n    {{if this.optionTwo \"optionTwo\"}}\n    {{if this.optionThree \"optionThree\"}}\n    {{if this.optionFour \"optionFour\"}}\n    {{if this.optionFive \"optionFive\"}}\n    {{this.class}}'\n  ...attributes\n>\n</div>\n\n<div\n  class='a-first-class\n    {{if this.optionOne \"optionOne\"}}\n    {{if this.optionTwo \"optionTwo\"}}\n    {{if this.optionThree \"optionThree\"}}\n    {{if this.optionFour \"optionFour\"}}\n    {{if this.optionFive \"optionFive\"}}\n    {{this.class}}'\n  ...attributes\n>\n</div>\n\n<div>\n  {{classic-component-with-many-properties\n    class='hello'\n    param=this.someValue\n    secondParam=this.someValue\n    thirdParam=this.someValue\n  }}\n</div>\n\n<div>\n  {{yield\n    (hash\n      title=(component title)\n      header=(component header)\n      language=(component language)\n    )\n  }}\n</div>\n\n<div>\n  {{a-helper\n    'that takes some arguments'\n    this.anotherOne\n    'why not'\n    @aLastOneLongEnoughToBreak\n  }}\n</div");
}
#[test]
fn test_basics_hbs_format_1_20a2b98f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("hbs")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<GlimmerComponent @progress={{aPropertyEngdingAfterEightiethColumnToHighlightAWeirdClosingParenIssue}}\n/>\n\n<GlimmerComponent\n  class=\"medium\"\n  @autocomplete=\"off\"\n  @errors={{or this.aLProp (and this.aProperty (v-get bike \"number\" \"message\"))}}\n  data-test-beneficiary-account-number\n/>\n\n<GlimmerComponent\n  class=\"medium\"\n  @autocomplete=\"off\"\n  @errors={{or this.aVeryLongProperty (and this.aProperty (v-get bike \"number\" \"message\"))}}\n  data-test-beneficiary-account-number\n/>\n\n<div class=\"a long list of classes more than 80 chars with a complex mustache statement {{if (or this.a this.b) this.c this.d}}\">\n</div>\n\n<div class=\"a multi line list of classes with a complex mustache statement\n           {{if (or this.a this.b) this.c this.d}}\">\n</div>\n\n<div class=\"a-first-class {{if this.optionOne \"optionOne\"}} {{if this.optionTwo \"optionTwo\"}} {{if this.optionThree \"optionThree\"}} {{if this.optionFour \"optionFour\"}} {{if this.optionFive \"optionFive\"}} {{this.class}}\" ...attributes >\n</div>\n\n<div\n  class=\"a-first-class\n    {{if this.optionOne \"optionOne\"}}\n    \n    {{if this.optionTwo \"optionTwo\"}}\n    \n    {{if this.optionThree \"optionThree\"}}\n    \n    {{if this.optionFour \"optionFour\"}}\n    \n    {{if this.optionFive \"optionFive\"}}\n    \n    {{this.class}}\"\n  ...attributes >\n</div>\n\n<div>\n  {{classic-component-with-many-properties\n    class=\"hello\"\n    param=this.someValue\n    secondParam=this.someValue\n    thirdParam=this.someValue\n  }}\n</div>\n\n<div>\n  {{yield (hash\n    title=(component title)\n    header=(component header)\n    language=(component language)\n  )}}\n</div>\n\n<div>\n  {{a-helper \"that takes some arguments\" this.anotherOne \"why not\" @aLastOneLongEnoughToBreak}}\n</div>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<GlimmerComponent\n  @progress={{aPropertyEngdingAfterEightiethColumnToHighlightAWeirdClosingParenIssue}}\n/>\n\n<GlimmerComponent\n  class=\"medium\"\n  @autocomplete=\"off\"\n  @errors={{or\n    this.aLProp\n    (and this.aProperty (v-get bike \"number\" \"message\"))\n  }}\n  data-test-beneficiary-account-number\n/>\n\n<GlimmerComponent\n  class=\"medium\"\n  @autocomplete=\"off\"\n  @errors={{or\n    this.aVeryLongProperty\n    (and this.aProperty (v-get bike \"number\" \"message\"))\n  }}\n  data-test-beneficiary-account-number\n/>\n\n<div\n  class=\"a long list of classes more than 80 chars with a complex mustache statement\n    {{if (or this.a this.b) this.c this.d}}\"\n>\n</div>\n\n<div\n  class=\"a multi line list of classes with a complex mustache statement\n    {{if (or this.a this.b) this.c this.d}}\"\n>\n</div>\n\n<div\n  class=\"a-first-class\n    {{if this.optionOne 'optionOne'}}\n    {{if this.optionTwo 'optionTwo'}}\n    {{if this.optionThree 'optionThree'}}\n    {{if this.optionFour 'optionFour'}}\n    {{if this.optionFive 'optionFive'}}\n    {{this.class}}\"\n  ...attributes\n>\n</div>\n\n<div\n  class=\"a-first-class\n    {{if this.optionOne 'optionOne'}}\n    {{if this.optionTwo 'optionTwo'}}\n    {{if this.optionThree 'optionThree'}}\n    {{if this.optionFour 'optionFour'}}\n    {{if this.optionFive 'optionFive'}}\n    {{this.class}}\"\n  ...attributes\n>\n</div>\n\n<div>\n  {{classic-component-with-many-properties\n    class=\"hello\"\n    param=this.someValue\n    secondParam=this.someValue\n    thirdParam=this.someValue\n  }}\n</div>\n\n<div>\n  {{yield\n    (hash\n      title=(component title)\n      header=(component header)\n      language=(component language)\n    )\n  }}\n</div>\n\n<div>\n  {{a-helper\n    \"that takes some arguments\"\n    this.anotherOne\n    \"why not\"\n    @aLastOneLongEnoughToBreak\n  }}\n</div");
}
#[test]
fn test_element_modifier_statement_hbs_single_quotetrue_format_1_759a3292() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("hbs")
        .print_width(80)
        .single_quote(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div {{hello param hash=key}} {{goodbye param}}>\n  Hello\n</div>\n\n<div {{hello param param param param param param param param param param param param}}>\n  Hello\n</div>\n\n<div {{hello hashPair=value hashPair=value hashPair=value hashPair=value hashPair=value}}>\n  Hello\n</div>\n\n<div {{hello param param param param hashPair=value hashPair=value hashPair=value hashPair=value hashPair=value}}>\n  Hello\n</div>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<div {{hello param hash=key}} {{goodbye param}}>\n  Hello\n</div>\n\n<div\n  {{hello\n    param\n    param\n    param\n    param\n    param\n    param\n    param\n    param\n    param\n    param\n    param\n    param\n  }}\n>\n  Hello\n</div>\n\n<div\n  {{hello\n    hashPair=value\n    hashPair=value\n    hashPair=value\n    hashPair=value\n    hashPair=value\n  }}\n>\n  Hello\n</div>\n\n<div\n  {{hello\n    param\n    param\n    param\n    param\n    hashPair=value\n    hashPair=value\n    hashPair=value\n    hashPair=value\n    hashPair=value\n  }}\n>\n  Hello\n</div");
}
#[test]
fn test_element_modifier_statement_hbs_format_1_759a3292() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("hbs")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div {{hello param hash=key}} {{goodbye param}}>\n  Hello\n</div>\n\n<div {{hello param param param param param param param param param param param param}}>\n  Hello\n</div>\n\n<div {{hello hashPair=value hashPair=value hashPair=value hashPair=value hashPair=value}}>\n  Hello\n</div>\n\n<div {{hello param param param param hashPair=value hashPair=value hashPair=value hashPair=value hashPair=value}}>\n  Hello\n</div>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<div {{hello param hash=key}} {{goodbye param}}>\n  Hello\n</div>\n\n<div\n  {{hello\n    param\n    param\n    param\n    param\n    param\n    param\n    param\n    param\n    param\n    param\n    param\n    param\n  }}\n>\n  Hello\n</div>\n\n<div\n  {{hello\n    hashPair=value\n    hashPair=value\n    hashPair=value\n    hashPair=value\n    hashPair=value\n  }}\n>\n  Hello\n</div>\n\n<div\n  {{hello\n    param\n    param\n    param\n    param\n    hashPair=value\n    hashPair=value\n    hashPair=value\n    hashPair=value\n    hashPair=value\n  }}\n>\n  Hello\n</div");
}
#[test]
fn test_escaped_hbs_single_quotetrue_format_1_43a33fe0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("hbs")
        .print_width(80)
        .single_quote(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("an escaped mustache:\n\\\\{{my-component}}\na non-escaped mustache:\n\\\\\\\\{{my-component}}\nanother non-escaped mustache:\n\\\\\\\\\\\\{{my-component}}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "an escaped mustache:\n\\\\{{my-component}} a non-escaped mustache:\n\\\\\\\\{{my-component}}\nanother non-escaped mustache: \\\\\\\\\\\\{{my-component}");
}
#[test]
fn test_escaped_hbs_format_1_43a33fe0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("hbs")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("an escaped mustache:\n\\\\{{my-component}}\na non-escaped mustache:\n\\\\\\\\{{my-component}}\nanother non-escaped mustache:\n\\\\\\\\\\\\{{my-component}}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "an escaped mustache:\n\\\\{{my-component}} a non-escaped mustache:\n\\\\\\\\{{my-component}}\nanother non-escaped mustache: \\\\\\\\\\\\{{my-component}");
}
#[test]
fn test_issue_8691_hbs_single_quotetrue_format_1_6d46caaf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("hbs")
        .print_width(80)
        .single_quote(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<li class=\"\n{{if showAsCarousel \"career-interests__pill--slide-item\" \"career-interests__pill\"}}\n\"></li>\n\n{{\n  yield\n  (hash\n    header=(component \"fluid-table/thead\")\n    body=(component \"fluid-table/tbody\")\n    th=(component \"fluid-table/th\")\n    td=(component \"fluid-table/td\")\n  )\n}}\n\n<LinkTo\n  @query={{\n    hash\n    filter=(compute this.computeFilterWithEmail this.filter campaignPic.userEmail)\n  }}\n  class=\"text-xs hover:underline\"\n  data-test-user-email\n>\n  {{campaignPic.userEmail}}\n</LinkTo>\n\n{{if abcdefghijklmnopqrstuvwxyz (t 'abcdefghijklmnopq') (t 'abcdefghijklmnopq')}}\n\n{{#if (eq abcdefghijklmnopqrstuvwxyzabcdefgh abcdefghijklmnopqrstuvwxyzabcdefgh)}}\nfoo{{/if}}\n\n{{some-helper abcdefghijklmnopqrstuvwxyz abcdefghijklmnopqrstuvwxyz abcdefghijkl}}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<li\n  class='{{if\n      showAsCarousel\n      \"career-interests__pill--slide-item\"\n      \"career-interests__pill\"\n    }}\n    '\n></li>\n\n{{yield\n  (hash\n    header=(component 'fluid-table/thead')\n    body=(component 'fluid-table/tbody')\n    th=(component 'fluid-table/th')\n    td=(component 'fluid-table/td')\n  )\n}}\n\n<LinkTo\n  @query={{hash\n    filter=(compute\n      this.computeFilterWithEmail this.filter campaignPic.userEmail\n    )\n  }}\n  class='text-xs hover:underline'\n  data-test-user-email\n>\n  {{campaignPic.userEmail}}\n</LinkTo>\n\n{{if\n  abcdefghijklmnopqrstuvwxyz\n  (t 'abcdefghijklmnopq')\n  (t 'abcdefghijklmnopq')\n}}\n\n{{#if\n  (eq abcdefghijklmnopqrstuvwxyzabcdefgh abcdefghijklmnopqrstuvwxyzabcdefgh)\n}}\n  foo{{/if}}\n\n{{some-helper\n  abcdefghijklmnopqrstuvwxyz\n  abcdefghijklmnopqrstuvwxyz\n  abcdefghijkl\n}");
}
#[test]
fn test_issue_8691_hbs_format_1_6d46caaf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("hbs")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<li class=\"\n{{if showAsCarousel \"career-interests__pill--slide-item\" \"career-interests__pill\"}}\n\"></li>\n\n{{\n  yield\n  (hash\n    header=(component \"fluid-table/thead\")\n    body=(component \"fluid-table/tbody\")\n    th=(component \"fluid-table/th\")\n    td=(component \"fluid-table/td\")\n  )\n}}\n\n<LinkTo\n  @query={{\n    hash\n    filter=(compute this.computeFilterWithEmail this.filter campaignPic.userEmail)\n  }}\n  class=\"text-xs hover:underline\"\n  data-test-user-email\n>\n  {{campaignPic.userEmail}}\n</LinkTo>\n\n{{if abcdefghijklmnopqrstuvwxyz (t 'abcdefghijklmnopq') (t 'abcdefghijklmnopq')}}\n\n{{#if (eq abcdefghijklmnopqrstuvwxyzabcdefgh abcdefghijklmnopqrstuvwxyzabcdefgh)}}\nfoo{{/if}}\n\n{{some-helper abcdefghijklmnopqrstuvwxyz abcdefghijklmnopqrstuvwxyz abcdefghijkl}}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<li\n  class=\"{{if\n      showAsCarousel\n      'career-interests__pill--slide-item'\n      'career-interests__pill'\n    }}\n    \"\n></li>\n\n{{yield\n  (hash\n    header=(component \"fluid-table/thead\")\n    body=(component \"fluid-table/tbody\")\n    th=(component \"fluid-table/th\")\n    td=(component \"fluid-table/td\")\n  )\n}}\n\n<LinkTo\n  @query={{hash\n    filter=(compute\n      this.computeFilterWithEmail this.filter campaignPic.userEmail\n    )\n  }}\n  class=\"text-xs hover:underline\"\n  data-test-user-email\n>\n  {{campaignPic.userEmail}}\n</LinkTo>\n\n{{if\n  abcdefghijklmnopqrstuvwxyz\n  (t \"abcdefghijklmnopq\")\n  (t \"abcdefghijklmnopq\")\n}}\n\n{{#if\n  (eq abcdefghijklmnopqrstuvwxyzabcdefgh abcdefghijklmnopqrstuvwxyzabcdefgh)\n}}\n  foo{{/if}}\n\n{{some-helper\n  abcdefghijklmnopqrstuvwxyz\n  abcdefghijklmnopqrstuvwxyz\n  abcdefghijkl\n}");
}
