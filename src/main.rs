use std::cell::Cell;
use std::sync::Arc;

mod maths;
use crate::maths::math::procedure;
use crate::maths::math::sigmoid;

#[derive(Clone, Copy, Debug)]
struct Node { 
    id:u32,
    level:usize,
    output:f32,
}

impl Node {
    fn new(id:u32) -> Self {
        //let level = 0;
        return Self { 
            id,
            level:0,
            output:0.0,
        }
    }

    fn calculate(&mut self, net: Network) -> f32 {
        let node_level = self.level.clone();
        let previous = &net.levels[node_level-2]; //-1 to out of bound, and -1 for previous nodes
        let mut entries:Vec<f32> = Vec::new();

        for node in previous.iter() {
            let out = node.get().output;
            entries.push(out);
        }
        let result = sigmoid(entries);
        self.output = result.clone();
        return result;
    }
}

struct Network {
    nil:Cell<u32>, //node in level
    total_nodes:Cell<usize>,
    total_levels:Cell<usize>,

    nodes:Vec<Cell<Node>>,
    levels:Vec<Vec<Cell<Node>>>,

    last:Cell<Node>,
    next:Cell<Node>,
}

impl Network {
    fn new(node:Node) -> Self {
        let mut nodes = Vec::new();
        nodes.push(Cell::new(node));
        return Self {
            nil:Cell::new(0),
            total_nodes:Cell::new(1),
            total_levels:Cell::new(0),

            last:Cell::new(Node { id:0, level:0, output:0.0 }),
            next:Cell::new(Node { id:0, level:0, output:0.0 }),

            nodes,
            levels:Vec::new(),
        }
    }

    fn forward(&self){
        for level in self.levels.iter() {
            for node in level.iter(){
                println!("node: {:?}", node.get());
            }
        }
    }

    fn push(&mut self, node:Cell<Node>){
        let level = self.total_levels.get();
        let mut newnode = node.get();
        newnode.level = level;
        node.set(newnode);

        let current = self.total_nodes.get();
        self.total_nodes.set(current + 1);
        self.nil.set(self.nil.get() + 1);

        self.nodes.push(node.clone());
        self.next.set(self.last.get());
        self.last.set(node.get());
    }

    fn pop(&mut self){
        self.last.set(self.next.get());
        self.nodes.pop();

        let previous = self.total_nodes.get();
        self.total_nodes.set(previous - 1);
        self.nil.set(self.nil.get() - 1)
    }

    fn save_line(&mut self) {
        self.levels.push(self.nodes.clone());
    }

    fn new_line(&mut self) {
        //I must have a vector of vectors
        //then save the last vector into it and clean "nodes"
        self.levels.push(self.nodes.clone());
        let current = self.total_levels.get();
        self.total_levels.set(current + 1);
        self.nodes = Vec::new();//empty
        self.nil.set(0);
    }

    fn set_nl(&self){
        //set how many nodes per level
        println!("setting");
    }
    
    fn head_info(&self) -> Node {
        return self.last.get();
    }

    fn next_info(&self) -> Node {
        return self.next.get();
    }

    fn nodes_info(&self) {
        for node in self.nodes.iter() {
            println!("{:?}", node.get());
        }
    }

    fn many_nodes(&self) {
        println!("{}",self.total_nodes.get())
    }
    fn many_levels(&self){
        println!("{}",self.total_levels.get())
    }
}

fn main() {

} 
