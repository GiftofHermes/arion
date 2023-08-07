pub struct Location {
    x: f32,
    y: f32,
    demand: i64, 
}

pub struct Vehicle {
    name: String,
    capacity: u32,
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

pub struct SolvedProblem { 
    routes: Vec<Route>,
}