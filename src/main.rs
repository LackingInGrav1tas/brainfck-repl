use std::io;
use std::io::Write;

fn error(message: &str) {
    println!("\n{}", message);
    panic!("Exiting...");
}

fn main() {
    // prints out some info
    println!("Brainfuck REPL\nA Rust implementation of the Brainfuck language, with some helper commands.\nType 'h' for more info.");

    // creating simulation of memory
    let mut memory: [i32; 30000] = [0; 30000];
    let mut ip: usize = 0;

    // the L in REPL
    loop {
        // getting line; the R
        print!("\nbf: ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Could not read input.");
        let char_vec: Vec<char> = line.chars().collect();

        let mut i: usize = 0; // for loops won't work, unless i'm missing something
        
        loop { // the E

            if i == line.len() { // breaking at the end of the line
                break;
            }

            match char_vec[i] {
                '+' => memory[ip] += 1,
                '-' => memory[ip] -= 1,
                '>' => {
                    ip += 1;
                    if ip > 30000 {
                        error("Stack overflow - out of memory cells."); // out of memory
                    }
                },
                '<' => {
                    if ip == 0 { // not necessary because of usize, but cleaner.
                        error("Stack underflow.");
                    }
                    ip -= 1;
                },
                '.' => print!("{}", memory[ip] as u8 as char), // converts to ascii, then prints
                ',' => {
                    // messy getch
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).expect("Could not read input.");
                    memory[ip] = input.chars().next().unwrap() as i32;
                },
                '[' => {
                    if memory[ip] == 0 { // skipping past the block
                        let mut nests = 0;
                        loop { // finds the matching ]
                            i += 1;
                            if i == char_vec.len() {
                                error("Unclosed '['");
                            }
                            match char_vec[i] {
                                '[' => nests += 1,
                                ']' => {
                                    if nests == 0 {
                                        break;
                                    }
                                    nests -= 1
                                },
                                _ => (),
                            }
                        }
                    }
                },

                ']' => {
                    if memory[ip] != 0 { // jumping back
                        let mut nests = 0;
                        loop { // finds the matching [
                            i -= 1; // doesn't need to check for underflow because usize cannot be negative
                            match char_vec[i] {
                                ']' => nests += 1,
                                '[' => {
                                    if nests == 0 {
                                        break;
                                    }
                                    nests -= 1
                                },
                                _ => (),
                            }
                        }
                    }
                },
                // <helper>
                'p' => memory[ip] += 48, // initializes the object for ascii printing if it's a num
                'e' => error("Exiting"),
                'c' => { // clears the environment
                    memory = [0; 30000];
                    ip = 0;
                    break;
                },
                'h' => { // prints out info
                    println!("Brainfuck REPL is a REPL (read-eval-print-loop) for the programming language Brainfuck.\nBrainfuck is a esoteric programming language comprised of 8 operations:\n'>': shift forward on the memory tape\n'<': shift backwards\n'+': increment the value at the pointer\n'-': decrement the value\n',': get user input and store as an int\n'.': print the ascii value of the number beneath the pointer\n'[': if the value under the pointer is 0, it skips to ]\n']': jumps back it the value is non-zero");
                    print!("Helper characters...\n'h': help\n'c': reset environment\n'e': exit\n'p': adds 48 to the value\n'v': prints the int value of the cell\n'i': prints the location on the memory strip\nHello World: ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
                    break;
                },
                'v' => print!("{}", memory[ip]),
                'i' => print!("{}", ip),
                // </helper>
                _ => (),
            }
            i += 1;
        }
    }
}
