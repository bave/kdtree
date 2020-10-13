# kdtree
kdtree

## Cargo.toml
```toml
[dependencies]
kdtree = { git = "https://github.com/bave/kdtree.git" }
```

## example
/src/bin/main.rs

```rust
extern crate kdtree;
//use self::kdtree::*;
use kdtree::*;

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
    print!("knn return indices: {:?}\n", k);
    print!("nearlest neighbor Point:{:?}\n", vec_point[k[0]]);
    let r = kdt.radius_search(&p, 1.0);
    print!("radius search return indices: {:?}\n", r);
    print!("nearlest neighbor Point:{:?}\n", vec_point[r[0]]);
}
```

## run
```bash
$ cargo run
KDTree {
    root: Some(
        Node {
            idx: 1,
            axis: 0,
            left: Some(
                Node {
                    idx: 0,
                    axis: 1,
                    left: None,
                    right: None,
                },
            ),
            right: Some(
                Node {
                    idx: 2,
                    axis: 1,
                    left: None,
                    right: None,
                },
            ),
        },
    ),
    points: [
        Point {
            id: "hoge1",
            vec: [
                1.0,
                2.0,
            ],
        },
        Point {
            id: "hoge2",
            vec: [
                3.0,
                4.0,
            ],
        },
        Point {
            id: "hoge3",
            vec: [
                5.0,
                6.0,
            ],
        },
    ],
}
knn return indices: [1]
nearlest neighbor Point:Point { id: "hoge2", vec: [3.0, 4.0] }
radius search return indices: [1]
nearlest neighbor Point:Point { id: "hoge2", vec: [3.0, 4.0] }
```

