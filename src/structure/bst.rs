use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type BstNodeLink = Rc<RefCell<BstNode>>;
pub type WeakBstNodeLink = Weak<RefCell<BstNode>>;

//this package implement BST wrapper
#[derive(Debug, Clone)]
pub struct BstNode {
    pub key: Option<i32>,
    pub parent: Option<WeakBstNodeLink>,
    pub left: Option<BstNodeLink>,
    pub right: Option<BstNodeLink>,
}

impl BstNode {
    //private interface
    fn new(key: i32) -> Self {
        BstNode {
            key: Some(key),
            left: None,
            right: None,
            parent: None,
        }
    }

    pub fn new_bst_nodelink(value: i32) -> BstNodeLink {
        let currentnode = BstNode::new(value);
        let currentlink = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    /**
     * Get a copy of node link
     */
    pub fn get_bst_nodelink_copy(&self) -> BstNodeLink {
        Rc::new(RefCell::new(self.clone()))
    }

    fn downgrade(node: &BstNodeLink) -> WeakBstNodeLink {
        Rc::<RefCell<BstNode>>::downgrade(node)
    }

    //private interface
    fn new_with_parent(parent: &BstNodeLink, value: i32) -> BstNodeLink {
        let mut currentnode = BstNode::new(value);
        //currentnode.add_parent(Rc::<RefCell<BstNode>>::downgrade(parent));
        currentnode.parent = Some(BstNode::downgrade(parent));
        let currentlink = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    //add new left child, set the parent to current_node_link
    pub fn add_left_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.left = Some(new_node);
    }

    //add new left child, set the parent to current_node_link
    pub fn add_right_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.right = Some(new_node);
    }

    //search the current tree which node fit the value
    pub fn tree_search(&self, value: &i32) -> Option<BstNodeLink> {
        if let Some(key) = self.key {
            if key == *value {
                return Some(self.get_bst_nodelink_copy());
            }
            if *value < key && self.left.is_some() {
                return self.left.as_ref().unwrap().borrow().tree_search(value);
            } else if self.right.is_some() {
                return self.right.as_ref().unwrap().borrow().tree_search(value);
            }
        }
        //default if current node is NIL
        None
    }

    /**seek minimum by recurs
     * in BST minimum always on the left
     */
    pub fn minimum(&self) -> BstNodeLink {
        if self.key.is_some() {
            if let Some(left_node) = &self.left {
                return left_node.borrow().minimum();
            }
        }
        self.get_bst_nodelink_copy()
    }

    pub fn maximum(&self) -> BstNodeLink {
        if self.key.is_some() {
            if let Some(right_node) = &self.right {
                return right_node.borrow().maximum();
            }
        }
        self.get_bst_nodelink_copy()
    }

    /**
     * Return the root of a node, return self if not exist
     */
    pub fn get_root(node: &BstNodeLink) -> BstNodeLink {
        let parent = BstNode::upgrade_weak_to_strong(node.borrow().parent.clone());
        if parent.is_none() {
            return node.clone();
        }
        return BstNode::get_root(&parent.unwrap());
    }

    /**
     * NOTE: Buggy from pull request
     * Find node successor according to the book
     * Should return None, if x_node is the highest key in the tree
     */
    pub fn tree_successor(x_node: &BstNodeLink) -> Option<BstNodeLink> {
        // directly check if the node has a right child, otherwise go to the next block
        if let Some(right_node) = &x_node.borrow().right {
            return Some(right_node.borrow().minimum());
        } 
        
        // empty right child case
        else { 
            let mut x_node = x_node;
            let mut y_node = BstNode::upgrade_weak_to_strong(x_node.borrow().parent.clone());
            let mut temp: BstNodeLink;

            while let Some(ref exist) = y_node {
                if let Some(ref left_child) = exist.borrow().left {
                    if BstNode::is_node_match(left_child, x_node) {
                        return Some(exist.clone());
                    }
                }

                temp = y_node.unwrap();
                x_node = &temp;
                y_node = BstNode::upgrade_weak_to_strong(temp.borrow().parent.clone());
            }

            None    
        }
    }

    /**
     * Alternate simpler version of tree_successor that made use of is_nil checking
     */
    #[allow(dead_code)]
    pub fn tree_successor_simpler(x_node: &BstNodeLink) -> Option<BstNodeLink>{
        //create a shadow of x_node so it can mutate
        let mut x_node = x_node;
        let right_node = &x_node.borrow().right.clone();
        if BstNode::is_nil(right_node)!=true{
            return Some(right_node.clone().unwrap().borrow().minimum());
        }

        let mut y_node = BstNode::upgrade_weak_to_strong(x_node.borrow().parent.clone());
        let y_node_right = &y_node.clone().unwrap().borrow().right.clone();
        let mut y_node2: Rc<RefCell<BstNode>>;
        while BstNode::is_nil(&y_node) && BstNode::is_node_match_option(Some(x_node.clone()), y_node_right.clone()) {
            y_node2 = y_node.clone().unwrap();
            x_node = &y_node2;
            let y_parent = y_node.clone().unwrap().borrow().parent.clone().unwrap();
            y_node = BstNode::upgrade_weak_to_strong(Some(y_parent));
        }

        //in case our sucessor traversal yield root, means self is the highest key
        if BstNode::is_node_match_option(y_node.clone(), Some(BstNode::get_root(&x_node))) {
            return None;
        }

        //default return self / x_node
        return Some(y_node.clone().unwrap())
    }

