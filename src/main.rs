use std::env;
use std::fs::File;
use std::path::Path;
use std::io::Read;

#[derive(Debug)]
struct Brainfuck {
    code: Vec<u8>,
    pointer: i32,
    mem: Vec<u8>,
}
impl Brainfuck {
    fn new(code: Vec<u8>) -> Brainfuck {
        Brainfuck {
            code: code,
            pointer: 0,
            mem: vec![0],
        }
    }

    fn dot(&mut self) {
        print!("{}", self.mem[self.pointer as usize] as char);
    }

    fn lt(&mut self) {
        if self.pointer != 0 {
            self.pointer -= 1;
        }
    }

    fn gt(&mut self) {
        self.pointer += 1;
        if self.mem.len() < self.pointer as usize +1 {
            self.mem.push(0);
        }
    }

    fn plus(&mut self) {
        self.mem[self.pointer as usize] += 1;
    }
    fn minus(&mut self) {
        self.mem[self.pointer as usize] -= 1;
    }
    fn clean(&mut self) {
        self.code.retain(|&c| {
                             c == b'+' || c == b'-' || c == b'.' || c == b'!' || c == b',' ||
                             c == b'[' || c == b']' ||
                             c == b'<' || c == b'>'
                         });
    }

    fn compile(&mut self) {
        self.clean();
        let mut begins: Vec<i32> = Vec::new();
        let mut i: usize = 0;
        let op = self.code.clone();
        while op.len() > i {
            if op[i] == b'+' {
                self.plus();
            }
            if op[i] == b'-' {
                self.minus();
            }
            if op[i] == b'>' {
                self.gt();
            }
            if op[i] == b'<' {
                self.lt();
            }
            if op[i] == b'.' {
                self.dot();
            }
            if op[i] == b'[' {
                if self.mem[self.pointer as usize] != 0 {
                    begins.push(i as i32);
                } 
                else {
                    begins.pop();
                }
            }
            if op[i] == b']' {
                if self.mem[self.pointer as usize] != 0 {
                    match begins.last() {
                        Some(&index) => {
                            i = index as usize;
                        },
                        None => println!(""),
                    };
                }else {
                    begins.pop();
                }
            }
            // println!("{:?}    {:?}      {:?}      {:?}      {:?}    {}  {}",i, op[i] as char, begins, 0, self.mem,self.pointer,self.mem[self.pointer as usize]);
            i += 1;
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let mut file = File::open(&path).expect("FILE NOT FOUND");
    let mut file_content = Vec::new();

    match file.read_to_end(&mut file_content)  {
        Err(why)=>panic!("{}",why),
        Ok(_)=>print!("{}",String::from_utf8_lossy(&file_content)),
    }    ;
    
    let mut bf = Brainfuck::new(file_content);
    bf.compile();
}
