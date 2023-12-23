use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    static ref CONTENTS: String = include_str!("input.txt").to_string();
}

#[derive(Debug,Clone, PartialEq)]
struct Rule {
    condition: i32,
    sign: char,
    variable: char,
    if_true: String,
}

fn calculate(rules: HashMap<String,Vec<Rule>>, rule: Vec<Rule>, vars: HashMap<String,i32>) -> bool {
    for rule in rule {
        rule.condition;
        let var = vars.get(&rule.variable.to_string()).unwrap().clone();
        let sign = rule.sign;
        let mut if_true = String::from(" ");
        
        println!("{} {:?}, {}", var, rule.sign, rule.condition);
        if sign == '<' {
            if var < rule.condition {
                if_true = rule.if_true;
            } else {
                continue;
            }
            
        } else if sign == '>' {
            if var > rule.condition {
                if_true = rule.if_true;
            } else {
                continue;
            }
        } else {
            if_true = rule.if_true;
        }
        if if_true == "A" {
            println!("Accepted");
            return true;
        } else if if_true == "R" {
            println!("Rejected");
            return false;
        }

        let new_rule = rules.get(if_true.as_str());
        
        return calculate(rules.clone(), new_rule.unwrap().clone(), vars.clone())
    }
    return false;
}

fn main() {
    let mut rules: HashMap<String,Vec<Rule>> = HashMap::new();
    
    let contents = CONTENTS.split_once("\n\n").unwrap();
    
    let mut workflows = contents.1.lines();
    
    for rule in contents.0.lines() {
        let split = rule.split_once("{").unwrap();
        let variable = split.0;
        let conditions =  split.1.split_once("}").unwrap().0;

        let mut condition_list: Vec<Rule> = Vec::new();

        for condition in conditions.split(",") {
            if !condition.contains(":") {
                condition_list.push(Rule {
                    if_true: condition.to_string(),
                    condition: 0,
                    sign: ' ',
                    variable: 'a',
                })
            } else {
                let split = condition.split_once(":").unwrap();
                let if_true = split.1.to_string();
                let con = split.0;
                let chars: Vec<char> = split.0.chars().collect();
    
                condition_list.push(Rule {
                    variable: chars[0],
                    sign: chars[1],
                    if_true,
                    condition: con[2..].parse::<i32>().unwrap(),
                });
            }
        }
        rules.insert(variable.to_string(), condition_list);
    }

    let mut part1 = 0;

    for workflow in workflows {
        let variables = workflow.split_once("{").unwrap().1.split_once("}").unwrap().0.split(",");
        let mut vars: HashMap<String,i32> = HashMap::new();

        for variable in variables {
            let var = variable[0..1].to_string();
            let value = variable[2..].parse::<i32>().unwrap();
            vars.insert(var,value);
        }

        let start = rules.get("in").unwrap().clone();
        if calculate(rules.clone(), start, vars.clone()) {
            part1 += vars.values().sum::<i32>();

        }
   }

    println!("Part 1: {}", part1);
}
