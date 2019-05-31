extern crate geo;

use geo::algorithm::centroid::Centroid;
use geo::{Coordinate, LineString};

fn main() {
    let mut vec = Vec::new();
    vec.push(Coordinate {
        x: 40.02f64,
        y: 116.34,
    });
    vec.push(Coordinate {
        x: 41.02f64,
        y: 116.34,
    });
    let linestring = LineString(vec);
    println!("Centroid {:?}", linestring.centroid());
}
