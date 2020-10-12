
extern crate kdtree;
use self::kdtree::*;

#[derive(Debug)]
struct Point
{
    id: String,
    vec: [f64;Point::DIM],
}

impl TraitPoint for Point
{
    const DIM: usize = 2;

    #[inline]
    fn dim(&self, depth: usize) -> f64
    {
        let axis = depth % Point::DIM;
        self.vec[axis]
    }
}

fn main()
{

    let mut vec_point : Vec<Point> = Vec::new();

    vec_point.push(Point{id: "hoge1".to_string(), vec: [1.0, 2.0]});
    vec_point.push(Point{id: "hoge2".to_string(), vec: [3.0, 4.0]});
    vec_point.push(Point{id: "hoge3".to_string(), vec: [5.0, 6.0]});

    let mut kdt = KDTree::new(&vec_point);
    kdt.build();

    print!("{:#?}\n", kdt);

    let p = Point{id: "query".to_string(), vec: [3.1, 4.1]};
    let k = kdt.knn_search(&p, 1);
    print!("{:?}\n", k);
    let r = kdt.radius_search(&p, 1.0);
    print!("{:?}\n", r);

    std::process::exit(0);
}
