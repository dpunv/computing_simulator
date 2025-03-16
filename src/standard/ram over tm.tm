// turing machine to execute a ram program
// initial state
1
// accept state

// reject state

// final states
h
// blanc symbol
_
// states
1 2 2a 2b 3 3a 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 51a 52 53 54 55 56 57 58 59 60 61 62 63 64 65 66 67 68 69 70 71 72 73 74 75 76 77 78 79 80 81 82 83 84 85 86 87 88 89 90 91 92 93 94 95 96 97 98 99 100 101 102 103 104 105 106 107 108 109 110 111 112 113 114 115 116 117 118 119 120 121 122 123 124 125 126 h
// input alphabet
0 1 # ,
// tape alphabet
0 1 # , _ $ a b c
// number of tapes
7
// tape 1: memory
// tape 2: input
// tape 3: IR (instruction register)
// tape 4: AR (address register)
// tape 5: ACC (accumulator)
// tape 6: PC (program counter)
// tape 7: output
// transitions
// initialization: move to the right position and copy the input sequence
1 2a _ _ R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
2a 2 1 _ R _ b R _ _ S _ _ S _ _ S _ _ S _ _ S
2a 2 0 _ R _ a R _ _ S _ _ S _ _ S _ _ S _ _ S
2 2 1 _ R _ 1 R _ _ S _ _ S _ _ S _ _ S _ _ S
2 2 0 _ R _ 0 R _ _ S _ _ S _ _ S _ _ S _ _ S
2 2b # # S _ _ L _ _ S _ _ S _ _ S _ _ S _ _ S
2b 2b # # S 0 0 L _ _ S _ _ S _ _ S _ _ S _ _ S
2b 2b # # S 1 1 L _ _ S _ _ S _ _ S _ _ S _ _ S
2b 2b # # S a a L _ _ S _ _ S _ _ S _ _ S _ _ S
2b 2b # # S b b L _ _ S _ _ S _ _ S _ _ S _ _ S
2b 3 # # R _ _ S _ _ S _ _ S _ _ S _ 0 S _ _ S
3a 3 # # R _ _ S _ _ S _ _ S _ _ S _ _ R _ _ S
// main loop starts here
// 1 fetch the instuction
// 1.1 find the cell in memory
3 3 0 0 R _ _ S _ _ S _ _ S _ _ S 0 0 R _ _ S
3 3 1 1 R _ _ S _ _ S _ _ S _ _ S 1 1 R _ _ S
3 4 , , R _ _ S _ _ R _ _ S _ _ S _ _ S _ _ S
3 5 1 1 R _ _ S _ _ S _ _ S _ _ S 0 0 L _ _ S
3 5 0 0 R _ _ S _ _ S _ _ S _ _ S 1 1 L _ _ S
3 5 , , R _ _ S _ _ S _ _ S _ _ S 0 0 L _ _ S
3 5 , , R _ _ S _ _ S _ _ S _ _ S 1 1 L _ _ S
5 5 0 0 R _ _ S _ _ S _ _ S _ _ S 0 0 L _ _ S
5 5 1 1 R _ _ S _ _ S _ _ S _ _ S 0 0 L _ _ S
5 5 , , R _ _ S _ _ S _ _ S _ _ S 0 0 L _ _ S
5 5 # # R _ _ S _ _ S _ _ S _ _ S 0 0 L _ _ S
5 5 , , R _ _ S _ _ S _ _ S _ _ S 1 1 L _ _ S
5 5 # # R _ _ S _ _ S _ _ S _ _ S 1 1 L _ _ S
5 5 0 0 R _ _ S _ _ S _ _ S _ _ S 1 1 L _ _ S
5 5 1 1 R _ _ S _ _ S _ _ S _ _ S 1 1 L _ _ S
5 5 0 0 R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
5 5 1 1 R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
5 5 , , R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
5 3 # # R _ _ S _ _ S _ _ S _ _ S _ _ R _ _ S
// 1.2 copy instruction in IR
4 7 0 0 R _ _ S 0 0 R _ _ S _ _ S _ _ S _ _ S
4 7 1 1 R _ _ S 0 1 R _ _ S _ _ S _ _ S _ _ S
4 7 0 0 R _ _ S 1 0 R _ _ S _ _ S _ _ S _ _ S
4 7 1 1 R _ _ S 1 1 R _ _ S _ _ S _ _ S _ _ S
4 7 0 0 R _ _ S _ 0 R _ _ S _ _ S _ _ S _ _ S
4 7 1 1 R _ _ S _ 1 R _ _ S _ _ S _ _ S _ _ S
7 8 0 0 R _ _ S 0 0 R _ _ S _ _ S _ _ S _ _ S
7 8 1 1 R _ _ S 0 1 R _ _ S _ _ S _ _ S _ _ S
7 8 0 0 R _ _ S 1 0 R _ _ S _ _ S _ _ S _ _ S
7 8 1 1 R _ _ S 1 1 R _ _ S _ _ S _ _ S _ _ S
7 8 0 0 R _ _ S _ 0 R _ _ S _ _ S _ _ S _ _ S
7 8 1 1 R _ _ S _ 1 R _ _ S _ _ S _ _ S _ _ S
8 9 0 0 R _ _ S 0 0 R _ _ S _ _ S _ _ S _ _ S
8 9 1 1 R _ _ S 0 1 R _ _ S _ _ S _ _ S _ _ S
8 9 0 0 R _ _ S 1 0 R _ _ S _ _ S _ _ S _ _ S
8 9 1 1 R _ _ S 1 1 R _ _ S _ _ S _ _ S _ _ S
8 9 0 0 R _ _ S _ 0 R _ _ S _ _ S _ _ S _ _ S
8 9 1 1 R _ _ S _ 1 R _ _ S _ _ S _ _ S _ _ S
9 10 0 0 R _ _ S 0 0 R _ _ S _ _ S _ _ S _ _ S
9 10 1 1 R _ _ S 0 1 R _ _ S _ _ S _ _ S _ _ S
9 10 0 0 R _ _ S 1 0 R _ _ S _ _ S _ _ S _ _ S
9 10 1 1 R _ _ S 1 1 R _ _ S _ _ S _ _ S _ _ S
9 10 0 0 R _ _ S _ 0 R _ _ S _ _ S _ _ S _ _ S
9 10 1 1 R _ _ S _ 1 R _ _ S _ _ S _ _ S _ _ S
// 1.3 copy address in AR
10 10 0 0 R _ _ S _ _ S 0 0 R _ _ S _ _ S _ _ S
10 10 0 0 R _ _ S _ _ S _ 0 R _ _ S _ _ S _ _ S
10 10 1 1 R _ _ S _ _ S _ 1 R _ _ S _ _ S _ _ S
// 1.4 return the head to the beginning of memory tape
10 11 # # L _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
10 12 _ _ R _ _ S _ _ S _ _ S _ _ S _ _ L _ _ S
11 11 1 1 L _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
11 11 0 0 L _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
11 11 # # L _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
11 11 , , L _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
11 12 _ _ S _ _ S _ _ S _ _ S _ _ S _ _ L _ _ S
// 1.5 increment the PC
12 12 _ _ S _ _ S _ _ S _ _ S _ _ S 1 0 L _ _ S
12 13 _ _ S _ _ S _ _ S _ _ S _ _ S _ 1 L _ _ S
12 13 _ _ S _ _ S _ _ S _ _ S _ _ S 0 1 L _ _ S
// 1.6 return the head to the beginning of PC tape
13 13 _ _ S _ _ S _ _ S _ _ S _ _ S 0 0 L _ _ S
13 13 _ _ S _ _ S _ _ S _ _ S _ _ S 1 1 L _ _ S
13 14 _ _ S _ _ S _ _ L _ _ S _ _ S _ _ S _ _ S
// 1.7 return the head to the beginning of IR tape
14 14 _ _ S _ _ S 0 0 L _ _ S _ _ S _ _ S _ _ S
14 14 _ _ S _ _ S 1 1 L _ _ S _ _ S _ _ S _ _ S
14 15 _ _ S _ _ S _ _ S _ _ L _ _ S _ _ S _ _ S
// 1.8 return the head to the beginning of AR tape
15 15 _ _ S _ _ S _ _ S 0 0 L _ _ S _ _ S _ _ S
15 15 _ _ S _ _ S _ _ S 1 1 L _ _ S _ _ S _ _ S
15 16 _ _ S _ _ S _ _ R _ _ S _ _ S _ _ S _ _ S
// 2 decode the instruction
// 1 -> 2
16 17 _ _ S _ _ S 0 _ R _ _ S _ _ S _ _ S _ _ S
16 18 _ _ S _ _ S 1 _ R _ _ S _ _ S _ _ S _ _ S
// 2 --> 4
17 19 _ _ S _ _ S 0 _ R _ _ S _ _ S _ _ S _ _ S
17 20 _ _ S _ _ S 1 _ R _ _ S _ _ S _ _ S _ _ S
18 21 _ _ S _ _ S 0 _ R _ _ S _ _ S _ _ S _ _ S
18 22 _ _ S _ _ S 1 _ R _ _ S _ _ S _ _ S _ _ S
// 4 --> 8
19 23 _ _ S _ _ S 0 _ R _ _ S _ _ S _ _ S _ _ S
19 24 _ _ S _ _ S 1 _ R _ _ S _ _ S _ _ S _ _ S
20 25 _ _ S _ _ S 0 _ R _ _ S _ _ S _ _ S _ _ S
20 26 _ _ S _ _ S 1 _ R _ _ S _ _ S _ _ S _ _ S
21 27 _ _ S _ _ S 0 _ R _ _ S _ _ S _ _ S _ _ S
21 28 _ _ S _ _ S 1 _ R _ _ S _ _ S _ _ S _ _ S
22 29 _ _ S _ _ S 0 _ R _ _ S _ _ S _ _ S _ _ S
22 30 _ _ S _ _ S 1 _ R _ _ S _ _ S _ _ S _ _ S
// 8 --> 16
23 31 _ _ S _ _ S 0 _ R _ _ S _ _ R _ _ S _ _ S
23 32 _ _ S _ _ S 1 _ R _ _ S _ _ S _ _ S _ _ S
24 33 _ _ S _ _ S 0 _ R _ _ S _ _ S _ _ S _ _ S
24 34 _ _ S _ _ S 1 _ R _ _ S _ _ R _ _ S _ _ S
25 35 _ _ S _ _ S 0 _ R _ _ S _ _ R _ _ S _ _ S
25 36 _ _ S _ _ S 1 _ R _ _ S _ _ R _ _ S _ _ S
26 37 _ _ S _ _ S 0 _ R _ _ S _ _ R _ _ S _ _ S
26 38 _ _ S _ _ S 1 _ R _ _ S _ _ S _ _ S _ _ S
27 39 _ _ R _ _ S 0 _ R _ _ S _ _ S _ _ S _ _ S
27 40 _ _ S _ _ S 1 _ R _ _ S _ _ S _ _ R _ _ S
28 41 _ _ S _ _ S 0 _ R _ _ S _ _ R _ _ S _ _ S
28 42 _ _ S _ _ S 1 _ R _ _ S _ _ S _ _ S _ _ S
29 43 _ _ S _ _ S 0 _ R _ _ S _ _ S _ _ S _ _ S
29 44 _ _ S _ _ S 1 _ R _ _ S _ _ S _ _ S _ _ S
30 45 _ _ S _ _ S 0 _ R _ _ S _ _ S _ _ S _ _ S
30 46 _ _ S _ _ S 1 _ R _ _ S _ _ S _ _ S _ _ S
// 3. instruction execution
// 3.1 READ
// 3.1.1 delete the content of ACC
31 31 _ _ S _ _ S _ _ S _ _ S 0 _ R _ _ S _ _ S
31 31 _ _ S _ _ S _ _ S _ _ S 1 _ R _ _ S _ _ S
31 48 _ _ S _ _ R _ _ S _ _ S _ _ S _ _ S _ _ S
// 3.1.2 go to the correct input position
48 48 _ _ S _ _ R _ _ S _ _ S _ _ S _ _ S _ _ S
48 48 _ _ S 0 0 R _ _ S _ _ S _ _ S _ _ S _ _ S
48 48 _ _ S 1 1 R _ _ S _ _ S _ _ S _ _ S _ _ S
48 47 _ _ S a a S _ _ S _ _ R _ _ S _ _ S _ _ S
48 47 _ _ S b b S _ _ S _ _ R _ _ S _ _ S _ _ S
// go to the end of the AR tape
47 47 _ _ S a a S _ _ S 0 0 R _ _ S _ _ S _ _ S
47 47 _ _ S a a S _ _ S 1 1 R _ _ S _ _ S _ _ S
47 47 _ _ S b b S _ _ S 0 0 R _ _ S _ _ S _ _ S
47 47 _ _ S b b S _ _ S 1 1 R _ _ S _ _ S _ _ S
47 47 _ _ S 0 0 S _ _ S 0 0 R _ _ S _ _ S _ _ S
47 47 _ _ S 0 0 S _ _ S 1 1 R _ _ S _ _ S _ _ S
47 47 _ _ S 1 1 S _ _ S 0 0 R _ _ S _ _ S _ _ S
47 47 _ _ S 1 1 S _ _ S 1 1 R _ _ S _ _ S _ _ S
47 47 _ _ S _ _ S _ _ S 0 0 R _ _ S _ _ S _ _ S
47 47 _ _ S _ _ S _ _ S 1 1 R _ _ S _ _ S _ _ S
47 49 _ _ S a a S _ _ S _ _ S _ _ S _ _ S _ _ S
47 49 _ _ S b b S _ _ S _ _ S _ _ S _ _ S _ _ S
47 49 _ _ S 0 0 S _ _ S _ _ S _ _ S _ _ S _ _ S
47 49 _ _ S 1 1 S _ _ S _ _ S _ _ S _ _ S _ _ S
47 49 _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
// 3.1.4 copy one symbol in input tape to ACC
49 50 _ _ S a a R _ _ S _ _ L _ 0 R _ _ S _ _ S
49 50 _ _ S b b R _ _ S _ _ L _ 1 R _ _ S _ _ S
49 50 _ _ S _ _ R _ _ S _ _ L _ _ R _ _ S _ _ S
49 50 _ _ S 0 0 R _ _ S _ _ L _ 0 R _ _ S _ _ S
49 50 _ _ S 1 1 R _ _ S _ _ L _ 1 R _ _ S _ _ S
// 3.1.5 decrease the AR by one
50 50 _ _ S 0 0 S _ _ S 0 1 L _ _ S _ _ S _ _ S
50 50 _ _ S 1 1 S _ _ S 0 1 L _ _ S _ _ S _ _ S
50 50 _ _ S _ _ S _ _ S 0 1 L _ _ S _ _ S _ _ S
50 47 _ _ S 0 0 S _ _ S 1 0 S _ _ S _ _ S _ _ S
50 47 _ _ S 1 1 S _ _ S 1 0 S _ _ S _ _ S _ _ S
50 47 _ _ S _ _ S _ _ S 1 0 S _ _ S _ _ S _ _ S
50 51 _ _ S 0 0 L _ _ S _ _ S _ _ S _ _ S _ _ S
50 51 _ _ S 1 1 L _ _ S _ _ S _ _ S _ _ S _ _ S
50 51 _ _ S _ _ L _ _ S _ _ S _ _ S _ _ S _ _ S
// 3.1.5 go to the initial position of the input tape
51 51 _ _ S 0 0 L _ _ S _ _ S _ _ S _ _ S _ _ S
51 51 _ _ S 1 1 L _ _ S _ _ S _ _ S _ _ S _ _ S
51 51 _ _ S _ _ L _ _ S _ _ S _ _ S _ _ S _ _ S
51 51a _ _ S a a L _ _ S _ _ S _ _ S _ _ S _ _ S
51 51a _ _ S b b L _ _ S _ _ S _ _ S _ _ S _ _ S
51a 51a _ _ S 0 0 L _ _ S _ _ S _ _ S _ _ S _ _ S
51a 51a _ _ S 1 1 L _ _ S _ _ S _ _ S _ _ S _ _ S
51a 52 _ _ S _ _ S _ _ S _ _ R _ _ S _ _ S _ _ S
// 3.1.6 go to the initial position of the AR tape
52 52 _ _ S _ _ S _ _ S 0 _ R _ _ S _ _ S _ _ S
52 52 _ _ S _ _ S _ _ S 1 _ R _ _ S _ _ S _ _ S
52 53 _ _ S _ _ S _ _ S _ _ S _ _ L _ _ S _ _ S
// 3.1.7 go to the initial position of the ACC tape
53 54 _ _ S _ _ S _ _ S _ _ S _ _ L _ _ S _ _ S
53 54 _ _ S _ _ S _ _ S _ _ S 0 _ L _ _ S _ _ S
53 54 _ _ S _ _ S _ _ S _ _ S 1 _ L _ _ S _ _ S
54 54 _ _ S _ _ S _ _ S _ _ S 0 0 L _ _ S _ _ S
54 54 _ _ S _ _ S _ _ S _ _ S 1 1 L _ _ S _ _ S
// 3.1.8 restart the loop
54 3a _ _ R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
// 3.2 MIR (Move Input Right)
// 3.2.1 go to the end of the AR tape
32 55 _ _ S _ _ S _ _ S _ _ R _ _ S _ _ S _ _ S
55 55 _ _ S _ _ S _ _ S 0 0 R _ _ S _ _ S _ _ S
55 55 _ _ S _ _ S _ _ S 1 1 R _ _ S _ _ S _ _ S
55 56 _ _ S _ _ R _ _ S _ _ S _ _ S _ _ S _ _ S
// 3.2.2 go to the correct input position
56 56 _ _ S 0 0 R _ _ S _ _ S _ _ S _ _ S _ _ S
56 56 _ _ S 1 1 R _ _ S _ _ S _ _ S _ _ S _ _ S
// 3.2.3 move one symbol to the right
56 57 _ _ S a 0 R _ _ S _ _ L _ _ S _ _ S _ _ S
56 57 _ _ S b 1 R _ _ S _ _ L _ _ S _ _ S _ _ S
// 3.2.3 subtract 1 from AR
57 57 _ _ S 0 0 S _ _ S 0 1 L _ _ S _ _ S _ _ S
57 57 _ _ S 1 1 S _ _ S 0 1 L _ _ S _ _ S _ _ S
57 57 _ _ S _ _ S _ _ S 0 1 L _ _ S _ _ S _ _ S
57 58 _ _ S 0 0 S _ _ S 1 0 S _ _ S _ _ S _ _ S
57 58 _ _ S 1 1 S _ _ S 1 0 S _ _ S _ _ S _ _ S
57 58 _ _ S _ _ S _ _ S 1 0 S _ _ S _ _ S _ _ S
57 59 _ _ S 0 0 L _ _ S _ _ R _ _ S _ _ S _ _ S
57 59 _ _ S 1 1 L _ _ S _ _ R _ _ S _ _ S _ _ S
57 59 _ _ S _ _ L _ _ S _ _ R _ _ S _ _ S _ _ S
// 3.2.4 go to the end of the AR tape
58 58 _ _ S 0 0 S _ _ S 0 0 R _ _ S _ _ S _ _ S
58 58 _ _ S 1 1 S _ _ S 0 0 R _ _ S _ _ S _ _ S
58 58 _ _ S _ _ S _ _ S 0 0 R _ _ S _ _ S _ _ S
58 58 _ _ S 0 0 S _ _ S 1 1 R _ _ S _ _ S _ _ S
58 58 _ _ S 1 1 S _ _ S 1 1 R _ _ S _ _ S _ _ S
58 58 _ _ S _ _ S _ _ S 1 1 R _ _ S _ _ S _ _ S
58 57 _ _ S 0 0 R _ _ S _ _ L _ _ S _ _ S _ _ S
58 57 _ _ S 1 1 R _ _ S _ _ L _ _ S _ _ S _ _ S
58 57 _ _ S _ _ R _ _ S _ _ L _ _ S _ _ S _ _ S
// 3.2.5 remove the content of ACC
59 60 _ _ S 0 a L _ _ S _ _ S _ _ S _ _ S _ _ S
59 60 _ _ S 1 b L _ _ S _ _ S _ _ S _ _ S _ _ S
59 60 _ _ S 0 a L _ _ S 0 _ R _ _ S _ _ S _ _ S
59 60 _ _ S 1 b L _ _ S 0 _ R _ _ S _ _ S _ _ S
59 60 _ _ S _ c L _ _ S 0 _ R _ _ S _ _ S _ _ S
59 60 _ _ S 0 a L _ _ S 1 _ R _ _ S _ _ S _ _ S
59 60 _ _ S 1 b L _ _ S 1 _ R _ _ S _ _ S _ _ S
59 60 _ _ S _ c L _ _ S 1 _ R _ _ S _ _ S _ _ S
60 60 _ _ S 0 0 L _ _ S 0 _ R _ _ S _ _ S _ _ S
60 60 _ _ S 1 1 L _ _ S 0 _ R _ _ S _ _ S _ _ S
60 60 _ _ S _ _ S _ _ S 0 _ R _ _ S _ _ S _ _ S
60 60 _ _ S 0 0 L _ _ S 1 _ R _ _ S _ _ S _ _ S
60 60 _ _ S 1 1 L _ _ S 1 _ R _ _ S _ _ S _ _ S
60 60 _ _ S _ _ S _ _ S 1 _ R _ _ S _ _ S _ _ S
60 60 _ _ S 0 0 L _ _ S _ _ S _ _ S _ _ S _ _ S
60 60 _ _ S 1 1 L _ _ S _ _ S _ _ S _ _ S _ _ S
// 3.2.6 restart the loop
60 3a _ _ R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
// 3.3 MIL (Move Input Left)
// 3.3.1 go to the end of the AR tape
33 61 _ _ S _ _ S _ _ S _ _ R _ _ S _ _ S _ _ S
61 61 _ _ S _ _ S _ _ S 0 0 R _ _ S _ _ S _ _ S
61 61 _ _ S _ _ S _ _ S 1 1 R _ _ S _ _ S _ _ S
61 62 _ _ S _ _ R _ _ S _ _ S _ _ S _ _ S _ _ S
// 3.3.2 go to the correct input position
62 62 _ _ S 0 0 R _ _ S _ _ S _ _ S _ _ S _ _ S
62 62 _ _ S 1 1 R _ _ S _ _ S _ _ S _ _ S _ _ S
// 3.3.3 move one symbol to the left
62 63 _ _ S a 0 L _ _ S _ _ L _ _ S _ _ S _ _ S
62 63 _ _ S b 1 L _ _ S _ _ L _ _ S _ _ S _ _ S
62 63 _ _ S c _ L _ _ S _ _ L _ _ S _ _ S _ _ S
// 3.3.3 subtract 1 from AR
63 63 _ _ S 0 0 S _ _ S 0 1 L _ _ S _ _ S _ _ S
63 63 _ _ S 1 1 S _ _ S 0 1 L _ _ S _ _ S _ _ S
63 63 _ _ S _ _ S _ _ S 0 1 L _ _ S _ _ S _ _ S
63 64 _ _ S 0 0 S _ _ S 1 0 S _ _ S _ _ S _ _ S
63 64 _ _ S 1 1 S _ _ S 1 0 S _ _ S _ _ S _ _ S
63 64 _ _ S _ _ S _ _ S 1 0 S _ _ S _ _ S _ _ S
63 65 _ _ S 0 0 R _ _ S _ _ R _ _ S _ _ S _ _ S
63 65 _ _ S 1 1 R _ _ S _ _ R _ _ S _ _ S _ _ S
63 65 _ _ S _ _ R _ _ S _ _ R _ _ S _ _ S _ _ S
// 3.3.4 go to the end of the AR tape
64 64 _ _ S 0 0 S _ _ S 0 0 R _ _ S _ _ S _ _ S
64 64 _ _ S 1 1 S _ _ S 0 0 R _ _ S _ _ S _ _ S
64 64 _ _ S _ _ S _ _ S 0 0 R _ _ S _ _ S _ _ S
64 64 _ _ S 0 0 S _ _ S 1 1 R _ _ S _ _ S _ _ S
64 64 _ _ S 1 1 S _ _ S 1 1 R _ _ S _ _ S _ _ S
64 64 _ _ S _ _ S _ _ S 1 1 R _ _ S _ _ S _ _ S
64 63 _ _ S 0 0 L _ _ S _ _ L _ _ S _ _ S _ _ S
64 63 _ _ S 1 1 L _ _ S _ _ L _ _ S _ _ S _ _ S
64 63 _ _ S _ _ L _ _ S _ _ L _ _ S _ _ S _ _ S
// 3.3.5 remove the content of ACC
65 66 _ _ S 0 a L _ _ S _ _ S _ _ S _ _ S _ _ S
65 66 _ _ S 1 b L _ _ S _ _ S _ _ S _ _ S _ _ S
65 66 _ _ S 0 a L _ _ S 0 _ R _ _ S _ _ S _ _ S
65 66 _ _ S 1 b L _ _ S 0 _ R _ _ S _ _ S _ _ S
65 66 _ _ S _ c L _ _ S 0 _ R _ _ S _ _ S _ _ S
65 66 _ _ S 0 a L _ _ S 1 _ R _ _ S _ _ S _ _ S
65 66 _ _ S 1 b L _ _ S 1 _ R _ _ S _ _ S _ _ S
65 66 _ _ S _ c L _ _ S 1 _ R _ _ S _ _ S _ _ S
66 66 _ _ S 0 0 L _ _ S 0 _ R _ _ S _ _ S _ _ S
66 66 _ _ S 1 1 L _ _ S 0 _ R _ _ S _ _ S _ _ S
66 66 _ _ S _ _ S _ _ S 0 _ R _ _ S _ _ S _ _ S
66 66 _ _ S 0 0 L _ _ S 1 _ R _ _ S _ _ S _ _ S
66 66 _ _ S 1 1 L _ _ S 1 _ R _ _ S _ _ S _ _ S
66 66 _ _ S _ _ S _ _ S 1 _ R _ _ S _ _ S _ _ S
66 66 _ _ S 0 0 L _ _ S _ _ S _ _ S _ _ S _ _ S
66 66 _ _ S 1 1 L _ _ S _ _ S _ _ S _ _ S _ _ S
// 3.3.6 restart the loop
66 3a _ _ R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
// 3.4 Write
// 3.4.1 copy the content of the ACC to the output tape
34 34 _ _ S _ _ S _ _ S _ _ S 0 0 R _ _ S _ 0 R
34 34 _ _ S _ _ S _ _ S _ _ S 1 1 R _ _ S _ 1 R
34 67 _ _ S _ _ S _ _ S _ _ S _ _ L _ _ S _ _ S
// 3.4.2 go to the beginning of the ACC tape
67 67 _ _ S _ _ S _ _ S _ _ S 0 0 L _ _ S _ _ S
67 67 _ _ S _ _ S _ _ S _ _ S 1 1 L _ _ S _ _ S
67 3a _ _ R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
// 3.5 Load execution
// 3.5.1 delete the content of ACC
35 35 _ _ S _ _ S _ _ S _ _ S 0 _ R _ _ S _ _ S
35 35 _ _ S _ _ S _ _ S _ _ S 1 _ R _ _ S _ _ S
35 68 _ _ R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
// 3.5.2 go to the correct memory position
68 69 # # R _ _ S _ _ S _ _ R _ _ S _ _ S _ _ S
68 69 0 0 R _ _ S _ _ S 0 0 R _ _ S _ _ S _ _ S
68 69 1 1 R _ _ S _ _ S 1 1 R _ _ S _ _ S _ _ S
68 71 0 0 R _ _ S _ _ S 1 1 S _ _ S _ _ S _ _ S
68 71 1 1 R _ _ S _ _ S 0 0 S _ _ S _ _ S _ _ S
69 69 0 0 R _ _ S _ _ S 0 0 R _ _ S _ _ S _ _ S
69 69 1 1 R _ _ S _ _ S 1 1 R _ _ S _ _ S _ _ S
69 70 , , R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
69 71 0 0 R _ _ S _ _ S 1 1 S _ _ S _ _ S _ _ S
69 71 1 1 R _ _ S _ _ S 0 0 S _ _ S _ _ S _ _ S
69 71 , , R _ _ S _ _ S 0 0 S _ _ S _ _ S _ _ S
69 71 , , R _ _ S _ _ S 1 1 S _ _ S _ _ S _ _ S
69 71 0 0 R _ _ S _ _ S _ _ L _ _ S _ _ S _ _ S
69 71 1 1 R _ _ S _ _ S _ _ L _ _ S _ _ S _ _ S
// 3.5.2.1 if the position is not correct go to the next
71 71 0 0 R _ _ S _ _ S 0 0 S _ _ S _ _ S _ _ S
71 71 0 0 R _ _ S _ _ S 1 1 S _ _ S _ _ S _ _ S
71 71 1 1 R _ _ S _ _ S 0 0 S _ _ S _ _ S _ _ S
71 71 1 1 R _ _ S _ _ S 1 1 S _ _ S _ _ S _ _ S
71 71 , , R _ _ S _ _ S 0 0 S _ _ S _ _ S _ _ S
71 71 , , R _ _ S _ _ S 1 1 S _ _ S _ _ S _ _ S
71 72 # # S _ _ S _ _ S 0 0 S _ _ S _ _ S _ _ S
71 72 # # S _ _ S _ _ S 1 1 S _ _ S _ _ S _ _ S
// 3.5.3 go to the beginning of the AR tape
72 72 # # S _ _ S _ _ S 0 0 L _ _ S _ _ S _ _ S
72 72 # # S _ _ S _ _ S 1 1 L _ _ S _ _ S _ _ S
72 68 # # S _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
// 3.5.4 copy the content of the memory to the ACC
70 70 0 0 R _ _ S _ _ S _ _ S _ 0 R _ _ S _ _ S
70 70 1 1 R _ _ S _ _ S _ _ S _ 1 R _ _ S _ _ S
70 73 # # L _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
// 3.5.5 go to the beginning of the memory tape
73 73 # # L _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
73 73 0 0 L _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
73 73 1 1 L _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
73 73 , , L _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
73 74 _ _ S _ _ S _ _ S _ _ S _ _ L _ _ S _ _ S
// 3.5.6 go to the beginning of the ACC tape
74 74 _ _ S _ _ S _ _ S _ _ S 0 0 L _ _ S _ _ S
74 74 _ _ S _ _ S _ _ S _ _ S 1 1 L _ _ S _ _ S
74 75 _ _ S _ _ S _ _ S _ _ L _ _ S _ _ S _ _ S
// 3.5.7 delete the content of the AR
75 75 _ _ S _ _ S _ _ S 0 _ L _ _ S _ _ S _ _ S
75 75 _ _ S _ _ S _ _ S 1 _ L _ _ S _ _ S _ _ S
// 3.5.8 restart the loop
75 3a _ _ R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
// 3.6 ADD
// 3.6.1 go to the end of the ACC tape
36 36 _ _ S _ _ S _ _ S _ _ S 0 0 R _ _ S _ _ S
36 36 _ _ S _ _ S _ _ S _ _ S 1 1 R _ _ S _ _ S
36 76 _ _ R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
// 3.6.2 go to the correct memory position
76 77 # # R _ _ S _ _ S _ _ R _ _ S _ _ S _ _ S
76 77 0 0 R _ _ S _ _ S 0 0 R _ _ S _ _ S _ _ S
76 77 1 1 R _ _ S _ _ S 1 1 R _ _ S _ _ S _ _ S
76 79 0 0 R _ _ S _ _ S 1 1 S _ _ S _ _ S _ _ S
76 79 1 1 R _ _ S _ _ S 0 0 S _ _ S _ _ S _ _ S
77 77 0 0 R _ _ S _ _ S 0 0 R _ _ S _ _ S _ _ S
77 77 1 1 R _ _ S _ _ S 1 1 R _ _ S _ _ S _ _ S
77 78 , , R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
77 79 0 0 R _ _ S _ _ S 1 1 S _ _ S _ _ S _ _ S
77 79 1 1 R _ _ S _ _ S 0 0 S _ _ S _ _ S _ _ S
77 79 , , R _ _ S _ _ S 0 0 S _ _ S _ _ S _ _ S
77 79 , , R _ _ S _ _ S 1 1 S _ _ S _ _ S _ _ S
77 79 0 0 R _ _ S _ _ S _ _ L _ _ S _ _ S _ _ S
77 79 1 1 R _ _ S _ _ S _ _ L _ _ S _ _ S _ _ S
// 3.6.2.1 if the position is not correct go to the next
79 79 0 0 R _ _ S _ _ S 0 0 S _ _ S _ _ S _ _ S
79 79 0 0 R _ _ S _ _ S 1 1 S _ _ S _ _ S _ _ S
79 79 1 1 R _ _ S _ _ S 0 0 S _ _ S _ _ S _ _ S
79 79 1 1 R _ _ S _ _ S 1 1 S _ _ S _ _ S _ _ S
79 79 , , R _ _ S _ _ S 0 0 S _ _ S _ _ S _ _ S
79 79 , , R _ _ S _ _ S 1 1 S _ _ S _ _ S _ _ S
79 80 # # S _ _ S _ _ S 0 0 S _ _ S _ _ S _ _ S
79 80 # # S _ _ S _ _ S 1 1 S _ _ S _ _ S _ _ S
// 3.6.3 go to the beginning of the AR tape
80 80 # # S _ _ S _ _ S 0 0 L _ _ S _ _ S _ _ S
80 80 # # S _ _ S _ _ S 1 1 L _ _ S _ _ S _ _ S
80 76 # # S _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
// 3.6.4 add the content of the memory to acc
// 3.6.4.1 go to the end of the memory cell
78 78 0 0 R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
78 78 1 1 R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
78 81 # # L _ _ S _ _ S _ _ S _ _ L _ _ S _ _ S
// 3.6.4.2 sum bit per bit
81 81 0 0 L _ _ S _ _ S _ _ S 0 0 L _ _ S _ _ S
81 81 0 0 L _ _ S _ _ S _ _ S 1 1 L _ _ S _ _ S
81 81 0 0 L _ _ S _ _ S _ _ S _ 0 L _ _ S _ _ S
81 81 1 1 L _ _ S _ _ S _ _ S 0 1 L _ _ S _ _ S
81 81 1 1 L _ _ S _ _ S _ _ S _ 1 L _ _ S _ _ S
81 82 1 1 L _ _ S _ _ S _ _ S 1 0 L _ _ S _ _ S
82 81 0 0 L _ _ S _ _ S _ _ S 0 1 L _ _ S _ _ S
82 81 0 0 L _ _ S _ _ S _ _ S _ 1 L _ _ S _ _ S
82 82 0 0 L _ _ S _ _ S _ _ S 1 0 L _ _ S _ _ S
82 82 1 1 L _ _ S _ _ S _ _ S 0 0 L _ _ S _ _ S
82 82 1 1 L _ _ S _ _ S _ _ S 1 1 L _ _ S _ _ S
82 82 1 1 L _ _ S _ _ S _ _ S _ 1 L _ _ S _ _ S
81 83 , , S _ _ S _ _ S _ _ S 0 0 L _ _ S _ _ S
81 83 , , S _ _ S _ _ S _ _ S 1 1 L _ _ S _ _ S
81 83 , , S _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
82 82 , , S _ _ S _ _ S _ _ S 1 0 L _ _ S _ _ S
82 83 , , S _ _ S _ _ S _ _ S 0 1 L _ _ S _ _ S
82 83 , , S _ _ S _ _ S _ _ S _ 1 L _ _ S _ _ S
// 3.6.5 go to the beginning of ACC tape
83 83 # # S _ _ S _ _ S _ _ S 0 0 L _ _ S _ _ S
83 83 # # S _ _ S _ _ S _ _ S 1 1 L _ _ S _ _ S
// 3.6.6 go to the beginning of the memory tape
83 83 # # L _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
83 83 0 0 L _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
83 83 1 1 L _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
83 83 , , L _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
83 84 _ _ S _ _ S _ _ S _ _ L _ _ S _ _ S _ _ S
// 3.6.7 delete the content of the AR
84 84 _ _ S _ _ S _ _ S 0 _ L _ _ S _ _ S _ _ S
84 84 _ _ S _ _ S _ _ S 1 _ L _ _ S _ _ S _ _ S
// 3.6.8 restart the loop
84 3a _ _ R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
// 3.7 SUB
// 3.7.1 go to the end of the ACC tape
37 37 _ _ S _ _ S _ _ S _ _ S 0 0 R _ _ S _ _ S
37 37 _ _ S _ _ S _ _ S _ _ S 1 1 R _ _ S _ _ S
37 85 _ _ R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
// 3.7.2 go to the correct memory position
85 86 # # R _ _ S _ _ S _ _ R _ _ S _ _ S _ _ S
85 86 0 0 R _ _ S _ _ S 0 0 R _ _ S _ _ S _ _ S
85 86 1 1 R _ _ S _ _ S 1 1 R _ _ S _ _ S _ _ S
85 88 0 0 R _ _ S _ _ S 1 1 S _ _ S _ _ S _ _ S
85 88 1 1 R _ _ S _ _ S 0 0 S _ _ S _ _ S _ _ S
86 86 0 0 R _ _ S _ _ S 0 0 R _ _ S _ _ S _ _ S
86 86 1 1 R _ _ S _ _ S 1 1 R _ _ S _ _ S _ _ S
86 87 , , R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
86 88 0 0 R _ _ S _ _ S 1 1 S _ _ S _ _ S _ _ S
86 88 1 1 R _ _ S _ _ S 0 0 S _ _ S _ _ S _ _ S
86 88 , , R _ _ S _ _ S 0 0 S _ _ S _ _ S _ _ S
86 88 , , R _ _ S _ _ S 1 1 S _ _ S _ _ S _ _ S
86 88 0 0 R _ _ S _ _ S _ _ L _ _ S _ _ S _ _ S
86 88 1 1 R _ _ S _ _ S _ _ L _ _ S _ _ S _ _ S
// 3.7.2.1 if the position is not correct go to the next
88 88 0 0 R _ _ S _ _ S 0 0 S _ _ S _ _ S _ _ S
88 88 0 0 R _ _ S _ _ S 1 1 S _ _ S _ _ S _ _ S
88 88 1 1 R _ _ S _ _ S 0 0 S _ _ S _ _ S _ _ S
88 88 1 1 R _ _ S _ _ S 1 1 S _ _ S _ _ S _ _ S
88 88 , , R _ _ S _ _ S 0 0 S _ _ S _ _ S _ _ S
88 88 , , R _ _ S _ _ S 1 1 S _ _ S _ _ S _ _ S
88 89 # # S _ _ S _ _ S 0 0 S _ _ S _ _ S _ _ S
88 89 # # S _ _ S _ _ S 1 1 S _ _ S _ _ S _ _ S
// 3.7.2.3 go to the beginning of the AR tape
89 89 # # S _ _ S _ _ S 0 0 L _ _ S _ _ S _ _ S
89 89 # # S _ _ S _ _ S 1 1 L _ _ S _ _ S _ _ S
89 85 # # S _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
// 3.7.3 subtract the content of the memory to acc
// 3.7.3.1 go to the end of the memory cell
87 87 0 0 R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
87 87 1 1 R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
87 90 # # L _ _ S _ _ S _ _ S _ _ L _ _ S _ _ S
// 3.7.3.2 subtract bit per bit
90 90 0 0 L _ _ S _ _ S _ _ S 0 0 L _ _ S _ _ S
90 90 0 0 L _ _ S _ _ S _ _ S 1 1 L _ _ S _ _ S
90 90 0 0 L _ _ S _ _ S _ _ S _ 0 L _ _ S _ _ S
90 90 1 1 L _ _ S _ _ S _ _ S 1 0 L _ _ S _ _ S
90 91 1 1 L _ _ S _ _ S _ _ S 0 1 L _ _ S _ _ S
90 92 1 1 L _ _ S _ _ S _ _ S _ 1 L _ _ S _ _ S
91 91 0 0 L _ _ S _ _ S _ _ S 0 1 L _ _ S _ _ S
91 90 0 0 L _ _ S _ _ S _ _ S 1 0 L _ _ S _ _ S
91 91 1 1 L _ _ S _ _ S _ _ S 1 1 L _ _ S _ _ S
91 91 1 1 L _ _ S _ _ S _ _ S 0 0 L _ _ S _ _ S
91 92 1 1 L _ _ S _ _ S _ _ S _ 1 L _ _ S _ _ S
90 92 , , S _ _ S _ _ S _ _ S 0 0 L _ _ S _ _ S
90 92 , , S _ _ S _ _ S _ _ S 1 1 L _ _ S _ _ S
90 92 , , S _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
91 92 , , S _ _ S _ _ S _ _ S 0 1 L _ _ S _ _ S
91 92 , , S _ _ S _ _ S _ _ S 1 0 L _ _ S _ _ S
91 92 , , S _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
// 3.7.3.3 go to the beginning of ACC tape
92 92 , , S _ _ S _ _ S _ _ S 0 0 L _ _ S _ _ S
92 92 , , S _ _ S _ _ S _ _ S 1 1 L _ _ S _ _ S
92 93 , , S _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
// 3.7.4 go to the beginning of memory tape
93 93 , , L _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
93 93 0 0 L _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
93 93 1 1 L _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
93 93 # # L _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
93 94 _ _ S _ _ S _ _ S _ _ L _ _ S _ _ S _ _ S
// 3.7.5 delete the content of the AR
94 94 _ _ S _ _ S _ _ S 0 _ L _ _ S _ _ S _ _ S
94 94 _ _ S _ _ S _ _ S 1 _ L _ _ S _ _ S _ _ S
// 3.7.6 restart the loop
94 3a _ _ R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
// 3.8 INIT
// 3.8.1 delete the content of the ACC
38 38 _ _ S _ _ S _ _ S _ _ S 0 _ R _ _ S _ _ S
38 38 _ _ S _ _ S _ _ S _ _ S 1 _ R _ _ S _ _ S
38 95 _ _ S _ _ S _ _ S _ _ R _ _ S _ _ S _ _ S
// 3.8.2 copy the content of AR to the ACC
95 95 _ _ S _ _ S _ _ S 0 _ R _ 0 R _ _ S _ _ S
95 95 _ _ S _ _ S _ _ S 1 _ R _ 1 R _ _ S _ _ S
95 95a _ _ S _ _ S _ _ S _ _ S _ _ L _ _ S _ _ S
95a 95a _ _ S _ _ S _ _ S _ _ S 0 0 L _ _ S _ _ S
95a 95a _ _ S _ _ S _ _ S _ _ S 1 1 L _ _ S _ _ S
// 3.8.3 restart the loop
95a 3a _ _ R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
// 3.10 JUMP
// 3.10.1 remove the content of the PC
40 40 _ _ S _ _ S _ _ S _ _ S _ _ S 0 _ R _ _ S
40 40 _ _ S _ _ S _ _ S _ _ S _ _ S 1 _ R _ _ S
40 96 _ _ S _ _ S _ _ S _ _ R _ _ S _ _ S _ _ S
// 3.10.2 copy the content of the AR to the PC
96 96 _ _ S _ _ S _ _ S 0 _ R _ _ S _ 0 R _ _ S
96 96 _ _ S _ _ S _ _ S 1 _ R _ _ S _ 1 R _ _ S
// 3.10.3 go to the initial position of the PC tape
96 96a _ _ S _ _ S _ _ S _ _ L _ _ S _ _ L _ _ S
96a 96a _ _ S _ _ S _ _ S _ _ S _ _ S 0 0 L _ _ S
96a 96a _ _ S _ _ S _ _ S _ _ S _ _ S 1 1 L _ _ S
// 3.10.3 restart the loop
96a 3a _ _ R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
// 3.11 CJUMP
// 3.11.1 check if the content of the ACC is 0
41 41 _ _ S _ _ S _ _ S _ _ S 0 0 R _ _ S _ _ S
41 97 _ _ S _ _ S _ _ S _ _ S 1 1 L _ _ S _ _ S
41 98 _ _ S _ _ S _ _ S _ _ S _ _ S _ _ R _ _ S
// 3.11.2 if the content of the ACC is 0 jump
// 3.11.2.1 remove the content of the PC
98 98 _ _ S _ _ S _ _ S _ _ S _ _ S 0 _ R _ _ S
98 98 _ _ S _ _ S _ _ S _ _ S _ _ S 1 _ R _ _ S
98 99 _ _ S _ _ S _ _ S _ _ R _ _ S _ _ S _ _ S
// 3.11.2.2 copy the content of the AR to the PC
99 99 _ _ S _ _ S _ _ S 0 _ R _ _ S _ 0 R _ _ S
99 99 _ _ S _ _ S _ _ S 1 _ R _ _ S _ 1 R _ _ S
99 99a _ _ S _ _ S _ _ S _ _ S _ _ S _ _ L _ _ S
99a 99a _ _ S _ _ S _ _ S _ _ S _ _ S 0 0 L _ _ S
99a 99a _ _ S _ _ S _ _ S _ _ S _ _ S 1 1 L _ _ S
// 3.11.3 if the content of ACC is not 0 continue
// 3.11.3.1 go to the beginning of the ACC tape
97 100 _ _ S _ _ S _ _ S _ _ S 1 1 L _ _ S _ _ S
97 100 _ _ S _ _ S _ _ S _ _ S 0 0 L _ _ S _ _ S
97 101 _ _ S _ _ S _ _ S _ _ R _ _ S _ _ S _ _ S
100 100 _ _ S _ _ S _ _ S _ _ S 1 1 L _ _ S _ _ S
100 100 _ _ S _ _ S _ _ S _ _ S 0 0 L _ _ S _ _ S
100 101 _ _ S _ _ S _ _ S _ _ R _ _ S _ _ S _ _ S
// 3.11.3.2 remove the content of the AR
101 101 _ _ S _ _ S _ _ S 0 _ R _ _ S _ _ S _ _ S
101 101 _ _ S _ _ S _ _ S 1 _ R _ _ S _ _ S _ _ S
// 3.11.4 restart the cycle
101 3a _ _ R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
99a 3a _ _ R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
// 3.12 HALT
42 125 _ _ R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
125 125 0 _ R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
125 125 1 _ R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
125 125 , _ R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
125 125 # _ R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
125 126 _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S _ _ L
126 126 _ 0 L _ _ S _ _ S _ _ S _ _ S _ _ S 0 0 L
126 126 _ 1 L _ _ S _ _ S _ _ S _ _ S _ _ S 1 1 L
126 h _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
// 3.9 STORE
// 3.9.1 go to the right cell of the memory
39 39 # # R _ _ S _ _ S _ _ R _ _ S _ _ S _ _ S
39 39 0 0 R _ _ S _ _ S 0 0 R _ _ S _ _ S _ _ S
39 39 1 1 R _ _ S _ _ S 1 1 R _ _ S _ _ S _ _ S
39 120 _ _ S _ _ S _ _ S 0 0 S _ _ S _ _ S _ _ S
39 120 _ _ S _ _ S _ _ S 1 1 S _ _ S _ _ S _ _ S
39 102 0 0 R _ _ S _ _ S 1 1 S _ _ S _ _ S _ _ S
39 102 1 1 R _ _ S _ _ S 0 0 S _ _ S _ _ S _ _ S
39 102 , , R _ _ S _ _ S 0 0 S _ _ S _ _ S _ _ S
39 102 , , R _ _ S _ _ S 1 1 S _ _ S _ _ S _ _ S
39 102 0 0 R _ _ S _ _ S _ _ L _ _ S _ _ S _ _ S
39 102 1 1 R _ _ S _ _ S _ _ L _ _ S _ _ S _ _ S
39 103 , , R _ _ S _ _ S _ _ S _ _ R _ _ S _ _ S
// 3.9.1.1 if the position is not correct go to the next
102 102 0 0 R _ _ S _ _ S 0 0 S _ _ S _ _ S _ _ S
102 102 0 0 R _ _ S _ _ S 1 1 S _ _ S _ _ S _ _ S
102 102 1 1 R _ _ S _ _ S 0 0 S _ _ S _ _ S _ _ S
102 102 1 1 R _ _ S _ _ S 1 1 S _ _ S _ _ S _ _ S
102 102 , , R _ _ S _ _ S 0 0 S _ _ S _ _ S _ _ S
102 102 , , R _ _ S _ _ S 1 1 S _ _ S _ _ S _ _ S
102 104 # # S _ _ S _ _ S 0 0 S _ _ S _ _ S _ _ S
102 104 # # S _ _ S _ _ S 1 1 S _ _ S _ _ S _ _ S
// 3.9.1.2 if the memory does not have the cell, initialize it
102 120 _ _ S _ _ S _ _ S 0 0 S _ _ S _ _ S _ _ S
102 120 _ _ S _ _ S _ _ S 1 1 S _ _ S _ _ S _ _ S
// 3.9.1.3 go to the beginning of the AR tape
104 104 # # S _ _ S _ _ S 0 0 L _ _ S _ _ S _ _ S
104 104 # # S _ _ S _ _ S 1 1 L _ _ S _ _ S _ _ S
104 39 # # S _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
// 3.9.2 copy the content of the ACC to the memory
103 103 0 0 R _ _ S _ _ S _ _ S 0 0 R _ _ S _ _ S
103 103 0 1 R _ _ S _ _ S _ _ S 1 1 R _ _ S _ _ S
103 103 1 0 R _ _ S _ _ S _ _ S 0 0 R _ _ S _ _ S
103 103 1 1 R _ _ S _ _ S _ _ S 1 1 R _ _ S _ _ S
103 112 0 0 S _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
103 112 1 1 S _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
103 112 # # S _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
103 105 # _ R _ _ S _ _ S _ _ S 0 0 S _ _ S _ _ S
103 105 # _ R _ _ S _ _ S _ _ S 1 1 S _ _ S _ _ S
// 3.9.2.1 shift the memory content to the right if the cell is full
105 106 0 _ R _ _ S _ _ S _ _ S 0 0 S _ _ S _ _ S
105 106 0 _ R _ _ S _ _ S _ _ S 1 1 S _ _ S _ _ S
105 107 1 _ R _ _ S _ _ S _ _ S 0 0 S _ _ S _ _ S
105 107 1 _ R _ _ S _ _ S _ _ S 1 1 S _ _ S _ _ S
105 108 , _ R _ _ S _ _ S _ _ S 0 0 S _ _ S _ _ S
105 108 , _ R _ _ S _ _ S _ _ S 1 1 S _ _ S _ _ S
105 109 # _ R _ _ S _ _ S _ _ S 0 0 S _ _ S _ _ S
105 109 # _ R _ _ S _ _ S _ _ S 1 1 S _ _ S _ _ S
106 106 0 0 R _ _ S _ _ S _ _ S 0 0 S _ _ S _ _ S
106 106 0 0 R _ _ S _ _ S _ _ S 1 1 S _ _ S _ _ S
106 107 1 0 R _ _ S _ _ S _ _ S 0 0 S _ _ S _ _ S
106 107 1 0 R _ _ S _ _ S _ _ S 1 1 S _ _ S _ _ S
106 108 , 0 R _ _ S _ _ S _ _ S 0 0 S _ _ S _ _ S
106 108 , 0 R _ _ S _ _ S _ _ S 1 1 S _ _ S _ _ S
106 109 # 0 R _ _ S _ _ S _ _ S 0 0 S _ _ S _ _ S
106 109 # 0 R _ _ S _ _ S _ _ S 1 1 S _ _ S _ _ S
107 106 0 1 R _ _ S _ _ S _ _ S 0 0 S _ _ S _ _ S
107 106 0 1 R _ _ S _ _ S _ _ S 1 1 S _ _ S _ _ S
107 107 1 1 R _ _ S _ _ S _ _ S 0 0 S _ _ S _ _ S
107 107 1 1 R _ _ S _ _ S _ _ S 1 1 S _ _ S _ _ S
107 108 , 1 R _ _ S _ _ S _ _ S 0 0 S _ _ S _ _ S
107 108 , 1 R _ _ S _ _ S _ _ S 1 1 S _ _ S _ _ S
107 109 # 1 R _ _ S _ _ S _ _ S 0 0 S _ _ S _ _ S
107 109 # 1 R _ _ S _ _ S _ _ S 1 1 S _ _ S _ _ S
108 106 0 , R _ _ S _ _ S _ _ S 0 0 S _ _ S _ _ S
108 106 0 , R _ _ S _ _ S _ _ S 1 1 S _ _ S _ _ S
108 107 1 , R _ _ S _ _ S _ _ S 0 0 S _ _ S _ _ S
108 107 1 , R _ _ S _ _ S _ _ S 1 1 S _ _ S _ _ S
108 108 , , R _ _ S _ _ S _ _ S 0 0 S _ _ S _ _ S
108 108 , , R _ _ S _ _ S _ _ S 1 1 S _ _ S _ _ S
108 109 # , R _ _ S _ _ S _ _ S 0 0 S _ _ S _ _ S
108 109 # , R _ _ S _ _ S _ _ S 1 1 S _ _ S _ _ S
109 106 0 # R _ _ S _ _ S _ _ S 0 0 S _ _ S _ _ S
109 106 0 # R _ _ S _ _ S _ _ S 1 1 S _ _ S _ _ S
109 107 1 # R _ _ S _ _ S _ _ S 0 0 S _ _ S _ _ S
109 107 1 # R _ _ S _ _ S _ _ S 1 1 S _ _ S _ _ S
109 108 , # R _ _ S _ _ S _ _ S 0 0 S _ _ S _ _ S
109 108 , # R _ _ S _ _ S _ _ S 1 1 S _ _ S _ _ S
109 109 # # R _ _ S _ _ S _ _ S 0 0 S _ _ S _ _ S
109 109 # # R _ _ S _ _ S _ _ S 1 1 S _ _ S _ _ S
106 110 _ 0 L _ _ S _ _ S _ _ S 0 0 S _ _ S _ _ S
106 110 _ 0 L _ _ S _ _ S _ _ S 1 1 S _ _ S _ _ S
107 110 _ 1 L _ _ S _ _ S _ _ S 0 0 S _ _ S _ _ S
107 110 _ 1 L _ _ S _ _ S _ _ S 1 1 S _ _ S _ _ S
108 110 _ , L _ _ S _ _ S _ _ S 0 0 S _ _ S _ _ S
108 110 _ , L _ _ S _ _ S _ _ S 1 1 S _ _ S _ _ S
109 110 _ # L _ _ S _ _ S _ _ S 0 0 S _ _ S _ _ S
109 110 _ # L _ _ S _ _ S _ _ S 1 1 S _ _ S _ _ S
110 110 0 0 L _ _ S _ _ S _ _ S 0 0 S _ _ S _ _ S
110 110 0 0 L _ _ S _ _ S _ _ S 1 1 S _ _ S _ _ S
110 110 1 1 L _ _ S _ _ S _ _ S 0 0 S _ _ S _ _ S
110 110 1 1 L _ _ S _ _ S _ _ S 1 1 S _ _ S _ _ S
110 110 , , L _ _ S _ _ S _ _ S 0 0 S _ _ S _ _ S
110 110 , , L _ _ S _ _ S _ _ S 1 1 S _ _ S _ _ S
110 110 # # L _ _ S _ _ S _ _ S 0 0 S _ _ S _ _ S
110 110 # # L _ _ S _ _ S _ _ S 1 1 S _ _ S _ _ S
110 111 _ # L _ _ S _ _ S _ _ S 0 0 S _ _ S _ _ S
110 111 _ # L _ _ S _ _ S _ _ S 1 1 S _ _ S _ _ S
111 103 _ 0 R _ _ S _ _ S _ _ S 0 0 R _ _ S _ _ S
111 103 _ 1 R _ _ S _ _ S _ _ S 1 1 R _ _ S _ _ S
// 3.9.2.2 shift the content of the memory cell to the right
112 113 0 a L _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
112 113 1 a L _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
112 114 # # S _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
113 113 0 0 L _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
113 113 1 1 L _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
113 115 , , R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
115 116 0 0 R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
115 117 1 0 R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
116 117 1 0 R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
116 116 0 0 R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
117 117 1 1 R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
117 116 0 1 R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
117 103 a 1 R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
116 103 a 0 R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
// 3.9.3 go to the beginning of the memory tape
114 114 # # L _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
114 114 0 0 L _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
114 114 1 1 L _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
114 114 , , L _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
114 118 _ _ S _ _ S _ _ S _ _ L _ _ S _ _ S _ _ S
// 3.9.4 delete the content of the AR
118 118 _ _ S _ _ S _ _ S 0 _ L _ _ S _ _ S _ _ S
118 118 _ _ S _ _ S _ _ S 1 _ L _ _ S _ _ S _ _ S
118 119 _ _ S _ _ S _ _ S _ _ S _ _ L _ _ S _ _ S
// 3.9.5 go to the beginning of the ACC tape
119 119 _ _ S _ _ S _ _ S _ _ S 0 0 L _ _ S _ _ S
119 119 _ _ S _ _ S _ _ S _ _ S 1 1 L _ _ S _ _ S
// 3.9.6 restart the loop
119 3a _ _ R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
// 3.9.7 initialize the memory cell
// 3.9.7.1 go to the beginning of the AR tape
120 120 _ _ S _ _ S _ _ S 0 0 L _ _ S _ _ S _ _ S
120 120 _ _ S _ _ S _ _ S 1 1 L _ _ S _ _ S _ _ S
120 121 _ _ S _ _ S _ _ S _ _ R _ _ S _ _ S _ _ S
// copy the content of the AR to the memory
121 121 _ 0 R _ _ S _ _ S 0 _ R _ _ S _ _ S _ _ S
121 121 _ 1 R _ _ S _ _ S 1 _ R _ _ S _ _ S _ _ S
121 122 _ , R _ _ S _ _ S _ _ R _ _ R _ _ S _ _ S
// copy the content of the ACC to the memory
122 122 _ 0 R _ _ S _ _ S _ _ S 0 0 R _ _ S _ _ S
122 122 _ 1 R _ _ S _ _ S _ _ S 1 1 R _ _ S _ _ S
122 123 _ # R _ _ S _ _ S _ _ S _ _ L _ _ S _ _ S
// return to the beginning of the ACC tape
123 123 _ _ S _ _ S _ _ S _ _ S 0 0 L _ _ S _ _ S
123 123 _ _ S _ _ S _ _ S _ _ S 1 1 L _ _ S _ _ S
// go to the beginning of the memory tape
123 124 _ _ L _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
124 124 0 0 L _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
124 124 1 1 L _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
124 124 , , L _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
124 124 # # L _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
// restart the loop
124 3a _ _ R _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S