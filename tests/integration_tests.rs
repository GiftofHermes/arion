use arion;
use arion::solver::routing::{Vehicle, Location, Problem};

#[test]
fn it_is_solvable()  {
    let problem = Problem::new(
        vec![Vehicle::new(String::from("0"), 10), Vehicle::new(String::from("1"), 10)],
        vec![Location::new(10.0,10.0, 3), Location::new(10.0,10.0,5), Location::new(10.0,10.0,7)]
    );

    assert!(problem.solvable())
}

#[test]
fn not_solvable()  {
    let problem = Problem::new(
        vec![Vehicle::new(String::from("0"), 10), Vehicle::new(String::from("1"), 10)],
        vec![Location::new(10.0,10.0, 9), Location::new(11.0,11.0,9), Location::new(12.0,12.0,7)]
    );
    
    assert!(!problem.solvable())
}