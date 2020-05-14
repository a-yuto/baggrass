use std::collections::HashSet;
use std::cmp::PartialEq;
use std::cmp::Eq;

extern crate rand;
use rand::Rng;

#[derive(Hash,PartialEq,Eq,Debug)]
struct member{
    id: usize,
    department: String,
    salary: i32
}

fn make_mamber(id: usize, department: &str, salary: i32) -> member {
    member{id: id,department: department.to_string(),salary: salary}
}
fn make_member_data(n: usize) -> HashSet<member> {
    let mut _table = HashSet::new();
    for i in 0..n {
        let mut salary = rand::thread_rng().gen_range(300, 1000);
        let mut department = match rand::thread_rng().gen_range(0, 3) {
            0 => "EC",
            1 => "engineer",
            2 => "jinji",
            _ => "jutyu"
        };
        _table.insert(make_mamber(i, department, salary));
    }
    _table
}

fn main() {
    let mut _table = HashSet::new();
    _table.insert( make_mamber(1,"EC",300) );
    let data = make_member_data(10);
}
