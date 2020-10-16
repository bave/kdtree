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
        let query_iteration = 1_000_000;

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
        //kdt.set_cross(2);
        print!("cross:{} / nodes: {}\n", kdt.get_cross(), node_numbers);

        let start = std::time::Instant::now();
        kdt.build();
        let end = start.elapsed();

        print!("	build: {}.{:09}[sec]\n", end.as_secs(), end.subsec_nanos());

        print!("ter:{}\n", query_iteration);
        let mut vec_query : Vec<Point> = Vec::new();
        for i in 0 .. query_iteration {
            let x: f64 = rng.gen_range(rmin, rmax);
            let y: f64 = rng.gen_range(rmin, rmax);
            let z: f64 = rng.gen_range(rmin, rmax);
            let q = Point{id: i.to_string(), vec: [x, y, z]};
            vec_query.push(q);
        }

        let tstart = std::time::Instant::now();
        for i in &vec_query {
            //let start = std::time::Instant::now();
            let _ = kdt.knn_search(i, 1);
            //let end = start.elapsed();
        }
        let tend = tstart.elapsed();

        print!("	knn:1      ");
        print!("	total_time: {}.{:09}[sec]\n", tend.as_secs(), tend.subsec_nanos());

        let tstart = std::time::Instant::now();
        for i in &vec_query {
            //let start = std::time::Instant::now();
            let _ = kdt.radius_search(i, 100.0);
            //let end = start.elapsed();
        }
        let tend = tstart.elapsed();

        print!("	radius:100 ");
        print!("	total_time: {}.{:09}[sec]\n", tend.as_secs(), tend.subsec_nanos());

        std::process::exit(0);
}
