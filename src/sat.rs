use std::{collections::HashMap, io::Write};

use super::triplet::*;

pub struct SAT {
    clauses: Vec<SATClause>,
    set: TripletSet,
    sequence_length: u8
}

impl SAT {
    pub fn new(clauses: Vec<SATClause>, set: TripletSet, sequence_length: u8) -> Self {
        Self {
            clauses,
            set,
            sequence_length
        }
    }

    fn collect_literals(&self) -> HashMap<(u16, u8), u16> {
        let mut map = HashMap::<(u16, u8), u16>::new();

        for clause in &self.clauses {
            clause.collect_literals(&mut map);
        }
        map
    }

    pub fn write_to_file(&self, file: &mut dyn Write) {
        let literal_map = self.collect_literals();

        write!(file, "c 2-Less - Set Max (n): {} - Sequence length (k): {}\n", self.set.max_value(), self.sequence_length).unwrap();
        write!(file, "\n").unwrap();
        write!(file, "p cnf {} {}\n", literal_map.len(), self.clauses.len()).unwrap();

        write!(file, "\n").unwrap();
        for ((triplet_index, triplet_index_in_sequence), literal_idx) in &literal_map {
            write!(file, "c VAR {}: Triplet: {}, Pos in sequence: {}\n", *literal_idx, self.set.triplet(*triplet_index as u16), *triplet_index_in_sequence).unwrap();
        }
        write!(file, "\n").unwrap();

        for clause in &self.clauses {
            clause.write_to_file(file, &literal_map);
        }
    }
}

pub struct SATClause {
    literals: Vec<SATLiteral>
}

impl SATClause {
    pub fn new(literals: Vec<SATLiteral>) -> Self {
        Self {
            literals
        }
    }

    fn collect_literals(&self, map: &mut HashMap<(u16, u8), u16>) {
        for literal in &self.literals {
            let key = (literal.triplet_index(), literal.triplet_index_in_sequence);
            if map.contains_key(&key) {
                continue;
            }
            map.insert(key, map.len() as u16 + 1);
        }
    }

    pub fn write_to_file(&self, file: &mut dyn Write, literal_map: &HashMap<(u16, u8), u16>) {
        for literal in &self.literals {
            literal.write_to_file(file, literal_map);
            write!(file, " ").unwrap();
        }
        write!(file, "0\n").unwrap();
    }
}

#[repr(packed)]
pub struct SATLiteral {
    triplet_index_packed: u16,
    triplet_index_in_sequence: u8
}

impl SATLiteral {
    pub fn new(triplet_index: u16, triplet_index_in_sequence: u8, inverted: bool) -> Self {
        assert_eq!(triplet_index & (1 << 15), 0);
        let mut triplet_index_packed = triplet_index;
        if inverted {
            triplet_index_packed |= 1 << 15;
        }
        
        Self {
            triplet_index_packed, triplet_index_in_sequence
        }
    }

    pub fn triplet_index(&self) -> u16 {
        self.triplet_index_packed & !(1 << 15)
    }

    pub fn is_inverted(&self) -> bool {
        (self.triplet_index_packed >> 15) == 1
    }

    pub fn write_to_file(&self, file: &mut dyn Write, literal_map: &HashMap<(u16, u8), u16>) {
        write!(file, "{}{}", if self.is_inverted() { "-" } else { "" }, literal_map.get(&(self.triplet_index(), self.triplet_index_in_sequence)).unwrap()).unwrap();
    }
}
