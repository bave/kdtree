//#![allow(dead_code)]
//#![allow(unused_imports)]
//#![allow(unused_variables)]

#[macro_export]
macro_rules! type_of { ($exp: expr) => ( type_of(&($exp))) }
pub fn type_of<T>(_: &T) -> String
{
    std::any::type_name::<T>().to_owned()
}

#[macro_export]
macro_rules! is_exist
{
    ($vec: expr, $val: expr) => {
        $vec.iter().any(|&x| x == $val)
    };
}

extern crate qselect;
#[allow(unused_imports)]
use qselect::*;

extern crate num_cpus;

/*
#[allow(unused_imports)]
use async_recursion::async_recursion;
*/

pub trait TraitPoint
{
    const DIM: usize;
    fn dim(&self, d: usize) -> f64;
}

type Link = Option<Box<Node>>;

#[derive(Debug)]
pub struct KDTree<'a, T: TraitPoint + std::marker::Sync>
{
    root: Link,
    points: &'a Vec<T>,
    cross: usize,
}

impl<'a, T: TraitPoint +  std::marker::Sync> KDTree<'a, T>
{
    pub fn new(points: &'a Vec<T>) -> Self
    {
        Self {
            root: None,
            points: points,
            cross: ((num_cpus::get() as f64).log2().floor()) as usize,
        }
    }

    pub fn set_cross(&mut self, cross: usize)
    {
        self.cross = cross;
    }

    pub fn get_cross(&mut self) -> usize
    {
        self.cross
    }

    pub fn build(&mut self)
    {
        self.cleaer();
        let mut indices = (0..self.points.len()).collect::<Vec<usize>>();
        let len = indices.len()-1;

        //println!("{:?}", indices);

        self.root = Self::recurs_build(&self.points, self.cross, &mut indices, 0, len, 0);
        //print!("cross_depth:{}\n", self.cross);
    }

    fn cleaer(&mut self)
    {
        self.root = None
    }

    #[inline]
    fn recurs_build(p: &'a Vec<T>, c: usize, indices: &mut [usize], left: usize, right: usize, depth: usize)
       -> Option<Box<Node>>
    {
        let axis = depth % T::DIM;
        //print!("left:{}, right:{}, axis:{} ,depth:{}\n", left, right, axis, depth);

        if right == left {
            let node = Box::new(Node {
                idx: indices[left],
                axis: axis,
                left : None,
                right: None,
            });
            return Some(node);
        } 
        /*
        else if right < left { 
            return None;
        };
        */

        let mid = if right == 1 {
            1
        } else {
            (right+left) >> 1
        };
        //print!("mid:{}\n", mid);

        qselect_indirect(indices, left, right, mid, &|x| p[x].dim(axis));
        let (indices_left, indices_tmp) =  indices.split_at_mut(mid);
        let (_indices_mid, indices_right) =  indices_tmp.split_at_mut(1);
        //print!("{}: {:?} / {:?}  / {:?}\n", depth, indices_left, indices_mid, indices_right);

        let l_len = indices_left.len();
        let r_len = indices_right.len();

        let (left_node, right_node) = if depth <= c {
            crossbeam::scope(|s| {
                let left_scope = s.spawn(|_| { 
                    if l_len == 0 { None }
                    else { Self::recurs_build(p, c, indices_left, 0, l_len-1, depth + 1) }
                });
                let right_scope = s.spawn(|_| {
                    if r_len == 0 { None }
                    else { Self::recurs_build(p, c, indices_right, 0, r_len-1, depth + 1) }
                });
                let l = left_scope.join().unwrap();  //ScopedJoinHandle.unwrap
                let r = right_scope.join().unwrap(); //ScopedJoinHandle.unwrap
                (l, r)
            }).unwrap() //result.unwrap
        } else {
             let l = if l_len == 0 {
                 None
             } else {
                 Self::recurs_build(p, c, indices_left, 0, l_len-1, depth + 1)
             };
             let r = if r_len == 0 {
                 None
             } else {
                 Self::recurs_build(p, c, indices_right, 0, r_len-1, depth + 1)
             };
             (l, r)
        };

        let node = Box::new(Node {
            idx: indices[mid],
            axis: axis,
            left : left_node,
            right: right_node,
        });
        return Some(node);
    }

    #[inline]
    pub fn knn_search(&mut self, query: &T, k: usize) -> Vec<usize>
    {
        let mut queue : FixedSizePriorityQueue<(f64, usize)> = FixedSizePriorityQueue::new(k);
        let node = &self.root;

        self.recurs_knn_search(k, query, node, &mut queue);
        //return queue.vec.iter().map(|x| x.1).collect::<Vec<usize>>();
        return queue.vec.iter().map(|x| x.1).collect();
    }

    #[inline]
    fn recurs_knn_search(&self,
                         k: usize,
                         query: &T,
                         node: &Option<Box<Node>>,
                         queue: &mut FixedSizePriorityQueue<(f64, usize)>)
    {
        let (idx, axis, left, right) = match node {
            None => { return; },
            Some(n) => { (n.idx, n.axis, &n.left, &n.right) }
        };

        let cur = &self.points[idx];
        let dist = Self::dist(query, cur);
        queue.push((dist, idx));

        let is_left = if &query.dim(axis) < &cur.dim(axis) {
            self.recurs_knn_search(k, query, left, queue);
            true
        } else {
            self.recurs_knn_search(k, query, right, queue);
            false
        };

        let cond1 = queue.len() < k;

        let diff = (&query.dim(axis) - &cur.dim(axis)).abs();
        let cond2 = match queue.last() {
            None => { f64::NAN },
            Some(tuple) => { tuple.0 },
        } > diff;

        if cond1 || cond2 {
            match is_left {
                true => {
                    self.recurs_knn_search(k, query, right, queue);
                },
                false => {
                    self.recurs_knn_search(k, query, left, queue);
                },
            }
        };
    }

    #[inline]
    pub fn radius_search(&mut self, query: &T, radius: f64) -> Vec<usize>
    {
        let node = &self.root;
        let mut indices : Vec<usize> = Vec::new();
        self.recurs_radius_search(radius, query, node, &mut indices);
        return indices;
    }

    fn recurs_radius_search(&self,
                         radius: f64,
                         query: &T,
                         node: &Option<Box<Node>>,
                         indices: &mut Vec<usize>)
    {
        let (idx, axis, left, right) = match node {
            None => { return; },
            Some(n) => { (n.idx, n.axis, &n.left, &n.right) }
        };

        let cur = &self.points[idx];
        let dist = Self::dist(query, cur);
        if dist < radius {
            indices.push(idx);
        }

        let is_left = if &query.dim(axis) < &cur.dim(axis) {
            self.recurs_radius_search(radius, query, left, indices);
            true
        } else {
            self.recurs_radius_search(radius, query, right, indices);
            false
        };

        let diff = (&query.dim(axis) - &cur.dim(axis)).abs();
        if diff < radius {
            match is_left {
                true => {
                    self.recurs_radius_search(radius, query, right, indices);
                },
                false => {
                    self.recurs_radius_search(radius, query, left, indices);
                },
            }
        }
    }

    fn dist(l: &T, r: &T) -> f64
    {
        let mut dist : f64 = 0.0;
        for i in 0..T::DIM {
            let base = l.dim(i) - r.dim(i);
            dist += base * base;
        }
        dist.sqrt()
    }
}

#[derive(Debug)]
struct Node
{
    idx: usize,
    axis: usize,
    left: Link,
    right: Link,
}

/*
impl Drop for Node
{
    fn drop(&mut self) {
        println!("drop {:?}\n", self);
    }
}
*/

//min prio
#[derive(Debug)]
pub struct FixedSizePriorityQueue<T>
{
    pub vec: Vec<T>,
    pub max: usize,
}

impl<T> FixedSizePriorityQueue<T>
where T: PartialOrd
{
    pub fn new(s: usize) -> Self
    {
        Self {
            vec: Vec::with_capacity(s+1),
            max: s,
        }
    }

    pub fn push(&mut self, v: T)
    {
        match self.vec.iter().position(|x| x > &v) {
            Some(idx) => {
                self.vec.insert(idx, v);
            },
            None => {
                self.vec.push(v);
            }
        }
        if self.vec.len() > self.max {
            self.vec.pop();
        }
    }

    pub fn get(&self, idx: usize) -> Option<&T>
    {
        let l = self.len();
        if l > 0  || l < self.max {
            Some(&self.vec[idx])
        } else {
            None
        }
    }

    pub fn len(&self) -> usize
    {
        self.vec.len()
    }

    pub fn last(&self) -> Option<&T>
    {
        self.vec.last()
    }

    pub fn clear(&mut self)
    {
        self.vec.clear()
    }
}


#[cfg(test)]
mod tests
{
    use super::*;

    #[derive(Debug)]
    struct Point
    {
        id: String,
        vec: Vec<f64>,
    }

    impl Point
    {
        fn new(id: String, x: f64, y: f64, z: f64) -> Self
        {
            Self {
                id: id,
                vec: vec![x, y, z],
            }
        }
    }

    impl TraitPoint for Point
    {
        const DIM: usize = 3;

        #[inline]
        fn dim(&self, depth: usize) -> f64
        {
            let axis = depth % Point::DIM;
            self.vec[axis]
        }
    }

    #[test]
    fn build()
    {
        let mut ps : Vec<Point>= Vec::new();
        ps.push(Point::new("hoge0".to_string(), 0.0, 7.0, 0.0));
        ps.push(Point::new("hoge1".to_string(), 1.0, 6.0, 0.0));
        ps.push(Point::new("hoge2".to_string(), 2.0, 5.0, 0.0));
        ps.push(Point::new("hoge3".to_string(), 3.0, 4.0, 0.0));
        ps.push(Point::new("hoge4".to_string(), 4.0, 3.0, 0.0));
        ps.push(Point::new("hoge5".to_string(), 5.0, 2.0, 0.0));
        ps.push(Point::new("hoge6".to_string(), 6.0, 1.0, 0.0));
        ps.push(Point::new("hoge7".to_string(), 7.0, 0.0, 0.0));
        let mut kdt = KDTree::new(&ps); 
        kdt.build();
        print!("{:#?}\n", kdt);
        let p = Point::new("query".to_string(), 6.0, 1.0, 0.0);
        let r = kdt.knn_search(&p, 2);
        assert_eq!(r, [6, 5]);
        print!("{:?}\n", &r);
    }

    /*
    #[test]
    fn fspq()
    {
        let mut q : FixedSizePriorityQueue<(f64, usize)> = FixedSizePriorityQueue::new(5);
        q.push((5.0, 1));
        q.push((4.0, 2));
        q.push((3.0, 3));
        q.push((2.0, 4));
        q.push((1.0, 5));
        q.push((6.0, 6));
        assert_eq!(q.vec, [(1.0, 5), (2.0, 4), (3.0, 3), (4.0, 2), (5.0, 1)]);
        print!("{:?}\n", q);
        assert_eq!(q.get(1), Some(&(2.0, 4)));
        print!("{:?}\n", q.get(1));
        q.clear();
        assert_eq!(q.vec, []);
        print!("{:?}\n", q);

        let mut q : FixedSizePriorityQueue<f64> = FixedSizePriorityQueue::new(5);
        q.push(5.0);
        q.push(4.0);
        q.push(3.0);
        q.push(2.0);
        q.push(1.0);
        q.push(6.0);
        assert_eq!(q.vec, [1.0, 2.0, 3.0, 4.0, 5.0]);
        print!("{:?}\n", q);
        q.clear();
        assert_eq!(q.vec, []);
        print!("{:?}\n", q);
    }
    */
}
