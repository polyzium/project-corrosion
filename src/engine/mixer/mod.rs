use super::plugins::interface::TimedEvent;

type Connection = (usize, usize, usize, usize);

pub trait Module {
    fn inputs(&self) -> &'static mut [&'static mut [f32]];
    fn outputs(&self) -> &'static [&'static [f32]];

    fn process(&mut self, events: &[TimedEvent]);
}

struct Mixer {
    modules: Vec<Box<dyn Module>>,
    connections: Vec<Connection>,

    // toposort_cache: Vec<usize>
}

impl Mixer {
    fn new() -> Mixer {
        Mixer {
            modules: Vec::new(),
            connections: Vec::new(),

            // toposort_cache: Vec::with_capacity(64),
        }
    }

    fn add_node(&mut self, module: Box<dyn Module>) -> usize {
        let index = self.modules.len();
        self.modules.push(module);
        index
    }

    fn connect(&mut self, src_module: usize, src_chan: usize, dst_module: usize, dst_chan: usize) {
        self.connections.push((src_module, src_chan, dst_module, dst_chan));
    }

    fn process(&mut self, events: &[TimedEvent]) {
        let order = topological_sort(&self.connections, self.modules.len());
        for node_index in order {
            for &(src_module, src_chan, dst_module, dst_chan) in &self.connections {
                if dst_module == node_index && dst_chan < self.modules[node_index].inputs().len() {
                    for index in 0..self.modules[node_index].inputs()[dst_chan].len() {
                        self.modules[node_index].inputs()[dst_chan][index] += self.modules[src_module].outputs()[src_chan][index];
                    }
                }
            }
            self.modules[node_index].process(events);
        }
    }
}

fn topological_sort(connections: &[Connection], num_nodes: usize) -> Vec<usize> {
    let mut in_degrees = vec![0; num_nodes];
    let mut neighbors = vec![Vec::new(); num_nodes];
    for &(src_node, _, dst_node, _) in connections {
        neighbors[src_node].push(dst_node);
    }

    let mut queue = Vec::new();
    for node_index in 0..num_nodes {
        if in_degrees[node_index] == 0 {
            queue.push(node_index);
        }
    }

    let mut order = Vec::new();
    while !queue.is_empty() {
        let node_index = queue.pop().unwrap();
        order.push(node_index);
        for &neighbor in &neighbors[node_index] {
            in_degrees[neighbor] -= 1;
            if in_degrees[neighbor] == 0 {
                queue.push(neighbor);
            }
        }
    }

    order
}