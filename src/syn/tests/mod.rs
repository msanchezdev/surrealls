#[cfg(test)]
use pest_test_gen::pest_tests;

#[pest_tests(
    crate::syn::SurrealQLParser,
    crate::syn::Rule,
    "file",
    dir = "src/syn/tests",
    ext = "pest-test",
    recursive = true,
    lazy_static = true,
    strict = false
)]
#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pest::{Parser, Token, iterators::Pair};

    use crate::syn::{Rule, SurrealQLParser};

    macro_rules! debug_surrealql {
        ($input:expr) => {
            let parsed = SurrealQLParser::parse(Rule::file, indoc! {$input})
                .unwrap()
                .next()
                .unwrap();

            println!("\n\n{}", $input);
            println!("--------------------------------");
            print_recursive(parsed.clone(), 0);
            println!("------------ Tokens ------------");
            for token in parsed.tokens() {
                println!(
                    "{} {:?}: {:?}",
                    if token.as_span().start() == token.as_span().end() {
                        format!("{:?}", token.as_span().start())
                    } else {
                        format!("{:?}-{:?}", token.as_span().start(), token.as_span().end(),)
                    },
                    token.as_rule(),
                    token.as_str()
                );
            }
            println!("--------------------------------");
        };
    }

    #[test]
    fn test_surrealql() {
        //         debug_surrealql!(r#"USE"#);
        //         debug_surrealql!(r#"USE NAMESPACE"#);
        //         debug_surrealql!(r#"USE NAMESPACE test"#);
        // debug_surrealql!(r#"USE DATABASE test;"#);
        let input = indoc! {r#"
        USE DATABASE test;
        USE NAMESPACE test;
        USE NAMESPACE test DATABASE test;
        "#};
        let parsed = SurrealQLParser::parse(Rule::file, input)
            .unwrap()
            .next()
            .unwrap();

        println!("\n\n{}", input);
        println!("--------------------------------");
        print_recursive(parsed.clone(), 0);
        println!("------------ Tokens ------------");
        for token in parsed.into_inner() {
            println!("{:?}", token.as_span());
            // println!(
            //     "{} {:?}: {:?}",
            //     if token.as_span().start() == token.as_span().end() {
            //         format!("{:?}", token.as_span().start())
            //     } else {
            //         format!("{:?}-{:?}", token.as_span().start(), token.as_span().end(),)
            //     },
            //     token.as_rule(),
            //     token.as_str()
            // );
        }
        println!("--------------------------------");

        //         debug_surrealql!(
        //             r#"USE DATABASE test;
        // "#
        //         );
    }

    fn print_recursive<'a, R: pest::RuleType>(parsed: Pair<'a, R>, depth: usize) {
        print!("{:>depth$}(", "", depth = depth);
        print!("{:?}", parsed.as_rule());

        let inner = parsed.clone().into_inner();
        if inner.peek().is_some() {
            println!();
            for rule in inner {
                print_recursive(rule, depth + 2);
            }
            print!("{:>depth$})", "", depth = depth);
        } else {
            print!(": {:?})", parsed.as_str());
        }
        println!();
    }
}
