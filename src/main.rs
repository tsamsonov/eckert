extern crate geo;
extern crate eckert;
extern crate wkt;
extern crate rand;

use geo::{Point};
use wkt::ToWkt;
use rand::{distributions::Uniform, Rng};

fn main() {
    let n = 100;
    let range = Uniform::from(0.0..100.0);
    let xvalues: Vec<f64> = rand::thread_rng().sample_iter(&range).take(n).collect();
    let yvalues: Vec<f64> = rand::thread_rng().sample_iter(&range).take(n).collect();

    let mut pts: Vec<Point> = vec![];
    for (x, y) in xvalues.iter().zip(yvalues.iter()) {
        pts.push(Point::new(*x, *y));
    }

    let (orders, cells) = eckert::voronoy_tree(&pts);
    let n_1 = pts.len()-1;
    println!("DROP TABLE IF EXISTS pts;");
    println!("CREATE TABLE public.pts(fid serial primary key, ord int, geom geometry);");
    println!("INSERT INTO pts (ord, geom) VALUES");
    for i in 0..n_1 {
        let wkt = pts[i].wkt_string();
        let order = orders[i];
        println!("({}, '{}'::geometry),", order, wkt);
    }

    let wkt = pts[n_1].wkt_string();
    let order = orders[n_1];
    println!("({}, '{}'::geometry);", order, wkt);

    println!("DROP TABLE IF EXISTS voronoy;");
    println!("CREATE TABLE public.voronoy(fid serial primary key, iter int, geom geometry);");

    for i in 0..cells.len() {
        println!("INSERT INTO voronoy (iter, geom) VALUES");
        let n_1 = cells[i].len()-1;
        for j in 0..n_1 {
            let wkt = cells[i][j].wkt_string();
            println!("({}, '{}'::geometry),", i, wkt);
        }
        let wkt = cells[i][n_1].wkt_string();
        println!("({}, '{}'::geometry);", i, wkt);
    }

}