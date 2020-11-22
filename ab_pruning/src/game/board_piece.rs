#[derive(Clone)]
pub struct Piece<T> {
    value: T, 
    location: u32,
    name: String
}

impl<T> Piece<T> {
    pub fn new(value: T, location: u32, name: String) -> Piece<T> {
        Piece{value, location, name}
    }

    pub fn update(&mut self, value: T, name: String) {
        self.value = value;
        self.name = name;
    }

    pub fn value(&self) -> &T { &self.value }
    pub fn name(&self) -> &String { &self.name }
}

#[derive(Clone, Debug)]
pub struct Move {
    pub from: [usize; 2], 
    pub to: [usize; 2],
    pub value: i32
}

impl Move {
    pub fn new(from: [usize; 2], to: [usize; 2], value: i32) -> Move {
        Move {from, to, value}
    }
    pub fn value(&self) -> &i32 {
        &self.value
    }
    pub fn set_value(&mut self, value: i32) {
        self.value = value;
    }
}