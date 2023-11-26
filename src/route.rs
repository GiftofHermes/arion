use crate::vehicle::Vehicle;
use crate::location::Location;

#[derive(Debug)]
pub struct Route {
    pub vehicle: Vehicle,
    pub destinations: Vec<Location>,
}

fn calculate_distance(a: &Location, b: &Location) -> f32 { 
    f32::sqrt(f32::powf(a.x - b.x, 2.0) + f32::powf(a.y - b.y, 2.0))
}

impl Route { 
    pub fn new(vehicle: Vehicle) -> Route { 
        Route {
            vehicle: vehicle, 
            destinations: Vec::new()
        }
    }
    pub fn cost(&self) -> f32 { 
        self.destinations
            .windows(2)
            .map(|slice| {calculate_distance(&slice[0], &slice[1])})
            .sum()
    }

    pub fn is_feasible(&self) -> bool { 
        let mut capacity = self.vehicle.capacity;

        for destination in self.destinations { 
            capacity =- destination.demand;
        }

        capacity >= 0
    }
}