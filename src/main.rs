mod structure;
mod tool;

use crate::structure::bst::BstNode;
use crate::structure::tree::Node;
use crate::structure::tree::NodeLink;
use crate::structure::bst::BstNodeLink;
use crate::tool::generate_dotfile;
use crate::tool::generate_dotfile_bst;

fn main() {
    //turn on to test the old code
    // test_binary_tree();
    test_binary_search_tree();
}

fn test_binary_search_tree(){
    let rootlink: BstNodeLink = BstNode::new_bst_nodelink(15);
    rootlink.borrow_mut().add_left_child(&rootlink, 6);
    rootlink.borrow_mut().add_right_child(&rootlink, 18);

    //add right subtree
    let right_subtree: &Option<BstNodeLink> = &rootlink.borrow().right;
    if let Some(right_tree_extract) = right_subtree{
        right_tree_extract
            .borrow_mut()
            .add_left_child(right_tree_extract, 17);
        right_tree_extract
            .borrow_mut()
            .add_right_child(right_tree_extract, 20);
    }

    //add left subtree
    let left_subtree: &Option<BstNodeLink> = &rootlink.borrow().left;
    if let Some(left_tree_extract) = left_subtree{
        left_tree_extract
            .borrow_mut()
            .add_left_child(left_tree_extract, 3);
        left_tree_extract
            .borrow_mut()
            .add_right_child(left_tree_extract, 7);

        //add left subtree terminal
        let left_subtree_terminal = &left_tree_extract.borrow().left;
        if let Some(terminal_left_tree_link) = left_subtree_terminal{
            terminal_left_tree_link.borrow_mut().add_left_child(terminal_left_tree_link, 2);
            terminal_left_tree_link.borrow_mut().add_right_child(terminal_left_tree_link, 4);
        }
        //add 2nd level right subtree of node 7
        let second_right_subtree = &left_tree_extract.borrow().right;
        if let Some(second_right_subtree_link) = second_right_subtree{
            second_right_subtree_link.borrow_mut().add_right_child(second_right_subtree_link, 13);

            let third_left_subtree = &second_right_subtree_link.borrow().right;
            if let Some(third_left_subtree_link) = third_left_subtree{
                third_left_subtree_link.borrow_mut().add_left_child(third_left_subtree_link, 9);
            }
        }
    }

    println!("Tree before change:");
    let main_tree_path = "bst_graph.dot";
    generate_dotfile_bst(&rootlink, main_tree_path);
    BstNode::print_tree(&Some(rootlink.clone()), "".to_string(), false);

    let values_to_insert = vec![10, 19, 8, 12, 10];
    for value in values_to_insert {
        println!("Inserting value: {}", value);
        BstNode::tree_insert(&rootlink, value);
    }
    
    println!("Final tree after insert:");
    let main_tree_path_after_insert = "bst_graph_insert.dot";
    generate_dotfile_bst(&rootlink, main_tree_path_after_insert);
    BstNode::print_tree(&Some(rootlink.clone()), "".to_string(), false);

    if let Some(node_to_delete) = rootlink.borrow().tree_search(&13) {
        println!("Deleting node with key 13...");
        BstNode::tree_delete(&rootlink, &node_to_delete);
    }

    println!("Tree after delete:");
    let main_tree_path_after_delete = "bst_graph_after_delete.dot";
    generate_dotfile_bst(&rootlink, main_tree_path_after_delete);
    BstNode::print_tree(&Some(rootlink.clone()), "".to_string(), false);

    if let Some(node_to_replace) = rootlink.borrow().tree_search(&7) {
        if let Some(new_subtree) = rootlink.borrow().tree_search(&9) {
            println!("Transplanting node 7 with subtree rooted at 9...");
            BstNode::transplant(&rootlink, &node_to_replace, Some(new_subtree));
        }
    }

    println!("Tree after transplant:");
    let main_tree_path_after_transplant = "bst_graph_after_transplant.dot";
    generate_dotfile_bst(&rootlink, main_tree_path_after_transplant);
    BstNode::print_tree(&Some(rootlink.clone()), "".to_string(), false);

    //tree search test
    let search_keys = vec![15, 9, 22];
    for &key in search_keys.iter(){
        print!("tree search result of key {} is ", key);

        if let Some(node_result) = rootlink.borrow().tree_search(&key) {
            println!("found -> {:?}", node_result.borrow().key);
        } else {
            println!("not found");
        }
    }

    //min test
    let min_node = rootlink.borrow().minimum();
    println!("minimum result {:?}", min_node.borrow().key);

    //max test
    let max_node = rootlink.borrow().maximum();
    println!("maximum result {:?}", max_node.borrow().key);

    //root node get test
    let root_node = BstNode::get_root(&max_node);
    println!("root node {:?}", root_node.borrow().key);

    //successor test
    let query_keys = vec![
        2, // min_node, should return its parent Some(3)
        20, // max_node, should return None
        15, // root_node, should return the minimum of its right tree
        // test case for node with empty right child
        // should return a parent of the node's ancestor if it's a left child of the parent
        13, // non-existent key, because it has been deleted
        9, 7, // other keys
        22 // non-existent key
    ];

    for &key in query_keys.iter(){
        if let Some(node) = rootlink.borrow().tree_search(&key) {
            print!("successor of node ({}) is ", key);

            if let Some(successor) = BstNode::tree_successor_simpler(&node) {
                println!("{:?}", successor.borrow().key);
            } else {
                println!("not found");
            }
        } else {
            println!("node with key of {} does not exist, failed to get successor", key)
        }
    }
}


