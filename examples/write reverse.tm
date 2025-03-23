// file: write reverse.tm
// Project: Computing Simulator
// author: dp
// Contents: Turing machine that writes the reverse of a string
tm
1


h
B
1 2 3 4 5 6 7 8 9 10 11 12 h
a b
a b B $
1
1 2 B B R
2 2 a a R
2 2 b b R
2 3 B B L
3 4 a $ R
4 4 $ $ R
4 5 B B R
5 5 a a R
5 5 b b R
5 6 B a L
6 6 a a L
6 6 b b L
6 7 B B L
7 7 $ $ L
7 4 a $ R
7 8 b $ R
3 8 b $ R
8 8 $ $ R
8 9 B B R
9 9 a a R
9 9 b b R
9 10 B b L
10 10 a a L
10 10 b b L
10 11 B B L
11 11 $ $ L
11 8 b $ R
11 4 a $ R
3 12 B B R
7 12 B B R
11 12 B B R
12 12 $ B R
12 h B B R