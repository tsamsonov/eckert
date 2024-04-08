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
    // for poly in tree {
    //     let wkt = poly.wkt_string();
    //     println!("SELECT ST_geomfromtext('{}')", wkt);
    //     println!("UNION ALL");
    // }

    println!("DROP TABLE IF EXISTS pts;");
    println!("CREATE TABLE public.pts(fid serial primary key, geom geometry);");
    println!("INSERT INTO pts (geom) VALUES");
    for i in 0..pts.len()-1 {
        let wkt = pts[i].wkt_string();
        println!("('{}'::geometry),", wkt);
    }

    let wkt = pts[pts.len()-1].wkt_string();
    println!("('{}'::geometry);", wkt);

    println!("DROP TABLE IF EXISTS voronoy;");
    println!("CREATE TABLE public.voronoy(fid serial primary key, ord int, geom geometry);");
    println!("INSERT INTO voronoy (ord, geom) VALUES");
    for i in 0..pts.len()-1 {
        let wkt = cells[i].wkt_string();
        let order = orders[i];
        println!("({}, '{}'::geometry),", order, wkt);
    }

    let wkt = cells[pts.len()-1].wkt_string();
    let order = orders[pts.len()-1];
    println!("({}, '{}'::geometry);", order, wkt);
}