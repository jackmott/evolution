use crate::parser::*;
use rand::prelude::*;
use std::sync::mpsc::*;
use variant_count::*;

#[derive(VariantCount, Clone)]
pub enum APTNode {
    Add(Vec<APTNode>),
    Sub(Vec<APTNode>),
    Mul(Vec<APTNode>),
    Div(Vec<APTNode>),
    FBM(Vec<APTNode>),
    Ridge(Vec<APTNode>),
    Turbulence(Vec<APTNode>),
    Cell1(Vec<APTNode>),
    Cell2(Vec<APTNode>),
    Sqrt(Vec<APTNode>),
    Sin(Vec<APTNode>),
    Atan(Vec<APTNode>),
    Atan2(Vec<APTNode>),
    Tan(Vec<APTNode>),
    Log(Vec<APTNode>),
    Abs(Vec<APTNode>),
    Constant(f32),
    X,
    Y,
    T,
    Empty,
}

impl APTNode {
    pub fn to_lisp(&self) -> String {
        match self {
            APTNode::Add(children) => {
                format!("( + {} {} )", children[0].to_lisp(), children[1].to_lisp())
            }
            APTNode::Sub(children) => {
                format!("( - {} {} )", children[0].to_lisp(), children[1].to_lisp())
            }
            APTNode::Mul(children) => {
                format!("( * {} {} )", children[0].to_lisp(), children[1].to_lisp())
            }
            APTNode::Div(children) => {
                format!("( / {} {} )", children[0].to_lisp(), children[1].to_lisp())
            }
            APTNode::FBM(children) => format!(
                "( FBM {} {} {} )",
                children[0].to_lisp(),
                children[1].to_lisp(),
                children[2].to_lisp()
            ),
            APTNode::Ridge(children) => format!(
                "( Ridge {} {} {} )",
                children[0].to_lisp(),
                children[1].to_lisp(),
                children[2].to_lisp()
            ),
            APTNode::Cell1(children) => format!(
                "( Cell1 {} {} {} )",
                children[0].to_lisp(),
                children[1].to_lisp(),
                children[2].to_lisp()
            ),
            APTNode::Cell2(children) => format!(
                "( Cell2 {} {} {} )",
                children[0].to_lisp(),
                children[1].to_lisp(),
                children[2].to_lisp()
            ),
            APTNode::Turbulence(children) => format!(
                "( Turbulence {} {} {} )",
                children[0].to_lisp(),
                children[1].to_lisp(),
                children[2].to_lisp()
            ),
            APTNode::Sqrt(children) => format!("( Sqrt {} )", children[0].to_lisp()),
            APTNode::Sin(children) => format!("( Sin {} )", children[0].to_lisp()),
            APTNode::Atan(children) => format!("( Atan {} )", children[0].to_lisp()),
            APTNode::Atan2(children) => format!(
                "( Atan2 {} {} )",
                children[0].to_lisp(),
                children[1].to_lisp()
            ),
            APTNode::Tan(children) => format!("( Tan {} )", children[0].to_lisp()),
            APTNode::Log(children) => format!("( Log {} )", children[0].to_lisp()),
            APTNode::Abs(children) => format!("( Abs {} )", children[0].to_lisp()),
            APTNode::Constant(v) => format!("{}", v),
            APTNode::X => format!("X"),
            APTNode::Y => format!("Y"),
            APTNode::T => format!("T"),
            APTNode::Empty => format!("EMPTY"),
        }
    }

    pub fn str_to_node(s: &str) -> APTNode {
        match &s.to_lowercase()[..] {
            "+" => APTNode::Add(vec![APTNode::Empty, APTNode::Empty]),
            "-" => APTNode::Sub(vec![APTNode::Empty, APTNode::Empty]),
            "*" => APTNode::Mul(vec![APTNode::Empty, APTNode::Empty]),
            "/" => APTNode::Div(vec![APTNode::Empty, APTNode::Empty]),
            "fbm" => APTNode::FBM(vec![APTNode::Empty, APTNode::Empty, APTNode::Empty]),
            "ridge" => APTNode::Ridge(vec![APTNode::Empty, APTNode::Empty, APTNode::Empty]),
            "turbulence" => {
                APTNode::Turbulence(vec![APTNode::Empty, APTNode::Empty, APTNode::Empty])
            }
            "cell1" => APTNode::Cell1(vec![APTNode::Empty, APTNode::Empty, APTNode::Empty]),
            "cell2" => APTNode::Cell2(vec![APTNode::Empty, APTNode::Empty, APTNode::Empty]),
            "sqrt" => APTNode::Sqrt(vec![APTNode::Empty]),
            "sin" => APTNode::Sin(vec![APTNode::Empty]),
            "atan" => APTNode::Atan(vec![APTNode::Empty]),
            "atan2" => APTNode::Atan2(vec![APTNode::Empty, APTNode::Empty]),
            "tan" => APTNode::Tan(vec![APTNode::Empty]),
            "log" => APTNode::Log(vec![APTNode::Empty]),
            "abs" => APTNode::Abs(vec![APTNode::Empty]),
            "x" => APTNode::X,
            "y" => APTNode::Y,
            "t" => APTNode::T,
            _ => panic!("malformed str in str_to_node:{}", s),
        }
    }

