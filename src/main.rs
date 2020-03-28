use iron::Iron;

mod web;

fn main() {
    let _server = Iron::new(web::router::app()).http("localhost:3000").unwrap();
    println!("On 3000");
}
