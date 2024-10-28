use std::fmt::Debug;

use a_start::{AStart, Neightbor, NodeIdentifier, World};
use anyhow::Result;

#[derive(Default, Clone, PartialEq, Eq)]
pub struct SlidePuzzle{
    data: Vec<Vec<usize>>,
    width: usize,
    heigth: usize,
}

impl Debug for SlidePuzzle{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl SlidePuzzle {
    fn new(width: usize, heigth: usize) -> Self{ 
        let mut col = Vec::with_capacity(heigth);
        col.resize(heigth, 0);
        let mut s = Self { data: Vec::new(), width, heigth};
        s.data.resize(width, col);
        for i in 0..width{
            for j in 0..heigth{
                s.data[i][j] = i + j * width;
            }
        }

        return s;
    }

    fn slice_left(&mut self) {
        let mut posx = 0;
        let mut posy = 0;
        for i in 0..self.width{
            for j in 0..self.heigth{
                if self.data[i][j] == 16 {
                    posx = i;
                    posy = j
                }
            }
        }

        if posx > 1 {
            self.data[posx][posy] = self.data[posx - 1][posy];
            self.data[posx - 1][posy] = 16;
        }
    }

    fn slice_right(&mut self) {
        let mut posx = 0;
        let mut posy = 0;
        for i in 0..self.width{
            for j in 0..self.heigth{
                if self.data[i][j] == 16 {
                    posx = i;
                    posy = j
                }
            }
        }

        if posx < self.width - 1{
            self.data[posx][posy] = self.data[posx + 1][posy];
            self.data[posx + 1][posy] = 16;
        }
    }

    fn slice_up(&mut self) {
        let mut posx = 0;
        let mut posy = 0;
        for i in 0..self.width{
            for j in 0..self.heigth{
                if self.data[i][j] == 16 {
                    posx = i;
                    posy = j
                }
            }
        }

        if posy < self.width  - 1{
            self.data[posx][posy] = self.data[posx][posy + 1];
            self.data[posx][posy + 1] = 16;
        }
    }

    fn slice_down(&mut self) {
        let mut posx = 0;
        let mut posy = 0;
        for i in 0..self.width{
            for j in 0..self.heigth{
                if self.data[i][j] == 16 {
                    posx = i;
                    posy = j
                }
            }
        }

        if posy > 1{
            self.data[posx][posy] = self.data[posx][posy - 1];
            self.data[posx][posy - 1] = 16;
        }
    }
}

#[derive(Debug, Default)]
struct SliceWorld{
    puzzles: Vec<SlidePuzzle>,
}

impl NodeIdentifier for SlidePuzzle { }

impl World<SlidePuzzle> for SliceWorld{
    fn get_start(&self) -> SlidePuzzle {
        SlidePuzzle::new(4, 4)
    }

    fn is_end(&self, n: SlidePuzzle) -> bool {
        n == SlidePuzzle::new(4, 4)
    }

    fn heuristic(&self, n: SlidePuzzle) -> f64 {
        0.
    }

    fn get_neightbors(&mut self, n: SlidePuzzle) -> Vec<Neightbor<SlidePuzzle>> {
        let mut posx = 0;
        let mut posy = 0;
        for i in 0..n.width{
            for j in 0..n.heigth{
                if n.data[i][j] == 16 {
                    posx = i;
                    posy = j
                }
            }
        }
        let mut v = Vec::new();
        if posx > 1 {
            let mut c = n.clone();
            c.data[posx][posy] = n.data[posx - 1][posy];
            c.data[posx - 1][posy] = 16;
            v.push(Neightbor::new(c));
        }
        if posx < n.width - 1{
            let mut c = n.clone();
            c.data[posx][posy] = n.data[posx + 1][posy];
            c.data[posx + 1][posy] = 16;
            v.push(Neightbor::new(c));
        }
        if posy > 1 {
            let mut c = n.clone();
            c.data[posx][posy] = n.data[posx][posy - 1];
            c.data[posx][posy - 1] = 16;
            v.push(Neightbor::new(c));
        }
        if posy < n.width - 1{
            let mut c = n.clone();
            c.data[posx][posy] = n.data[posx][posy + 1];
            c.data[posx][posy + 1] = 16;
            v.push(Neightbor::new(c));
        }
        return v;
    }
}

impl SliceWorld {
    fn new() -> Self{
        let mut start = SlidePuzzle::new(4, 4);
        start.slice_up();
        start.slice_down();
        start.slice_right();
        start.slice_left();

        Self {puzzles : vec![start]}
    }
}

#[test]
fn slider() -> Result<()>{
    let mut a = AStart::new(SliceWorld::new());
    a.start();
    Ok(())
}
