use super::operator::Operator;

pub struct Location {
    x: f32,
    y: f32,
    demand: i64, 
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

pub struct Vehicle {
    name: String,
    capacity: i64, // capacity should be more general and be able to include things like volume of items, not just weight. 
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

pub struct Problem { 
    vehicles: Vec<Vehicle>,
    destinations: Vec<Location>,
}

impl Problem { 
    pub fn new(vehicles: Vec<Vehicle>, destinations: Vec<Location>) -> Self { 
        Problem { 
            vehicles, 
            destinations
        }
    }

    pub fn solve(self) -> SolvedProblem { 
        todo!()
    }

    /// https://en.wikipedia.org/wiki/First-fit-decreasing_bin_packing
    /// Order the items from largest to smallest.
    /// Open a new empty bin, bin #1.
    /// For each item from largest to smallest, find the first bin into which the item fits, if any.
    /// If such a bin is found, put the new item in it.
    /// Otherwise, open a new empty bin put the new item in it.
    pub fn solvable(self) -> bool {
        let mut demands: Vec<i64> = self.destinations.iter().map(|location| &location.demand).copied().collect();
        let mut capacities: Vec<i64> = self.vehicles.iter().map(|vehicle| &vehicle.capacity).copied().collect();

        demands.sort_by(|a, b| b.cmp(a));
        capacities.sort_by(|a, b| b.cmp(a));
        
        let mut put_in_bin: bool;
        for demand in demands { 
            put_in_bin = false;
            for capacity in capacities.iter_mut() { 
                dbg!(&demand, &capacity);
                if demand <= *capacity { 
                    *capacity -= demand;
                    put_in_bin = true;
                    break;
                }
            }
            if !put_in_bin { 
                return false
                }
        }
        true
    }
}

pub struct SolvedProblem { 
    routes: Vec<Route>,
}

impl SolvedProblem { 
    pub fn improve(self, _sequence: Vec<Operator>) -> SolvedProblem { 
        todo!()
    }
}