#[allow(dead_code)]
fn test_binary_tree() {
    //create the nodelink of the root node
    let rootlink: NodeLink = Node::new_nodelink(5);

    //add a new left node value
    rootlink.borrow_mut().add_left_child(&rootlink, 3);
    //add a new right node value
    rootlink.borrow_mut().add_right_child(&rootlink, 7);

    //println!("{:?}", rootlink);

    //print the tree at this time
    let mut main_tree_path = "prime.dot";
    generate_dotfile(&rootlink, main_tree_path);

    //add new child values to the left subtree
    let left_subtree = &rootlink.borrow().left;
    if let Some(left_tree_extract) = left_subtree {
        left_tree_extract
            .borrow_mut()
            .add_left_child(left_tree_extract, 2);
        left_tree_extract
            .borrow_mut()
            .add_right_child(left_tree_extract, 4);
    }

    //add new child values to the right subtree
    let right_subtree = &rootlink.borrow().right;
    if let Some(right_tree_extract) = right_subtree {
        right_tree_extract
            .borrow_mut()
            .add_right_child(right_tree_extract, 10);
    }

    //print the tree again, now been added with more values
    main_tree_path = "prime_t2.dot";
    generate_dotfile(&rootlink, main_tree_path);

    //Call tree depth function at this time
    let recorded_depth = rootlink.borrow().tree_depth();
    println!("Current tree depth: {0}", recorded_depth);

    //Call count_nodes function
    let total_nodes = rootlink.borrow().count_nodes();
    println!("Amount of nodes in current tree: {0}", total_nodes);

    //Call count_nodes_by_nodelink function, supplied right subtree as parameter
    let subtree_count = Node::count_nodes_by_nodelink(&right_subtree.clone().unwrap(), 0);
    println!("Amount of nodes in current subtree: {0}", subtree_count);

    //Get the sibling of the leftsubtree from parent
    let _left_subtree_sibling = Node::get_sibling(&left_subtree.as_ref().unwrap());
    //println!("sibling of left subtree {:?}", left_subtree_sibling);

    //get the left subtree by value
    let left_subtree = rootlink.borrow().get_node_by_value(3);
    println!("left subtree seek by value {:?}", left_subtree);
    //get the left subtree by full properties
    let another_left_subtree = rootlink
        .borrow()
        .get_node_by_full_property(&left_subtree.as_ref().unwrap());
    println!(
        "left subtree seek by full property {:?}",
        another_left_subtree
    );

    //Discard the right subtree from parent
    let rootlink2 = rootlink.borrow().get_nodelink_copy();

    let flag = rootlink2.borrow_mut().discard_node_by_value(3);
    println!("status of node deletion: {0}", flag);

    //print the tree again
    main_tree_path = "prime_t3.dot";
    generate_dotfile(&rootlink2, main_tree_path);

    //Call tree depth function at this time
    let depth_now = rootlink2.borrow().tree_depth();
    println!("Depth after discard {0}", depth_now);

    //Call count_nodes function
    let count_now = rootlink2.borrow().count_nodes();
    println!("Count nodes after discard {0}", count_now);

    //print the tree again
    main_tree_path = "prime_t4.dot";
    generate_dotfile(&rootlink, main_tree_path);
}
