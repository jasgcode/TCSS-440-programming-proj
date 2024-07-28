# Simple DFA Simulator

This project implements a Deterministic Finite Automaton (DFA) Simulator in Rust. It allows users to define a DFA using JSON, simulate its operation on input strings, and determine whether input strings are accepted or rejected by the DFA.

## Features

1. Define a DFA using JSON encoding
2. Simulate the DFA's operation on input strings
3. Determine whether input strings are accepted or rejected by the DFA

## Requirements

- Rust (latest stable version)
- Cargo (Rust's package manager)

## Setup

1. Clone this repository or download the source code.
2. Navigate to the project directory in your terminal.

## Usage

1. Define your DFA in a file named `dfa_definition.json` in the project root. The JSON should follow this structure:

   ```json
   {
     "states": {
       "state_name": {
         "name": "state_name",
         "is_accepting": boolean,
         "transitions": {
           "input_symbol": "next_state_name"
         }
       },
       ...
     },
     "alphabet": ["symbol1", "symbol2", ...],
     "start_state": "initial_state_name"
   }

# Run the simulator:
### cargo run

### When prompted, enter a string to test against the DFA. The simulator will inform you whether the string is accepted or rejected.
Enter 'q' to quit the program.

## Example
### Given a DFA that accepts strings ending with "00":
    ```{
    "states": {
        "q0": {
        "name": "q0",
        "is_accepting": false,
        "transitions": {
            "0": "q1",
            "1": "q0"
        }
        },
        "q1": {
        "name": "q1",
        "is_accepting": false,
        "transitions": {
            "0": "q2",
            "1": "q0"
        }
        },
        "q2": {
        "name": "q2",
        "is_accepting": true,
        "transitions": {
            "0": "q2",
            "1": "q0"
        }
        }
    },
    "alphabet": ["0", "1"],
    "start_state": "q0"
    }

## Running the simulator Example:
cargo run
Enter a string to test (or 'q' to quit):
100
The string '100' is rejected by the DFA.
Enter a string to test (or 'q' to quit):
1100
The string '1100' is accepted by the DFA.
Enter a string to test (or 'q' to quit):
q


# Notes

### Ensure your input strings only contain symbols defined in the DFA's alphabet.
### The DFA simulator processes the entire input string before determining acceptance.