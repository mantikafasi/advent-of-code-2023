// this is not complete

use std::{collections::{BTreeMap, HashMap}, borrow::{Borrow, BorrowMut}, thread::sleep};

static CONTENTS: &str = include_str!("input.txt");

#[derive(Debug,Clone, PartialEq,Copy)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcast,
    None,
}

#[derive(Debug,Clone, PartialEq,Copy)]
enum SignalType {
    LOW,
    HIGH
}

#[derive(Debug,Clone)]
struct Module {
    module_type: ModuleType,
    signal_level:SignalType,
    name:String,
    low_count: u32,
    high_count: u32,
    memory: BTreeMap<String,SignalType>,
    inputs: Vec<SignalType>,
    targets: Vec<String>,
}

fn main() {

    let mut modules: BTreeMap<String,Module> = BTreeMap::new();
    
    for line in CONTENTS.lines() {
        let (source,targets) = line.split_once("->").unwrap();
        
        let mut module = Module {
            module_type: ModuleType::Broadcast,
            signal_level: SignalType::LOW,
            targets: Vec::new(),
            name: if source.trim() != "broadcaster" {source[1..].trim().to_string()} else {String::from("broadcaster")},
            inputs: Vec::new(),
            low_count:0,
            high_count:0,
            memory: BTreeMap::new(),
        };

        if targets.contains(",") {
            module.targets = targets.split(",").map(|x| x.trim().to_string()).collect();
        } else {
            module.targets.push(targets.trim().to_string());
        }

        match &source[0..1] {
            "b" => {
                module.module_type = ModuleType::Broadcast;
                // sends same pulse to all targets                
            },
            "%" => {
                module.module_type = ModuleType::FlipFlop;
                module.signal_level = SignalType::LOW;
                // if it gets low signal it flips
            },
            "&" => {
                module.module_type = ModuleType::Conjunction;
                // store pulse to memory if both inputs are high it sends low input otherwise it sends high pulse
            }
            _ => {
                println!("Unknown module type")
            }
        }
        modules.insert(module.name.clone(),module);
    }

    for module in modules.clone().iter() {
        for output in &module.1.targets {
            modules.entry(output.clone()).or_insert(Module { 
                module_type: ModuleType::None, 
                signal_level: SignalType::LOW, 
                name: output.clone(), 
                low_count: 0, 
                high_count: 0, 
                inputs: Vec::new(), 
                targets: Vec::new(),
                memory: BTreeMap::new(),
            });
        }
    }

    for module in modules.clone().iter().filter(|x| x.1.module_type == ModuleType::Conjunction) {
        let clone = modules.clone();

        let inputs = clone.iter().filter(|x| x.1.targets.contains(&module.0));
        let memory = inputs.clone().map(|i| (i.0.clone(),SignalType::LOW)).collect();
        let mut_module = modules.get_mut(module.0).unwrap();
        mut_module.memory = memory;
        // println!("{} -> {}",module.0,input_count);
    }


    let click_count = 1000;
    for i in 0..click_count {
        println!("Iteration {}",i);

        let module = modules.get("broadcaster").unwrap().clone();
        modules.get_mut("broadcaster").unwrap().low_count += (module.targets.len() + 1) as u32;
    
        run_module(&mut modules);
    }

    for module in modules.clone() { 
        println!("{} -> {:?}",module.1.high_count,module.1.low_count);
    }

    let total_low :u32 = modules.iter().map(|m| m.1.low_count).sum();
    let total_high:u32 = modules.iter().map(|m| m.1.high_count).sum();

    println!("High Count: {}\nLow Count: {}\nPart 1: {}",total_high, total_low, total_high * total_low);
}


fn run_module(modules: &mut BTreeMap<String,Module>) {
            // let module = modules.get(&module_name).unwrap().clone();
            
    let mut modules_to_run: Vec<String> = Vec::new();
    modules_to_run.push("broadcaster".to_string());

    while modules_to_run.len() > 0 {
        //println!("Modules to run: {:?}",modules_to_run.len());
        //sleep(std::time::Duration::from_millis(10));
        let module_name = modules_to_run.pop().unwrap();
        
        let module = modules.get(&module_name).unwrap().clone();
                
        match module.module_type {
            ModuleType::Broadcast => {
                for target in &module.targets {
                    let target_module = modules.get_mut(target).unwrap();
                    
                    if target_module.module_type == ModuleType::Conjunction {
                        target_module.memory.insert(module.name.clone(), SignalType::LOW);
                    } else {
                        target_module.inputs.push(SignalType::LOW);
                    }

                    modules_to_run.insert(0,target_module.name.clone());
                    println!("Broadcasting {:?} to {}",SignalType::LOW,target_module.name)
                }
            },
            ModuleType::Conjunction => {
                let memory = module.memory.clone();

                println!("Memory: {:?}",memory);
                let output_type = memory.iter().all(|(_,x)| *x == SignalType::HIGH);

                let mutable_module = modules.get_mut(&module.name).unwrap();
                mutable_module.memory = memory;
                
                for target in module.targets {
                    let target_module = modules.get_mut(&target).unwrap();                    
                    
                    if output_type {
                        target_module.low_count += 1;
                    } else {
                        target_module.high_count += 1;
                    }
                    println!("Broadcasting {:?} to {}",if output_type {SignalType::LOW} else {SignalType::HIGH},target_module.name);

                    if target_module.module_type == ModuleType::Conjunction {
                        target_module.memory.insert(module.name.clone(), if output_type {SignalType::LOW} else {SignalType::HIGH});
                    } else {
                        target_module.inputs.push(if output_type {SignalType::LOW} else {SignalType::HIGH});
                    }
                    
                    modules_to_run.insert(0,target_module.name.clone());
                }

            },
            ModuleType::FlipFlop => {
                let mut inputs = module.inputs.clone();

                while inputs.len() > 0 {
                    let input = inputs.pop().unwrap();


                    if input == SignalType::LOW {
                        let mutable_module = modules.get_mut(&module.name).unwrap();

                        mutable_module.signal_level = if module.signal_level == SignalType::LOW {SignalType::HIGH} else {SignalType::LOW};
                        let signal_level = mutable_module.signal_level.clone();

                        for target in &module.targets {
                            let target_module = modules.get_mut(target).unwrap();
            
                            if signal_level == SignalType::HIGH {
                                target_module.high_count += 1;
                            } else {
                                target_module.low_count += 1;
                            }

                            if target_module.module_type == ModuleType::Conjunction {
                                target_module.memory.insert(module.name.clone(), signal_level);
                            } else {
                                target_module.inputs.push(signal_level);
                            }
                            
            
                            modules_to_run.insert(0,target_module.name.clone());
                                                        
                            println!("Broadcasting {:?} to {}",signal_level,target_module.name)
                        }
                    }
                }

    
            }  ,
            _ => { }
        }

        modules.get_mut(&module.name).unwrap().inputs = Vec::new();
    }
}