    pub fn get_random_node(rng: &mut StdRng) -> APTNode {
        let r = rng.gen_range(0, APTNode::VARIANT_COUNT - 5);
        match r {
            0 => APTNode::Add(vec![APTNode::Empty, APTNode::Empty]),
            1 => APTNode::Sub(vec![APTNode::Empty, APTNode::Empty]),
            2 => APTNode::Mul(vec![APTNode::Empty, APTNode::Empty]),
            3 => APTNode::Div(vec![APTNode::Empty, APTNode::Empty]),
            4 => APTNode::FBM(vec![APTNode::Empty, APTNode::Empty, APTNode::Empty]),
            5 => APTNode::Ridge(vec![APTNode::Empty, APTNode::Empty, APTNode::Empty]),
            6 => APTNode::Turbulence(vec![APTNode::Empty, APTNode::Empty, APTNode::Empty]),
            7 => APTNode::Cell1(vec![APTNode::Empty, APTNode::Empty, APTNode::Empty]),
            8 => APTNode::Cell2(vec![APTNode::Empty, APTNode::Empty, APTNode::Empty]),
            9 => APTNode::Sqrt(vec![APTNode::Empty]),
            10 => APTNode::Sin(vec![APTNode::Empty]),
            11 => APTNode::Atan(vec![APTNode::Empty]),
            12 => APTNode::Atan2(vec![APTNode::Empty, APTNode::Empty]),
            13 => APTNode::Tan(vec![APTNode::Empty]),
            14 => APTNode::Log(vec![APTNode::Empty]),
            15 => APTNode::Abs(vec![APTNode::Empty]),
            _ => panic!("get_random_node generated unhandled r:{}", r),
        }
    }

    pub fn get_random_leaf(rng: &mut StdRng) -> APTNode {
        let r = rng.gen_range(0, 3);
        match r {
            0 => APTNode::X,
            1 => APTNode::Y,
            2 => APTNode::Constant(rng.gen_range(-1.0, 1.0)),
            _ => panic!("get_random_leaf generated unhandled r:{}", r),
        }
    }

    pub fn get_random_leaf_video(rng: &mut StdRng) -> APTNode {
        let r = rng.gen_range(0, 4);
        match r {
            0 => APTNode::X,
            1 => APTNode::Y,
            2 => APTNode::T,
            3 => APTNode::Constant(rng.gen_range(-1.0, 1.0)),
            _ => panic!("get_random_leaf generated unhandled r:{}", r),
        }
    }

    pub fn add_random(&mut self, node: APTNode, rng: &mut StdRng) {
        let children = match self.get_children_mut() {
            Some(children) => children,
            None => panic!("tried to add_random to a leaf"),
        };
        let add_index = rng.gen_range(0, children.len());
        match children[add_index] {
            APTNode::Empty => children[add_index] = node,
            _ => children[add_index].add_random(node, rng),
        }
    }

    pub fn add_leaf(&mut self, leaf: &APTNode) -> bool {
        match self.get_children_mut() {
            None => false,
            Some(children) => {
                for i in 0..children.len() {
                    match children[i] {
                        APTNode::Empty => {
                            children[i] = leaf.clone();
                            return true;
                        }
                        _ => {
                            if !children[i].is_leaf() && children[i].add_leaf(leaf) {
                                return true;
                            }
                        }
                    }
                }
                return false;
            }
        }
    }

    pub fn constant_eval(&self) -> f32 {
        unsafe {
            match self {
                APTNode::Add(children) => children[0].constant_eval() + children[1].constant_eval(),
                APTNode::Sub(children) => children[0].constant_eval() - children[1].constant_eval(),
                APTNode::Mul(children) => children[0].constant_eval() * children[1].constant_eval(),
                APTNode::Div(children) => children[0].constant_eval() / children[1].constant_eval(),                
                APTNode::Constant(v) => *v,
                _ => panic!("invalid node passed to constant_eval")               
            }
        }
    }

