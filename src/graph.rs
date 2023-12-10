use index_vec::IndexVec;
use serde_json::value::Index;

use crate::{GameState, BattleSnake};

index_vec::define_index_type! {
    // Define StrIdx to use only 32 bits internally (you can use usize, u16,
    // and even u8).
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
            index_vec::index_vec![index_vec::index_vec![Node::Empty; n_width]; n_height];

        for coord in &game_state.board.food {
            let node = nodes.get_node_mut(coord.x, coord.y);
            *node = Node::Food; 
        }

        for snakes in &game_state.board.snakes {
            for coord in &snakes.body {
                let node = nodes.get_node_mut(coord.x, coord.y);
                let rc_snakes_id: std::rc::Rc<String> = std::rc::Rc::new(snakes.id.clone());
                *node = Node::Snake(rc_snakes_id); 
            }
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
enum Node {
    Empty,
    Food,
    Snake(std::rc::Rc<String>),
}