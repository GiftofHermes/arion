use crate::vehicle::Vehicle;
use crate::location::Location;
use crate::route::Route;
use crate::errors::InfeasableProblem;
use itertools::Itertools;

pub struct UnsolvedProblem { 
    vehicles: Vec<Vehicle>,
    destinations: Vec<Location>,
}

impl UnsolvedProblem { 
    pub fn new(vehicles: Vec<Vehicle>, destinations: Vec<Location>) -> Self { 
        UnsolvedProblem { 
            vehicles, 
            destinations
        }
    }

    pub fn initial_solution(self) -> Result<SolvedProblem, InfeasableProblem> { 
        if !self.solvable() { 
            return Err(InfeasableProblem)
        }

        let mut routes: Vec<Route> = Vec::new();
        let mut destinations = self.destinations;
        let mut vehicles = self.vehicles;

        destinations.sort_by(|a, b| b.cmp(a));        
        vehicles.sort_by(|a, b| b.cmp(a));        
        
        for vehicle in vehicles {
            let mut route = Route::new(vehicle);

            let mut remainder = Vec::new();
            for destination in destinations.into_iter() { 
                
                if destination.demand <= route.vehicle.capacity { 
                    dbg!(&route.vehicle);
                    route.vehicle.capacity -= destination.demand;
                    route.destinations.push(destination)
                } else { 
                    remainder.push(destination)
                }
            }
            destinations = remainder;

            routes.push(route)
        }

        dbg!(&routes);

        Ok(SolvedProblem::new(routes))
    }

    /// https://en.wikipedia.org/wiki/First-fit-decreasing_bin_packing
    /// Order the items from largest to smallest.
    /// Open a new empty bin, bin #1.
    /// For each item from largest to smallest, find the first bin into which the item fits, if any.
    /// If such a bin is found, put the new item in it.
    /// Otherwise, open a new empty bin put the new item in it.
    pub fn solvable(&self) -> bool {
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
    pub fn new(routes: Vec<Route>) -> SolvedProblem { 
        SolvedProblem { 
            routes: routes
        }
    }
    
    pub fn intra_two_opt(mut self) -> SolvedProblem { 
        fn two_opt(route: &mut Route) {
            let n_nodes = 2;
            let mut best_cost = route.cost();
            for i in 0..route.destinations.len()-(n_nodes-1) {
                let mut candidate_route_destinations = route.destinations.clone();
                candidate_route_destinations.swap(i, i+n_nodes);
                
                std::mem::swap(&mut route.destinations, &mut candidate_route_destinations);
                let cost = route.cost();
                if cost < best_cost { 
                    best_cost = cost;
                } else {
                    std::mem::swap(&mut route.destinations, &mut candidate_route_destinations);
                }
            }
         }

        for route in self.routes.iter_mut() { 
            two_opt(route)
        }

        self
    }

    pub fn intra_three_opt(mut self) -> SolvedProblem { 
        fn three_opt(route: &mut Route) { 
            let n_nodes: usize = 3;
            let mut best_cost = route.cost();

            let route_len = route.destinations.len();
            for i in 0..route_len-(n_nodes-1) {
                for j in i..route_len-(n_nodes-2) { 
                    for k in j..route_len {
                        let index = [i,j,k];
                        let options = index.iter().permutations(3);
                        'options: for option in options { 
                            let i2 = *option[0]; // vec length should be 3 for 3-opt
                            let j2 = *option[1]; // vec length should be 3 for 3-opt
                            let k2 = *option[2]; // vec length should be 3 for 3-opt

                            let mut candidate_route_destinations = route.destinations.clone();
                            candidate_route_destinations.swap(i, i2);
                            candidate_route_destinations.swap(j, j2);
                            candidate_route_destinations.swap(k, k2);

                            std::mem::swap(&mut route.destinations, &mut candidate_route_destinations);
                            let cost = route.cost();
                            if cost < best_cost { 
                                best_cost = cost;
                                break 'options;
                            } else {
                                std::mem::swap(&mut route.destinations, &mut candidate_route_destinations);
                            }
                        }
                    }
                }
            }
        }
        for route in self.routes.iter_mut() { 
            three_opt(route)
        }

        self
    }


}