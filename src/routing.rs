use super::operator::Operator;
use crate::vehicle::Vehicle;
use crate::location::Location;
use crate::route::Route;
use crate::errors::InfeasableProblem;

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