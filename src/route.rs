use crate::vehicle::Vehicle;
use crate::location::Location;


pub struct Route {
    vehicle: Vehicle,
    destinations: Vec<Location>,
}

fn calculate_distance(a: &Location, b: &Location) -> f32 { 
    f32::sqrt(f32::powf(a.x - b.x, 2.0) + f32::powf(a.y - b.y, 2.0))
}

impl Route { 
    fn cost(self) -> f32 { 
        self.destinations
            .windows(2)
            .map(|slice| {calculate_distance(&slice[0], &slice[1])})
            .sum()
    }
}