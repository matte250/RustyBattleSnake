use index_vec::IndexVec;

use crate::{GameState, BattleSnake};

index_vec::define_index_type! {
    pub struct NodeIdx = u32;
}

pub struct Graph {
    /// Row based table e.g to access a node with x and y coordinates as following (nodes\[y]\[x]).
    nodes: IndexVec<NodeIdx, IndexVec<NodeIdx, Node>>
}

impl Graph {
    pub fn new(game_state: &GameState) -> Self {
        let n_height = usize::try_from(game_state.board.height).unwrap();
        let n_width = usize::try_from(game_state.board.width).unwrap();
        let mut nodes: IndexVec<NodeIdx, IndexVec<NodeIdx, Node>> = 
            index_vec::index_vec![index_vec::index_vec![Node::new(0, 0, NodeType::Empty); n_width]; n_height];

        for coord in &game_state.board.food {
            let node = nodes.get_node_mut(coord.x, coord.y);
            *node = Node::new(coord.x, coord.y, NodeType::Food); 
        }

        for coord in &game_state.board.hazards {
            let node = nodes.get_node_mut(coord.x, coord.y);
            *node = Node::new(coord.x, coord.y, NodeType::Hazard);
        }

        for snake in &game_state.board.snakes {
            for coord in &snake.body {
                let node = nodes.get_node_mut(coord.x, coord.y);
                *node = Node::new(coord.x, coord.y, NodeType::SnakeBody(snake.clone())); 
            }

            let coord = &snake.head;
            let node = nodes.get_node_mut(coord.x, coord.y);
            *node = Node::new(coord.x, coord.y, NodeType::SnakeHead(snake.clone()));
        }
        
        return Self {
            nodes
        }
    }
}

trait NodeIndex {
    fn get_node(&self, x: u32, y: u32) -> &Node;

    fn get_node_mut(&mut self, x: u32, y: u32) -> &mut Node;
}

impl NodeIndex for IndexVec<NodeIdx, IndexVec<NodeIdx, Node>> {
    fn get_node(&self, x: u32, y: u32) -> &Node {
        return self.get::<NodeIdx>(NodeIdx::from_raw(y)).unwrap().get::<NodeIdx>(NodeIdx::from_raw(x)).unwrap();
    }

    fn get_node_mut(&mut self, x: u32, y: u32) -> &mut Node {
        return self.get_mut::<NodeIdx>(NodeIdx::from_raw(y)).unwrap().get_mut::<NodeIdx>(NodeIdx::from_raw(x)).unwrap();

    }
}

#[derive(Clone)]
struct Node {
    x: u32,
    y: u32,
    node_type: NodeType
}

impl Node {
    fn new(x: u32, y: u32, node_type: NodeType) -> Self {
        return Self {
            x,
            y,
            node_type
        }
    }
}

#[derive(Clone)]
enum NodeType {
    Empty,
    Food,
    SnakeBody(std::rc::Rc<BattleSnake>),
    SnakeHead(std::rc::Rc<BattleSnake>),
    Hazard
}