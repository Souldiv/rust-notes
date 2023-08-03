// Definition for a binary tree node.
use std::io;
use std::rc::Rc;
use std::cell::RefCell;
use core::slice::Iter;

// use num_traits::Signed;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: isize,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: isize) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }

  pub fn build_tree(iter: &mut Iter<Option<isize>>) ->  Option<Rc<RefCell<TreeNode>>>{
    match iter.next() {
        Some(&Some(Value)) => {
            let left = Self::build_tree(iter);
            let right = Self::build_tree(iter);
            Some(Rc::new(RefCell::new(TreeNode {
                val: Value,
                left: left.clone(),
                right: right.clone(),
            })))
        }
        _ => return None,
    }
  }

  pub fn in_order_traversal(root: &Option<Rc<RefCell<TreeNode>>>) {
    match root {
        Some(x) => {
            let node = x.borrow();
            Self::in_order_traversal(&node.left);
            println!("{}", node.val );
            Self::in_order_traversal(&node.right);
        },
        None => (),

    }
  }

  pub fn pre_order_traversal(root: &Option<Rc<RefCell<TreeNode>>>) {
    match root {
        Some(x) => {
            let node = x.borrow();
            println!("{}", node.val );
            Self::pre_order_traversal(&node.left);
            Self::pre_order_traversal(&node.right);
        },
        None => ()
    }
  }

  pub fn post_order_traversal(root: &Option<Rc<RefCell<TreeNode>>>) {
    match root{
        Some(x) => {
            let node = x.borrow();
            Self::post_order_traversal(&node.left);
            Self::post_order_traversal(&node.right);
            println!("{}", node.val );
        }
        ,None=>(),
    }
  }

  pub fn find_max_depth(root: &Option<Rc<RefCell<TreeNode>>>) -> isize{
    match root {
        Some(x) => { 
            let node = x.borrow();
            let left = Self::find_max_depth(&node.left);
            let right = Self::find_max_depth(&node.right);
            return std::cmp::max(left, right) + 1;
        },
        _=> 0
    }
  }
  pub fn count_visible_nodes(root: &Option<Rc<RefCell<TreeNode>>>, cur_max: isize) -> isize {
    match root {
        Some(x) => {
            let node = x.borrow();
            
            let the_max = std::cmp::max(node.val, cur_max);


            let left = Self::count_visible_nodes(&node.left, the_max);
            let right = Self::count_visible_nodes(&node.right, the_max);

            if node.val == the_max {
                left + right + 1
            }
            else {
                left + right
            }
            
            
        },
        None => 0,
    }
  }

  pub fn check_if_balanced(root: &Option<Rc<RefCell<TreeNode>>>, current_height: isize) -> isize {
    match root {
        Some(x) => {
            let node = x.borrow();
            let left = Self::check_if_balanced(&node.left, current_height + 1);
            let right = Self::check_if_balanced(&node.right, current_height + 1);

            if left == -1 || right == -1 {
                return -1
            }
            else if (left - right).abs() > 1 {
                return -1
            }
            else {
                std::cmp::max(std::cmp::max(left, right), current_height)
            }

        },
        None => current_height,
        
    }

  }

  pub fn deserialize_tree(root: &Option<Rc<RefCell<TreeNode>>>, list: &RefCell<Vec<String>>){
    match root {
        Some(x) => {
            let node = x.borrow();
            
            list.borrow_mut().push(node.val.to_string());

            Self::deserialize_tree(&node.left, list);
            Self::deserialize_tree(&node.right, list);
        },
        None => {
            list.borrow_mut().push(String::from("x"));
        }
    }
  }

  pub fn invert_tree(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>{
    match root{
        Some(x) => {
            let node = x.borrow_mut();

            let left = Self::invert_tree(&node.left);
            let right = Self::invert_tree(&node.right);

            Some(Rc::new(RefCell::new(TreeNode {
                val: node.val,
                left: right,
                right: left,
            })))

        },
        None => {
            None
        }
    }
  }

  pub fn lowest_common_ancestor(root: &Option<Rc<RefCell<TreeNode>>>, p: isize, q: isize) -> Option<isize> {
    match root {
        Some(x) => {
            let node = x.borrow();
            if node.val == p || node.val == q {
                return Some(node.val);
            }

            let left = Self::lowest_common_ancestor(&node.left, p, q);
            let right = Self::lowest_common_ancestor(&node.right, p, q);

            let lr1: isize = left.unwrap_or(-1);
            let rr1: isize = right.unwrap_or(-1);

            if lr1 != -1 && rr1 != -1{
                return Some(node.val);
            }
            else if lr1 != -1 {
                return Some(lr1);
            }
            else if rr1 != -1 {
                return Some(rr1);
            }
            else{
                return None;
            }
        },
        None => None
    }
  }
}

pub fn btree(){
    let mut input = String::new();
    // Take input and store in Vec<Option<i32>>
    io::stdin().read_line(&mut input).expect("Failed");

    let trimmed_input = input.trim();

    let lof: Vec<Option<isize>> = trimmed_input
                    .split(' ')
                    .map(|item| {
                        if item.trim() == "x"{
                            None
                        } else {
                            Some(item.trim().parse::<isize>().expect("Invalid Integer"))
                        }
                    })
                    .collect();
    
    println!("list of values in btree list: {:?}", lof);

    let mut iter = lof.iter();
    
    let root = TreeNode::build_tree(&mut iter);

    println!("\n In Order Traversal");
    TreeNode::in_order_traversal(&root);

    println!("\n Pre Order Traversal");
    TreeNode::pre_order_traversal(&root);

    println!("\n Post Order Traversal");
    TreeNode::post_order_traversal(&root);
    
    if !root.is_none(){
        println!("\n Max Depth {}", TreeNode::find_max_depth(&root) - 1);
    }
    else {
        println!("\n Max Depth {}", 0);
    }

    let cur_max = -99999999999;

    println!("\n Counting Visible Nodes {}", TreeNode::count_visible_nodes(&root, cur_max));

    println!("\n Check if balanced {}", TreeNode::check_if_balanced(&root, 0));
    println!("\n Printing Deserialized Tree ");

    let list: RefCell<Vec<String>> = RefCell::new(vec![]);

    TreeNode::deserialize_tree(&root, &list);

    let combined_string = list
        .borrow()
        .iter()
        .cloned()
        .collect::<Vec<String>>()
        .join(" ");

    println!(" {}", combined_string);
    println!("Inverting Tree");
    let inverted_tree = TreeNode::invert_tree(&root);
    println!("Pre Order Traversal");
    TreeNode::pre_order_traversal(&inverted_tree);

    println!("Lowest Common Ancestor");
    let result = TreeNode::lowest_common_ancestor(&root, 3, 5);

    println!("Result is {}", result.unwrap_or(-1));

}

