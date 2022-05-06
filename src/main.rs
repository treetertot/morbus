use std::{intrinsics::transmute, iter::repeat, env::args, fs::File, io::Read};

fn main() {
    let args = args();
    let path = args.skip(1).next().expect("expected file path as argument");
    let mut code = String::new();
    File::open(path)
        .expect("could not open file")
        .read_to_string(&mut code)
        .expect("could not read code to string");

    let memory = code.lines()
        .map(|line| {
            line.parse().unwrap_or(0u32)
        }).collect();
    let mut state = ProgramState { memory, stack: Vec::new() };

    let result = state.exec();
    println!("{:?}", result)
}

#[repr(u32)]
#[allow(dead_code)]
enum Operation {
    PushNxt = 0,
    PushAddr = 1,
    PopAddr = 2,
    Duplicate = 3,
    Swap = 4,
    Add = 5,
    Sub = 6,
    Mul = 7,
    Div = 8,
    Quit = 9
}
impl Operation {
    fn load(n: u32) -> Self {
        let n = n % 10;
        unsafe { transmute(n) }
    }
}
struct ProgramState {
    memory: Vec<u32>,
    stack: Vec<u32>
}
impl ProgramState {
    fn exec(&mut self) -> &[u32] {
        use Operation::*;
        let mut index = 0;
        loop {
            let next_index = self.get(index);
            let op = Operation::load(next_index);
            match op {
                PushNxt => self.stack.push(self.get(next_index + 1)),
                PushAddr => {
                    let addr = self.stack.pop().unwrap_or(0);
                    let item = self.get(addr);
                    self.stack.push(item);
                },
                PopAddr => {
                    let addr = self.stack.pop().unwrap_or(0);
                    let item = self.stack.pop().unwrap_or(0);
                    self.insert(addr, item);
                },
                Duplicate => {
                    let item = self.stack.last().map(|&n| n).unwrap_or(0);
                    self.stack.push(item);
                },
                Swap if self.stack.len() >= 2 => {
                    let a = self.stack.len() - 2;
                    let b = self.stack.len() - 1;
                    self.stack.swap(a, b)
                }
                Swap => self.stack.push(0),
                Add if self.stack.len() >= 2 => if let Some(item) = self.stack.pop() {
                    self.stack.last_mut().map(|a| *a += item);
                },
                Add => (),
                Sub if self.stack.len() >= 2 => if let Some(item) = self.stack.pop() {
                    self.stack.last_mut().map(|a| *a -= item);
                },
                Sub if self.stack.len() == 1 => {
                    let flipped = -(self.stack[1] as i32) as u32;
                    self.stack[1] = flipped;
                },
                Sub => (),
                Mul if self.stack.len() >= 2 => if let Some(item) = self.stack.pop() {
                    self.stack.last_mut().map(|a| *a *= item);
                }
                Mul => (),
                Div if self.stack.len() >= 2 => if let Some(item) = self.stack.pop() {
                    self.stack.last_mut().map(|a| *a /= item);
                }
                Div => (),
                Quit => {
                    let next_next_index = self.get(next_index);
                    return &self.memory[next_next_index as usize..];
                },
            }
            index = next_index;
        }
    }
    fn get(&self, i: u32) -> u32 {
        self.memory.get(i as usize).map(|a| *a).unwrap_or(0)
    }
    fn insert(&mut self, i: u32, value: u32) {
        let index = i as usize;
        if index >= self.memory.len() {
            let diff = index - self.memory.len();
            let iter = repeat(0).take(diff+1);
            self.memory.extend(iter);
        }
        self.memory[index] = value
    }
}
