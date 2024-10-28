use std::{fmt::Debug, marker::PhantomData};


#[derive(Debug)]
pub struct Neightbor<N: NodeIdentifier>{
    pub distance: f64,
    pub node: N
}

impl<N: NodeIdentifier> Neightbor<N>{
    pub const fn new(node: N) -> Self{
        Self{node, distance: 1.}
    }
}

pub trait World<N: NodeIdentifier>{
    fn get_start(&self) -> N;
    fn is_end(&self, n: N) -> bool;
    fn get_neightbors(&mut self, n: N) -> Vec<N>;
    fn is_end_mut(&mut self, n: N) -> bool;
}

pub trait NodeIdentifier where
    Self: Sized + Clone
{
    fn heuristic(&self) -> f64;
}

pub struct AStart<N: NodeIdentifier, W: World<N>>{
    world: W,
    n: PhantomData<N>,
}

#[derive(Debug, Clone)]
struct Node<N: NodeIdentifier + Debug + Clone>{
    score: f64,
    ident: N,
    parent: Option<Box<Node<N>>>,
}

impl<N: NodeIdentifier + Debug> Node<N> {
    fn new(ident: N) -> Self{
        let score = ident.heuristic();
        Self{
            ident, score, parent: None,
        }
    }
}


impl<N,  W> AStart<N, W>
where
    N: NodeIdentifier + Debug,
    W: World<N> + Debug,
{
    pub fn new(world: W) -> Self{
        let s = Self{ world, n: PhantomData };
        return s;
    }

    fn __start(&mut self) -> Option<Box<Node<N>>>{
        let start = self.world.get_start();

        let mut candidates = vec![Node::new(start)];
        loop{
            let last = candidates.pop().unwrap();
            if self.world.is_end(last.ident.clone()){
                return Some(Box::new(last));
            }

            let mut ns : Vec<_> = self.world
                .get_neightbors(last.ident.clone())
                .into_iter()
                .map(|n|{
                    let mut n = Node::new(n);
                    n.parent = Some(Box::new(last.clone()));
                    n.score += last.score;
                    return n
                }).collect();
            if ns.len() == 0{
                return None;
            }
            candidates.append(&mut ns);
            candidates.sort_by(|a, b| b.score.total_cmp(&a.score));
        }
    }

    pub fn start(&mut self) -> Vec<N>{
        let mut last = self.__start();
        let mut v = Vec::new();

        while last.is_some(){
            let data = last.unwrap();
            v.push(data.ident);
            last = data.parent;
        }
        return v
            .into_iter()
            .rev()
            .collect();
    }
}
