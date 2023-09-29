
use std::env;

fn main() {
    let mut tape: [u8; 30000] = [0_u8; 30000];
    let mut curr_index: usize = 0;
    let mut for_ind: Vec<usize> = Vec::new();
    let chars: [char; 8] = ['+', '-', '<', '>', '.', ',', '[',']'];
    
    let args: Vec<String> = env::args().collect();
    
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    if args.len() == 1{
        println!("Brainf: version: {}", VERSION);
        std::process::exit(0);
    }
    if args.len() > 2{
        println!("Brainf: Arguments Error!!");
        std::process::exit(1); 
    }
    
    let sequence = std::fs::read_to_string(&args[1]);

    let sequence = match sequence{
        Ok(sequence) => sequence,
        Err(error) =>{
            eprint!("Brainf: File Not Found.\n{:?}", error);
            std::process::exit(1);
        }
    };
    
    let char_seq: Vec<char> = sequence
                    .chars()
                    .filter(|x| (&chars).contains(x))
                    .collect();
    
    let mut seq_i = 0;
    
    while seq_i < char_seq.len(){
        
        let current = char_seq[seq_i];
        match current{ 
            '+' =>{
                tape[curr_index] = if tape[curr_index] == u8::MAX {0} else {tape[curr_index] + 1}; 
            }
            '-' =>{
                tape[curr_index] = if tape[curr_index] == u8::MIN {255} else {tape[curr_index] - 1};
            }
            '>' =>{
                if curr_index == 29999{
                    println!("Brainf: Out of Tape Bounds (positive)");
                    break;
                }
                else{curr_index += 1};
            }
            '<' =>{
                if curr_index == 0{
                    println!("Brainf: Out of Tape Bounds (negative).");
                    break;
                }
                curr_index -= 1;
            }
            '.' =>{
                    print!("{}", tape[curr_index] as char);
            }
                
            ',' =>{
                let mut inpt = String::new();
                std::io::stdin().read_line(&mut inpt)
                                .expect("Brainf: Input Error.");
                // let inpt_vec: Vec<char> = inpt.chars().collect();
                    
                // for (ind, char) in inpt_vec.iter().enumerate(){
                //     tape[curr_index+ ind] = *char as u8;
                // }

            }
            '[' =>{
                for_ind.push(seq_i.clone()); 
            }
            ']' =>{
                // if for_ind.len() == 0{
                //     println!("Brainf: Syntax Error (mismatched brackets).");
                //     break;
                // }
                if tape[curr_index] == 0{
                    for_ind.pop();
                }
                else {seq_i = for_ind.last().expect("Brainf: Syntax Error (mismatched brackets).").to_owned();}
            }
            _ =>{
                println!("Brainf: Syntax Error (Foreign Character).");
                break;
            }
        }
        seq_i+=1;
    }       
        
}

