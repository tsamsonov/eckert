extern crate geo;
extern crate eckert;
extern crate wkt;

use geo::{Point};
use wkt::ToWkt;
fn main() {
    let pts = vec![
        Point::new(10., 10.),
        Point::new(15., 20.),
        Point::new(20., 10.),
    ];
    let tree = eckert::voronoy_tree(&pts);
    for poly in tree {
        let wkt = poly.wkt_string();
        println!("{}", wkt)
    }
}