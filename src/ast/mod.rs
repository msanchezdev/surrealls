// use indoc::indoc;

// macro_rules! debug_surrealql {
//     ($input:expr) => {
//         let parsed = SurrealQLParser::parse(Rule::file, indoc! {$input})
//             .unwrap()
//             .next()
//             .unwrap();

//         println!("\n\n{}", $input);
//         println!("--------------------------------");
//         print_recursive(parsed.clone(), 0);
//         println!("--------------------------------");
//     };
// }

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

use indoc::indoc;
use pest::{Parser, iterators::Pair};
use rowan::TextSize;

use crate::syn::{Lang, Rule, SurrealQLParser};

#[ignore]
#[test]
pub fn test_ast() {
    let input = indoc! {r#"USE NAMESPACE test`;
USE DATABASE test;
USE NS myns DB mydb;
"#};
    let sql = SurrealQLParser::parse_to_syntax_node(input).unwrap();

    println!("\n--------------------------------");
    print_recursive(
        SurrealQLParser::parse(Rule::file, input)
            .unwrap()
            .next()
            .unwrap(),
        0,
    );
    println!("--------------------------------");
    println!("{:#?}", sql);
    println!("--------------------------------");
    println!("{}", sql.text());
    println!("--------------------------------");
    println!(
        "{:#?}",
        sql.token_at_offset(TextSize::new(14))
            .right_biased()
            .unwrap()
            .parent()
    );
    // debug_surrealql!("USE NAMESPACE test;");

    // println!("{:?}",);
    // let input = r#"USE NAMESPACE test;"#;
    // let parsed = SurrealQLParser::parse(Rule::file, indoc! {r#"USE NAMESPACE test;"#})
    //     .unwrap()
    //     .next()
    //     .unwrap();

    // println!("--------------------------------");
    // for token in parsed.tokens() {
    //     match token {
    //         pest::Token::Start { rule, pos } => {
    //             println!("Start: {:?} @ {:?}", rule, pos);
    //             parsed.
    //         }
    //         pest::Token::End { rule, pos } => {
    //             println!("End: {:?} @ {:?}", rule, pos);
    //         }
    //     }
    // }
    // // print_recursive(parsed.clone(), 0);
    // println!("--------------------------------");
}
