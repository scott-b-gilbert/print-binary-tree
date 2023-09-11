#![allow(unused_assignments)]
use std::cell::RefCell;
use std::rc::Rc;

// static mut self.lprofile: Vec<i32> = Vec::new();
// static mut self.rprofile: Vec<i32> = Vec::new();
static mut GAP: i32 = 3;
static mut PRINT_NEXT: i32 = 0;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    pub fn from_vec(vals: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() {
            return None;
        }
        let mut root = Self::new(vals[0]);
        root.fill(vals, 0);

        Some(Rc::new(RefCell::new(root)))
    }

    fn fill(&mut self, vals: &[i32], index: usize) {
        let left_node = index * 2 + 1;
        if left_node < vals.len() && vals[left_node] != i32::MIN {
            self.left = Some(Rc::new(RefCell::new(Self::new(vals[left_node]))));
            self.left
                .as_ref()
                .unwrap()
                .borrow_mut()
                .fill(vals, left_node);
        }

        let right_node = left_node + 1;
        if right_node < vals.len() && vals[right_node] != i32::MIN {
            self.right = Some(Rc::new(RefCell::new(Self::new(vals[right_node]))));
            self.right
                .as_ref()
                .unwrap()
                .borrow_mut()
                .fill(vals, right_node);
        }
    }

    pub fn into_array(&self) -> Vec<i32> {
        let mut result = Vec::new();
        self.walk(&mut result);
        result
    }

    fn walk(&self, result: &mut Vec<i32>) {
        if let Some(left) = &self.left {
            left.borrow().walk(result);
        }

        result.push(self.val);

        if let Some(right) = &self.right {
            right.borrow().walk(result);
        }
    }
}

#[derive(PartialEq)]
pub enum Parent {
    LeftParent = -1,
    RightParent = 1,
    Root = 0,
}

pub struct AsciiNode {
    pub left: Option<Rc<RefCell<AsciiNode>>>,
    pub right: Option<Rc<RefCell<AsciiNode>>>,
    pub edge_length: i32,
    pub height: i32,
    pub lablen: i32,
    pub parent_dir: Parent,
    pub label: String,
    pub lprofile: Vec<i32>,
    pub rprofile: Vec<i32>,
}

impl Default for AsciiNode {
    fn default() -> Self {
        Self::new()
    }
}

impl AsciiNode {
    const MAX_HEIGHT: i32 = 1000;

    pub fn new() -> Self {
        Self {
            left: None,
            right: None,
            edge_length: 0,
            height: 0,
            lablen: 0,
            parent_dir: Parent::Root,
            label: "".to_string(),
            rprofile: Vec::new(),
            lprofile: Vec::new(),
        }
    }

    fn print_ascii_tree(&mut self, t: Option<Rc<RefCell<TreeNode>>>) {
        if t.is_none() {
            return;
        }
        let mut xmin = 0;
        let proot = AsciiNode::build_ascii_tree_recursive(&t, Parent::Root);
        self.compute_edge_lengths(&proot);
        for _i in 0..std::cmp::min(
            proot.as_ref().unwrap().borrow().height,
            AsciiNode::MAX_HEIGHT,
        ) {
            self.lprofile.push(i32::MAX);
        }
        self.compute_lprofile(&proot, 0, 0);
        xmin = 0;
        for i in 0..std::cmp::min(
            proot.as_ref().unwrap().borrow().height,
            AsciiNode::MAX_HEIGHT,
        ) {
            xmin = std::cmp::min(xmin, self.lprofile[i as usize]);
        }
        for i in 0..proot.as_ref().unwrap().borrow().height {
            unsafe {
                PRINT_NEXT = 0;
            }
            AsciiNode::print_level(&proot, -xmin, i);
            println!();
        }
        if proot.as_ref().unwrap().borrow().height >= AsciiNode::MAX_HEIGHT {
            println!(
                "This tree is taller than {}, and may be drawn incorrectly.",
                AsciiNode::MAX_HEIGHT
            )
        }
    }

