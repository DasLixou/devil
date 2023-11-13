use devil::define_graph;

define_graph!(
    /// My custom simple undirected graph using hashbrowns hashmaps
    pub graph SimpleGraph {

    }
);

fn main() {
    let _graph = SimpleGraph::new();
}
