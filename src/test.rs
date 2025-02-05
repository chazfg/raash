use super::*;
use shparp::Parser;
use shparp::Rule;
use shparp::ShellParser;
fn drill_down(mut p: Pairs<Rule>) -> Pair<Rule> {
    p.next()
        .unwrap()
        .into_inner()
        .find(|c| c.as_rule() == Rule::complete_commands)
        .unwrap()
        .into_inner()
        .find(|c| c.as_rule() == Rule::complete_command)
        .unwrap()
        .into_inner()
        .find(|c| c.as_rule() == Rule::list)
        .unwrap()
        .into_inner()
        .find(|c| c.as_rule() == Rule::and_or)
        .unwrap()
        .into_inner()
        .find(|c| c.as_rule() == Rule::pipeline)
        .unwrap()
        .into_inner()
        .find(|c| c.as_rule() == Rule::pipe_sequence)
        .unwrap()
        .into_inner()
        .find(|c| c.as_rule() == Rule::command)
        .unwrap()
        .into_inner()
        .find(|c| c.as_rule() == Rule::simple_command)
        .unwrap()
        .into_inner()
        .find(|c| c.as_rule() == Rule::cmd_name)
        .unwrap()
        .into_inner()
        .next()
        .unwrap()
}

//#[test]
//fn parse_test() {
//    let input = "oijef iojioajfe kiweof".to_string();
//    let mut p = parser::Parser::default();
//    p.parse(input);
//    println!("{:?}", p);
//}
//#[test]
//fn parse_exit() {
//    let input = "exit iojioajfe kiweof".to_string();
//    let mut p = parser::Parser::default();
//    p.parse(input);
//    println!("{:?}", p);
//}
//#[test]
//fn parse_export() {
//    let input = "export oaiejf=aeofji iofjewof=oka__f third=3209.19023".to_string();
//    let mut p = parser::Parser::default();
//    p.parse(input);
//    println!("{:?}", p);
//}
