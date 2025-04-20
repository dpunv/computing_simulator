# Computing Simulator

A program to simulate computational models execution. In particular, the supported models are:

- Turing machine (singletape, multitape, deterministic, non-deterministic)
- Finite states automata (deterministic, non-deterministic, epsilon)
- Pushdown automata (deterministic, non-deterministic, epsilon)
- RAM machine
- lambda calculus

## Description of the input files

The input files are in text format. Each line that starts with // is a comment and is ignored. The first line of the file is the name of the model:

- `tm` for Turing Machine
- `fsm` for Finite States Automata
- `pda` for Pushdown Automata
- `ram` for RAM Machine
- `lambda` for lambda calculus
- `tm_e` for Turing Machine encoding
- `ram_e` for RAM Machine encoding

Then there are parameters specific for each model, on each line.
More details on the parameters and what they mean can be found in the documentation.

## Example of a Turing Machine input file

```text
// This is a comment
tm
// initial state
q0
// accept state
q1
// reject state
q2
// halt state
q3
// blank symbol
_
// states
q0 q1 q2 q3
// input symbols
a b c
// tape symbols
a b c _
// transitions
q0 a q1 b R
q1 b q2 c L
q2 c q3 a R
q3 _ q1 _ R
```

## How to run the program

To run the program, you need to have cargo installed. More infromation on rust, cargo and how to install them can be found [here](https://www.rust-lang.org).

First, clone the repository:

```bash
git clone https://github.com/dpunv/computing_simulator.git
cd computing_simulator
```

Then, build the project:

```bash
cargo build --release
```

This will create an executable file in the `target/release` directory.
To run the program, you need to pass the input file, the input string and other optional parameters. To check the parameters, and how to use the program, run:

```bash
./target/release/computing_simulator --help
```

To build the documentation, run:

```bash
cargo doc --open
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

If you want to contribute to the project, feel free to open an issue or a pull request. Any help is welcome.

## Authors

- dp
