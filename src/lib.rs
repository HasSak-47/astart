mod ffi;
use std::{fmt::Debug, marker::PhantomData};

pub trait NodeIdentifier where
    Self: Sized + Clone + PartialEq{}


#[derive(Debug)]
pub struct Neightbor<N: NodeIdentifier>{
    pub distance: f64,
    pub ident: N
}

impl<N: NodeIdentifier> Neightbor<N>{
    pub const fn new(ident: N) -> Self{
        Self{ident, distance: 1.}
    }
}

pub trait World<N: NodeIdentifier>{
    fn get_start(&self) -> N;
    fn is_end(&self, n: N) -> bool;
    fn get_neightbors(&mut self, n: N) -> Vec<Neightbor<N>>;
    fn heuristic(&self, n: N) -> f64;
}

pub struct AStart<N, W>
where
    N: NodeIdentifier,
    W: World<N>
{
    world: W,
    n: PhantomData<N>,
}

#[derive(Debug, Clone)]
struct Node<N: NodeIdentifier + Clone>{
    distance: f64, // g
    heuristic: f64, // h
    score: f64, // f
    ident: N,
    parent: Option<Box<Node<N>>>,
}

impl<N: NodeIdentifier + Clone> Node<N>{
    fn new(ident: N) -> Self{
        Self { score: 0., distance: 0., heuristic: 0., ident, parent: None }
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
                    let dist = n.distance;
                    let mut n = Node::new(n.ident);
                    n.distance = dist + last.distance;
                    n.parent = Some(Box::new(last.clone()));
                    n.heuristic = self.world.heuristic(n.ident.clone());
                    n.score = n.distance + n.heuristic;
                    return n
                }).collect();
            // candidates.append(&mut ns);
            for ns in ns{
                if let Some(repetition) = candidates.iter_mut() .find(|node| ns.ident == node.ident){
                    if repetition.distance  < ns.distance{
                        *repetition = ns;
                    }
                }
                else{
                    candidates.push(ns);
                }
            }
            candidates.sort_by(|a, b| b.score.total_cmp(&a.score));
            if candidates.len() == 0{
                return None;
            }
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
