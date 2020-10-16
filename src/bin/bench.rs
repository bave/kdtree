extern crate kdtree;

#[allow(unused_imports)]
use self::kdtree::*;
//use kdtree::*;

#[allow(unused_imports)]
use rand::prelude::*;

/*
#![feature(asm)]
#[allow(dead_code)]
fn rdtscp_unsafe() -> u64
{
    let mut aux : u32 = 0;
    let aux_ptr : *mut u32 = &mut aux;
    unsafe { std::arch::x86_64::__rdtscp(aux_ptr) }
}
*/

#[derive(Debug)]
struct Point
{
    id: String,
    vec: [f64;Point::DIM],
}

impl TraitPoint for Point
{
    const DIM: usize = 3;

    #[inline]
    fn dim(&self, depth: usize) -> f64
    {
        self.vec[depth]
    }
}

fn main()
{
        let node_numbers = 1_000_000;
        let query_iteration = 10;

        let mut rng = thread_rng();
        //let rmin = (-0x80000000/2) as f64;
        //let rmax = (0x7fffffff/2) as f64;

        let rmin = -10000 as f64;
        let rmax = 10000 as f64;

        let mut vec_point : Vec<Point> = Vec::new();
        for i in 0 .. node_numbers {
            let x: f64 = rng.gen_range(rmin, rmax);
            let y: f64 = rng.gen_range(rmin, rmax);
            let z: f64 = rng.gen_range(rmin, rmax);
            let p = Point{id: i.to_string(), vec: [x, y, z]};
            vec_point.push(p);
        }

        let mut kdt = KDTree::new(&vec_point);
        //kdt.set_cross(4);
        let start = std::time::Instant::now();
        kdt.build();
        let end = start.elapsed();
        print!("{} nodes\n", node_numbers);
        print!("build: {}.{:09}[sec]\n", end.as_secs(), end.subsec_nanos());

        let mut vec_query : Vec<Point> = Vec::new();
        for i in 0 .. query_iteration {
            let x: f64 = rng.gen_range(rmin, rmax);
            let y: f64 = rng.gen_range(rmin, rmax);
            let z: f64 = rng.gen_range(rmin, rmax);
            let q = Point{id: i.to_string(), vec: [x, y, z]};
            vec_query.push(q);
        }
        for i in &vec_query {
            let start = std::time::Instant::now();
            let k = kdt.knn_search(i, 5);
            let end = start.elapsed();
            print!("knn:5 time: {}.{:09}[sec], indecis:{:?}\n", end.as_secs(), end.subsec_nanos(), k);
        }

        for i in &vec_query {
            let start = std::time::Instant::now();
            let k = kdt.radius_search(i, 100.0);
            let end = start.elapsed();
            print!("radius:100 time: {}.{:09}[sec], indecis:{:?}\n", end.as_secs(), end.subsec_nanos(), k);
        }

        std::process::exit(0);
}
