use std::collections::HashMap;
use std::error::Error;
use serde::{Serialize, Deserialize};
use std::io::{self, Write};

mod performance;
use performance::{run_performance_test, generate_performance_graph};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct State {
    name: String,
    is_accepting: bool,
    transitions: HashMap<char, String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct DFA {
    states: HashMap<String, State>,
    alphabet: Vec<char>,
    start_state: String,
}

impl DFA {
    fn process(&self, input: &str, verbose: bool) -> Result<bool, Box<dyn Error>> {
        let mut current_state = self.states.get(&self.start_state)
            .ok_or("Start state not found")?;
        
        if verbose {
            println!("Starting state: {}", current_state.name);
        }
        
        for (i, c) in input.chars().enumerate() {
            if !self.alphabet.contains(&c) {
                return Err(format!("Invalid input character '{}' at position {}", c, i).into());
            }
            
            if let Some(next_state_name) = current_state.transitions.get(&c) {
                current_state = self.states.get(next_state_name)
                    .ok_or_else(|| format!("State '{}' not found", next_state_name))?;
                
                if verbose {
                    println!("Transition: {} --({})--> {}", 
                             current_state.name, c, next_state_name);
                }
            } else {
                return Err(format!("No transition for '{}' from state '{}'", c, current_state.name).into());
            }
        }
        
        if verbose {
            println!("Final state: {} ({})", 
                     current_state.name, 
                     if current_state.is_accepting { "accepting" } else { "non-accepting" });
        }
        
        Ok(current_state.is_accepting)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Load DFA from JSON file
    let dfa_json = std::fs::read_to_string("dfa_definition.json")?;
    println!("Loaded DFA definition from JSON file.");
    let dfa: DFA = serde_json::from_str(&dfa_json)?;
    
    // Define test cases
    let test_cases = [
        "00",
        "100",
        "1100",
        "101",
        "10",
        "",
        "0000000000", // Longer string to test performance
    ];

    // Run performance test
    println!("\nRunning performance test...");
    let performance_results = run_performance_test(&dfa, &test_cases, 1000)?;
    generate_performance_graph(&performance_results)?;

    // Interactive testing
    println!("\nEntering interactive testing mode.");
    loop {
        print!("Enter a string to test (or 'q' to quit): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input == "q" {
            break;
        }

        match dfa.process(input, true) {
            Ok(result) => println!("The string '{}' is {} by the DFA.", 
                                   input, 
                                   if result { "accepted" } else { "rejected" }),
            Err(e) => println!("Error: {}", e),
        }
    }

    println!("Thank you for using the DFA Simulator!");

    Ok(())
}