    /**
     * private function return true if node doesn't has parent nor children nor key
     */
    fn is_nil(node: &Option<BstNodeLink>) -> bool {
        match node {
            None => true,
            Some(x) => {
                if x.borrow().parent.is_none()
                    || x.borrow().left.is_none()
                    || x.borrow().right.is_none()
                {
                    return true;
                }
                return false;
            }
        }
    }

    //helper function to compare both nodelink
    fn is_node_match_option(node1: Option<BstNodeLink>, node2: Option<BstNodeLink>) -> bool {
        if node1.is_none() && node2.is_none() {
            return true;
        }
        if let Some(node1v) = node1 {
            return node2.is_some_and(|x: BstNodeLink| x.borrow().key == node1v.borrow().key);
        }
        return false;
    }

    fn is_node_match(anode: &BstNodeLink, bnode: &BstNodeLink) -> bool {
        if anode.borrow().key == bnode.borrow().key {
            return true;
        }
        return false;
    }

    /**
     * As the name implied, used to upgrade parent node to strong nodelink
     */
    fn upgrade_weak_to_strong(node: Option<WeakBstNodeLink>) -> Option<BstNodeLink> {
        match node {
            None => None,
            Some(x) => Some(x.upgrade().unwrap()),
        }
    }
    
    pub fn tree_insert(current: &BstNodeLink, value: i32) {
        if current.borrow().tree_search(&value).is_some() {
            println!("Value {} already exists in the tree. Skipping insertion.", value);
            return;
        }

        let current_key = current.borrow().key.unwrap();
        if value < current_key{
            let left_child = current.borrow().left.clone();
            if let Some(left) = left_child {
                BstNode::tree_insert(&left, value);
            } else {
                current.borrow_mut().add_left_child(current, value);
            }
        } else {
            let right_child = current.borrow().right.clone();
            if let Some(right) = right_child {
                BstNode::tree_insert(&right, value);
            } else {
                current.borrow_mut().add_right_child(current, value);
            }
        }
    }

    pub fn transplant(root: &BstNodeLink, u: &BstNodeLink, v: Option<BstNodeLink>) {
        let parent = BstNode::upgrade_weak_to_strong(u.borrow().parent.clone());
        if let Some(parent_node) = parent {
            let is_left_child = parent_node
                .borrow()
                .left
                .as_ref()
                .map(|left_child| BstNode::is_node_match(left_child, u))
                .unwrap_or(false);

            if is_left_child {
                parent_node.borrow_mut().left = v.clone();
            } else {
                parent_node.borrow_mut().right = v.clone();
            }
        } else {
            if let Some(v_node) = &v {
                let v_key = v_node.borrow().key;
                let v_left = v_node.borrow().left.clone(); // Simpan left ke variabel lokal
                let v_right = v_node.borrow().right.clone(); // Simpan right ke variabel lokal

                let mut root_mut = root.borrow_mut();
                root_mut.key = v_key;
                root_mut.left = v_left;
                root_mut.right = v_right;
            } else {
                let mut root_mut = root.borrow_mut();
                root_mut.key = None;
                root_mut.left = None;
                root_mut.right = None;
            }
        }

        if let Some(v_node) = v {
            v_node.borrow_mut().parent = u.borrow().parent.clone();
        }
    }

    pub fn tree_delete(root: &BstNodeLink, z: &BstNodeLink) {
        if z.borrow().left.is_none() && z.borrow().right.is_none() {
            BstNode::transplant(root, z, None);
        } else if z.borrow().left.is_none() {
            let right_child = z.borrow().right.clone();
            BstNode::transplant(root, z, right_child);
        } else if z.borrow().right.is_none() {
            let left_child = z.borrow().left.clone();
            BstNode::transplant(root, z, left_child);
        } else {
            let right_subtree = z.borrow().right.clone().unwrap();
            let successor = right_subtree.borrow().minimum();

            if !BstNode::is_node_match(&successor, z.borrow().right.as_ref().unwrap()) {
                let successor_right = successor.borrow().right.clone();
                BstNode::transplant(root, &successor, successor_right);
                
                successor.borrow_mut().right = z.borrow().right.clone();
                
                if let Some(right) = &successor.borrow().right {
                    right.borrow_mut().parent = Some(BstNode::downgrade(&successor));
                }
            }
            BstNode::transplant(root, z, Some(successor.clone()));
    
            successor.borrow_mut().left = z.borrow().left.clone();

            let left_child = successor.borrow().left.clone();
            if let Some(left) = left_child {
                left.borrow_mut().parent = Some(BstNode::downgrade(&successor));
            }
        }
        if let Some(next_node) = root.borrow().tree_search(&z.borrow().key.unwrap()) {
            BstNode::tree_delete(root, &next_node);
        }
    }

    pub fn print_tree(node: &Option<BstNodeLink>, prefix: String, is_left: bool) {
        if let Some(node_ref) = node {
            let key = node_ref.borrow().key.unwrap_or(-1); // Gunakan -1 jika key None
            println!("{}{}── {}", prefix, if is_left { "├" } else { "└" }, key);

            let new_prefix = format!("{}{}", prefix, if is_left { "│   " } else { "    " });
            BstNode::print_tree(&node_ref.borrow().left, new_prefix.clone(), true);
            BstNode::print_tree(&node_ref.borrow().right, new_prefix, false);
        }
    }
}