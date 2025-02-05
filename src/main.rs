mod parser;
#[cfg(test)]
mod test;
mod utils;
use shparp::Pair;
use shparp::Pairs;
use shparp::Parser;
use shparp::Rule;
use shparp::ShellParser;
use std::io::Write;
use std::process::Command;
use utils::{SpecialUtil, TOrU, Util};

fn main() {
    let mut rl = rustyline::DefaultEditor::new().unwrap();
    //    let mut shell = Shell::from_environ();
    loop {
        let readline = rl.readline(">> ").unwrap();
        let t = ShellParser::parse(Rule::program, &readline);

        match t {
            Ok(mut pairs) => handle_program(pairs.next().unwrap().into_inner()),
            Err(e) => println!("{e:?}"),
        }
    }
}

fn handle_program(mut input: Pairs<Rule>) {
    println!("{input:?}");
    for t in input.by_ref() {
        match t.as_rule() {
            Rule::linebreak => (),
            Rule::complete_commands => handle_complete_commands(t.into_inner()),
            _ => unreachable!(),
        }
    }
}

fn handle_complete_commands(mut input: Pairs<Rule>) {
    for t in input.by_ref() {
        match t.as_rule() {
            Rule::newline_list => (),
            Rule::complete_command => handle_complete_command(t.into_inner()),
            _ => unreachable!(),
        }
    }
}

fn handle_complete_command(mut input: Pairs<Rule>) {
    for t in input.by_ref() {
        match t.as_rule() {
            Rule::list => handle_list(t.into_inner()),
            Rule::separator_op => println!("sep op, command --> {}", t.as_str()),
            _ => unreachable!(),
        }
    }
}

fn handle_list(mut input: Pairs<Rule>) {
    for t in input.by_ref() {
        match t.as_rule() {
            Rule::and_or => handle_and_or(t.into_inner()),
            Rule::separator_op => println!("sep op, list --> {}", t.as_str()),
            _ => unreachable!(),
        }
    }
}

fn handle_and_or(mut input: Pairs<Rule>) {
    for t in input.by_ref() {
        match t.as_rule() {
            Rule::pipeline => handle_pipeline(t.into_inner()),
            Rule::and_if => todo!("and_if"),
            Rule::linebreak => (),
            Rule::or_if => todo!("or_if"),
            _ => unreachable!(),
        }
    }
}

fn handle_pipeline(mut input: Pairs<Rule>) {
    for t in input.by_ref() {
        match t.as_rule() {
            Rule::pipe_sequence => handle_pipe_sequence(t.into_inner()),
            Rule::bang => todo!("bang"),
            _ => unreachable!(),
        }
    }
}

fn handle_pipe_sequence(mut input: Pairs<Rule>) {
    for t in input.by_ref() {
        match t.as_rule() {
            Rule::command => handle_command(t.into_inner()),
            Rule::linebreak => (),
            _ => unreachable!(),
        }
    }
}

fn handle_command(mut input: Pairs<Rule>) {
    let t = input.next().unwrap();
    println!("gets here");
    match t.as_rule() {
        Rule::simple_command => handle_simple_command(t.into_inner()),
        Rule::compound_command => todo!(),
        Rule::redirect_list => todo!(),
        Rule::function_definition => todo!(),
        _ => unreachable!(),
    }
}

fn handle_simple_command(input: Pairs<Rule>) {
    let mut cmd = CmdBuilder::default();

    for t in input {
        match t.as_rule() {
            Rule::cmd_suffix => cmd.set_cmd_suffix(t),
            Rule::cmd_prefix => todo!("cmd_prefix"),
            Rule::cmd_word => todo!("cmd_word"),
            Rule::cmd_name => cmd.set_cmd_word(handle_cmd_name(t.into_inner())),
            _ => unreachable!(),
        }
    }

    cmd.execute();
}

fn handle_cmd_name(mut input: Pairs<Rule>) -> ShellCommand {
    let t = input.next().unwrap();
    ShellCommand::from(t.as_str())
}

#[derive(Default, Debug)]
struct CmdBuilder {
    cmd_word: Option<ShellCommand>,
    cmd_suffix: Vec<String>,
    cmd_prefix: Option<String>,
}

impl CmdBuilder {
    pub fn set_cmd_word(&mut self, s: ShellCommand) {
        self.cmd_word = Some(s);
    }
    pub fn set_cmd_suffix(&mut self, pair: Pair<Rule>) {
        pair.into_inner().for_each(|p| match p.as_rule() {
            Rule::WORD => self.cmd_suffix.push(p.as_str().to_string()),
            Rule::tilde_expansion => self.cmd_suffix.push(std::env::var("HOME").unwrap()),
            Rule::param_sub => {
                let param = p.into_inner().next().unwrap().as_str();
                self.cmd_suffix.push(std::env::var(param).unwrap());
            }
            e => panic!("{e:?}"),
        });
    }
    pub fn set_cmd_prefix(&mut self, s: String) {
        self.cmd_prefix = Some(s);
    }
    pub fn execute(&self) {
        match self {
            Self { cmd_word: None, .. } => println!("unrecognized command"),
            Self {
                cmd_word: Some(cmd_stem),
                cmd_suffix,
                cmd_prefix,
            } => cmd_stem.execute(cmd_prefix, cmd_suffix),
        }
    }
}

#[derive(Debug)]
enum ShellCommand {
    SpecialUtil(SpecialUtil),
    Utility(Util),
    TypeOrUlimit(TOrU),
    Other(String),
}

impl ShellCommand {
    pub fn from(i: &str) -> Self {
        if let Some(sc) = SpecialUtil::from(i) {
            return Self::SpecialUtil(sc);
        } else if let Some(sc) = Util::from(i) {
            return Self::Utility(sc);
        } else if let Some(sc) = TOrU::from(i) {
            return Self::TypeOrUlimit(sc);
        } else {
            return Self::Other(i.to_string());
        }
    }

    pub fn execute(&self, cmd_prefix: &Option<String>, cmd_suffix: &Vec<String>) {
        match self {
            ShellCommand::SpecialUtil(special_util) => todo!(),
            ShellCommand::Utility(util) => todo!(),
            ShellCommand::TypeOrUlimit(tor_u) => todo!(),
            ShellCommand::Other(i) => {
                let path = std::env::var("PATH").unwrap();
                let paths: Vec<std::path::PathBuf> =
                    path.split(":").map(std::path::PathBuf::from).collect();
                for p in paths {
                    if p.join(i).exists() {
                        Command::new(p.join(i))
                            .args(cmd_suffix)
                            .spawn()
                            .expect("failed to run")
                            .wait()
                            .expect("failed to wait");
                        break;
                    }
                }
            }
        }
    }
}
