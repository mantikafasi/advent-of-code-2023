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
    work_conditions: Vec<Condition>,
}

#[derive(Debug,Clone, PartialEq)]
struct Condition {
    variable: char,
    sign: char,
    condition: i32,
}

struct Status {
    max: i32,
    min: i32,
}

fn calculate(total_rules: &HashMap<String,Vec<Rule>>, rule_id: String) -> i64 {
    let rules = total_rules.get(rule_id.as_str()).unwrap().clone();
    let mut new_total_rules = total_rules.clone();
    let mut fail_possibilities: Vec<Condition> = Vec::new();
    let mut total_sum = 0;

    for rule in rules.iter() {
        // println!("Rule ID: {:?}", rule);
        
        if rule.if_true == "A" {
            let mut sum = 1;
            
            let mut conditions = rule.work_conditions.clone();
            conditions.push(Condition {
                variable: rule.variable,
                sign: rule.sign,
                condition: rule.condition,
            });

            conditions.append(&mut fail_possibilities.clone());

            // for cond in conditions.clone() {
            //     print!("{}{}{}  ", cond.variable, cond.sign, cond.condition);
            // }


            let mut statuses: HashMap<char, Status> = HashMap::new();

            statuses.insert('x', Status {max: 4000, min: 1});
            statuses.insert('m', Status {max: 4000, min: 1});
            statuses.insert('a', Status {max: 4000, min: 1});
            statuses.insert('s', Status {max: 4000, min: 1});

            for condition in conditions {
                statuses.entry(condition.variable).and_modify(|e| {
                    if condition.sign == '<' {
                        if condition.condition + 1 < e.max {
                            e.max = condition.condition - 1;
                        }
                    } else {
                        if condition.condition - 1 > e.min {
                            e.min = condition.condition + 1;
                        }
                    }
                });
            }
        
            for status in statuses.values() {
                sum *= (status.max - status.min + 1) as i64;
            }

            total_sum += sum;            
        }

        if rule.sign == ' ' && rule.if_true != "R" && rule.if_true != "A" {
            let mut next_rules = total_rules.get(rule.if_true.as_str()).unwrap().clone();

            for next_rule in next_rules.iter_mut() {
                next_rule.work_conditions.append(&mut rule.work_conditions.clone());
                next_rule.work_conditions.append(&mut fail_possibilities.clone());
            }

            new_total_rules.insert(rule.if_true.clone(), next_rules);
            
            total_sum += calculate(&new_total_rules, rule.if_true.clone());
        } else {
            let next_rules = total_rules.get(rule.if_true.as_str());
            if next_rules.is_some() {
                let mut next_rules = next_rules.unwrap().clone();
                for next_rule in next_rules.iter_mut() {
                    next_rule.work_conditions.append(&mut rule.work_conditions.clone());
                    next_rule.work_conditions.push(Condition {
                        variable: rule.variable,
                        condition: rule.condition,
                        sign: rule.sign,
                    });
                    next_rule.work_conditions.append(&mut fail_possibilities.clone());
                }
                new_total_rules.insert(rule.if_true.clone(), next_rules);
                total_sum += calculate(&new_total_rules, rule.if_true.clone());
            }

        }

        if rule.sign != ' ' {
            fail_possibilities.push(Condition {
                variable: rule.variable,
                condition: if rule.sign == '<' {rule.condition - 1} else {rule.condition + 1},
                sign: if rule.sign == '<' {'>'} else {'<'},
            });
        }
    }

    return total_sum;
}

fn main() {
    let mut rules: HashMap<String,Vec<Rule>> = HashMap::new();
    
    let contents = CONTENTS.split_once("\n\n").unwrap();

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
                    variable: 'z',
                    work_conditions: Vec::new(),
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
                    work_conditions: Vec::new(),
                });
            }
        }
        rules.insert(variable.to_string(), condition_list);
    }

    let part2 = calculate(&rules, String::from("in"));

    println!("Part 2: {}", part2);
}