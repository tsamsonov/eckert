extern crate geo;
extern crate eckert;
extern crate wkt;
extern crate rand;

use geo::{Point};
use wkt::ToWkt;
use rand::{distributions::Uniform, Rng};

fn main() {
    let n = 20;
    let range = Uniform::from(0.0..100.0);
    let xvalues: Vec<f64> = rand::thread_rng().sample_iter(&range).take(n).collect();
    let yvalues: Vec<f64> = rand::thread_rng().sample_iter(&range).take(n).collect();

    let mut pts: Vec<Point> = vec![];
    for (x, y) in xvalues.iter().zip(yvalues.iter()) {
        pts.push(Point::new(*x, *y));
    }

    let tree = eckert::voronoy_tree(&pts);
    for poly in tree {
        let wkt = poly.wkt_string();
        println!("SELECT ST_geomfromtext('{}')", wkt);
        println!("UNION ALL");
    }

    for p in pts {
        let wkt = p.wkt_string();
        println!("SELECT ST_geomfromtext('{}')", wkt);
        println!("UNION ALL");
    }
}