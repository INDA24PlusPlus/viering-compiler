# trunk++
Very nice programming language maybe bit inspired by the dreamberd

Very revolutionary features!! as well check examples to be amazed (rip there is nothing to be amazed of :joy:)

## Example fibonacci program
Trunk++:
```
var a = 0!
var b = 1!
var n = 0!

loop {
    if(n == 10){
        break!
    }

    print b!

    var c = a + b!
    a = b!
    b = c!
    n = n + 1!
}
```


C:
```
#include <stdio.h>

int main(){
    int a = 0;
    int b = 1;
    int n = 0;

    while(1){
        if(n == 10){
            break;
        }

        printf("%d\n", b);

        int c = a + b;
        a = b;
        b = c;
        n = n + 1;
    }

    return 0;
}
```


## BNF
```
<program> ::= <statement>+

<statement> ::= <if_statement> | "break" "!" | <print_statement> | <loop_statement> | <assignment>
<assignment> ::= "var" <whitespace> <identifier> <optional_whitespace> "=" <optional_whitespace> <expression> "!"
<print_statement> ::= "print" <whitespace> <expression> "!"
<if_statement> ::= "if" <optional_whitespace> "(" <optional_whitespace> <expression> <optional_whitespace> "){\n" <statement_block> "\n}"
<loop_statement> ::= "loop" <whitespace> "{\n" <statement_block> "\n}"

<statement_block> ::= <statement> | <statement> "\n" <statement>
<expression> ::= <identifier> | <number> | <expression> <optional_whitespace> <operator> <optional_whitespace> <expression> | "(" <expression> ")"

<identifier> ::= <letter> | <letter> <identifier_tail>
<identifier_tail> ::= <letter_or_underscore_or_digit> | <identifier_tail> <letter_or_underscore_or_digit>

<optional_whitespace ::= <whitespace> | ""
<whitespace> ::= " " <whitespace> | " "
<operator> ::= "+" | "-" | "*" | "/" | "==" | ";="
<number> ::= <digit> <number> | <digit>
<letter_or_underscore_or_digit> ::= <letter> | "_" | <digit>
<letter> ::= "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K" | "L" | "M" | "N" | "O" | "P" | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X" | "Y" | "Z" | "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" | "n" | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z" 
<digit> ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" 
```