    fn compute_edge_lengths(&mut self, a: &Option<Rc<RefCell<AsciiNode>>>) {
        let mut h: i32 = 0;
        let mut hmin: i32 = 0;
        let mut delta: i32 = 0;

        if a.is_none() {
            return;
        }
        if let Some(node) = a {
            self.compute_edge_lengths(&node.borrow().left);
            self.compute_edge_lengths(&node.borrow().right);

            if node.borrow().left.is_none() && node.borrow().right.is_none() {
                node.borrow_mut().edge_length = 0;
            } else {
                if node.borrow().left.is_some() {
                    let height = node.borrow().left.as_ref().unwrap().borrow().height;
                    for _i in 0..std::cmp::min(height, AsciiNode::MAX_HEIGHT) {
                        self.rprofile.push(i32::MIN);
                    }
                    self.compute_rprofile(&a.as_ref().unwrap().borrow().left, 0, 0);
                    hmin = node.borrow().left.as_ref().unwrap().borrow().height;
                } else {
                    hmin = 0;
                }
                if node.borrow().right.is_some() {
                    let height = node.borrow().right.as_ref().unwrap().borrow().height;
                    for _i in 0..std::cmp::min(height, AsciiNode::MAX_HEIGHT) {
                        self.lprofile.push(i32::MAX);
                    }
                    self.compute_lprofile(&node.borrow().right, 0, 0);
                    let right_height = node.borrow().right.as_ref().unwrap().borrow().height;
                    hmin = std::cmp::min(right_height, hmin)
                } else {
                    hmin = 0;
                }
                delta = 4;
                for i in 0..hmin {
                    unsafe {
                        delta = std::cmp::max(
                            delta,
                            GAP.saturating_add(self.rprofile[i as usize])
                                .saturating_sub(self.lprofile[i as usize]),
                        );
                    }
                }

                if (node.borrow().left.is_some()
                    && node.borrow().left.as_ref().unwrap().borrow().height == 1)
                    || (node.borrow().right.is_some()
                        && node.borrow().right.as_ref().unwrap().borrow().height == 1)
                {
                    delta -= 1;
                }
                node.borrow_mut().edge_length = ((delta + 1) / 2) - 1;
            }

            // now fill in the height of the node
            h = 1;
            if node.borrow().left.is_some() {
                h = std::cmp::max(
                    node.borrow().left.as_ref().unwrap().borrow().height
                        + node.borrow().edge_length
                        + 1,
                    h,
                );
            }
            if node.borrow().right.is_some() {
                h = std::cmp::max(
                    node.borrow().right.as_ref().unwrap().borrow().height
                        + node.borrow().edge_length
                        + 1,
                    h,
                );
            }
            node.borrow_mut().height = h;
        }
    }

    fn print_level(a: &Option<Rc<RefCell<AsciiNode>>>, x: i32, level: i32) {
        if a.is_none() {
            return;
        }
        let is_left = a.as_ref().unwrap().borrow().parent_dir == Parent::LeftParent;
        unsafe {
            if level == 0 {
                let mut final_i = 0;
                let print_space =
                    x - PRINT_NEXT - ((a.as_ref().unwrap().borrow().lablen - is_left as i32) / 2);
                for _i in 0..print_space {
                    print!(" ");
                    final_i += 1
                }
                PRINT_NEXT += final_i;
                print!("{}", a.as_ref().unwrap().borrow().label);
                PRINT_NEXT += a.as_ref().unwrap().borrow().lablen;
            } else if a.as_ref().unwrap().borrow().edge_length >= level {
                if a.as_ref().unwrap().borrow().left.is_some() {
                    let print_space = x - PRINT_NEXT - level;
                    let mut final_i = 0;
                    for _i in 0..print_space {
                        print!(" ");
                        final_i += 1;
                    }
                    PRINT_NEXT += final_i;
                    print!("/");
                    PRINT_NEXT += 1;
                }
                if a.as_ref().unwrap().borrow().right.is_some() {
                    let print_space = x - PRINT_NEXT + level;
                    let mut final_i = 0;
                    for _i in 0..print_space {
                        print!(" ");
                        final_i += 1;
                    }
                    PRINT_NEXT += final_i;
                    print!("\\");
                    PRINT_NEXT += 1;
                }
            } else {
                AsciiNode::print_level(
                    &a.as_ref().unwrap().borrow().left,
                    x - a.as_ref().unwrap().borrow().edge_length - 1,
                    level - a.as_ref().unwrap().borrow().edge_length - 1,
                );
                AsciiNode::print_level(
                    &a.as_ref().unwrap().borrow().right,
                    x + a.as_ref().unwrap().borrow().edge_length + 1,
                    level - a.as_ref().unwrap().borrow().edge_length - 1,
                );
            }
        }
    }

    fn compute_rprofile(&mut self, a: &Option<Rc<RefCell<AsciiNode>>>, x: i32, y: i32) {
        let y: usize = y as usize;
        if a.is_none() {
            return;
        }
        let not_left = (a.as_ref().unwrap().borrow().parent_dir != Parent::LeftParent) as i32;
        self.rprofile[y] = std::cmp::max(
            self.rprofile[y],
            x + ((a.as_ref().unwrap().borrow().lablen - not_left) / 2),
        );
        if a.as_ref().unwrap().borrow().right.is_some() {
            for i in 1..std::cmp::min(
                a.as_ref().unwrap().borrow().edge_length,
                AsciiNode::MAX_HEIGHT - y as i32,
            ) {
                self.rprofile[y + i as usize - 1] =
                    std::cmp::max(self.rprofile[y + i as usize - 1], x + i);
            }
        }
        self.compute_rprofile(
            &a.as_ref().unwrap().borrow().left,
            x - a.as_ref().unwrap().borrow().edge_length - 1,
            y as i32 + a.as_ref().unwrap().borrow().edge_length + 1,
        );
        self.compute_rprofile(
            &a.as_ref().unwrap().borrow().right,
            x + a.as_ref().unwrap().borrow().edge_length + 1,
            y as i32 + a.as_ref().unwrap().borrow().edge_length + 1,
        );
    }

