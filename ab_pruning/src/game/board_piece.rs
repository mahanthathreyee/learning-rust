pub struct Piece<T> {
    value: T, 
    location: u32,
    pub name: String,
}

impl<T> Piece<T> {
    pub fn new(value: T, location: u32, name: String) -> Piece<T> {
        Piece{value, location, name}
    }

    pub fn get_value(&self) -> &T { &self.value }
    pub fn get_location(&self) -> &u32 { &self.location }
    pub fn get_name(&self) -> &String { &self.name }
}