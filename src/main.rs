use std::cell::Cell;

mod maths;
use crate::maths::math::procedure;
use crate::maths::math::sigmoid;
use crate::maths::math::random;

#[derive(Clone, Copy, Debug)]
struct Node { 
    id:u32,
    level:usize,
    output:f32,
}

impl Node {
    fn new(id:u32) -> Self {
        return Self { 
            id,
            level:0,
            output:0.0,
        }
    }

    fn polinom(&mut self, entry:Vec<f32>) -> f32 {
        //I am going to form the polynom here
        let mut ws:Vec<f32> = Vec::new();
        for ent in entry.iter(){
            let w = random();
            ws.push(w);
        }
        //calculating the output of the polinom
        let mut results:Vec<f32> = Vec::new();
        let sentry:usize = entry.len();
        for i in 1..=sentry {
            if let Some(element) = entry.get(i-1){
                if let Some(weigth) = ws.get(i-1){
                    let result = element * weigth;
                    results.push(result);
                }
            } else {
                println!("index out of bounds");
            }
        }
        let mut output:f32 = 0.0;
        for res in results.iter(){
            output = output + res;
        }
        return output;
    }
   
    fn kickstart(&mut self, entry:Vec<f32>) -> (f32, Vec<f32>) {
        let mut polinom:f32 = 0.0;
        let mut ws:Vec<f32> = Vec::new();
        for ent in entry.iter(){
            let out = sigmoid(*ent);
            let w = random();
            ws.push(w);
            polinom = polinom + (out * w)
        }
        //each ent is sismoigded
        //each sismoig output is balanced with a random w weight
        //at the end we have just a polinom as the output
        //of that node
        //return polinom;
        (polinom, ws.clone())
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

    output:Cell<f32>,
    end_newron:Cell<Node>,
    //entry:Vec<f32>,

    ws:Vec<Cell<f32>>,
    weights:Vec<Vec<Cell<f32>>>,
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
            output:Cell::new(0.0),

            ws:Vec::new(),
            weights:Vec::new(),

            end_newron:Cell::new(Node { id:0, level:0, output:0.0 }),
        }
    }

    fn newforward(&mut self, entry:Vec<f32>){
        let mut nentry:Vec<f32> = entry;
        let mut output:f32 = 0.0;
        let mut level_reg = 0;
        for level in self.levels.iter() {
            for node in level.iter(){
                if level_reg == 0 {
                    output = node.get().polinom(nentry.clone());
                    let sigmoided = sigmoid(output);
                    let mut newnode = node.get();
                    newnode.output = sigmoided;
                    node.set(newnode);
                } else {
                    let prevs = &self.levels[level_reg - 1];
                    let mut previous:Vec<f32> = Vec::new();
                    for prev in prevs.iter(){
                        previous.push(prev.get().output);
                    }
                    output = node.get().polinom(previous);
                    let sigmoided = sigmoid(output);
                    let mut newnode = node.get();
                    newnode.output = sigmoided;
                    node.set(newnode);
                }
            }
            level_reg = level_reg + 1;
        }
    }

    fn forward(&mut self, entry:Vec<f32>){
        let mut polinoms:Vec<f32> = Vec::new();
        let mut nentry:Vec<f32> = entry;
        for level in self.levels.iter() {
            for node in level.iter(){
                //kickstart returns a polinom(w,s) output for each node
                //I am going to save polinoms (output of a node)
                let (polinom, ws) = node.get().kickstart(nentry.clone());
                for each_ws in ws {
                    self.ws.push(Cell::new(each_ws));
                }
                let mut newnode = node.get();
                //println!("node:{:?} has entry:{:?} and output:{}", node.get().id, nentry, polinom);
                newnode.output = polinom;
                node.set(newnode);
                polinoms.push(polinom);
            }
            //freeing self.ws
            self.weights.push(self.ws.clone());
            self.ws = Vec::new();
            nentry = polinoms.clone();
            polinoms = Vec::new();
        }
    }

    fn final_output(&mut self) -> f32 {
        let mut fentry:Vec<f32> = Vec::new();
        for node in self.levels[self.levels.len()-1].iter() {
            fentry.push(node.get().output);
        }

        let fnode = Node::new(100);
        let cell_fnode = Cell::new(fnode.clone());
        self.end_newron.set(fnode.clone());
        let (final_result, weight) = cell_fnode.get().kickstart(fentry.clone());
        self.output.set(final_result);
        println!("final weight:{}", weight[0]);
        return final_result;
    }
    

    fn show_weights(&self){
        for wlevel in self.weights.iter() {
            for w in wlevel.iter() {
                println!("w:{}", w.get());
            }
        }
    }

    fn show(&self) {
        for level in self.levels.iter(){
            for node in level.iter(){
                println!("{:?}", node.get());
            }
        }
    }

    fn backward(&self, new_result:f32){
        self.output.set(new_result);
        let mut old_newron = self.end_newron.get();
        old_newron.output = self.output.get();

        for level in self.levels.iter().rev() {
            for node in level.iter().rev() {
                println!("id:{}", node.get().id);
                println!("output: {}", node.get().output);
            };
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
