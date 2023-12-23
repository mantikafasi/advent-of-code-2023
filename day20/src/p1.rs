use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    static ref CONTENTS: String = include_str!("input.txt").to_string();
}

#[derive(Debug,Clone, PartialEq,Copy)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcast,
}

#[derive(Debug,Clone, PartialEq,Copy)]
enum SignalType {
    LOW,
    HIGH,
    OFF
}

#[derive(Debug,Clone)]
struct Module {
    module_type: ModuleType,
    signal_level:SignalType,
    name:String,
    low_count: u32,
    high_count: u32,
    inputs: Vec<SignalType>,
    targets: Vec<String>,
}

fn main() {

    let mut modules: HashMap<String,Module> = HashMap::new();
    
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
                module.signal_level = SignalType::OFF;
                // store pulse to memory if both inputs are high it sends low input otherwise it sends high pulse
            }
            _ => {
                println!("Unknown module type")
            }
        }
        modules.insert(module.name.clone(),module);
    }

    modules.insert(String::from("output"),Module {
        module_type: ModuleType::Broadcast,
        signal_level: SignalType::LOW,
        targets: Vec::new(),
        name: String::from("output"),
        inputs: Vec::new(),
        low_count:0,
        high_count:0,
    });


    let click_count = 1;
    for _ in 0..click_count {
        run(&mut modules);
    }

    for module in modules.clone() { 
        println!("{} -> {:?}",module.1.high_count,module.1.low_count);
    }

    let total_low :u32 = modules.iter().map(|m| m.1.low_count).sum();
    let total_high:u32 = modules.iter().map(|m| m.1.high_count).sum();

    println!("High Count: {}\nLow Count: {}",total_high, total_low);
}


fn run_module(modules: &mut HashMap<String,Module>, module_name: String) {
            // let module = modules.get(&module_name).unwrap().clone();
    let module = modules.get(&module_name).unwrap().clone();

    match module.module_type {
        ModuleType::Conjunction => {
            let output_type = module.inputs.iter().all(|x| *x == SignalType::HIGH);
            
            // let mutable_module = modules.get_mut(&module.name).unwrap();
        
            for target in &module.targets {
                let target_module = modules.get_mut(target).unwrap();
                target_module.inputs.push(if output_type {SignalType::LOW} else {SignalType::HIGH});
                
                if output_type {
                    target_module.low_count += 1;
                } else {
                    target_module.high_count += 1;
                }
            
                // modules_to_run.push(target_module.name.clone());
                
                println!("Broadcasting {:?} to {}",if output_type {SignalType::LOW} else {SignalType::HIGH},target_module.name)
            }
        },
        ModuleType::FlipFlop => {
            let mutable_module = modules.get_mut(&module.name).unwrap();

            let mut signal_level = module.signal_level.clone();

            if module.inputs[0] == SignalType::LOW {
                mutable_module.signal_level = if module.signal_level == SignalType::LOW {SignalType::HIGH} else {SignalType::LOW};
                signal_level = mutable_module.signal_level.clone();
            }

            for target in &module.targets {
                let target_module = modules.get_mut(target).unwrap();
                target_module.inputs.push(signal_level);

                if signal_level == SignalType::HIGH {
                    target_module.high_count += 1;
                } else {
                    target_module.low_count += 1;
                }
                                
                println!("Broadcasting {:?} to {}",signal_level,target_module.name)
            }
        }  ,
        _ => {}
    }

    modules.get_mut(&module.name).unwrap().inputs = Vec::new();

}

fn run(modules:&mut HashMap<String,Module>) {
    let mut modules_to_run: Vec<String> = Vec::new();

    modules_to_run.push(String::from("broadcaster"));

    let module = modules.get("broadcaster").unwrap().clone();
    for target in &module.targets {
        let target_module = modules.get_mut(target).unwrap();
        target_module.inputs.push(SignalType::LOW);
        modules_to_run.push(target_module.name.clone());
        println!("Broadcasting {:?} to {}",SignalType::LOW,target_module.name)
    }
    modules.get_mut("broadcaster").unwrap().low_count += module.targets.len() as u32;

    // !modules.iter().all(|(n,m)| n == "output" || m.inputs.len() == 0)
    let mut i= 0;
    while i == 0 || !modules.iter().all(|(_,m)| m.module_type != ModuleType::FlipFlop || m.signal_level == SignalType::LOW) {
        i = 1;
        // || !modules.clone().iter().all(|(_,m)| m.module_type != ModuleType::FlipFlop || m.signal_level == SignalType::LOW)

        for module in modules.clone().keys() {
            run_module(modules,module.to_owned());
        }
    }
}