use std::fmt::{Display, Formatter};

use crate::sat::{SATClause, SATLiteral, SAT};


#[derive(PartialEq, Eq, Debug)]
pub struct Triplet {
    x: u8,
    y: u8,
    z: u8
}

impl Triplet {
    pub fn two_less(&self, other: &Triplet) -> bool {
        (self.x < other.x && self.y < other.y) || (self.x < other.x && self.z < other.z) || (self.y < other.y && self.z < other.z)
    }
}

impl Display for Triplet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[derive(Clone)]
pub struct TripletSet {
    n: u8
}

impl TripletSet {
    pub fn new(n: u8) -> Self {
        assert!(n >= 1);
        Self {
            n
        }
    }
    
    pub fn max_value(&self) -> u8 {
        self.n as u8
    }

    pub fn max_index(&self) -> u16 {
        let n_16 = self.n as u16;
        n_16 * n_16 * n_16
    }

    pub fn triplet(&self, i: u16) -> Triplet {
        if i > self.max_index() {
            println!("idx {} bigger than max: {}", i, self.max_index());
        }
        assert!(i <= self.max_index());
        let n_16 = self.n as u16;
        Triplet {
            x: (i % n_16 as u16 + 1) as u8,
            y: ((i / n_16) % n_16 + 1) as u8,
            z: ((i / (n_16 * n_16)) % n_16 + 1) as u8
        }
    }
}

pub struct TripletSequence {
    k: u8,
    set: TripletSet
}

impl TripletSequence {
    pub fn new(set: TripletSet, k: u8) -> Self {
        Self {
            set, k
        }
    }

    pub fn generate_sat(&self) -> SAT {
        let mut sat_clauses = Vec::<SATClause>::new();

        for r in 1..=self.k as i64 {
            let mut sat_literals = Vec::<SATLiteral>::new();
            for i in 1..self.set.max_index() {
                sat_literals.push(SATLiteral::new(i as u16, r as u8, false));
            }
            sat_clauses.push(SATClause::new(sat_literals));
        }

        for i in 1..=self.set.max_index() {
            for s in 1..=self.k {
                for r in 1..s {
                    let mut sat_literals = Vec::<SATLiteral>::new();
                    sat_literals.push(SATLiteral::new(i as u16, r as u8, true));
                    sat_literals.push(SATLiteral::new(i as u16, s as u8, true));
                    sat_clauses.push(SATClause::new(sat_literals));
                }
            }
        }

        for s in 1..=self.k {
            for r in 1..s {
                for j in 1..=self.set.max_index() {
                    for i in 1..self.set.max_index() {
                        if self.set.triplet(i).two_less(&self.set.triplet(j)) {
                            continue;
                        }

                        let mut sat_literals = Vec::<SATLiteral>::new();
                        sat_literals.push(SATLiteral::new(i as u16, r as u8, true));
                        sat_literals.push(SATLiteral::new(j as u16, s as u8, true));
                        sat_clauses.push(SATClause::new(sat_literals));
                    }
                }
            }
        }

        SAT::new(sat_clauses, self.set.clone(), self.k as u8)
    }
}
