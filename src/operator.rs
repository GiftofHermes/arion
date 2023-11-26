
pub enum Domain {
    Intra, 
    Inter
}

pub enum Operator { 
    Opt(Domain, usize),
    Exchange(Domain, usize, usize)
}
