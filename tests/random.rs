use a_start::{AStart, World};
use rand::random;


#[derive(Debug, Default, Clone)]
struct RandomWorld{
    nodes: Vec<(f64, f64)>,
    connections: Vec<(usize, usize)>,
    heuristic: bool,
}

impl RandomWorld{
    fn new() -> Self{
        let mut d = Self::default();
        d.nodes.resize_with(10, || {(random::<f64>(), random::<f64>())});
        d.connections.resize_with(5, || {(random::<usize>() % 10, random::<usize>() % 10)});
        for i in 0..9{
            d.connections.push((i, i + 1));
        }


        return d;
    }

    fn dist(&self, a: usize, b: usize) -> f64{
        let dx = self.nodes[a].0 - self.nodes[b].0;
        let dy = self.nodes[a].1 - self.nodes[b].1;
        return (dx * dx + dy * dy).sqrt();
    }
}

impl World<usize> for RandomWorld{
    fn is_end(&self, n: usize) -> bool {
        n == 9
    }

    fn get_start(&self) -> usize {
        0
    }

    fn heuristic(&self, n: usize) -> f64 {
        if !self.heuristic{
            return 0.;
        }
        let dx = self.nodes[n].0 - self.nodes[9].0;
        let dy = self.nodes[n].1 - self.nodes[9].1;
        return (dx * dx + dy * dy).sqrt();
    }

    fn get_neightbors(&mut self, n: usize) -> Vec<a_start::Neightbor<usize>> {
        let mut c = Vec::new();
        for conn in &mut self.connections{
            if conn.0 == n{
                c.push(*conn);
            }
            if conn.1 == n{
                let k = (conn.1, conn.0);
                c.push(k);
            }
        }

        return c.into_iter().map(|k|{
            a_start::Neightbor { distance: self.dist(k.0, k.1), ident: k.1 }

        }).collect();
    }
}


#[test]
fn rand_world() {
    let w = RandomWorld::new();
    let wstart = w.clone();
    let mut dstart = w;
    dstart.heuristic = false;
    let mut astart   = AStart::new(wstart);
    let mut dijsktra = AStart::new(dstart);
    let a = astart.start();
    let d = dijsktra.start();
    assert_eq!(a, d, "heuristic 0 == to some heuristic");

}
