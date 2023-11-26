use std::cmp::Ordering;
use std::fmt;

#[derive(Debug)]
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

impl fmt::Display for Vehicle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.name, self.capacity)
    }
}



impl Ord for Vehicle { 
    fn cmp(&self, other: &Self) -> Ordering {
        self.capacity.cmp(&other.capacity)
    }
}

impl PartialOrd for Vehicle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Vehicle {
    fn eq(&self, other: &Self) -> bool {
        self.capacity == other.capacity
    }
}

impl Eq for Vehicle {}