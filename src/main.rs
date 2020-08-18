use std::io;
use std::io::Write;

struct Machine {
    memory: [i32; 30000],
    ip: usize,
    highest: usize,
}

fn error(message: &str, vm: &mut Machine) {
    println!("\n{}, resetting environment.", message);
    *vm = Machine { memory:[0; 30000], ip: 0, highest: 1 };
}

fn main() {
    // prints out some info
    println!("Brainfuck REPL\nA Rust implementation of the Brainfuck language, with some helper commands.\nType 'h' for more info.");

    // initializing vm
    let mut vm: Machine = Machine {memory: [0; 30000], ip: 0, highest: 1};

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

            if i >= line.len() { // breaking at the end of the line
                break;
            }
            if vm.ip == 30000 {
                error("Memory overflow (no cells left)", &mut vm);
                break;
            }
            match char_vec[i] {
                '+' => vm.memory[vm.ip] += 1,
                '-' => vm.memory[vm.ip] -= 1,
                '>' => {
                    vm.ip += 1;
                    if vm.ip+1 > vm.highest {
                        vm.highest = vm.ip+1;
                    }
                    if vm.ip > 30000 {
                        error("Memory overflow - out of memory cells", &mut vm); // out of memory
                    }
                },
                '<' => {
                    if vm.ip == 0 { // not necessary because of usize, but cleaner.
                        error("Memory underflow", &mut vm);
                    } else {
                        vm.ip -= 1;
                    }
                },
                '.' => print!("{}", vm.memory[vm.ip] as u8 as char), // converts to ascii, then prints
                ',' => {
                    // messy getch
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).expect("Could not read input.");
                    vm.memory[vm.ip] = input.chars().next().unwrap() as i32;
                },
                '[' => {
                    if vm.memory[vm.ip] == 0 { // skipping past the block
                        let mut nests = 0;
                        loop { // finds the matching ]
                            i += 1;
                            if i == char_vec.len() {
                                error("Unclosed '['", &mut vm);
                                break;
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
                    if vm.memory[vm.ip] != 0 { // jumping back
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
                'p' => vm.memory[vm.ip] += 48, // initializes the object for ascii printing if it's a num
                'e' => {print!("Exiting...");return;},
                'r' => { // resets the environment
                    vm = Machine { memory:[0; 30000], ip: 0, highest: 1 };
                    break;
                },
                'h' => { // prints out info
                    println!("Brainfuck REPL is a REPL (read-eval-print-loop) for the programming language Brainfuck.\nBrainfuck is a esoteric programming language comprised of 8 operations:\n'>': shift forward on the memory tape\n'<': shift backwards\n'+': increment the value at the pointer\n'-': decrement the value\n',': get user input and store as an int\n'.': print the ascii value of the number beneath the pointer\n'[': if the value under the pointer is 0, it skips to ]\n']': jumps back it the value is non-zero");
                    print!("Helper characters...\n'h': help\n'r': reset environment\n'e': exit\n'p': adds 48 to the value\n'v': prints the int value of the cell\n'i': prints the location on the memory strip\n'n': visualizes the nodes\nHello World: ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
                    break;
                },
                'v' => print!("{}", vm.memory[vm.ip]),
                'i' => print!("{}", vm.ip),
                'n' => {
                    for index in 0..vm.highest{
                        print!("[{}] ", vm.memory[index]);
                    }
                    println!();
                },
                // </helper>
                _ => (),
            }
            i += 1;
        }
    }
}
