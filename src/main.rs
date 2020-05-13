use std::collections::HashSet;
use std::cmp::PartialEq;
use std::cmp::Eq;
extern crate rand;
use rand::Rng;
type Table = HashSet<Member>;

#[derive(Hash,PartialEq,Eq,Debug)]
struct Member{
    id: usize,
    department: String,
    salary: i32
}

fn make_mamber(id: usize, department: &str, salary: i32) -> Member {
    Member{id: id,department: department.to_string(),salary: salary}
}
fn make_member_data(n: usize) -> Table {
    let mut _table = HashSet::new();
    for i in 0..n {
        let salary = rand::thread_rng().gen_range(300, 1000);
        let department = match rand::thread_rng().gen_range(0, 3) {
            0 => "EC",
            1 => "engineer",
            2 => "jinji",
            _ => "jutyu"
        };
        _table.insert(make_mamber(i, department, salary));
    }
    _table
}

fn group_by(table :&Table,column_name: &str) -> Vec<table> {
    
}

fn print_table(table: &Table) {
    let colum = vec!["id","department","salary"];
    for val in &colum {
        print!("{} ",val);
    }
    println!("\n-------------------------");
    for tmp_column in table{
        let first_brank = match tmp_column.id < 10 {
            true  => " ",
            false => "",
        };
        let mut second_brank = "".to_string();
        for _i in 0..(colum[1].len() - tmp_column.department.len()) {
            second_brank += " ";
        }
        let mut third_brank = " ".to_string();
        for _i in 0..(colum[2].len() -3) {
            third_brank += " ";
        }
        println!("{}{}{}{}{}{}",first_brank,tmp_column.id,second_brank,tmp_column.department,third_brank,tmp_column.salary);
    } 
} 

fn main() {
    let _data = make_member_data(100);
    print_table(&_data);
}
