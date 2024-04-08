extern crate geo;
extern crate voronoice;
use geo::{Coord, Point, LineString, Polygon, Area};
use voronoice::{Point as VoronoiPoint, VoronoiBuilder, BoundingBox};
use std::cmp::Ordering;

fn cmp_f64(a: &f64, b: &f64) -> Ordering {
    if a < b {
        return Ordering::Less;
    } else if a > b {
        return Ordering::Greater;
    }
    return Ordering::Equal;
}

pub fn voronoy_tree(points: &Vec<Point>) -> Vec<Polygon> {

    let sites = points
        .iter()
        .map(|&p| VoronoiPoint { x: p.x(), y: p.y() })
        .collect();

    let diagram = VoronoiBuilder::default()
        .set_sites(sites)
        .set_bounding_box(BoundingBox::new(VoronoiPoint { x: 15., y: 15. }, 10., 10.))
        .set_lloyd_relaxation_iterations(5)
        .build()
        .unwrap();

    let cells: Vec<Polygon> = diagram
        .iter_cells()
        .map(|cell|
             Polygon::new(
                 LineString::new(
                    cell
                        .iter_vertices()
                        .collect::<Vec<&VoronoiPoint>>()
                            .iter()
                            .map(|&vp| Coord {x: vp.x, y: vp.y})
                            .collect()
                 ),
                 vec![]
             )
        )
        .collect();

    let mut areas : Vec<(usize, f64)>  = cells
        .iter()
        .enumerate()
        .map(|(i, elem)| (i, elem.unsigned_area()))
        .collect();

    areas.sort_by(|a, b| cmp_f64(&a.1, &b.1));

    let mut free = vec![true; points.len()];

    for (i, _) in &areas {
        if free[*i] {
            free[*i] = false;
            for nb in diagram.cell(*i).iter_neighbors() {
                free[nb] = false;
            }
        }
    }

    for el in &areas {
        println!("{} {} {}", el.0, el.1, free[el.0]);
    }



    return cells;
}

