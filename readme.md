# Turing machine simulator

Program to simulate a (Non-Deterministic) (Multi-tape) Turing Machine with a tape of infinite length to the right and left and a finite set of states
The program also accept a description for (Non-Deterministic) Finite State Automaton and (Non-Deterministic Pushdown automata).

## Description file of a Turing Machine

The description of the Turing Machine is a text file with the following format:

1. The first line contains the initial state of the Turing Machine (name is without spaces)
2. The second line contains the acceptance state of the Turing Machine (name is without spaces)
3. The third line contains the acceptance state of the Turing Machine (name is without spaces)
4. The fourth line contains the final states of the Turing Machine (names are without spaces), separated by spaces
5. The fifth line contains the blank symbol of the Turing Machine (symbol is without spaces) (omitted for Finite State Automaton)
6. The sixth line contains the set of states of the Turing Machine (names are without spaces), separated by spaces
7. The seventh line contains the input alphabet of the Turing Machine (symbols are without spaces), separated by spaces
8. The eight line contains the tape alphabet of the Turing Machine (symbols are without spaces), separated by spaces (omitted for Finite State Automaton)
9. The tenth line contains the number of tapes
10. From the tenth line to the end of the file, each line contains the description of a transition of the Turing Machine. The description of a transition is a string with the following format: \<state> <new_state> \<symbols_list> <new_symbol_lists> \<directions_list> where:

- \<state> is the name of the state of the Turing Machine
- <new_state> is the name of the new state of the Turing Machine
- \<symbols_list> is the symbols read by the Turing Machine on the tapes
- <new_symbols_list> is the symbol written by the Turing Machine on the tapes
- \<directions_list> is the direction in which the Turing Machine moves the heads on the tapes (L for left, R for right, S for stay)

11. Lines starting with // are considered comments and are ignored.
