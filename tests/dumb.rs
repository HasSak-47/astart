use a_start::{AStart, Neightbor, NodeIdentifier, World};
use anyhow::Result;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TestNode{
    x: i32,
    y: i32,
}

const TARGET : TestNode = TestNode::new(2, 2);

impl TestNode{
    const fn new(x: i32, y: i32) -> Self{
        Self{ x, y, }
    }
}

impl NodeIdentifier for TestNode {
    fn heuristic(&self) -> f64 {
        let dx = (self.x - TARGET.x) as f64;
        let dy = (self.y - TARGET.y) as f64;

        return (dx * dx + dy * dy).sqrt();
    }
}


#[derive(Debug, Default)]
struct Node{
    ident: TestNode,
    explored: bool,
}

#[derive(Debug, Default)]
struct WorldGrid{
    data: Vec<Node>,
}

impl World<TestNode> for WorldGrid{
    fn is_end(&self, n: TestNode) -> bool {
        n == TARGET
    }

    fn get_start(&self) -> TestNode {
        return TestNode { x: 0, y: 0 };
    }

    fn get_neightbors(&mut self, n: TestNode) -> Vec<TestNode> {
        let mut next = vec![
            TestNode::new(n.x + 1, n.y),
            TestNode::new(n.x - 1, n.y),
        ];
        if n.x % 2 == 1{

            next.push( TestNode::new(n.x, n.y + 1) );
            next.push( TestNode::new(n.x, n.y - 1) );
        }

        let mut send = Vec::new();
        for n in next{
            for d in &self.data{
                if n != d.ident{
                    send.push(n);
                }
            }
        }

        return send;
    }

    fn is_end_mut(&mut self, n: TestNode) -> bool {
        n == TARGET
    }
}

impl WorldGrid{
    fn new() -> Self{
        Self { data: vec![ Node { ident: TestNode {x: 0, y: 0}, explored : true } ]}
    }
}

#[test]
fn test() -> Result<()>{
    let mut world = WorldGrid::new();
    let mut a = AStart::new(world);
    let path = a.start();
    for p in path{
        println!("node: {p:#?}");
    }
    Ok(())
}