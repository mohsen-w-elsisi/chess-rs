use std::ops::Neg;

use rayon::prelude::*;

use crate::{
    board::{Board, MoveApplicationError},
    r#move::Move,
    piece::Color,
    robot::evaluate::Evaluater,
};

#[derive(Clone)]
pub struct PositionTree {
    nodes: Vec<PositionNode>,
    root_index: usize,
}

#[derive(Clone)]
pub struct PositionNode {
    pub position: Board,
    pub move_from_parent: Option<Move>,
    pub color: Color,
    index: usize,
    parent_index: Option<usize>,
    child_indices: Vec<usize>,
    is_terminal: bool,
}

impl PositionNode {
    pub fn new(
        position: Board,
        move_from_parent: Option<Move>,
        color: Color,
        index: usize,
        parent_index: Option<usize>,
    ) -> Self {
        let is_terminal = position.is_checkmate(color) || position.is_stalemate(color);
        PositionNode {
            position,
            move_from_parent,
            color,
            index,
            parent_index,
            child_indices: Vec::new(),
            is_terminal,
        }
    }

    fn has_children(&self) -> bool {
        !self.child_indices.is_empty()
    }
}

impl PositionTree {
    pub fn new(root_position: Board, root_color: Color) -> Self {
        let root_node = PositionNode::new(root_position, None, root_color, 0, None);

        PositionTree {
            nodes: vec![root_node],
            root_index: 0,
        }
    }

    pub fn of_depth(starting_position: Board, color: Color, depth: i32) -> Self {
        let mut tree = PositionTree::new(starting_position, color);
        for _ in 0..depth {
            tree.add_submove_layer();
        }
        tree
    }

    pub fn root(&self) -> &PositionNode {
        &self.nodes[self.root_index]
    }

    pub fn parent_of(&self, node: &PositionNode) -> Option<&PositionNode> {
        if let Some(parent_index) = node.parent_index {
            Some(&self.nodes[parent_index])
        } else {
            None
        }
    }

    pub fn children_of(&self, node: &PositionNode) -> Vec<&PositionNode> {
        node.child_indices
            .iter()
            .map(|&index| &self.nodes[index])
            .collect()
    }

    pub fn depth(&self) -> i32 {
        let mut depth = 0;
        let mut current_node = self.root();
        while let Some(child) = self.children_of(current_node).first() {
            depth += 1;
            current_node = child;
        }
        depth
    }

    pub fn add_submove_layer(&mut self) {
        let new_node_info: Vec<(usize, Move, Board)> = self
            .nodes
            .par_iter()
            .filter(|node| !node.has_children())
            .filter(|node| !node.is_terminal)
            .flat_map(|node| {
                available_legal_moves_with_positions(&node.position, node.color.opposite())
                    .into_par_iter()
                    .map(|(mv, new_position)| (node.index, mv, new_position))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        for (parent_index, mv, new_position) in new_node_info {
            let parent = self.nodes[parent_index].clone();
            self.add_node(new_position, mv, parent.color.opposite(), &parent);
        }
    }

    fn add_node(
        &mut self,
        position: Board,
        move_from_parent: Move,
        color: Color,
        parent: &PositionNode,
    ) {
        let new_node_index = self.nodes.len();

        let new_node = PositionNode::new(
            position,
            Some(move_from_parent),
            color,
            new_node_index,
            Some(parent.index),
        );

        self.nodes.push(new_node);
        self.nodes[parent.index].child_indices.push(new_node_index);
    }

    pub fn evaluation_of(&self, node: &PositionNode, evaluator: &Evaluater) -> f64 {
        if !node.has_children() {
            evaluator.evaluate(&node.position, node.color)
        } else {
            self.children_of(node)
                .par_iter()
                .map(|child| self.evaluation_of(child, evaluator))
                .reduce(|| -1.0, |a, b| a.max(b))
                .neg()
        }
    }
}

fn available_legal_moves_with_positions(position: &Board, color: Color) -> Vec<(Move, Board)> {
    position
        .get_pieces()
        .into_iter()
        .filter(|piece_info| piece_info.1.color == color)
        .flat_map(|(square, piece)| piece.get_available_moves(&square, position))
        .filter_map(|m| new_position_if_legal_move(position, &m, color).map(|new_pos| (m, new_pos)))
        .collect::<Vec<_>>()
}

fn new_position_if_legal_move(position: &Board, m: &Move, color: Color) -> Option<Board> {
    let mut new_board = position.clone();
    match new_board.apply_move(&m, color) {
        Ok(_) => Some(new_board),
        Err(MoveApplicationError::KingInCheck) => None,
        Err(e) => unreachable!("Unexpected error while applying move: {:?}", e),
    }
}
