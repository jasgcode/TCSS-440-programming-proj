use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
struct State {
    name: String,
    is_accepting: bool,
    transitions: HashMap<char, String>,
}

#[derive(Serialize, Deserialize)]
struct DFA {
    states: HashMap<String, State>,
    alphabet: Vec<char>,
    start_state: String,
}

impl DFA {
    fn process(&self, input: &str) -> bool {
        let mut current_state = self.states.get(&self.start_state).unwrap();
        
        for c in input.chars() {
            if let Some(next_state_name) = current_state.transitions.get(&c) {
                current_state = self.states.get(next_state_name).unwrap();
            } else {
                return false; // Invalid input
            }
        }
        
        current_state.is_accepting
    }
}

fn main() {
    // Load DFA from JSON file
    let dfa_json = std::fs::read_to_string("dfa_definition.json").expect("Failed to read JSON file");
    println!("JSON content:\n{}", dfa_json);  // Add this line
    let dfa: DFA = serde_json::from_str(&dfa_json).expect("Failed to parse JSON");

    // Simple command-line interface
    loop {
        println!("Enter a string to test (or 'q' to quit):");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if input == "q" {
            break;
        }

        let result = dfa.process(input);
        println!("The string '{}' is {} by the DFA.", input, if result { "accepted" } else { "rejected" });
    }
}
