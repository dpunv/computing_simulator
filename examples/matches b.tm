// file: matches b.tm
// Project: Computing Simulator
// author: dp
// this is a turing machine that matches the number of a's and b's in a string
// given a string made of a number of a's, it maches the number of b's that follow
// if the number of b's is greater or equal to the number of a's, it accepts
tm
// initial state
1
// accept state

//reject state

// halt state
h
// blank symbol
B
// states
1 2 3 4 5 h
// input alphabet
a b
// tape alphabet
a b B $ #
// number of tapes
1
// transitions
1 2 B B R
2 h B B L
2 3 a $ R
3 3 a a R
3 3 $ $ R
3 3 # # R
3 4 B # L
3 4 b # L
4 4 $ $ L
4 4 # # L
4 3 a $ R
4 5 B B R
5 5 $ a R
5 5 # b R
5 h B B R
