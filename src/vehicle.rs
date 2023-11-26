pub struct Vehicle {
    name: String,
    pub capacity: i64, // capacity should be more general and be able to include things like volume of items, not just weight. 
    // capacity also should be optional to use.
}

impl Vehicle { 
    pub fn new(name: String, capacity: i64) -> Self { 
        Vehicle { 
            name, 
            capacity
        }
    }
}