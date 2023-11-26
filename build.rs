use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(not(all(debug_assertions, feature = "generated_tests")))] {
        fn main() {}
    } else {
        use quote::ToTokens;
        use convert_case::{Case, Casing};
        use quote::quote;
        use std::{collections::HashMap, io::Write};

        const SNAPSHORT_START: &str = "exports[`";
        const _CONFIG_SEPARATOR: &str =
            "====================================options=====================================";
        const INPUT_SEPARATOR: &str =
            "=====================================input======================================";
        const OUTPUT_SEPARATOR: &str =
            "=====================================output=====================================";
        const SNAPSHOT_END: &str =
            "================================================================================";
        const OUT_DIR: &str = "./tests/generated/";

        // static IGNORE_RULES: &[&str] = &[
        //     // Embedded languages in template literals
        //     "js/test_multiparser_comments",
        //     "js/test_multiparser_css",
        //     "js/test_multiparser_graphql",
        //     "js/test_multiparser_html",
        //     "js/test_multiparser_invalid",
        //     "js/test_multiparser_markdown",
        //     "js/test_multiparser_text",
        //     // v8-specific syntaxes
        //     "js/test_v8_intrinsic",
        //     // Babel plugins (mostly experimental syntaxes)
        //     "js/test_babel_plugins",
        //     // Experimental syntax: `do {}`
        //     "js/test_async_do_expressions",
        //     "js/test_do",
        //     // Experimental syntax: `export X from "mod"`
        //     // "js/export_default/export_default_from/",
        //     // "js/export_default/escaped/default_escaped.js",
        //     // Experimental syntax: `module <id> {}`
        //     "js/test_module_blocks",
        //     // "js/test_explicit_resource_management/valid_module_block_top_level_await_using_binding.js",
        //     // "js/test_explicit_resource_management/valid_module_block_top_level_using_binding.js",
        //     // Experimental syntax: `#[]` and `#{}`
        //     "js/test_tuple",
        //     "js/test_record",
        //     // "js/test_arrays/tuple_and_record.js",
        //     // "js/test_arrows/tuple_and_record.js",
        //     // "js/test_binary_expressions/tuple_and_record.js",
        //     // "js/test_class_extends/tuple_and_record.js",
        //     // "js/test_comments_closure_typecast/tuple_and_record.js",
        //     // "js/test_comments/tuple_and_record.js",
        //     // "js/test_function_single_destructuring/tuple_and_record.js",
        //     // "js/test_method_chain/tuple_and_record.js",
        //     // Experimental syntax: pipeline operator `|>`
        //     "js/test_comments_pipeline_own_line",
        //     "js/test_partial_application",
        //     "js/test_pipeline_operator",
        //     // Experimental syntax: `::`
        //     "js/test_arrows_bind",
        //     "js/test_bind_expressions",
        //     // "js/objects/expression.js",
        //     // "js/no_semi_babylon_extensions/no_semi.js",
        //     // Experimental syntax: `let { #x: x } = ...`
        //     "js/test_destructuring_private_fields",
        //     // Experimental syntax: `import defer`
        //     "js/test_deferred_import_evaluation",
        //     // Experimental syntax: `import source`
        //     "js/test_source_phase_imports",
        //     // Experimental syntax: `import module`
        //     "js/test_import_reflection",
        // ];

        #[deny(clippy::all, clippy::pedantic, clippy::nursery, clippy::perf)]
        #[derive(Debug)]
        struct Snapshot {
            content: String,
            parser: String,
            rule: String,
        }

        struct TestCase {
            name: String,
            config: HashMap<syn::Ident, ValueType>,
            input: String,
            output: String,
        }

        enum ValueType {
            Literal(syn::Lit),
            Parser(syn::Lit),
            Ident(syn::Ident),
        }

        impl ToTokens for ValueType {
            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                match self {
                    ValueType::Literal(lit) => lit.to_tokens(tokens),
                    ValueType::Ident(ident) => ident.to_tokens(tokens),
                    ValueType::Parser(lit) => {
                        lit.to_tokens(tokens)
                    },
                }
            }
        }

        impl TestCase {
            fn parse(test_case: &str) -> Self {
                let mut lines = test_case.lines();
                let mut config = HashMap::new();
                let mut input = String::new();
                let mut output = String::new();

                // first line
                let first_line = lines.next().unwrap();

                // parse out the language
                let language = first_line
                    .trim_start_matches(SNAPSHORT_START)
                    .trim_end_matches("`] = `")
                    .splitn(2, ".")
                    .nth(1)
                    .and_then(|s| s.splitn(2, " ").nth(0));

                //  use the langauge name to determine the parser to use, and add it to the config
                eprintln!("{:?}", language);
                if let Some(language) = language {
                    config.insert(
                        syn::parse_str::<syn::Ident>(&format!("parser")).unwrap(),
                        ValueType::Parser(syn::parse_str::<syn::Lit>(&format!("\"{}\"", language)).unwrap()),
                    );
                }

                // parse out the name
                let name = first_line
                    .trim_end_matches("`] = `")
                    .replace('.', "_")
                    .replace(['[', ']', '{', '}', '\"', '/', ':'], "")
                    .to_case(Case::Snake);

                // consume the config separator
                lines.next();

                // parse out the config
                #[allow(clippy::while_let_on_iterator)]
                while let Some(line) = lines.next() {
                    if line == INPUT_SEPARATOR {
                        break;
                    }

                    let parts = line.splitn(2, ": ").collect::<Vec<_>>();

                    if parts.len() != 2 {
                        continue;
                    }

                    // map jsx_bracket_same_line to bracket_same_line
                    let key = parts.first().expect("key").to_case(Case::Snake);
                    let key = match key.as_str() {
                        "jsx_bracket_same_line" => "bracket_same_line",
                        other => other,
                    };

                    let Ok(key) =
                        syn::parse_str::<syn::Ident>(key)
                    else {
                        continue;
                    };
                    let value = if let Ok(val) = syn::parse_str::<syn::Lit>(parts.get(1).expect("value").to_owned()) {
                        ValueType::Literal(val)
                    // } else if let Ok(val) = syn::parse_str::<syn::ExprArray>(parts.get(1).expect("value").to_owned()) {
                    //     ValueType::Array(val)
                    } else if let Ok(val) = syn::parse_str::<syn::Ident>(&parts.get(1).expect("value").to_case(Case::UpperSnake)) {
                        ValueType::Ident(val)
                    } else {
                        continue;
                    };

                    config.insert(key.to_owned(), value);
                }

                // parse out the input
                #[allow(clippy::while_let_on_iterator)]
                while let Some(line) = lines.next() {
                    if line == OUTPUT_SEPARATOR {
                        break;
                    }

                    input.push_str(line);
                    input.push('\n');
                }

                // parse out the output
                #[allow(clippy::while_let_on_iterator)]
                while let Some(line) = lines.next() {
                    if line == SNAPSHOT_END {
                        break;
                    }

                    output.push_str(line);
                    output.push('\n');
                }

                // reformat the name to start with "test_", and end with a hashing of the input
                let name = format!(
                    "test_{name}_{hash}",
                    name = name,
                    hash = format!("{:x}", md5::compute(input.as_bytes()))[..8].to_owned()
                );

                // remove trailing "\n\n" from input and output
                input.pop();
                input.pop();
                output.pop();
                output.pop();

                // replace <LF>\n with \n, <CR>\n with \r, and <CRLF>\n with \r\n
                input = input
                    .replace("<LF>\n", "\n").replace("<CR>\n", "\r").replace("<CRLF>\n", "\r\n")
                    .replace("<LF>", "\n").replace("<CR>", "\r").replace("<CRLF>", "\r\n")
                    .replace("\\`", "`").replace("\\$", "$").replace("\\\\", "\\");
                output = output
                    .replace("<LF>\n", "\n").replace("<CR>\n", "\r").replace("<CRLF>\n", "\r\n")
                    .replace("<LF>", "\n").replace("<CR>", "\r").replace("<CRLF>", "\r\n")
                    .replace("\\`", "`").replace("\\$", "$").replace("\\\\", "\\");

                TestCase {
                    name,
                    config,
                    input,
                    output,
                }
            }
        }

        fn main() -> anyhow::Result<()> {
            use git2::Repository;

            // using git2, clone the prettier repo if it doesn't exist locally
            let _ = Repository::open("prettier")
                .or_else(|_| Repository::clone("https://github.com/prettier/prettier.git", "prettier"))?; // TODO: pull if exists

            // // delete contents of OUT_DIR
            // std::fs::remove_dir_all(OUT_DIR).unwrap();
            // // create OUT_DIR
            // std::fs::create_dir_all(OUT_DIR).unwrap();

            println!("cargo:rerun-if-changed=prettier/tests/format");
            println!("cargo:rerun-if-changed=build.rs");

            // create mod.rs for OUT_DIR
            std::fs::write(format!("{OUT_DIR}mod.rs", OUT_DIR = OUT_DIR), "").unwrap();

            // get the text and simplified path to every prettier/tests/format/**/__snapshots__/*.snap file (every format test)
            let snapshots = std::fs::read_dir("./prettier/tests/format")?
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.file_type().unwrap().is_dir())
                .filter_map(|entry| {
                    let parser = entry.file_name().to_str()?.to_owned().replace("-", "_");
                    // append to the mod.rs file
                    std::fs::OpenOptions::new()
                        .append(true)
                        .open(format!("{OUT_DIR}mod.rs"))
                        .unwrap()
                        .write_all(format!("mod {parser};\n", parser = parser).as_bytes())
                        .unwrap();

                    Some((parser, std::fs::read_dir(entry.path()).ok()?))
                })
                .flat_map(|(parser, parser_dir)| {
                    let rules = parser_dir
                        .filter_map(|entry| entry.ok())
                        .filter(|entry| entry.file_type().unwrap().is_dir())
                        .filter_map(|entry| {
                            let rule = entry
                                .file_name()
                                .to_str()?
                                .to_owned()
                                .replace("-", "_")
                                .replace(".", "_").to_case(Case::Snake);
                            Some((format!("test_{rule}",), entry.path()))
                        })
                        // // filter out tests we are ignoring
                        // .filter(|(rule, _)| {
                        //     !IGNORE_RULES.contains(&format!("{parser}/{rule}").as_str())
                        // })
                        .collect::<Vec<_>>();

                    // create the parser directory if it doesn't exist
                    std::fs::create_dir_all(format!("{OUT_DIR}{parser}")).unwrap();
                    // create mod.rs file for the parser
                    std::fs::write(
                        format!("{OUT_DIR}{parser}/mod.rs"),
                        rules
                            .iter()
                            .filter(|(_, path)| {
                                std::path::Path::new(&format!("{}/__snapshots__", path.to_str().unwrap()))
                                    .exists()
                            })
                            .map(|(rule, _)| format!("#[cfg(test)]\nmod {rule};"))
                            .collect::<Vec<_>>()
                            .join("\n"),
                    )
                    .unwrap();

                    // rustfmt the file
                    rustfmt(format!("{OUT_DIR}{parser}/mod.rs")).unwrap();

                    rules.into_iter().filter_map(move |(rule, path)| {
                        Some((
                            parser.clone(),
                            rule.to_owned(),
                            std::fs::read_dir(path).ok()?,
                        ))
                    })
                })
                .flat_map(|(parser, rule, rule_dir)| {
                    rule_dir
                        .filter_map(|entry| entry.ok())
                        .filter(|entry| {
                            entry.file_type().unwrap().is_dir() && entry.file_name() == "__snapshots__"
                        })
                        .filter_map(move |entry| {
                            Some((
                                parser.clone(),
                                rule.clone(),
                                std::fs::read_dir(entry.path()).ok()?,
                            ))
                        })
                })
                .flat_map(|(parser, rule, snapshot_dir)| {
                    snapshot_dir
                        .filter_map(|entry| entry.ok())
                        .filter(|entry| {
                            entry.file_type().unwrap().is_file()
                                && entry.path().extension().unwrap() == "snap"
                        })
                        .map(move |entry| {
                            let content = std::fs::read_to_string(entry.path()).unwrap();
                            Snapshot {
                                content,
                                parser: parser.clone(),
                                rule: rule.to_owned(),
                            }
                        })
                })
                .collect::<Vec<Snapshot>>();

            // rustfmt the generated/mod.rs file
            rustfmt(format!("{OUT_DIR}mod.rs", OUT_DIR = OUT_DIR)).unwrap();

            // now, for each of those snapshots, we need to parse out all the test cases, the configs they use, and the expected output
            let test_cases = snapshots
                .iter()
                // split the snapshots into test cases, ignoring the first one because that's just junk
                .map(|snapshot| {
                    (
                        &snapshot.parser,
                        &snapshot.rule,
                        snapshot.content.split(SNAPSHORT_START).collect::<Vec<_>>()[1..].to_owned(),
                    )
                })
                // parse those test cases into TestCase structs
                .map(|(parser, rule, test_cases)| {
                    (
                        parser,
                        rule,
                        test_cases
                            .iter()
                            .map(|test_case| TestCase::parse(test_case))
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>();

            // now, we need to generate a test for each of those test cases
            for (parser, rule, test_cases) in test_cases {
                let test_module = generate_test_module(test_cases);

                // create the parser directory if it doesn't exist
                std::fs::create_dir_all(format!("{OUT_DIR}{parser}"))?;
                // write the test module to a file
                std::fs::write(format!("{OUT_DIR}{parser}/{rule}.rs"), test_module)?;

                // rustfmt the file
                rustfmt(format!("{OUT_DIR}{parser}/{rule}.rs"))?;
            }

            Ok(())
        }

        fn generate_test_module(test_cases: Vec<TestCase>) -> String {
            let imports_and_helpers = quote! {
                #[allow(unused_imports)]
                use rust_prettier::PrettyPrinterBuilder;
                #[allow(unused_imports)]
                use pretty_assertions::assert_eq;
                #[allow(unused_imports)]
                use anyhow::Result;
                #[allow(dead_code)]
                static INFINITY: usize = usize::MAX;
            };

            let tests = test_cases
                .iter()
                .filter_map(|test_case| {
                    let name = syn::parse_str::<syn::Ident>(&test_case.name).ok()?;
                    let config = &test_case.config;
                    let mut config_keys = config.keys().collect::<Vec<_>>();
                    config_keys.sort();
                    let config_values = config_keys
                        .iter()
                        .map(|key| config.get(key).unwrap())
                        .collect::<Vec<_>>();
                    let input = &test_case.input;
                    let output = &test_case.output;

                    Some(quote! {
                        #[test]
                        fn #name() -> Result<()>{
                            let pretty_printer = PrettyPrinterBuilder::default()
                                #(.#config_keys(#config_values))*
                                .build()
                                .unwrap();

                            let formatted = pretty_printer.format(#input)?;
                            assert_eq!(formatted, #output);
                            Ok(())
                        }
                    })
                })
                .collect::<Vec<_>>();

            quote!(
                #imports_and_helpers

                #(#tests)*
            )
            .to_string()
        }

        fn rustfmt(path: impl AsRef<str>) -> anyhow::Result<()> {
            use std::process::Command;

            Command::new("rustfmt")
                .args(["--emit", "files", "--edition", "2021"])
                .arg(path.as_ref())
                .output()?;

            Ok(())
        }
    }
}
