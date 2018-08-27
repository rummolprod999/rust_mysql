extern crate mysql;

use mysql as my;

fn main() {
    let pool = my::Pool::new("mysql://root:1234@localhost:3306/tender").unwrap();
    let result = pool.prep_exec("SELECT purchase_object_info FROM tender", ());
    let rows = result.unwrap();
    for row in rows{
        let mut res = row.unwrap();
        let t: String = res.take(0).unwrap();
        println!("{}", t);

    };
}

