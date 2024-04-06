extern crate geo;
extern crate voronoice;
use geo::{Coord, Point, LineString, Polygon};
use voronoice::{Point as VoronoiPoint, VoronoiBuilder};

fn voronoy_tree(points: &Vec<Point>) -> Vec<Polygon> {
    // let orders = vec![0.; points.len()];

    let sites = points
        .iter()
        .map(|&p| VoronoiPoint { x: p.x(), y: p.y() })
        .collect();

    let diagram = VoronoiBuilder::default()
        .set_sites(sites)
        .set_lloyd_relaxation_iterations(5)
        .build()
        .unwrap();

    let cells = diagram
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


    return cells;
}

