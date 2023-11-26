use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Location {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub demand: i64, 
}


impl Location { 
    pub fn new(x:f32, y:f32, demand: i64) -> Self { 
        Location {
            x, 
            y,
            demand
        }
    }
}


impl Ord for Location { 
    fn cmp(&self, other: &Self) -> Ordering {
        self.demand.cmp(&other.demand)
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.demand == other.demand
    }
}

impl Eq for Location {}