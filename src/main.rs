use std::collections::HashSet;
use std::cmp::PartialEq;
use std::cmp::Eq;
extern crate rand;
use rand::Rng;
type Table = HashSet<Member>;

#[derive(Hash,PartialEq,Eq,Debug,Clone)]
struct Member{
    id: usize,
    department: String,
    salary: i32
}
impl Member{
    fn make_mamber(id: usize, department: &str, salary: i32) -> Member {
        Member{id: id,department: department.to_string(),salary: salary}
    }


    fn make_sample(n: usize) -> Table {
        let mut _table = HashSet::new();
        for i in 0..n {
            let salary = rand::thread_rng().gen_range(300, 1000);
            let department = match rand::thread_rng().gen_range(0, 3) {
                0 => "EC",
                1 => "engineer",
                2 => "jinji",
                _ => "jutyu"
            };
            _table.insert(Member::make_mamber(i, department, salary));
        }
        _table
    }

    fn group_by(table :&Table, column_name: &str) -> HashSet<HashSet<Member>> {
        let mut _values: HashSet<String> = HashSet::new();
        let mut _ans  = HashSet::new();
        for record in table {
            _values.insert((&record.department).to_string());
        }
        for department in _values {
            let mut tmp_table = HashSet::new();
            for record in table {
                if record.department == department {
                    tmp_table.insert(Member::make_mamber(record.id,&record.department,record.salary));
                }
            }
            _ans.insert(tmp_table);
        }
        _ans
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
            println!("{}{}{}{}{}{}",
                     first_brank,tmp_column.id,second_brank,tmp_column.department,third_brank,tmp_column.salary);
        } 
    } 
}
fn main() {
    Member::group_by(&Member::make_sample(10),"department");
}


#[test]
fn group_by_test() {
    let mut ec       = HashSet::new();
    let mut engineer = HashSet::new();
    let mut jinji    = HashSet::new();
    ec.insert(Member::make_mamber(1,"EC",300));
    ec.insert(Member::make_mamber(2,"EC",500));
    engineer.insert(Member::make_mamber(3,"engineer",400));
    jinji.insert(Member::make_mamber(4,"jinji",500));
    let     ans  = vec![ec,engineer,jinji].into_iter().collect::<HashSet<Member>>();
    let mut test = HashSet::new();
    test.insert(Member::make_mamber(1,"EC",300));
    test.insert(Member::make_mamber(2,"EC",500));
    test.insert(Member::make_mamber(3,"engineer",400));
    test.insert(Member::make_mamber(4,"jinji",500));
    assert_eq!(ans,Member::group_by(&test,"department"))
}
