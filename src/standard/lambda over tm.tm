// file: lambda over tm.tm
// Project: Computing Simulator
// author: dp
// turing machine to execute beta reduction on a lambda formula
tm
// initial state
0
// accept state

//reject state

// final state
h
// blank symbol
_
// states
0 1 1b 1c 1ca 1cb 1cc 1cd 1ce 1cf 1d 1e 1f 1g 1h 1ha 1i 1j 1k 1l 1m 1n 1o 1p 1pa 1q 1r 1s 1t 1u 1v 1w 1x 1y 1z 1aa 1bb 1cc 1dd 1ee 2 3 3a 3b 4 5 6 7 8 9 10 10b 11 12 13 14 14a 14b 14c 14d 15 16 17 18 19 20 21 22 23 23a 23c 24 24b 25 26 27 28 29 30 31 h
// input alphabet
( ) / .
// tape alphabet
( ) / . _ $
// number of tapes
6
// transitions:
// start move
0 1b _ _ R _ _ S _ _ S _ _ S _ _ S _ _ S
// parentheses checking
1b 1b A A R _ _ S _ _ S _ _ S _ _ S _ _ S
1b 1u _ _ L _ _ S _ _ S _ _ S _ _ S _ _ S
1b 1ca ( $ R _ _ S _ _ S _ _ S _ _ S _ _ S
1ca 1cb ) $ R _ _ S _ _ S _ _ S _ _ S _ _ S
1cb 1cb D _ R _ D R _ _ S _ _ S _ _ S _ _ S
1cb 1cc _ _ L _ _ L _ _ S _ _ S _ _ S _ _ S
1cc 1cc _ _ L D D L _ _ S _ _ S _ _ S _ _ S
1cc 1cd $ _ L _ _ R _ _ S _ _ S _ _ S _ _ S
1cd 1ce $ D R D _ R _ _ S _ _ S _ _ S _ _ S
1ce 1ce _ D R D _ R _ _ S _ _ S _ _ S _ _ S
1ce 1cf _ _ L _ _ S _ _ S _ _ S _ _ S _ _ S
1cf 1cf D D L _ _ S _ _ S _ _ S _ _ S _ _ S
1cf 1b _ _ R _ _ S _ _ S _ _ S _ _ S _ _ S 
1ca 1c F F S _ _ S _ _ S _ _ S _ _ S _ _ S
1c 1d A A L _ _ S _ _ S _ _ S _ _ S _ _ S
1d 1b $ ( R _ _ S _ _ S _ _ S _ _ S _ _ S
1c 1e ( ( R _ _ S _ _ S _ _ S _ _ S _ _ S
1e 1e C C R _ _ S _ _ S _ _ S _ _ S _ _ S
1e 1h ) ) R _ _ S _ _ S _ _ S _ _ S _ _ S
1e 1f ( ( R _ . S _ _ S _ _ S _ _ S _ _ S
1f 1f C C R . . S _ _ S _ _ S _ _ S _ _ S
1f 1f C C R _ _ S _ _ S _ _ S _ _ S _ _ S
1f 1g ( ( S . . R _ _ S _ _ S _ _ S _ _ S
1f 1f ( ( S _ . S _ _ S _ _ S _ _ S _ _ S
1g 1f ( ( R _ . S _ _ S _ _ S _ _ S _ _ S
1f 1f ) ) R . _ L _ _ S _ _ S _ _ S _ _ S
1f 1h ) ) R _ _ S _ _ S _ _ S _ _ S _ _ S
1h 1ha F F L _ _ S _ _ S _ _ S _ _ S _ _ S
1h 1ha _ _ L _ _ S _ _ S _ _ S _ _ S _ _ S
1ha 1ha D D L _ _ S _ _ S _ _ S _ _ S _ _ S
1ha 1b $ ( R _ _ S _ _ S _ _ S _ _ S _ _ S
1h 1i ) $ L _ _ S _ _ S _ _ S _ _ S _ _ S
1i 1j ) $ L _ _ S _ ) S _ _ S _ _ S _ _ S
1j 1j D D2 L _ _ S D2 D S _ _ S _ _ S _ _ S
1j 1k $ D R _ _ S D _ S _ _ S _ _ S _ _ S
1k 1l ( $ R _ _ S _ _ S _ _ S _ _ S _ _ S
1k 1k A A L _ _ S _ _ S _ _ S _ _ S _ _ S
1l 1l D D R _ _ S _ _ S _ _ S _ _ S _ _ S
1l 1m $ _ R _ _ S _ _ S _ _ S _ _ S _ _ S
1m 1n $ _ R _ _ S _ _ S _ _ S _ _ S _ _ S
1n 1n D _ R _ D R _ _ S _ _ S _ _ S _ _ S
1n 1o _ _ L _ _ L _ _ S _ _ S _ _ S _ _ S
1o 1o _ _ L D D L _ _ S _ _ S _ _ S _ _ S
1o 1o _ _ L _ _ S _ _ S _ _ S _ _ S _ _ S
1o 1o D _ L _ D L _ _ S _ _ S _ _ S _ _ S
1o 1p $ ( R _ _ R _ _ S _ _ S _ _ S _ _ S
1p 1p _ D R D _ R _ _ S _ _ S _ _ S _ _ S
1p 1pa _ _ L _ _ S _ _ S _ _ S _ _ S _ _ S
1pa 1pa D D L _ _ S _ _ S _ _ S _ _ S _ _ S
1pa 1b _ _ R _ _ S _ _ S _ _ S _ _ S _ _ S
1u 1u D D L _ _ S _ _ S _ _ S _ _ S _ _ S
1u 1u $ ( L _ _ S _ _ S _ _ S _ _ S _ _ S
1u 1v _ _ R _ _ S _ _ S _ _ S _ _ S _ _ S
1v 1v D3 _ R _ D3 R _ _ S _ _ S _ _ S _ _ S
1v 1ee _ _ L _ _ L _ _ S _ _ S _ _ S _ _ S
1v 1w x x S _ _ L _ _ S _ _ S _ _ S _ _ S
1w 1x x x S A A R _ _ S _ _ S _ _ S _ _ S
1x 1v x _ R _ x R _ _ S _ _ S _ _ S _ _ S
1w 1y x x S ( ( S _ _ S _ _ S _ _ S _ _ S
1y 1z x x R ( ( S _ _ S _ _ S _ _ S _ _ S
1z 1aa ) ) L ( ( S _ _ S _ _ S _ _ S _ _ S
1aa 1bb x _ R ( x R _ _ S _ _ S _ _ S _ _ S
1bb 1v ) _ R _ _ S _ _ S _ _ S _ _ S _ _ S
1z 1dd F F L ( ( R _ _ S _ _ S _ _ S _ _ S
1dd 1v x _ R _ x R _ _ S _ _ S _ _ S _ _ S
1ee 1ee _ D L D _ L _ _ S _ _ S _ _ S _ _ S
1ee 1 _ _ R _ _ S _ _ S _ _ S _ _ S _ _ S
// if found a ( it can be a start of application
1 2 ( _ R _ ( R _ _ S _ _ S _ _ S _ _ S
// else copy
1 1 A _ R _ A R _ _ S _ _ S _ _ S _ _ S
// if found another ( it can be a start of application
2 3 ( _ R _ ( R _ _ S _ _ S _ _ S _ _ S
// else copy
2 2 x _ R _ x R _ _ S _ _ S _ _ S _ _ S
2 1 / _ R _ / R _ _ S _ _ S _ _ S _ _ S
2 1 ) _ R _ ) R _ _ S _ _ S _ _ S _ _ S
// start of function found
3 4 / / R _ _ S _ _ S _ _ S _ _ S _ _ S
// else copy
3 3a E E S _ _ L _ _ S _ _ S _ _ S _ _ S
3a 3b E E S ( ( R _ _ S _ _ S _ _ S _ _ S
3b 2 A A S _ _ S _ _ S _ _ S _ _ S _ _ S
3b 3 ( _ R _ ( R _ _ S _ _ S _ _ S _ _ S
// copy the bound variable on the variable tape
4 5 x x R _ _ S _ _ S _ _ S _ x R _ _ S
// skip all chars except .
5 5 B B R _ _ S _ _ S _ _ S _ _ S _ _ S
// . found, start of the argument of the function
5 6 . . R _ _ R _ _ S _ _ S _ _ S _ _ S
// copy the function argument, maintaining a trace of the parentheses
6 6 ( _ R _ $ R _ ( R _ _ S _ _ S _ _ S
6 6 C _ R _ _ S _ C R _ _ S _ _ S _ _ S
6 7 ) ) S _ _ L _ _ S _ _ S _ _ S _ _ S
7 6 ) _ R $ _ S _ ) R _ _ S _ _ S _ _ S
// found end of function, searching for application parameter
7 8 ) _ R _ _ S _ ) R _ _ R _ _ S _ _ S
// copy the application parameter on its tape, maintaining trace of the parentheses
8 10 ( _ R _ $ S _ _ S _ ( R _ _ S _ _ S
8 9 x _ L _ _ S _ _ S _ x R _ _ S _ _ S
10 10 C _ R $ $ S _ _ S _ C R _ _ S _ _ S
10 10b ( ( S $ $ R _ _ S _ _ S _ _ S _ _ S
10b 10 ( _ R _ $ S _ _ S _ ( R _ _ S _ _ S
10 11 ) ) S $ _ L _ _ S _ _ S _ _ S _ _ S
11 10 ) _ R $ $ S _ _ S _ ) R _ _ S _ _ S
11 9 ) _ L D D R _ _ S _ ) R _ _ S _ _ S
// copied parameter, now go to the beginning of its tape
9 12 _ _ S _ _ S _ _ S _ _ L _ _ S _ _ S
12 12 _ _ S _ _ S _ _ S D2 D2 L _ _ S _ _ S
// go to the beginning of the argument tape
12 13 _ _ S _ _ S _ _ L _ _ S _ _ S _ _ S
13 13 _ _ S _ _ S D2 D2 L _ _ S _ _ S _ _ S
13 14a _ _ L _ _ S _ _ S _ _ S _ _ S _ _ S
// go to the "beginning" of the first tape
14a 14a _ _ L _ _ S _ _ S _ _ S _ _ S _ _ S
14a 14 D D R _ _ S _ _ R _ _ S _ _ L _ _ S
// copy the argument until find an x
14 14 _ _ S _ _ S ( _ R _ _ S x x S _ ( R
14 14 _ _ S _ _ S ) _ R _ _ S x x S _ ) R
14 14 _ _ S _ _ S . _ R _ _ S x x S _ . R
14 14b _ _ S _ _ R / _ R _ _ S x x S _ / R
14b 14b _ _ S _ _ S x2 _ R _ _ S x x S _ x2 R
14b 14c _ _ S _ _ S x _ R _ _ S x x S _ x R
14c 14c _ _ S _ _ S C _ R _ _ S x x S _ C R
14c 14c _ _ S . . S C _ R _ _ S x x S _ C R
14c 14c _ _ S . _ L ) _ R _ _ S x x S _ ) R
14c 14c _ _ S _ . S ( _ R _ _ S x x S _ ( R
14c 14d _ _ S . . R ( _ S _ _ S x x S _ _ S
14d 14c _ _ S _ . S _ _ R _ _ S x x S _ ( R
14c 14 _ _ S _ _ S ) _ R _ _ S x x S _ ) R
14b 14 _ _ S _ _ L . . R _ _ S x x S _ . R
14 14 _ _ S _ _ S x2 _ R _ _ S x x S _ x2 R
14 15 _ _ S _ _ S x x S _ _ R x x R _ _ S
// copy the parameter of application
15 15 _ _ S _ _ S x x S D2 D2 R _ _ S _ D2 R
15 16 _ _ S _ _ S x x S _ _ L _ _ S _ _ S
16 16 _ _ S _ _ S x x S D2 D2 L _ _ S _ _ S
// return to the beginning of its tape
16 17 _ _ S _ _ S x x S _ _ S _ _ S _ _ S
// go ahead continuing to copy the argument until another x is found or the argument is finished
17 14 _ _ S _ _ S x _ R _ _ S _ _ L _ _ S
// return to the beginning od the function we are reducting
14 18 _ _ L _ _ S _ _ S _ _ S x x R _ _ S
18 18 D D L _ _ S _ _ S _ _ S _ _ S _ _ S
18 19 _ _ R _ _ S _ _ S _ _ S _ _ S _ _ S
// copy it to the tape if other variables remains, else don't copy it
19 20 / / R _ _ S _ _ S _ _ S _ _ L _ _ S
20 21 x x R _ _ S _ _ S _ _ S x x S _ _ S
21 22 x1 x1 L _ _ S _ _ S _ _ S x x S _ _ S
21 23a . . L _ _ S _ _ S _ _ S x _ S _ _ S
23a 23a D _ L _ _ S _ _ S _ _ S _ _ S _ _ S
23a 23c _ _ R _ _ S _ _ S _ _ S _ _ S _ _ S
23c 23c _ _ R _ _ S _ _ S _ _ S _ _ S _ _ S
23c 23 . . S _ _ S _ _ S _ _ S _ _ S _ _ L 
22 22 x x L _ _ S _ _ S _ _ S x x S _ _ S
22 24 / _ R _ / R _ _ S _ _ S x x S _ _ S
24 24b x _ R _ _ S _ _ S _ _ S x _ S _ _ S
24b 24b x _ R _ x R _ _ S _ _ S _ _ S _ _ S
// go to the beginning of the modified argument tape
24b 23 . . S _ . R _ _ S _ _ S _ _ S _ _ L
23 23 . . S _ _ S _ _ S _ _ S _ _ S D D L
23 25 . _ S _ _ S _ _ S _ _ S _ _ S _ _ R
// copy the result to the final tape
25 25 _ _ S _ D R _ _ S _ _ S _ _ S D _ R
25 26 _ _ S _ _ S _ _ S _ _ R _ _ S _ _ S
// delete the original argument
26 26 _ _ S _ _ S _ _ S D _ R _ _ S _ _ S
26 27 _ _ R _ _ S _ _ S _ _ S _ _ S _ _ S
27 27 _ _ R _ _ S _ _ S _ _ S _ _ S _ _ S
27 29 D D S _ _ S _ _ S _ _ S _ _ S _ _ S
// final copy
29 29 D _ R _ D R _ _ S _ _ S _ _ S _ _ S
29 30 _ _ S _ _ L _ _ S _ _ S _ _ S _ _ S
30 30 _ D L D _ L _ _ S _ _ S _ _ S _ _ S
30 1b _ _ R _ _ S _ _ S _ _ S _ _ S _ _ S
// no other beta reduction, halt
1 31 _ _ L _ _ L _ _ S _ _ S _ _ S _ _ S
31 31 _ D L D _ L _ _ S _ _ S _ _ S _ _ S
31 h _ _ S _ _ S _ _ S _ _ S _ _ S _ _ S
// A: All but ( and _
// B: All but . and _
// C: All but ( and ) and _
// D and D2: All but _
// E: All but / and _
// F: All but ) and _
// D3: All but variables
// x and x1 and x2: All variables