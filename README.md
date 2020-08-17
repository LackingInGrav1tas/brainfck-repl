# brainfck-repl
## Small Rust Brainfuck REPL implementation.


The REPL's interpreting works by iterating through the input and handling each character. It isn't really necessary to create a parser/lexer/scanner/tokenizer because of the grammar of Brainfuck. Each command is independent of the next, which allows us to jump straight into the emulation. Speaking of which, the implemented Brainfuck has 30,000 memory nodes, as is common. It is possible to create an "infinitely" large implementation using vectors, but 30,000 seems like enough.

Example programs (not by me) can be found at http://www.hevanet.com/cristofd/brainfuck/

### Added helper commands:
```
'h': help
'c': reset environment
'e': exit
'p': adds 48 to the value
'v': prints the int value of the cell
'i': prints the location on the memory strip
```

### Sample usage
```
Brainfuck REPL
A Rust implementation of the Brainfuck language, with some helper commands.
Type 'h' for more info.

bf: ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
Hello World!

bf:
```

### Brainfuck EBNF
```EBNF
<program> ::= ( '>' | '<' | '+' | '-' | ',' | '.' | '[' | ']' )* ; (*as you can see, quite complex*)
```
