/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/


use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord, // 它表示类型 T具有 ​​全序关系​​（total ordering），即该类型的任意两个值都可以进行比较
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        let mut node = self.root.as_mut();// self.root是 Option<Box<TreeNode<T>>> Option::as_mut()：将 &mut Option<Box<T>>→ Option<&mut Box<T>>
        if node .is_none() {
            self.root = Some(Box::new(TreeNode::new(value)));
            return;
        }
        while let Some(n) = node {
            if value < n.value {
                if n.left.is_none() {
                    n.left = Some(Box::new(TreeNode::new(value)));
                    return;
                } else {
                    node = n.left.as_mut();
                }
            } else if value > n.value {
                if n.right.is_none() {
                    n.right = Some(Box::new(TreeNode::new(value)));
                    return;
                } else {
                    node = n.right.as_mut();
                }
            } else {
                // Value already exists in the tree, do nothing
                return;
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        let mut node = self.root.as_ref(); // self.root是 Option<Box<TreeNode<T>>> Option::as_ref()：将 &Option<Box<T>>→ Option<&Box<T>>
        while let Some(n) = node {
            match value.cmp(&n.value) {
                Ordering::Less => {
                    node = n.left.as_ref(); 
                }
                Ordering::Greater => {
                    node = n.right.as_ref(); 
                }
                Ordering::Equal => {
                    return true;
                }
            }
    }
        false
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


