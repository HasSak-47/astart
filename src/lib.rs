pub mod ffi;
use std::{fmt::Debug, marker::PhantomData};

#[derive(Debug)]
pub struct Neightbor<N>{
    pub distance: f64,
    pub ident: N
}

impl<N> Neightbor<N>{
    pub const fn new(ident: N) -> Self{
        Self{ident, distance: 1.}
    }
}

pub trait World<N>{
    fn get_start(&self) -> N;
    fn is_end(&self, n: N) -> bool;
    fn get_neightbors(&mut self, n: N) -> Vec<Neightbor<N>>;
    fn heuristic(&self, n: N) -> f64;
}

pub struct AStart<N, W>
where
    N: Clone,
    W: World<N>
{
    n: PhantomData<N>,
    candidates : Vec<Node<N>>,
    pub world: W,
    pub explored : Vec<N>,
}

#[derive(Debug, Clone)]
pub struct Node<N: Clone>{
    distance: f64, // g
    heuristic: f64, // h
    score: f64, // f
    ident: N,
    parent: Option<Box<Node<N>>>,
}

impl<N: Clone> Node<N>{
    fn new(ident: N) -> Self{
        Self { score: 0., distance: 0., heuristic: 0., ident, parent: None }
    }
}

pub enum Step<N> where
    N: Clone,
{
    End(Option<Box<Node<N>>>),
    Continue
}

impl<N,  W> AStart<N, W>
where
    N: Clone + PartialEq,
    W: World<N>,
{
    pub fn new(world: W) -> Self{
        let mut s = Self{
            world,
            n: PhantomData,
            candidates: Vec::new(),
            explored: Vec::new(),

        };
        s.candidates.push( Node::new(s.world.get_start()) );

        return s;
    }


    pub fn step(&mut self) -> Step<N>{
        let last = self.candidates.pop().unwrap();
        if self.world.is_end(last.ident.clone()){
            return Step::End(Some(Box::new(last)));
        }
        let ns : Vec<_> = self.world
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
        for ns in ns{
            if self.explored.iter().find(|n| **n == ns.ident).is_some() {
                continue;
            }
            if let Some(repetition) = self.candidates.iter_mut() .find(|node| ns.ident == node.ident){
                if repetition.score < ns.score{
                    continue;
                }
                else{
                    *repetition = ns.clone();
                }
            }
            else{
                self.candidates.push(ns.clone());
            }
        }
        self.candidates.sort_by(|a, b| b.score.total_cmp(&a.score));
        if self.candidates.len() == 0{
            return Step::End(None);
        }
        self.explored.push(last.ident);
        return Step::Continue;
    }

    pub fn start(&mut self) -> Vec<N>{
        loop{
            if let Step::End(mut last) = self.step(){
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

    }
}
