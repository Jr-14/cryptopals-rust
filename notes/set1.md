# Set 1

# Convert hex to base64
## What is hex and base64?
Hex (short for Hexadecimal, Base 16) is a way to represent numbers. We've standardised our number representation to use Decimal
(Base 10), meaning we represent one digit with 10 different symbols. In Hexadecimal a digit is represented by 16 different
symbols (Symbols are 0,1,2,3,4,5,6,7,8,9,a,b,c,d,e,f where a represents the Decimal number 10 and f represents the
decimal number 15).

Likewise Base64 is a way to represent numbers; where one digit can be represented by 64 different symbols. Here are the
full length of Base64 digits from lowest to highest where 'padding' digit is represented as '='.
```
0  = A      16 = Q      32 = g      48 = w      padding = 
1  = B      17 = R      33 = h      49 = x
2  = C      18 = S      34 = i      50 = y
3  = D      19 = T      35 = j      51 = z
4  = E      20 = U      36 = k      52 = 0
5  = F      21 = V      37 = l      53 = 1
6  = G      22 = W      38 = m      54 = 2
7  = H      23 = X      39 = n      55 = 3
8  = I      24 = Y      40 = o      56 = 4
9  = J      25 = Z      41 = p      57 = 5
10 = K      26 = a      42 = q      58 = 6
11 = L      27 = b      43 = r      59 = 7
12 = M      28 = c      44 = s      60 = 8
13 = N      29 = d      45 = t      61 = 9
14 = O      30 = e      46 = u      62 = +
15 = P      31 = f      47 = v      63 = /
```

Notice that with Base64 the letters are case sensitive where as with Hexadecimal we can represent letters 'a' to 'f' as
either lowercase or uppercase.

## Hexadecimal and Base64 representation in Binary
Fortunately Hexadecimal and Base64 is a factor of 2 :) Notice that to represent all Hexadecimal symbols we will require
16 symbols, but we only have 2 symbols in Binary. Therefore to represent a single Hexadecimal symbol, we require 4 bits
max.

```
0000 = 0
0001 = 1
0010 = 2
0011 = 3
0100 = 4
0101 = 5
0110 = 6
0111 = 7
1000 = 8
1001 = 9
1010 = a = 10 (in base10)
1011 = b = 11 (in base10)
1100 = c = 12 (in base10)
1101 = d = 13 (in base10)
1110 = e = 14 (in base10)
1111 = f = 15 (in base10)
```

Perfect! We can do the same with Base64. We require 64 Symbols but only have 2 symbols. How many bits do we require?
$x = log_2(64) \therefore x = 6$ Therefore we require 6 bits!