    fn compute_lprofile(&mut self, a: &Option<Rc<RefCell<AsciiNode>>>, x: i32, y: i32) {
        let y: usize = y as usize;
        if a.is_none() {
            return;
        }
        let is_left = (a.as_ref().unwrap().borrow().parent_dir == Parent::LeftParent) as i32;
        self.lprofile[y] = std::cmp::min(
            self.lprofile[y],
            x - ((a.as_ref().unwrap().borrow().lablen - is_left) / 2),
        );
        if a.as_ref().unwrap().borrow().left.is_some() {
            for i in 1..std::cmp::min(
                a.as_ref().unwrap().borrow().edge_length,
                AsciiNode::MAX_HEIGHT - y as i32,
            ) {
                self.lprofile[y + i as usize] = std::cmp::min(self.lprofile[y + i as usize], x - i);
            }
        }
        self.compute_lprofile(
            &a.as_ref().unwrap().borrow().left,
            x - a.as_ref().unwrap().borrow().edge_length - 1,
            y as i32 + a.as_ref().unwrap().borrow().edge_length + 1,
        );
        self.compute_lprofile(
            &a.as_ref().unwrap().borrow().right,
            x + a.as_ref().unwrap().borrow().edge_length + 1,
            y as i32 + a.as_ref().unwrap().borrow().edge_length + 1,
        );
    }

    fn build_ascii_tree_recursive(
        t: &Option<Rc<RefCell<TreeNode>>>,
        p: Parent,
    ) -> Option<Rc<RefCell<AsciiNode>>> {
        if t.is_none() {
            return None;
        }
        let mut node = AsciiNode::new();
        node.left = AsciiNode::build_ascii_tree_recursive(
            &t.as_ref().unwrap().borrow().left,
            Parent::LeftParent,
        );
        node.right = AsciiNode::build_ascii_tree_recursive(
            &t.as_ref().unwrap().borrow().right,
            Parent::RightParent,
        );

        node.label = t.as_ref().unwrap().borrow().val.to_string();
        node.lablen = t.as_ref().unwrap().borrow().val.to_string().len() as i32;
        node.parent_dir = p;
        Some(Rc::new(RefCell::new(node)))
    }
}
fn main() {
    let tree = TreeNode::from_vec(&[10]);
    let mut ascii = AsciiNode::new();
    ascii.print_ascii_tree(tree);
    println!();
    println!();
    println!();

    let tree = TreeNode::from_vec(&[10, 5, i32::MIN]);
    let mut ascii = AsciiNode::new();
    ascii.print_ascii_tree(tree);
    println!();
    println!();
    println!();

    let tree = TreeNode::from_vec(&[10, 5, 15]);
    let mut ascii = AsciiNode::new();
    ascii.print_ascii_tree(tree);
    println!();
    println!();
    println!();

    let tree = TreeNode::from_vec(&[10, 5, 15, i32::MIN, 9, 13, i32::MIN]);
    let mut ascii = AsciiNode::new();
    ascii.print_ascii_tree(tree);
    println!();
    println!();
    println!();

    let tree = TreeNode::from_vec(&[
        10,
        5,
        15,
        2,
        9,
        13,
        i32::MIN,
        i32::MIN,
        i32::MIN,
        6,
        i32::MIN,
        12,
        14,
    ]);
    let mut ascii = AsciiNode::new();
    ascii.print_ascii_tree(tree);
    println!();
    println!();
    println!();

    let tree = TreeNode::from_vec(&[
        10,
        5,
        15,
        2,
        9,
        13,
        i32::MIN,
        i32::MIN,
        i32::MIN,
        6,
        i32::MIN,
        12,
        i32::MIN,
    ]);
    let mut ascii = AsciiNode::new();
    ascii.print_ascii_tree(tree);
    println!();
    println!();
    println!();

    let tree = TreeNode::from_vec(&[10, 5, 15, 2, 9, 12, i32::MIN, i32::MIN, i32::MIN, 6]);
    let mut ascii = AsciiNode::new();
    ascii.print_ascii_tree(tree);

    println!();
    println!();
    println!();

    let tree = TreeNode::from_vec(&[10, 6, 15, 2, 9, 12]);
    let mut ascii = AsciiNode::new();
    ascii.print_ascii_tree(tree);

    println!();
    println!();
    println!();

    let tree = TreeNode::from_vec(&[12, 6, 15, 2, 9]);
    let mut ascii = AsciiNode::new();
    ascii.print_ascii_tree(tree);
}