    fn set_children(&self, children:Vec<APTNode>) -> Self {
        match self {
            APTNode::Add(_) => APTNode::Add(children),
            APTNode::Sub(_) => APTNode::Sub(children),
            APTNode::Mul(_) => APTNode::Mul(children),
            APTNode::Div(_) => APTNode::Div(children),
            APTNode::FBM(_) => APTNode::FBM(children),
            APTNode::Ridge(_) => APTNode::Ridge(children),
            APTNode::Turbulence(_) => APTNode::Turbulence(children),
            APTNode::Sqrt(_) => APTNode::Sqrt(children),
            APTNode::Sin(_) => APTNode::Sin(children),
            APTNode::Atan(_) => APTNode::Atan(children),
            APTNode::Atan2(_) => APTNode::Atan(children),
            APTNode::Tan(_) => APTNode::Tan(children),
            APTNode::Log(_) => APTNode::Log(children),
            APTNode::Constant(v) => APTNode::Constant(*v),
            APTNode::X => APTNode::X,
            APTNode::Y => APTNode::Y,
            APTNode::T => APTNode::T,
            APTNode::Empty => panic!("tried to eval an empty node"),
            _ => panic!("add to set children")
        }
    }

    pub fn constant_fold(&self) -> APTNode {
        match self {
            APTNode::Constant(v) => APTNode::Constant(*v),
            APTNode::X => APTNode::X,
            APTNode::Y => APTNode::Y,
            APTNode::T => APTNode::T,
            _ => {
                let children = self.get_children().unwrap();
                //foreach child -> constant_fold(child), if you get back all constants -> compute the new constant, and create it                
                let folded_children : Vec<APTNode> = children.iter().map(|child| child.constant_fold()).collect();                                            
                if folded_children.iter().all(|child| match child { APTNode::Constant(_) => true, _ => false }) {  
                    let clone = self.set_children(folded_children);                          
                    APTNode::Constant(clone.constant_eval())
                } else {
                    let clone = self.set_children(folded_children);        
                    clone
                }
            }
        }
    }

    pub fn generate_tree(count: usize, video: bool, rng: &mut StdRng) -> APTNode {
        let leaf_func = if video {
            APTNode::get_random_leaf_video
        } else {
            APTNode::get_random_leaf
        };
        let mut first = APTNode::get_random_node(rng);
        for _ in 1..count {
            first.add_random(APTNode::get_random_node(rng), rng);
        }
        while first.add_leaf(&leaf_func(rng)) {}
        first
    }

    pub fn get_children_mut(&mut self) -> Option<&mut Vec<APTNode>> {
        match self {
            APTNode::Add(children) => Some(children),
            APTNode::Sub(children) => Some(children),
            APTNode::Mul(children) => Some(children),
            APTNode::Div(children) => Some(children),
            APTNode::FBM(children) => Some(children),
            APTNode::Ridge(children) => Some(children),
            APTNode::Turbulence(children) => Some(children),
            APTNode::Cell1(children) => Some(children),
            APTNode::Cell2(children) => Some(children),
            APTNode::Sqrt(children) => Some(children),
            APTNode::Sin(children) => Some(children),
            APTNode::Atan(children) => Some(children),
            APTNode::Atan2(children) => Some(children),
            APTNode::Tan(children) => Some(children),
            APTNode::Log(children) => Some(children),
            APTNode::Abs(children) => Some(children),
            _ => None,
        }
    }

    pub fn get_children(&self) -> Option<&Vec<APTNode>> {
        match self {
            APTNode::Add(children) => Some(children),
            APTNode::Sub(children) => Some(children),
            APTNode::Mul(children) => Some(children),
            APTNode::Div(children) => Some(children),
            APTNode::FBM(children) => Some(children),
            APTNode::Ridge(children) => Some(children),
            APTNode::Turbulence(children) => Some(children),
            APTNode::Cell1(children) => Some(children),
            APTNode::Cell2(children) => Some(children),
            APTNode::Sqrt(children) => Some(children),
            APTNode::Sin(children) => Some(children),
            APTNode::Atan(children) => Some(children),
            APTNode::Atan2(children) => Some(children),
            APTNode::Tan(children) => Some(children),
            APTNode::Log(children) => Some(children),
            APTNode::Abs(children) => Some(children),
            _ => None,
        }
    }

    pub fn is_leaf(&self) -> bool {
        match self {
            APTNode::X | APTNode::Y | APTNode::T | APTNode::Constant(_) | APTNode::Empty => true,
            _ => false,
        }
    }

    pub fn parse_apt_node(receiver: &Receiver<Token>) -> APTNode {
        loop {
            match receiver.recv() {
                Ok(token) => {
                    match token {
                        Token::Operation(s) => {
                            println!("returning a node");
                            let mut node = APTNode::str_to_node(s);
                            match node.get_children_mut() {
                                Some(children) => {
                                    for child in children {
                                        *child = APTNode::parse_apt_node(receiver);
                                    }
                                    return node;
                                }
                                None => return node,
                            }
                        }
                        Token::Constant(vstr) => {
                            let v = vstr.parse::<f32>().unwrap();
                            println!("returning a node");
                            return APTNode::Constant(v);
                        }
                        _ => (), //parens don't matter
                    }
                }
                Err(_) => panic!("malformed input"),
            }
        }
    }
}
