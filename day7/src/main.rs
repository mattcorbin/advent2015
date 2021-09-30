use std::fs;
use std::collections::HashMap;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum OpCodes {
    ASSIGN,
    NOT,
    AND,
    OR,
    LSHIFT,
    RSHIFT,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Operation {
    op_code: OpCodes,
    numeric_inputs:Option<[u16; 2]>,
    wire_inputs: Option<[String; 2]>,
    output: String
}


fn parse_line(line: &str) -> Operation {
    let retval: Operation;
    let halves = line.split(" -> ").collect::<Vec<&str>>();
    let output = halves[1].to_string();
    if line.contains("NOT") {
        let input = halves[0].replace("NOT ", "");
        match input.parse::<u16>() {
            Ok(result) => {
                let numeric_inputs = [result, 0];
                retval = Operation {
                    op_code: OpCodes::NOT,
                    numeric_inputs: Some(numeric_inputs),
                    wire_inputs: None,
                    output
                };
            },
            Err(_) => {
                let wire_inputs = [input.to_string(), "".to_string()];
                retval = Operation {
                    op_code: OpCodes::NOT,
                    numeric_inputs: None,
                    wire_inputs: Some(wire_inputs),
                    output
                };
            },
        };
    } else if line.contains("AND") {
        let inputs = halves[0].split(" AND ").collect::<Vec<&str>>();

        let mut numeric_inputs_array = [0u16; 2];
        let mut wire_inputs_array =  ["".to_string(), "".to_string()];

        let mut has_numeric_inputs = false;
        let mut has_wire_inputs = false;

        let mut numeric_inputs: Option<[u16; 2]> = None;
        let mut wire_inputs: Option<[String; 2]> = None;

        let input = inputs[0];

        match inputs[0].parse::<u16>() {
            Ok(result) => {
                numeric_inputs_array[0] = result;
                has_numeric_inputs = true;
            },
            Err(_) => {
                wire_inputs_array[0] = input.to_string();
                has_wire_inputs = true;
            }
        }

        let input = inputs[1];

        match inputs[1].parse::<u16>() {
            Ok(result) => {
                if has_numeric_inputs {
                    numeric_inputs_array[1] = result;

                } else {
                    numeric_inputs_array[0] = result;
                    has_numeric_inputs = true;
                }
            },
            Err(_) => {
                if has_wire_inputs {
                    wire_inputs_array[1] = input.to_string();
                } else {
                    wire_inputs_array[0] = input.to_string();
                    has_wire_inputs = true;
                }
            }
        }

        if has_numeric_inputs {
            numeric_inputs = Some(numeric_inputs_array);
        }

        if has_wire_inputs {
            wire_inputs = Some(wire_inputs_array);
        }

        retval = Operation {
            op_code: OpCodes::AND,
            numeric_inputs,
            wire_inputs,
            output
        }
    } else if line.contains("OR") {
        let inputs = halves[0].split(" OR ").collect::<Vec<&str>>();

        let mut numeric_inputs_array = [0u16; 2];
        let mut wire_inputs_array =  ["".to_string(), "".to_string()];

        let mut has_numeric_inputs = false;
        let mut has_wire_inputs = false;

        let mut numeric_inputs: Option<[u16; 2]> = None;
        let mut wire_inputs: Option<[String; 2]> = None;

        let input = inputs[0];

        match inputs[0].parse::<u16>() {
            Ok(result) => {
                numeric_inputs_array[0] = result;
                has_numeric_inputs = true;
            },
            Err(_) => {
                wire_inputs_array[0] = input.to_string();
                has_wire_inputs = true;
            }
        }

        let input = inputs[1];

        match inputs[1].parse::<u16>() {
            Ok(result) => {
                if has_numeric_inputs {
                    numeric_inputs_array[1] = result;

                } else {
                    numeric_inputs_array[0] = result;
                    has_numeric_inputs = true;
                }
            },
            Err(_) => {
                if has_wire_inputs {
                    wire_inputs_array[1] = input.to_string();
                } else {
                    wire_inputs_array[0] = input.to_string();
                    has_wire_inputs = true;
                }
            }
        }

        if has_numeric_inputs {
            numeric_inputs = Some(numeric_inputs_array);
        }

        if has_wire_inputs {
            wire_inputs = Some(wire_inputs_array);
        }

        retval = Operation {
            op_code: OpCodes::OR,
            numeric_inputs,
            wire_inputs,
            output
        }
    } else if line.contains("LSHIFT") {
        let inputs = halves[0].split(" LSHIFT ").collect::<Vec<&str>>();
        let wire_inputs = [inputs[0].to_string(), "".to_string()];
        let numeric_inputs = [inputs[1].parse::<u16>().expect("LSHIFT second input should be integer literal"), 0];
        retval = Operation {
            op_code: OpCodes::LSHIFT,
            numeric_inputs: Some(numeric_inputs),
            wire_inputs: Some(wire_inputs),
            output
        }
    } else if line.contains("RSHIFT") {
        let inputs = halves[0].split(" RSHIFT ").collect::<Vec<&str>>();
        let wire_inputs = [inputs[0].to_string(), "".to_string()];
        let numeric_inputs = [inputs[1].parse::<u16>().expect("RSHIFT second input should be integer literal"), 0];
        retval = Operation {
            op_code: OpCodes::RSHIFT,
            numeric_inputs: Some(numeric_inputs),
            wire_inputs: Some(wire_inputs),
            output
        }
    } else {
        let input = halves[0];
        match input.parse::<u16>() {
            Ok(result) => {
                let input_array = [result, 0];
                retval = Operation {
                    op_code: OpCodes::ASSIGN,
                    numeric_inputs: Some(input_array),
                    wire_inputs: None,
                    output
                }
            }
            Err(_) => {
                let input_array = [input.to_string(), "".to_string()];
                retval = Operation {
                    op_code: OpCodes::ASSIGN,
                    numeric_inputs: None,
                    wire_inputs: Some(input_array),
                    output
                }
            }
        }
    }
    retval
}

fn perform_operation(operation: Operation) -> u16 {
    match operation.op_code {
        OpCodes::ASSIGN => operation.numeric_inputs.expect("malformed op")[0],
        OpCodes::NOT => !operation.numeric_inputs.expect("malformed op")[0],
        OpCodes::AND => operation.numeric_inputs.expect("malformed op")[0] & operation.numeric_inputs.expect("malformed op")[1],
        OpCodes::OR => operation.numeric_inputs.expect("malformed op")[0] | operation.numeric_inputs.expect("malformed op")[1],
        OpCodes::LSHIFT => operation.numeric_inputs.expect("malformed op")[1] << operation.numeric_inputs.expect("malformed op")[0],
        OpCodes::RSHIFT => operation.numeric_inputs.expect("malformed op")[1] >> operation.numeric_inputs.expect("malformed op")[0],
    }
}

fn run_stuff(override_b: Option<u16>) -> u16 {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    let mut instruction_map: HashMap<String, Operation> = HashMap::new();
    let mut dependancy_map: HashMap<String, Option<[String; 2]>> = HashMap::new();
    let mut values_map: HashMap<String, u16> = HashMap::new();
    for line in input.lines() {
        let op = parse_line(line);
        dependancy_map.insert(op.output.clone(), op.wire_inputs.clone());
        instruction_map.insert(op.output.clone(), op);
    }
    for (key, value) in dependancy_map.clone().iter() {
        if *value == None {
            values_map.insert(
                key.to_string(),
                perform_operation(
                    instruction_map
                        .get(key)
                        .expect(&format!("Operation {0} should exist in instruction map", key))
                        .clone()
                )
            );
            dependancy_map.remove(key);
        }
    }
    if let Some(b) = override_b {
        values_map.insert("b".to_string(), b);
    }
    while dependancy_map.len() > 0 {
        for (key, value) in dependancy_map.clone().iter() {
            let values = value.clone().expect("Values should exist at this point");
            let mut ready = true;
            for item in values {
                if item != "".to_string() && !values_map.contains_key(&item) {
                    ready = false;
                    break;
                }
            }
            if !ready {
                continue;
            }
            let old_operation = instruction_map.get(key).expect("should exist");
            let new_operation: Operation;
            match old_operation.op_code {
                OpCodes::ASSIGN => {
                    let numeric_inputs = [*values_map.get(&old_operation.wire_inputs.clone().expect("")[0]).expect(""), 0];
                    new_operation = Operation {
                        op_code: OpCodes::ASSIGN,
                        numeric_inputs: Some(numeric_inputs),
                        wire_inputs: None,
                        output: "".to_string()
                    }
                }
                OpCodes::NOT => {
                    let numeric_inputs = [*values_map.get(&old_operation.wire_inputs.clone().expect("")[0]).expect(""), 0];
                    new_operation = Operation {
                        op_code: OpCodes::NOT,
                        numeric_inputs: Some(numeric_inputs),
                        wire_inputs: None,
                        output: "".to_string()
                    }
                }
                OpCodes::AND => {
                    let mut numeric_inputs = [0u16, 0u16];
                    match old_operation.numeric_inputs {
                        None => {
                            for (idx, val) in old_operation.wire_inputs.clone().expect("").iter().enumerate() {
                                numeric_inputs[idx] = *values_map.get(val).expect("");
                            }
                        }
                        Some(inputs) => {
                            match old_operation.wire_inputs.clone().clone() {
                                None => {
                                    numeric_inputs = inputs;
                                }
                                Some(wires) => {
                                    numeric_inputs[0] = inputs[0];
                                    numeric_inputs[1] = *values_map.get(&wires[0]).expect("");
                                }
                            }
                        }
                    }
                    new_operation = Operation {
                        op_code: OpCodes::AND,
                        numeric_inputs: Some(numeric_inputs),
                        wire_inputs: None,
                        output: "".to_string()
                    }
                }
                OpCodes::OR => {
                    let mut numeric_inputs = [0u16, 0u16];
                    match old_operation.numeric_inputs {
                        None => {
                            for (idx, val) in old_operation.wire_inputs.clone().expect("").iter().enumerate() {
                                numeric_inputs[idx] = *values_map.get(val).expect("");
                            }
                        }
                        Some(inputs) => {
                            match old_operation.wire_inputs.clone().clone() {
                                None => {
                                    numeric_inputs = inputs;
                                }
                                Some(wires) => {
                                    numeric_inputs[0] = inputs[0];
                                    numeric_inputs[1] = *values_map.get(&wires[0]).expect("");
                                }
                            }
                        }
                    }
                    new_operation = Operation {
                        op_code: OpCodes::OR,
                        numeric_inputs: Some(numeric_inputs),
                        wire_inputs: None,
                        output: "".to_string()
                    }
                }
                OpCodes::LSHIFT => {
                    let numeric_inputs = [
                        old_operation.numeric_inputs.expect("")[0],
                        *values_map.get(&old_operation.wire_inputs.clone().expect("")[0]).expect("")
                    ];
                    new_operation = Operation {
                        op_code: OpCodes::LSHIFT,
                        numeric_inputs: Some(numeric_inputs),
                        wire_inputs: None,
                        output: "".to_string()
                    };
                }
                OpCodes::RSHIFT => {
                    let numeric_inputs = [
                        old_operation.numeric_inputs.expect("")[0],
                        *values_map.get(&old_operation.wire_inputs.clone().expect("")[0]).expect("")
                    ];
                    new_operation = Operation {
                        op_code: OpCodes::RSHIFT,
                        numeric_inputs: Some(numeric_inputs),
                        wire_inputs: None,
                        output: "".to_string()
                    };
                }
            }
            values_map.insert(key.to_string(), perform_operation(new_operation));
            dependancy_map.remove(key);
        }
    }
    *values_map.get("a").expect("a should exist")
}

fn main() {
    let a = run_stuff(None);
    println!("signal to a: {0}", run_stuff(None));
    println!("signal to a with override: {0}", run_stuff(Some(a)));
}
