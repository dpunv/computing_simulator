// this is a turing machine that matches the number of a's and b's in a string
// given a string made of a number of a's, it maches the number of b's that follow
// if the number of b's is greater or equal to the number of a's, it accepts
1
h
B
1 2 3 4 5 h
a b
a b B $ #
1 B 2 B R
2 B h B L
2 a 3 $ R
3 a 3 a R
3 $ 3 $ R
3 # 3 # R
3 B 4 # L
3 b 4 # L
4 $ 4 $ L
4 # 4 # L
4 a 3 $ R
4 B 5 B R
5 $ 5 a R
5 # 5 b R
5 B h B R
