use std::cell::Cell;

#[derive(Clone, Copy, Debug)]
struct Node { 
    id:u32,
}

struct Network {
    nperl:u32, //node per level
    total_nodes:Cell<usize>,
    total_levels:Cell<usize>,

    nodes:Vec<Cell<Node>>,
    levels:Vec<Cell<Node>>,

    last:Cell<Node>,
    next:Cell<Node>,
}

impl Network {
    fn new(node:Node) -> Self {
        let mut nodes = Vec::new();
        nodes.push(Cell::new(node));

        return Self {
            nperl:0,
            total_nodes:Cell::new(1),
            total_levels:Cell::new(0),

            nodes,
            levels:Vec::new(),

            last:Cell::new(node),
            next:Cell::new(node),
        }
    }

    fn push(&mut self, node:Cell<Node>){
        //ingress level 
        //new node at the right
        self.nodes.push(node.clone());
        self.last = node;

        let current = self.total_nodes.get();
        self.total_nodes.set(current + 1);
    }

    fn pop(&mut self, _level:u32){
        //ingress level
        //delete the rightest node in the level
        self.last = self.next.clone();
        let current = self.total_nodes.get() - 1;
        self.total_nodes.set(current - 1);
    }

    fn set_nl(&self){
        //set how many nodes per level
        println!("setting");
    }
    
    fn head_info(&self) -> Cell<Node> {
        return self.last.clone();
    }

    fn nodes_info(&self) -> Vec<Cell<Node>> {
        return self.nodes.clone();
    }

    fn many_nodes(&self) -> usize {
        return self.total_nodes.get();
    }
    fn many_levels(&self)-> usize {
        return self.total_levels.get();
    }
}

impl Node {
    fn new(id:u32) -> Self {
        return Self { id,}
    }
}


/*pub trait Firstline {
 *
    fn add(&self) -> u32; 
}

impl Firstline for Node {
    fn add(&self) -> u32 {
        return self.id;
    }
}

fn check_data(a:&dyn Firstline){
    println!("something: {}", a.add());
}
*/

fn main() {
    let node1 = Node::new(33);
    let node2 = Node::new(44);
    let node3 = Node::new(66);

    let mut network = Network::new(node1);

    network.push(Cell::new(node2));
    network.push(Cell::new(node3));

    println!("{}", network.many_nodes());
    println!("{}", network.many_levels());
    for node in network.nodes_info().iter() {
        println!("{:?}", node.get());
    };

    println!("head: {:?}",network.head_info().get());
} 
