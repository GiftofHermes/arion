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