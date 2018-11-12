/// An adaptation of the example from
/// https://rosettacode.org/wiki/100_doors#Rust

extern crate prusti_contracts;

pub struct VecWrapperBool{
    v: Vec<bool>
}

impl VecWrapperBool {
    // Encoded as body-less Viper function
    #[trusted]
    #[pure]
    #[ensures="result >= 0"]
    pub fn len(&self) -> usize {
        self.v.len()
    }

    // Encoded as body-less Viper method
    #[trusted]
    #[ensures="result.len() == size"]
    #[ensures="forall i: usize :: (0 <= i && i < result.len()) ==> result.lookup(i) == value"]
    pub fn new(value: bool, size: usize) -> Self {
        VecWrapperBool{ v: vec![value; size] }
    }

    // Encoded as body-less Viper function
    #[trusted]
    #[pure]
    #[requires="0 <= index && index < self.len()"]
    pub fn lookup(&self, index: usize) -> bool {
        self.v[index]
    }

    // Encoded as body-less Viper method
    #[trusted]
    #[requires="index < self.len()"]
    #[ensures="self.len() == old(self.len())"]
    #[ensures="self.lookup(index) == value"]
    #[ensures="forall i: usize :: (0 <= i && i < self.len() && i != index) ==>
                    self.lookup(i) == old(self.lookup(i))"]
    pub fn store(&mut self, index: usize, value: bool) {
        self.v[index] = value;
    }

    #[trusted]
    #[ensures="self.len() == old(self.len()) + 1"]
    #[ensures="self.lookup(old(self.len())) == value"]
    #[ensures="forall i: usize :: (0 <= i && i < old(self.len())) ==>
                    self.lookup(i) == old(self.lookup(i))"]
    pub fn push(&mut self, value: bool) {
        self.v.push(value);
    }
}

#[trusted]
fn print_door_state(i: usize, is_open: bool) {
    println!("Door {} is {}.", i + 1, if is_open {"open"} else {"closed"});
}

fn doors1() {
    let mut door_open = VecWrapperBool::new(false, 100);
    let mut pass = 1;
    #[invariant="1 <= pass"]
    #[invariant="door_open.len() == 100"]
    while pass < 100 {
        let mut door = pass;
        #[invariant="1 <= door"]
        #[invariant="door_open.len() == 100"]
        while door <= 100 {
            let door_state = door_open.lookup(door - 1);
            door_open.store(door - 1, !door_state);
            door += pass;
        }
        pass += 1;
    }
    let mut i = 0;
    let mut continue_loop = i < door_open.len();
    #[invariant="0 <= i"]
    #[invariant="i <= door_open.len()"]
    #[invariant="continue_loop ==> i < door_open.len()"]
    while continue_loop {
        let is_open = door_open.lookup(i);
        print_door_state(i, is_open);
        i += 1;
        continue_loop = i < door_open.len();
    }
}

#[trusted]
fn pow(base: u32, exp: u32) -> u32 {
    exp.pow(base)
}

#[trusted]
fn print_door_open(i: u32) {
    println!("Door {} is open", i);
}

fn doors4() {
    let mut i = 1u32;
    let base = 2;
    while i < 10u32 {
        let door = pow(base, i);
        print_door_open(door);
        i += 1;
    }
}

fn main() {}
