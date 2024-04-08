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

pub fn voronoy_tree(points: &Vec<Point>) -> (Vec<usize>, Vec<Vec<Polygon>>) {

    let mut sites: Vec<VoronoiPoint> = points
        .iter()
        .map(|&p| VoronoiPoint { x: p.x(), y: p.y() })
        .collect();

    let mut cells: Vec<Polygon>;
    let mut kept_all: Vec<usize> = (0..points.len()).step_by(1).collect();
    let mut orders: Vec<usize> = vec![0; points.len()];

    let width = 100_f64;
    let height = 100_f64;
    let mut k = 0_usize;

    let mut cells_all: Vec<Vec<Polygon>> = vec![];

    loop {
        let diagram = VoronoiBuilder::default()
            .set_sites(sites.clone())
            .set_bounding_box(BoundingBox::new(VoronoiPoint { x: 50., y: 50. }, width, height))
            .build()
            .unwrap();

        cells = diagram
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

        cells_all.push(cells);

        let mut free = vec![true; sites.len()];

        let mut excluded: Vec<usize> = vec![];

        for (i, _) in &areas {
            if free[*i] {
                free[*i] = false;
                for nb in diagram.cell(*i).iter_neighbors() {
                    free[nb] = false;
                }
                excluded.push(*i);
                orders[kept_all[*i]] = k;
                k+=1;
            }
        }

        excluded.sort();

        for i in 0..excluded.len() {
            sites.remove(excluded[i] - i);
        }

        if sites.len() < 3 {
            for ex in excluded.iter() {
                areas.retain(|(i, _) | i != ex)
            }
            for (i, _) in areas.iter() {
                orders[kept_all[*i]] = k;
                k+=1;
            }
            break;
        } else {
            for i in 0..excluded.len() {
                kept_all.remove(excluded[i] - i);
            }
        }

    }

    return (orders, cells_all);
}

