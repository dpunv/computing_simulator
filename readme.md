# Turing machine simulator

Program to simulate a (Non-Deterministic) Single-Tape Turing Machine with a tape of infinite length to the right and left and a finite set of states
The program also accept a description for (Non-Deterministic) Finite State Automaton.

## Description file of a Turing Machine

The description of the Turing Machine is a text file with the following format:

1. The first line contains the initial state of the Turing Machine (name is without spaces)
2. The second line contains the final states of the Turing Machine (names are without spaces), separated by spaces
3. The third line contains the blank symbol of the Turing Machine (symbol is without spaces) (omitted for Finite State Automaton)
4. The third line contains the set of states of the Turing Machine (names are without spaces), separated by spaces
5. The fourth line contains the input alphabet of the Turing Machine (symbols are without spaces), separated by spaces
6. The fifth line contains the tape alphabet of the Turing Machine (symbols are without spaces), separated by spaces (omitted for Finite State Automaton)
7. From the sixth line to the end of the file, each line contains the description of a transition of the Turing Machine
   The description of a transition is a string with the following format:
   \<state> \<symbol> <new_state> <new_symbol> \<direction>
   where:

   - \<state> is the name of the state of the Turing Machine
   - \<symbol> is the symbol read by the Turing Machine
   - <new_state> is the name of the new state of the Turing Machine
   - <new_symbol> is the symbol written by the Turing Machine (omitted for Finite State Automaton)
   - \<direction> is the direction in which the Turing Machine moves the head (omitted for Finite State Automaton)(L for left, R for right, S for stay)
8. Lines starting with // are considered comments and are ignored.
