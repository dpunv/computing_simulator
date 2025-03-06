// Contents: Turing machine that writes the reverse of a string
1
h
B
1 2 3 4 5 6 7 8 9 10 11 12 h
a b
a b B $
1 B 2 B R
2 a 2 a R
2 b 2 b R
2 B 3 B L
3 a 4 $ R
4 $ 4 $ R
4 B 5 B R
5 a 5 a R
5 b 5 b R
5 B 6 a L
6 a 6 a L
6 b 6 b L
6 B 7 B L
7 $ 7 $ L
7 a 4 $ R
7 b 8 $ R
3 b 8 $ R
8 $ 8 $ R
8 B 9 B R
9 a 9 a R
9 b 9 b R
9 B 10 b L
10 a 10 a L
10 b 10 b L
10 B 11 B L
11 $ 11 $ L
11 b 8 $ R
11 a 4 $ R
3 B 12 B R
7 B 12 B R
11 B 12 B R
12 $ 12 B R
12 B h B R