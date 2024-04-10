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

    let (orders, lods) = eckert::voronoy_tree(&pts);
    let n_1 = pts.len()-1;
    println!("DROP TABLE IF EXISTS pts;");
    println!("CREATE TABLE public.pts(fid serial primary key, lod int, ord int, geom geometry);");
    println!("INSERT INTO pts (lod, ord, geom) VALUES");
    for i in 0..n_1 {
        let wkt = pts[i].wkt_string();
        let lod = lods[i];
        let order = orders[i];
        println!("({}, {}, '{}'::geometry),", lod, order, wkt);
    }

    let wkt = pts[n_1].wkt_string();
    let lod = lods[n_1];
    let order = orders[n_1];
    println!("({}, {}, '{}'::geometry);", lod, order, wkt);

    // SELECT st_collect(st_voronoipolygons(st_collect(geom)), st_collect(geom)) FROM public.pts
    // WHERE lod >= 10

}