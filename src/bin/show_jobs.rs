extern crate mythus;
extern crate diesel;

use self::mythus::*;
use self::models::*;
use self::diesel::prelude::*;

fn main(){
    use mythus::schema::jobs::dsl::*;

    let connection = establish_connection();
    let results = jobs.filter(published.eq(true))
        .limit(5)
        .load::<Job>(&connection)
        .expect("Error Loading Posts");

    println!("Displaying {} jobs", results.len());

    for job in results {
        println!("{}", job.title);
        println!("----------\n");
        println!("{}", job.body);
    }
}
