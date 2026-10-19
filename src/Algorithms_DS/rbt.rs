

pub use tree::RedBlackTree;
pub use iter::{IterInOrder, Range};
pub use iter::IterPreOrder;
pub use iter::IterReverseInOrder;
pub use iter::IterPostOrder;
 
pub mod iter{
    use std::ptr::NonNull;

    use crate::node::{Link, Node};

    ///```text
    ///
    ///            4
    ///          /   \
    ///         2     9
    ///        / \   //\
    ///       1   3  7   10
    ///             / \
    ///            6   8
    ///```
    ///    Depth-First Search / Pre-order (NLR):     4, 2, 1, 3, 9, 7, 6, 8, 10
    ///    Depth-First Search / In-order (LNR):      1, 2, 3, 4, 6, 7, 8, 9, 10
    ///    Depth-First Search / Post-order (LRN):    1, 3, 2, 6, 8, 7, 10, 9, 4
    ///    Breadth-First Search / Level-order:       4, 2, 9, 1, 3, 7, 10, 6, 8 
    pub use dfs_in_order::{IterInOrder, Range};

    pub(crate) use dfs_in_order::{leftmost_child_in_order, successor_of_node_in_order};

    mod dfs_in_order {
        use super::{Link, Node, NonNull};
        use std::iter::FusedIterator;
        use std::marker::PhantomData;

        /// Returns the next node in an in-order (`LNR`) traversal.
        ///
        /// # Safety
        ///
        /// `node` must point to a live node that belongs to the current tree, and
        /// all parent/child links reachable from it must be valid.
        pub unsafe fn successor_of_node_in_order<T>(node: NonNull<Node<T>>) -> Link<T> {
            unsafe {
                if node.as_ref().right.is_some() {
                    // If a right subtree exists, the successor is its leftmost node.
                    leftmost_child_in_order(node.as_ref().right)
                } else {
                    // Otherwise, walk up through parent links until the current
                    // node is found in a left subtree.
                    parent_with_left(node)
                }
            }
        }

        /// Returns the leftmost node reachable from `node`.
        ///
        /// Returns `None` when `node` is `None`.
        ///
        /// # Safety
        ///
        /// Every pointer reachable through `left` links must refer to a live node.
        pub unsafe fn leftmost_child_in_order<T>(node: Link<T>) -> Link<T> {
            unsafe {
                if let Some(node) = node {
                    if node.as_ref().left.is_none() {
                        Some(node)
                    } else {
                        leftmost_child_in_order(node.as_ref().left)
                    }
                } else {
                    node
                }
            }
        }

        /// Walks up the parent chain until an ancestor is reached from its left child.
        ///
        /// # Safety
        ///
        /// `node` and all parent links traversed by this function must be valid.
        unsafe fn parent_with_left<T>(node: NonNull<Node<T>>) -> Link<T> {
            unsafe {
                let parent = node.as_ref().parent;
                if let Some(parent) = parent
                    && let Some(left) = parent.as_ref().left
                {
                    if std::ptr::eq(left.as_ptr(), node.as_ptr()) {
                        return Some(parent);
                    }
                    return parent_with_left(parent);
                }
                // No parent means that the root has been reached.
                None
            }
        }

        /// Iterator over a tree in in-order (`Left -> Node -> Right`, or `LNR`).
        ///
        /// For a binary search tree, this traversal yields values in ascending order.
        pub struct IterInOrder<'a, T> {
            current_node: Link<T>,
            count: usize,
            elem: Option<&'a T>,
            _boo: PhantomData<&'a T>,
        }

        impl<'a, T> IterInOrder<'a, T> {
            /// Creates an in-order iterator starting at `node`.
            pub(crate) fn new(node: Link<T>, count: usize) -> Self {
                Self {
                    current_node: node,
                    count,
                    elem: None,
                    _boo: PhantomData,
                }
            }
        }

        impl<'a, T> Iterator for IterInOrder<'a, T> {
            type Item = &'a T;

            fn next(&mut self) -> Option<Self::Item> {
                if self.count > 0 {
                    self.count -= 1;
                    if let Some(node) = self.current_node {
                        unsafe {
                            self.elem = Some(&node.as_ref().value);
                            self.current_node = successor_of_node_in_order(node);
                        }
                    } else {
                        self.elem = None;
                    }
                } else {
                    self.elem = None;
                }
                self.elem
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                (self.count, Some(self.count))
            }
        }

        impl<'a, T> ExactSizeIterator for IterInOrder<'a, T> {
            fn len(&self) -> usize {
                self.count
            }
        }

        impl<'a, T> FusedIterator for IterInOrder<'a, T> {}

        /// A lazy iterator over an inclusive range of values in a [`crate::RedBlackTree`].
        ///
        /// `Range` is created by [`crate::RedBlackTree::range`]. It yields references to
        /// values in ascending order, starting from the first value greater than or
        /// equal to the lower bound and stopping after the upper bound.
        ///
        /// The iterator does not allocate an intermediate collection. Values are
        /// produced one at a time as the iterator advances.
        ///
        /// # Examples
        ///
        /// ```
        /// use rbtree_extended::RedBlackTree;
        ///
        /// let mut tree = RedBlackTree::new();
        /// tree.extend([1, 3, 5, 7, 9, 11]);
        ///
        /// let values: Vec<_> = tree
        ///     .range(&3, &9)
        ///     .copied()
        ///     .collect();
        ///
        /// assert_eq!(values, vec![3, 5, 7, 9]);
        /// ```
        ///
        /// The iterator can be combined with standard iterator adapters:
        ///
        /// ```
        /// use rbtree_extended::RedBlackTree;
        ///
        /// let mut tree = RedBlackTree::new();
        /// tree.extend(1..=100);
        ///
        /// let values: Vec<_> = tree
        ///     .range(&20, &80)
        ///     .take(3)
        ///     .copied()
        ///     .collect();
        ///
        /// assert_eq!(values, vec![20, 21, 22]);
        /// ```
        pub struct Range<'a, T> {
            current_node: Link<T>,
            end_node: Link<T>,
            _marker: PhantomData<&'a T>,
        }

        impl<'a, T: Ord> Range<'a, T> {
            pub(crate) fn new(root: Link<T>, min: &T, max: &T) -> Self {
                if min > max {
                    return Self {
                        current_node: None,
                        end_node: None,
                        _marker: PhantomData,
                    };
                }

                unsafe {
                    let mut current = root;
                    let mut start: Link<T> = None;

                    // First node >= min.
                    while let Some(node) = current {
                        match node.as_ref().value.cmp(min) {
                            std::cmp::Ordering::Less => {
                                current = node.as_ref().right;
                            }

                            std::cmp::Ordering::Equal => {
                                start = Some(node);
                                break;
                            }

                            std::cmp::Ordering::Greater => {
                                start = Some(node);
                                current = node.as_ref().left;
                            }
                        }
                    }

                    current = root;

                    let mut end: Link<T> = None;

                    // First node > max.
                    while let Some(node) = current {
                        match node.as_ref().value.cmp(max) {
                            std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                                current = node.as_ref().right;
                            }

                            std::cmp::Ordering::Greater => {
                                end = Some(node);
                                current = node.as_ref().left;
                            }
                        }
                    }

                    Self {
                        current_node: start,
                        end_node: end,
                        _marker: PhantomData,
                    }
                }
            }
        }

        impl<'a, T> Iterator for Range<'a, T> {
            type Item = &'a T;

            fn next(&mut self) -> Option<Self::Item> {
                let node = self.current_node?;

                if self.end_node == Some(node) {
                    self.current_node = None;
                    return None;
                }

                unsafe {
                    let value = &node.as_ref().value;

                    self.current_node = successor_of_node_in_order(node);

                    Some(value)
                }
            }
        }
    }

    pub use dfs_reverse_in_order::IterReverseInOrder;

    pub(crate) use dfs_reverse_in_order::{
        rightmost_child_reverse_in_order, successor_of_node_reverse_in_order,
    };

    mod dfs_reverse_in_order {
        use super::{Link, Node, NonNull};
        use std::marker::PhantomData;

        /// Returns the next node in reverse in-order (`Right -> Node -> Left`, or `RNL`).
        ///
        /// # Safety
        ///
        /// `node` must point to a live node that belongs to the current tree, and
        /// all parent/child links reachable from it must be valid.
        pub unsafe fn successor_of_node_reverse_in_order<T>(node: NonNull<Node<T>>) -> Link<T> {
            unsafe {
                if node.as_ref().left.is_some() {
                    // After the current node, descend into the left subtree and
                    // choose its rightmost node.
                    rightmost_child_reverse_in_order(node.as_ref().left)
                } else {
                    // Without a left subtree, walk upward until reaching an
                    // ancestor from its right subtree.
                    parent_with_right(node)
                }
            }
        }

        /// Returns the rightmost node reachable from `node`.
        ///
        /// Returns `None` when `node` is `None`.
        ///
        /// # Safety
        ///
        /// Every pointer reachable through `right` links must refer to a live node.
        pub unsafe fn rightmost_child_reverse_in_order<T>(node: Link<T>) -> Link<T> {
            unsafe {
                if let Some(node) = node {
                    if node.as_ref().right.is_none() {
                        Some(node)
                    } else {
                        rightmost_child_reverse_in_order(node.as_ref().right)
                    }
                } else {
                    None
                }
            }
        }

        /// Walks up the parent chain until an ancestor is reached from its right child.
        ///
        /// # Safety
        ///
        /// `node` and all parent links traversed by this function must be valid.
        unsafe fn parent_with_right<T>(node: NonNull<Node<T>>) -> Link<T> {
            unsafe {
                let parent = node.as_ref().parent;

                if let Some(parent) = parent
                    && let Some(right) = parent.as_ref().right
                {
                    if std::ptr::eq(right.as_ptr(), node.as_ptr()) {
                        return Some(parent);
                    }

                    return parent_with_right(parent);
                }

                None
            }
        }

        /// Iterator over a tree in reverse in-order (`Right -> Node -> Left`, or `RNL`).
        ///
        /// For a binary search tree, this traversal yields values in descending order.
        pub struct IterReverseInOrder<'a, T> {
            current_node: Link<T>,
            count: usize,
            elem: Option<&'a T>,
            _boo: PhantomData<&'a T>,
        }

        impl<'a, T> IterReverseInOrder<'a, T> {
            /// Creates a reverse in-order iterator starting at `node`.
            pub(crate) fn new(node: Link<T>, count: usize) -> Self {
                Self {
                    current_node: node,
                    count,
                    elem: None,
                    _boo: PhantomData,
                }
            }
        }

        impl<'a, T> Iterator for IterReverseInOrder<'a, T> {
            type Item = &'a T;

            fn next(&mut self) -> Option<Self::Item> {
                if self.count > 0 {
                    self.count -= 1;

                    if let Some(node) = self.current_node {
                        unsafe {
                            self.elem = Some(&node.as_ref().value);
                            self.current_node = successor_of_node_reverse_in_order(node);
                        }
                    } else {
                        self.elem = None;
                    }
                } else {
                    self.elem = None;
                }

                self.elem
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                (self.count, Some(self.count))
            }
        }
    }

    pub use dfs_pre_order::IterPreOrder;

    pub(crate) use dfs_pre_order::successor_of_node_pre_order;

    mod dfs_pre_order {
        use super::{Link, Node, NonNull};
        use std::marker::PhantomData;

        /// Returns the next node in a pre-order (`Node -> Left -> Right`, or `NLR`) traversal.
        ///
        /// # Safety
        ///
        /// `node` must point to a live node in the current tree and all traversed
        /// links must be valid.
        pub unsafe fn successor_of_node_pre_order<T>(node: NonNull<Node<T>>) -> Link<T> {
            unsafe {
                if let Some(node) = node.as_ref().left {
                    Some(node)
                } else if let Some(node) = node.as_ref().right {
                    Some(node)
                } else {
                    right_with_parent(node)
                }
            }
        }

        /// Finds the next right subtree by walking through parent links.
        ///
        /// # Safety
        ///
        /// `node` and all parent links traversed by this function must be valid.
        unsafe fn right_with_parent<T>(node: NonNull<Node<T>>) -> Link<T> {
            unsafe {
                if let Some(parent) = node.as_ref().parent {
                    next_right(Some(parent), node)
                } else {
                    None
                }
            }
        }

        /// Searches ancestors for the next unvisited right child.
        ///
        /// # Safety
        ///
        /// `child`, `node`, and all links traversed by this function must be valid.
        unsafe fn next_right<T>(node: Link<T>, child: NonNull<Node<T>>) -> Link<T> {
            unsafe {
                if let Some(n) = node {
                    if let Some(right) = n.as_ref().right {
                        if std::ptr::eq(right.as_ptr(), child.as_ptr()) {
                            next_right(n.as_ref().parent, n)
                        } else {
                            Some(right)
                        }
                    } else {
                        next_right(n.as_ref().parent, n)
                    }
                } else {
                    None
                }
            }
        }

        /// Iterator over a tree in pre-order (`Node -> Left -> Right`, or `NLR`).
        ///
        /// Pre-order traversal visits a node before either of its subtrees.
        pub struct IterPreOrder<'a, T> {
            current_node: Link<T>,
            count: usize,
            elem: Option<&'a T>,
            _boo: PhantomData<&'a T>,
        }

        impl<'a, T> IterPreOrder<'a, T> {
            /// Creates a pre-order iterator starting at `root`.
            pub(crate) fn new(root: Link<T>, count: usize) -> Self {
                Self {
                    current_node: root,
                    count,
                    elem: None,
                    _boo: PhantomData,
                }
            }
        }

        impl<'a, T> Iterator for IterPreOrder<'a, T> {
            type Item = &'a T;

            fn next(&mut self) -> Option<Self::Item> {
                if self.count > 0 {
                    self.count -= 1;
                    if let Some(node) = self.current_node {
                        unsafe {
                            self.elem = Some(&node.as_ref().value);
                            self.current_node = successor_of_node_pre_order(node);
                        }
                    } else {
                        self.elem = None;
                    }
                } else {
                    self.elem = None;
                }
                self.elem
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                (self.count, Some(self.count))
            }
        }
    }

    pub use dfs_post_order::IterPostOrder;

    pub(crate) use dfs_post_order::{leaf_post_order, successor_of_node_post_order};

    mod dfs_post_order {
        use super::{Link, Node, NonNull};
        use std::marker::PhantomData;

        /// Returns the next node in a post-order (`Left -> Right -> Node`, or `LRN`) traversal.
        ///
        /// # Safety
        ///
        /// `current_node` must point to a live node in the current tree and all
        /// traversed parent/child links must be valid.
        pub unsafe fn successor_of_node_post_order<T>(current_node: NonNull<Node<T>>) -> Link<T> {
            unsafe {
                if let Some(parent) = current_node.as_ref().parent {
                    if let Some(right) = parent.as_ref().right {
                        if std::ptr::eq(current_node.as_ptr(), right.as_ptr()) {
                            Some(parent)
                        } else {
                            Some(leaf_post_order(right))
                        }
                    } else {
                        Some(parent)
                    }
                } else {
                    None
                }
            }
        }

        /// Returns the first leaf visited by a post-order traversal starting at `node`.
        ///
        /// # Safety
        ///
        /// `node` must point to a live node and all descendant links traversed by
        /// this function must be valid.
        pub unsafe fn leaf_post_order<T>(node: NonNull<Node<T>>) -> NonNull<Node<T>> {
            unsafe {
                if let Some(left) = node.as_ref().left {
                    leaf_post_order(left)
                } else if let Some(right) = node.as_ref().right {
                    leaf_post_order(right)
                } else {
                    node
                }
            }
        }

        /// Iterator over a tree in post-order (`Left -> Right -> Node`, or `LRN`).
        ///
        /// Post-order traversal visits both subtrees before the current node.
        pub struct IterPostOrder<'a, T> {
            current_node: Link<T>,
            count: usize,
            _boo: PhantomData<&'a T>,
        }

        impl<'a, T> IterPostOrder<'a, T> {
            /// Creates a post-order iterator starting at the first post-order node.
            pub(crate) fn new(root: Link<T>, count: usize) -> Self {
                Self {
                    current_node: root,
                    count,
                    _boo: PhantomData,
                }
            }
        }

        impl<'a, T> Iterator for IterPostOrder<'a, T> {
            type Item = &'a T;

            fn next(&mut self) -> Option<Self::Item> {
                if self.count == 0 {
                    return None;
                }

                let node = self.current_node?;

                unsafe {
                    let value = &node.as_ref().value;
                    self.current_node = successor_of_node_post_order(node);
                    self.count -= 1;
                    Some(value)
                }
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                (self.count, Some(self.count))
            }
        }
    }

    /// Breadth-First Search
    mod bfs {
        use super::{Node, NonNull};
        use std::collections::VecDeque;

        /// Appends tree values to `ret` in breadth-first / level-order.
        pub fn iter_level_order_impl<T>(root: NonNull<Node<T>>, ret: &mut Vec<&T>) {
            let mut deque: VecDeque<NonNull<Node<T>>> = VecDeque::new();
            unsafe {
                deque.push_back(root);
                let mut node: NonNull<Node<T>>;
                while !deque.is_empty() {
                    node = deque.pop_front().unwrap();
                    ret.push(&node.as_ref().value);
                    if let Some(left) = node.as_ref().left {
                        deque.push_back(left);
                    }
                    if let Some(right) = node.as_ref().right {
                        deque.push_back(right);
                    }
                }
            }
        }
    }

    pub(crate) use bfs::iter_level_order_impl;
}

/// Internal node representation used by the Red-Black Tree.
///
/// Nodes are allocated on the heap and connected using [`NonNull`] pointers.
/// The tree owns the allocated nodes and is responsible for releasing them.
///
/// `Node` and [`Link`] are crate-private implementation details and are not
/// exposed through the public API.
pub mod node{

    use std::ptr::NonNull;

    /// A single node in the Red-Black Tree.
    ///
    /// Each node stores:
    ///
    /// - a link to its left child;
    /// - a link to its right child;
    /// - a link to its parent;
    /// - its Red-Black color;
    /// - the stored value.
    ///
    /// Tree links are represented by [`Link<T>`], which is an
    /// `Option<NonNull<Node<T>>>`.
    ///
    /// A `None` link represents the absence of a parent or child, while
    /// `Some(pointer)` refers to another allocated node.
    ///
    /// Memory ownership is managed by [`crate::RedBlackTree`]. The pointers stored
    /// here are non-owning links between nodes.
    #[derive(Debug)]
    pub(crate) struct Node<T> {
        /// Left child of this node.
        pub(crate) left: Link<T>,

        /// Right child of this node.
        pub(crate) right: Link<T>,

        /// Parent of this node.
        ///
        /// The root node has no parent.
        pub(crate) parent: Link<T>,

        /// Red-Black color of the node.
        ///
        /// `true` means red and `false` means black.
        pub(crate) is_red: bool,

        /// Value stored in this node.
        pub(crate) value: T,
    }

    /// A link between nodes in the tree.
    ///
    /// `None` represents a missing parent or child.
    ///
    /// `Some(ptr)` contains a non-null pointer to another allocated [`Node`].
    ///
    /// Using `Option<NonNull<Node<T>>>` allows the implementation to represent
    /// nullable tree links while keeping direct pointer-based navigation between
    /// parents and children.
    pub(crate) type Link<T> = Option<NonNull<Node<T>>>;

    impl<T: Default> Node<T> {
        /// Creates the sentinel head node together with the initial tree root.
        ///
        /// The sentinel does not represent a value stored by the user. Its
        /// `value` field is initialized with [`Default::default`] and its left
        /// child becomes the first actual node in the tree.
        ///
        /// The initial root node is created as black, as required by the
        /// Red-Black Tree invariants.
        ///
        /// # Returns
        ///
        /// A link to the newly allocated sentinel node.
        ///
        /// # Implementation
        ///
        /// Both the sentinel and the root node are allocated on the heap.
        /// Ownership of these allocations is transferred to the tree and later
        /// recovered when the tree releases its nodes.
        pub(crate) fn new_fixed_head(value: T) -> Link<T> {
            let fixed_head = Box::into_non_null(Box::new(Self {
                left: None,
                right: None,
                parent: None,
                is_red: false,
                value: T::default(),
            }));
            unsafe {
                (*fixed_head.as_ptr()).left = Node::new_black(value);
            }
            Some(fixed_head)
        }
    }

    impl<T> Node<T> {
        /// Allocates a new black node.
        ///
        /// The node is created without a parent or children.
        ///
        /// # Returns
        ///
        /// A non-null link to the newly allocated node.
        ///
        /// The allocation becomes owned by the tree and must eventually be
        /// released by the tree implementation.
        pub(crate) fn new_black(value: T) -> Link<T> {
            let new = Box::into_non_null(Box::new(Self {
                left: None,
                right: None,
                parent: None,
                is_red: false,
                value,
            }));

            Some(new)
        }

        /// Allocates a new red node with the specified parent.
        ///
        /// Newly inserted Red-Black Tree nodes are initially red. The balancing
        /// algorithm may later recolor the node or rotate the surrounding
        /// subtree.
        ///
        /// The new node has no children.
        ///
        /// # Parameters
        ///
        /// - `value` — value stored in the node.
        /// - `parent` — non-null pointer to the node's parent.
        ///
        /// # Returns
        ///
        /// A non-null link to the newly allocated node.
        ///
        /// The allocation becomes owned by the tree and must eventually be
        /// released by the tree implementation.
        pub(crate) fn new_red(value: T, parent: NonNull<Node<T>>) -> Link<T> {
            let new = Box::into_non_null(Box::new(Self {
                left: None,
                right: None,
                parent: Some(parent),
                is_red: true,
                value,
            }));

            Some(new)
        }
    }
}


pub mod validation{
    use crate::node::Link;
    use crate::tree::RedBlackTree;

    impl<T: Ord + Default> RedBlackTree<T> {
        // red-red violations,
        // min black-height,
        // max black-height,
        // BST ordering,
        // parent links,
        // reachable node count
        unsafe fn validate(
            node: &Link<T>,
            expected_parent: Link<T>,
            parent_is_red: bool,
            black_height: usize,
            min: Option<&T>,
            max: Option<&T>,
        ) -> (usize, usize, usize, bool, bool, usize) {
            unsafe {
                if let Some(n) = node {
                    let value = &n.as_ref().value;

                    // 1. Check parent
                    let parent_valid = match (n.as_ref().parent, expected_parent) {
                        (None, None) => true,

                        (Some(actual), Some(expected)) => {
                            std::ptr::eq(actual.as_ptr(), expected.as_ptr())
                        }

                        _ => false,
                    };

                    // 2. Check BST ordering
                    if let Some(min) = min
                        && value <= min
                    {
                        return (0, 0, 0, false, parent_valid, 1);
                    }

                    if let Some(max) = max
                        && value >= max
                    {
                        return (0, 0, 0, false, parent_valid, 1);
                    }

                    let is_red = n.as_ref().is_red;

                    // 3. A red node cannot have a red parent.
                    let red_red = if parent_is_red && is_red { 1 } else { 0 };

                    // 4. Black height
                    let black_height = if is_red {
                        black_height
                    } else {
                        black_height + 1
                    };

                    let left = Self::validate(
                        &n.as_ref().left,
                        Some(*n),
                        is_red,
                        black_height,
                        min,
                        Some(value),
                    );

                    let right = Self::validate(
                        &n.as_ref().right,
                        Some(*n),
                        is_red,
                        black_height,
                        Some(value),
                        max,
                    );

                    (
                        red_red + left.0 + right.0,
                        std::cmp::min(left.1, right.1),
                        std::cmp::max(left.2, right.2),
                        left.3 && right.3,
                        parent_valid && left.4 && right.4,
                        1 + left.5 + right.5,
                    )
                } else {
                    (0, black_height, black_height, true, true, 0)
                }
            }
        }

        pub fn is_valid(&self) -> bool {
            unsafe {
                // tree is empty
                if self.count == 0 {
                    return match self.fixed_head {
                        None => true,

                        Some(fixed_head) => (*fixed_head.as_ptr()).left.is_none(),
                    };
                }

                // count > 0, so sentinel must exist.
                let Some(fixed_head) = self.fixed_head else {
                    return false;
                };

                // And root must exist.
                let root_link = (*fixed_head.as_ptr()).left;

                let Some(root) = root_link else {
                    return false;
                };

                // Root is always BLACK
                if root.as_ref().is_red {
                    return false;
                }

                // Root has no parent
                if root.as_ref().parent.is_some() {
                    return false;
                }

                let result = Self::validate(&root_link, None, false, 0, None, None);

                let red_red = result.0;
                let black_height_min = result.1;
                let black_height_max = result.2;
                let bst_valid = result.3;
                let parents_valid = result.4;
                let reachable_nodes = result.5;

                red_red == 0
                    && black_height_min == black_height_max
                    && bst_valid
                    && parents_valid
                    && reachable_nodes == self.count
            }
        }
    }
}


pub mod tree{
    use std::cmp::Ordering;
    use std::fmt::{Debug, Display};

    use crate::node::{Link, Node};

    use std::marker::PhantomData;
    use std::ptr::NonNull;

    use crate::iter::{IterInOrder, Range, leftmost_child_in_order, successor_of_node_in_order};

    use crate::iter::{
        IterReverseInOrder, rightmost_child_reverse_in_order, successor_of_node_reverse_in_order,
    };

    use crate::iter::{IterPreOrder, successor_of_node_pre_order};

    use crate::iter::{IterPostOrder, leaf_post_order, successor_of_node_post_order};

    use crate::iter::iter_level_order_impl;

    /// A self-balancing Red-Black Tree.
    ///
    /// `RedBlackTree` stores unique values in sorted order.
    ///
    /// Insert, lookup and removal operations have logarithmic
    /// expected tree height due to Red-Black Tree balancing.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbtree_extended::RedBlackTree;
    ///
    /// let mut tree = RedBlackTree::new();
    ///
    /// tree.insert(3);
    /// tree.insert(1);
    /// tree.insert(2);
    ///
    /// assert_eq!(tree.len(), 3);
    /// assert!(tree.contains(&2));
    /// ```
    pub struct RedBlackTree<T: Ord + Default> {
        pub(crate) fixed_head: Link<T>,
        pub(crate) count: usize,
        _marker: PhantomData<T>,
    }

    enum OperationPut {
        Left,
        Right,
        FlipColors,
        Nothing,
    }

    enum OperationRemove {
        RedLeaf,
        BlackLeaf,
        NodeWithChildren,
        BlackNodeWithRedLeaf,
    }

    enum OperationRemoveBlackLeaf {
        LeftRedABlackBRedCleaf,                      // 2.1.1.1
        RightRedABlackBRedCleaf,                     // 2.1.1.2
        LeftRedABlackBleaf,                          // 2.1.2.1
        RightRedABlackBleaf,                         // 2.1.2.2
        BlackARedBWithBlackChildrenLeaf,             // 2.2.4
        BlackARedBWithBlackChildrenRightHaveRedLeaf, // 2.2.1
        BlackALeftBlackBRedDleaf,                    // 2.3.1.1
        BlackARightBlackBRedDleaf,                   // 2.3.1.2
        BlackALeftBlackBleaf,                        // 2.3.2.1
        BlackARightBlackBleaf,                       // 2.3.2.2
        Root,
        Unimplemented,
    }

    impl<T: Ord + Default + Display> RedBlackTree<T> {
        /// Exports the tree in Graphviz DOT format.
        ///
        /// The returned string can be rendered using Graphviz-compatible tools.
        ///
        /// An empty tree produces a valid empty DOT graph.
        ///
        /// # Examples
        ///
        /// ```
        /// use rbtree_extended::RedBlackTree;
        ///
        /// let mut tree = RedBlackTree::new();
        /// tree.insert(10);
        /// tree.insert(5);
        /// tree.insert(20);
        ///
        /// let dot = tree.to_dot();
        ///
        /// println!("{dot}");
        /// ```
        pub fn to_dot(&self) -> String {
            unsafe {
                let Some(fixed_head) = self.fixed_head else {
                    return "digraph Tree {}".into();
                };

                let Some(root) = (*fixed_head.as_ptr()).left else {
                    return "digraph Tree {}".into();
                };

                format!(
                    "\ndigraph Tree {{\n\tratio = fill;\n\tnode [style=filled fontcolor=\"white\"];\n{}}}",
                    helper_display_tree(root)
                )
            }
        }
    }

    impl<T: Ord + Default> RedBlackTree<T> {
        /// Creates an empty Red-Black Tree.
        ///
        /// # Examples
        ///
        /// ```
        /// use rbtree_extended::RedBlackTree;
        ///
        /// let tree = RedBlackTree::<i32>::new();
        ///
        /// assert!(tree.is_empty());
        /// ```
        pub fn new() -> Self {
            Self {
                fixed_head: None,
                count: 0,
                _marker: PhantomData,
            }
        }

        /// Returns the number of values stored in the tree.
        ///
        /// # Examples
        ///
        /// ```
        /// use rbtree_extended::RedBlackTree;
        ///
        /// let mut tree = RedBlackTree::new();
        /// tree.insert(10);
        /// tree.insert(20);
        ///
        /// assert_eq!(tree.len(), 2);
        /// ```
        pub fn len(&self) -> usize {
            self.count
        }

        /// Returns `true` if the tree contains no values.
        ///
        /// # Examples
        ///
        /// ```
        /// use rbtree_extended::RedBlackTree;
        ///
        /// let mut tree = RedBlackTree::<i32>::new();
        /// assert!(tree.is_empty());
        ///
        /// tree.insert(10);
        /// assert!(!tree.is_empty());
        /// ```
        pub fn is_empty(&self) -> bool {
            self.count == 0
        }

        /// Removes all values from the tree.
        ///
        /// After calling `clear`, the tree is empty and can be reused.
        ///
        /// # Examples
        ///
        /// ```
        /// use rbtree_extended::RedBlackTree;
        ///
        /// let mut tree = RedBlackTree::new();
        /// tree.extend([1, 2, 3]);
        ///
        /// tree.clear();
        ///
        /// assert!(tree.is_empty());
        /// assert_eq!(tree.len(), 0);
        /// ```
        pub fn clear(&mut self) {
            unsafe {
                if let Some(fixed_head) = self.fixed_head {
                    let root = (*fixed_head.as_ptr()).left;

                    (*fixed_head.as_ptr()).left = None;

                    self.remove_tree(root);
                }

                debug_assert_eq!(self.count, 0);
            }
        }

        /// Returns the smallest value in the tree.
        ///
        /// Returns `None` if the tree is empty.
        ///
        /// # Examples
        ///
        /// ```
        /// use rbtree_extended::RedBlackTree;
        ///
        /// let mut tree = RedBlackTree::new();
        /// tree.extend([8, 3, 10, 1]);
        ///
        /// assert_eq!(tree.first(), Some(&1));
        /// ```
        pub fn first(&self) -> Option<&T> {
            self.minimum()
        }

        /// Returns the largest value in the tree.
        ///
        /// Returns `None` if the tree is empty.
        ///
        /// # Examples
        ///
        /// ```
        /// use rbtree_extended::RedBlackTree;
        ///
        /// let mut tree = RedBlackTree::new();
        /// tree.extend([8, 3, 10, 14]);
        ///
        /// assert_eq!(tree.last(), Some(&14));
        /// ```
        pub fn last(&self) -> Option<&T> {
            self.maximum()
        }

        fn minimum(&self) -> Option<&T> {
            unsafe {
                let root = self.get_root()?;
                let node = self.find_min(root);
                Some(&node.as_ref().value)
            }
        }

        fn maximum(&self) -> Option<&T> {
            unsafe {
                let root = self.get_root()?;
                let node = self.find_max(root);

                Some(&node.as_ref().value)
            }
        }

        /// Returns the height of the tree.
        ///
        /// An empty tree has height `0`.
        ///
        /// # Examples
        ///
        /// ```
        /// use rbtree_extended::RedBlackTree;
        ///
        /// let mut tree = RedBlackTree::new();
        /// assert_eq!(tree.height(), 0);
        ///
        /// tree.insert(10);
        /// assert_eq!(tree.height(), 1);
        /// ```
        pub fn height(&self) -> usize {
            unsafe fn height_from<T>(node: Link<T>) -> usize {
                unsafe {
                    match node {
                        None => 0,

                        Some(node) => {
                            let left_height = height_from(node.as_ref().left);
                            let right_height = height_from(node.as_ref().right);

                            1 + left_height.max(right_height)
                        }
                    }
                }
            }

            unsafe { height_from(self.get_root()) }
        }

        /// Returns `true` if the tree contains `value`.
        ///
        /// # Examples
        ///
        /// ```
        /// use rbtree_extended::RedBlackTree;
        ///
        /// let mut tree = RedBlackTree::new();
        /// tree.extend([1, 3, 5]);
        ///
        /// assert!(tree.contains(&3));
        /// assert!(!tree.contains(&4));
        /// ```
        pub fn contains(&self, value: &T) -> bool {
            !find_node(self.get_root(), value).is_none()
        }

        /// Inserts a value into the tree.
        ///
        /// Returns `true` if the value was inserted.
        ///
        /// Returns `false` if the value already exists.
        ///
        /// # Examples
        ///
        /// ```
        /// use rbtree_extended::RedBlackTree;
        ///
        /// let mut tree = RedBlackTree::new();
        ///
        /// assert!(tree.insert(10));
        /// assert!(!tree.insert(10));
        /// ```
        pub fn insert(&mut self, value: T) -> bool {
            unsafe {
                if let Some(fixed_head) = self.fixed_head {
                    if (*fixed_head.as_ptr()).left.is_none() {
                        (*fixed_head.as_ptr()).left = Node::new_black(value);
                        self.count += 1;
                        return true;
                    }

                    let parent = self.find_put_parent_candidate(self.get_root(), &value);

                    if let Some(parent) = parent {
                        if self.attach_node(parent, value) {
                            self.put_balancing(parent);
                        } else {
                            return false;
                        }
                    } else {
                        return false;
                    }
                } else {
                    self.fixed_head = Node::new_fixed_head(value);
                }

                self.count += 1;
                true
            }
        }

        /// Removes a value from the tree.
        ///
        /// Returns `true` if the value existed and was removed.
        ///
        /// Use [`RedBlackTree::take`] when ownership of the removed value is needed.
        ///
        /// # Examples
        ///
        /// ```
        /// use rbtree_extended::RedBlackTree;
        ///
        /// let mut tree = RedBlackTree::new();
        /// tree.insert(10);
        ///
        /// assert!(tree.remove(&10));
        /// assert!(!tree.contains(&10));
        /// assert!(!tree.remove(&10));
        /// ```
        pub fn remove(&mut self, value: &T) -> bool {
            self.take(value).is_some()
        }

        /// Inserts `value` into the tree, replacing an existing equal value.
        ///
        /// If the tree already contains a value that compares equal to `value`,
        /// the existing value is replaced and returned.
        ///
        /// If no equal value exists, `value` is inserted and `None` is returned.
        ///
        /// This is especially useful for custom types whose [`Ord`] implementation
        /// compares only a key field.
        ///
        /// # Examples
        ///
        /// ```
        /// use std::cmp::Ordering;
        /// use rbtree_extended::RedBlackTree;
        ///
        /// #[derive(Debug, Default)]
        /// struct User {
        ///     id: u32,
        ///     name: String,
        /// }
        ///
        /// impl PartialEq for User {
        ///     fn eq(&self, other: &Self) -> bool {
        ///         self.id == other.id
        ///     }
        /// }
        ///
        /// impl Eq for User {}
        ///
        /// impl PartialOrd for User {
        ///     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        ///         Some(self.cmp(other))
        ///     }
        /// }
        ///
        /// impl Ord for User {
        ///     fn cmp(&self, other: &Self) -> Ordering {
        ///         self.id.cmp(&other.id)
        ///     }
        /// }
        ///
        /// let mut users = RedBlackTree::new();
        ///
        /// users.insert(User {
        ///     id: 1,
        ///     name: "Alice".into(),
        /// });
        ///
        /// let old = users.replace(User {
        ///     id: 1,
        ///     name: "Bob".into(),
        /// });
        ///
        /// assert_eq!(old.unwrap().name, "Alice");
        /// assert_eq!(users.first().unwrap().name, "Bob");
        /// ```
        pub fn replace(&mut self, value: T) -> Option<T> {
            unsafe {
                if let Some(mut node) = find_node(self.get_root(), &value) {
                    return Some(std::mem::replace(&mut node.as_mut().value, value));
                }
            }

            self.insert(value);

            None
        }

        unsafe fn take_node(&mut self, mut node: NonNull<Node<T>>) -> Option<T> {
            unsafe {
                // Save the original value before the internal removal algorithm
                // starts swapping values between nodes.
                let removed_value = std::mem::take(&mut node.as_mut().value);

                if !self.remove_node(node) {
                    return None;
                }

                self.count -= 1;

                Some(removed_value)
            }
        }

        /// Removes and returns the value equal to `value`.
        ///
        /// Returns `None` if the value is not present in the tree.
        ///
        /// Unlike [`RedBlackTree::remove`], this method gives ownership of the
        /// removed value back to the caller.
        ///
        /// # Examples
        ///
        /// ```
        /// use rbtree_extended::RedBlackTree;
        ///
        /// let mut tree = RedBlackTree::new();
        /// tree.extend([10, 5, 20]);
        ///
        /// assert_eq!(tree.take(&10), Some(10));
        /// assert!(!tree.contains(&10));
        /// assert_eq!(tree.take(&100), None);
        /// ```
        pub fn take(&mut self, value: &T) -> Option<T> {
            if self.is_empty() {
                return None;
            }

            unsafe {
                let node = find_node(self.get_root(), value)?;
                self.take_node(node)
            }
        }

        /// Retains only the values specified by the predicate.
        ///
        /// In other words, removes every value for which `f(&value)` returns `false`.
        ///
        /// The predicate receives a shared reference to each stored value.
        ///
        /// # Examples
        ///
        /// ```
        /// use rbtree_extended::RedBlackTree;
        ///
        /// let mut tree = RedBlackTree::new();
        /// tree.extend([1, 2, 3, 4, 5, 6]);
        ///
        /// tree.retain(|value| value % 2 == 0);
        ///
        /// assert_eq!(
        ///     tree.iter().copied().collect::<Vec<_>>(),
        ///     vec![2, 4, 6]
        /// );
        /// ```
        ///
        /// Retaining no values empties the tree:
        ///
        /// ```
        /// use rbtree_extended::RedBlackTree;
        ///
        /// let mut tree = RedBlackTree::new();
        /// tree.extend([1, 2, 3]);
        ///
        /// tree.retain(|_| false);
        ///
        /// assert!(tree.is_empty());
        /// ```
        pub fn retain<F>(&mut self, mut f: F)
        where
            F: FnMut(&T) -> bool,
        {
            let mut retained = Self::new();

            while let Some(value) = self.pop_first() {
                if f(&value) {
                    retained.insert(value);
                }
            }

            *self = retained;
        }

        /// Removes and returns the smallest value in the tree.
        ///
        /// Returns `None` if the tree is empty.
        ///
        /// This operation is useful when values need to be processed in
        /// ascending order.
        ///
        /// # Examples
        ///
        /// ```
        /// use rbtree_extended::RedBlackTree;
        ///
        /// let mut tree = RedBlackTree::new();
        /// tree.extend([8, 3, 10, 1, 6]);
        ///
        /// assert_eq!(tree.pop_first(), Some(1));
        /// assert_eq!(tree.pop_first(), Some(3));
        /// assert_eq!(tree.len(), 3);
        /// ```
        pub fn pop_first(&mut self) -> Option<T> {
            unsafe {
                let root = self.get_root()?;
                let node = self.find_min(root);

                self.take_node(node)
            }
        }

        /// Removes and returns the largest value in the tree.
        ///
        /// Returns `None` if the tree is empty.
        ///
        /// This operation can be used to implement a priority queue where the
        /// greatest value represents the highest priority.
        ///
        /// # Examples
        ///
        /// ```
        /// use rbtree_extended::RedBlackTree;
        ///
        /// let mut tree = RedBlackTree::new();
        /// tree.extend([8, 3, 10, 1, 6]);
        ///
        /// assert_eq!(tree.pop_last(), Some(10));
        /// assert_eq!(tree.pop_last(), Some(8));
        /// assert_eq!(tree.len(), 3);
        /// ```
        pub fn pop_last(&mut self) -> Option<T> {
            unsafe {
                let root = self.get_root()?;
                let node = self.find_max(root);

                self.take_node(node)
            }
        }

        /// Returns a reference to the stored value equal to `value`.
        ///
        /// Returns `None` if no matching value exists.
        ///
        /// # Examples
        ///
        /// ```
        /// use rbtree_extended::RedBlackTree;
        ///
        /// let mut tree = RedBlackTree::new();
        /// tree.extend([1, 3, 5]);
        ///
        /// assert_eq!(tree.get(&3), Some(&3));
        /// assert_eq!(tree.get(&4), None);
        /// ```
        pub fn get(&self, value: &T) -> Option<&T> {
            unsafe {
                let node = find_node(self.get_root(), value)?;
                Some(&node.as_ref().value)
            }
        }

        /// Returns the next greater value in sorted order.
        ///
        /// Returns `None` if `value` is not present or if it is the largest value.
        ///
        /// # Examples
        ///
        /// ```
        /// use rbtree_extended::RedBlackTree;
        ///
        /// let mut tree = RedBlackTree::new();
        /// tree.extend([1, 3, 5, 7]);
        ///
        /// assert_eq!(tree.successor(&3), Some(&5));
        /// assert_eq!(tree.successor(&7), None);
        /// ```
        pub fn successor(&self, value: &T) -> Option<&T> {
            unsafe {
                let node = find_node(self.get_root(), value)?;

                // Case 1:
                // there is a right subtree - successor = minimum element on the right
                if let Some(right) = node.as_ref().right {
                    let successor = self.find_min(right);
                    return Some(&successor.as_ref().value);
                }

                // Case 2:
                // there is no right subtree
                // Move up while the current node is a right child
                let mut current = node;
                let mut parent = current.as_ref().parent;

                while let Some(p) = parent {
                    if let Some(left) = p.as_ref().left
                        && std::ptr::eq(left.as_ptr(), current.as_ptr())
                    {
                        return Some(&p.as_ref().value);
                    }

                    current = p;
                    parent = p.as_ref().parent;
                }

                None
            }
        }

        /// Returns the previous smaller value in sorted order.
        ///
        /// Returns `None` if `value` is not present or if it is the smallest value.
        ///
        /// # Examples
        ///
        /// ```
        /// use rbtree_extended::RedBlackTree;
        ///
        /// let mut tree = RedBlackTree::new();
        /// tree.extend([1, 3, 5, 7]);
        ///
        /// assert_eq!(tree.predecessor(&5), Some(&3));
        /// assert_eq!(tree.predecessor(&1), None);
        /// ```
        pub fn predecessor(&self, value: &T) -> Option<&T> {
            unsafe {
                let node = find_node(self.get_root(), value)?;

                // Case 1:
                // there is a left subtree - predecessor = maximum element on the left
                if let Some(left) = node.as_ref().left {
                    let predecessor = self.find_max(left);
                    return Some(&predecessor.as_ref().value);
                }

                // Case 2:
                // there is no left subtree
                // Move up while the current node is a left child
                let mut current = node;
                let mut parent = current.as_ref().parent;

                while let Some(p) = parent {
                    if let Some(right) = p.as_ref().right
                        && std::ptr::eq(right.as_ptr(), current.as_ptr())
                    {
                        return Some(&p.as_ref().value);
                    }

                    current = p;
                    parent = p.as_ref().parent;
                }

                None
            }
        }

        /// Returns the first value greater than or equal to `value`.
        ///
        /// # Examples
        ///
        /// ```
        /// use rbtree_extended::RedBlackTree;
        ///
        /// let mut tree = RedBlackTree::new();
        /// tree.extend([1, 3, 6, 8]);
        ///
        /// assert_eq!(tree.lower_bound(&5), Some(&6));
        /// assert_eq!(tree.lower_bound(&6), Some(&6));
        /// ```
        pub fn lower_bound(&self, value: &T) -> Option<&T> {
            unsafe {
                let mut current = self.get_root();
                let mut candidate: Link<T> = None;

                while let Some(node) = current {
                    match node.as_ref().value.cmp(value) {
                        Ordering::Less => {
                            current = node.as_ref().right;
                        }

                        Ordering::Equal => {
                            return Some(&node.as_ref().value);
                        }

                        Ordering::Greater => {
                            candidate = Some(node);
                            current = node.as_ref().left;
                        }
                    }
                }

                candidate.map(|node| &node.as_ref().value)
            }
        }

        /// Returns the first value strictly greater than `value`.
        ///
        /// # Examples
        ///
        /// ```
        /// use rbtree_extended::RedBlackTree;
        ///
        /// let mut tree = RedBlackTree::new();
        /// tree.extend([1, 3, 6, 8]);
        ///
        /// assert_eq!(tree.upper_bound(&6), Some(&8));
        /// assert_eq!(tree.upper_bound(&8), None);
        /// ```
        pub fn upper_bound(&self, value: &T) -> Option<&T> {
            unsafe {
                let mut current = self.get_root();
                let mut candidate: Link<T> = None;

                while let Some(node) = current {
                    match node.as_ref().value.cmp(value) {
                        Ordering::Less | Ordering::Equal => {
                            current = node.as_ref().right;
                        }
                        Ordering::Greater => {
                            candidate = Some(node);
                            current = node.as_ref().left;
                        }
                    }
                }

                candidate.map(|node| &node.as_ref().value)
            }
        }

        /// Returns an iterator over values in the inclusive range `[min, max]`.
        ///
        /// Values are yielded in ascending order.
        ///
        /// The iterator is lazy: values are produced only as they are requested,
        /// rather than being collected into an intermediate `Vec`.
        ///
        /// # Examples
        ///
        /// ```
        /// use rbtree_extended::RedBlackTree;
        ///
        /// let mut tree = RedBlackTree::new();
        /// tree.extend([1, 3, 5, 7, 9, 11]);
        ///
        /// let values: Vec<_> = tree
        ///     .range(&3, &9)
        ///     .copied()
        ///     .collect();
        ///
        /// assert_eq!(values, vec![3, 5, 7, 9]);
        /// ```
        pub fn range(&self, min: &T, max: &T) -> Range<'_, T> {
            Range::new(self.get_root(), min, max)
        }

        /// Returns the next value in an in-order (`LNR`) traversal.
        ///
        /// For a binary search tree, in-order traversal visits values in ascending order.
        ///
        /// Returns `None` if `value` is not present or has no successor in this traversal.
        ///
        /// # Examples
        ///
        /// ```
        /// use rbtree_extended::RedBlackTree;
        ///
        /// let mut tree = RedBlackTree::new();
        /// tree.extend([4, 2, 6, 1, 3, 5, 7]);
        ///
        /// assert_eq!(tree.successor_dfs_in_order(4), Some(&5));
        /// ```
        pub fn successor_dfs_in_order(&self, value: T) -> Option<&T> {
            unsafe {
                let node = find_node(self.get_root(), &value);
                if let Some(n) = node
                    && let Some(nodesucc) = successor_of_node_in_order(n)
                {
                    return Some(&nodesucc.as_ref().value);
                }
                None
            }
        }

        /// Returns the next value in reverse in-order (`RNL`) traversal.
        ///
        /// For a binary search tree, reverse in-order traversal visits values in
        /// descending order.
        ///
        /// Returns `None` if `value` is not present or has no successor in this traversal.
        ///
        /// # Examples
        ///
        /// ```
        /// use rbtree_extended::RedBlackTree;
        ///
        /// let mut tree = RedBlackTree::new();
        /// tree.extend([4, 2, 6, 1, 3, 5, 7]);
        ///
        /// assert_eq!(tree.successor_dfs_reverse_in_order(6), Some(&5));
        /// ```
        pub fn successor_dfs_reverse_in_order(&self, value: T) -> Option<&T> {
            unsafe {
                if let Some(node) = find_node(self.get_root(), &value)
                    && let Some(next) = successor_of_node_reverse_in_order(node)
                {
                    return Some(&next.as_ref().value);
                }
                None
            }
        }

        /// Returns the next value in post-order (`LRN`) traversal.
        ///
        /// Post-order visits the left subtree, then the right subtree,
        /// and finally the current node.
        ///
        /// Returns `None` if `value` is not present or has no successor in this traversal.
        ///
        /// # Examples
        ///
        /// ```
        /// use rbtree_extended::RedBlackTree;
        ///
        /// let mut tree = RedBlackTree::new();
        /// tree.extend([4, 2, 6, 1, 3, 5, 7]);
        ///
        /// // Post-order: 1, 3, 2, 5, 7, 6, 4
        /// assert_eq!(tree.successor_dfs_post_order(3), Some(&2));
        /// assert_eq!(tree.successor_dfs_post_order(2), Some(&5));
        /// assert_eq!(tree.successor_dfs_post_order(4), None);
        /// ```
        pub fn successor_dfs_post_order(&self, value: T) -> Option<&T> {
            unsafe {
                if let Some(n) = find_node(self.get_root(), &value)
                    && let Some(nodesucc) = successor_of_node_post_order(n)
                {
                    return Some(&nodesucc.as_ref().value);
                }
                None
            }
        }

        /// Returns the next value in pre-order (`NLR`) traversal.
        ///
        /// Pre-order visits the current node first, followed by the left and right
        /// subtrees.
        ///
        /// Returns `None` if `value` is not present or has no successor in this traversal.
        ///
        /// # Examples
        ///
        /// ```
        /// use rbtree_extended::RedBlackTree;
        ///
        /// let mut tree = RedBlackTree::new();
        /// tree.extend([4, 2, 6, 1, 3, 5, 7]);
        ///
        /// let first = *tree.iter_preorder().next().unwrap();
        /// assert_eq!(tree.successor_dfs_pre_order(first), tree.iter_preorder().nth(1));
        /// ```
        pub fn successor_dfs_pre_order(&self, value: T) -> Option<&T> {
            unsafe {
                if let Some(n) = find_node(self.get_root(), &value)
                    && let Some(nodesucc) = successor_of_node_pre_order(n)
                {
                    return Some(&nodesucc.as_ref().value);
                }
                None
            }
        }

        fn get_root(&self) -> Link<T> {
            if let Some(fixed_head) = self.fixed_head {
                unsafe { (*fixed_head.as_ptr()).left }
            } else {
                None
            }
        }

        unsafe fn operation_remove(&self, node_x: &NonNull<Node<T>>) -> OperationRemove {
            unsafe {
                if node_x.as_ref().is_red
                    && node_x.as_ref().left.is_none()
                    && node_x.as_ref().right.is_none()
                {
                    return OperationRemove::RedLeaf;
                }
                if node_x.as_ref().is_red
                    && node_x.as_ref().left.is_some()
                    && node_x.as_ref().right.is_some()
                {
                    return OperationRemove::NodeWithChildren;
                }
                if !node_x.as_ref().is_red {
                    if node_x.as_ref().left.is_some() && node_x.as_ref().right.is_some() {
                        return OperationRemove::NodeWithChildren;
                    } else if node_x.as_ref().left.is_some() && node_x.as_ref().right.is_none() {
                        if let Some(left) = node_x.as_ref().left
                            && left.as_ref().is_red
                            && left.as_ref().left.is_none()
                            && left.as_ref().right.is_none()
                        {
                            return OperationRemove::BlackNodeWithRedLeaf;
                        }
                    } else if node_x.as_ref().left.is_none() && node_x.as_ref().right.is_none() {
                        return OperationRemove::BlackLeaf;
                    }
                }
                panic!();
            }
        }

        unsafe fn operation_remove_black_leaf(
            &self,
            node_x: &NonNull<Node<T>>,
        ) -> OperationRemoveBlackLeaf {
            unsafe {
                if let Some(node_a) = node_x.as_ref().parent {
                    if node_a.as_ref().is_red {
                        if let Some(node_b) = node_a.as_ref().left
                            && !std::ptr::eq(node_b.as_ptr(), node_x.as_ptr())
                            && !node_b.as_ref().is_red
                        {
                            if let Some(node_c) = node_b.as_ref().left
                                && node_c.as_ref().is_red
                            {
                                return OperationRemoveBlackLeaf::LeftRedABlackBRedCleaf;
                                // 2.1.1.1
                            }

                            // sibling B is black and its left child is either absent or also black
                            return OperationRemoveBlackLeaf::LeftRedABlackBleaf;
                            // 2.1.2.1
                        }

                        if let Some(node_b) = node_a.as_ref().right
                            && !std::ptr::eq(node_b.as_ptr(), node_x.as_ptr())
                            && !node_b.as_ref().is_red
                        {
                            if let Some(node_c) = node_b.as_ref().left
                                && node_c.as_ref().is_red
                            {
                                return OperationRemoveBlackLeaf::RightRedABlackBRedCleaf;
                                // 2.1.1.2
                            }

                            // sibling B is black and its left child is either absent or also black
                            return OperationRemoveBlackLeaf::RightRedABlackBleaf;
                            // 2.1.2.2
                        }
                    } else {
                        if let Some(node_b) = node_a.as_ref().left
                            && !std::ptr::eq(node_b.as_ptr(), node_x.as_ptr())
                        {
                            if node_b.as_ref().is_red && node_b.as_ref().right.is_some() {
                                let node_c = node_b.as_ref().right.unwrap();
                                if !node_c.as_ref().is_red {
                                    if node_c.as_ref().left.is_some()
                                        && let Some(node_d) = node_c.as_ref().left
                                        && node_d.as_ref().is_red
                                    {
                                        return OperationRemoveBlackLeaf::BlackARedBWithBlackChildrenRightHaveRedLeaf;
                                        // 2.2.1
                                    }
                                    return OperationRemoveBlackLeaf::BlackARedBWithBlackChildrenLeaf;
                                    // 2.2.2
                                }
                            }
                            if !node_b.as_ref().is_red
                                && node_b.as_ref().left.is_some()
                                && let Some(node_d) = node_b.as_ref().left
                                && node_d.as_ref().is_red
                            {
                                return OperationRemoveBlackLeaf::BlackALeftBlackBRedDleaf;
                                // 2.3.1.1
                            }
                            if !node_b.as_ref().is_red {
                                return OperationRemoveBlackLeaf::BlackALeftBlackBleaf;
                                // 2.3.2.1
                            }
                        }
                        if let Some(node_b) = node_a.as_ref().right
                            && !std::ptr::eq(node_b.as_ptr(), node_x.as_ptr())
                        {
                            if !node_b.as_ref().is_red
                                && node_b.as_ref().left.is_some()
                                && let Some(node_d) = node_b.as_ref().left
                                && node_d.as_ref().is_red
                            {
                                return OperationRemoveBlackLeaf::BlackARightBlackBRedDleaf;
                                // 2.3.1.2
                            }
                            if !node_b.as_ref().is_red {
                                return OperationRemoveBlackLeaf::BlackARightBlackBleaf;
                                // 2.3.2.2
                            }
                        }
                    }
                } else {
                    return OperationRemoveBlackLeaf::Root;
                }
                OperationRemoveBlackLeaf::Unimplemented
            }
        }

        unsafe fn remove_black_leaf(&mut self, node_x: NonNull<Node<T>>) -> bool {
            unsafe {
                let mut next = node_x;
                let mut removed = false;
                loop {
                    match self.operation_remove_black_leaf(&next) {
                        OperationRemoveBlackLeaf::LeftRedABlackBRedCleaf => {
                            self.remove_black_leaf_2_1_1_1_balancing(&next);
                            if !removed {
                                removed = self.remove_leaf(node_x);
                            }
                            break;
                        }
                        OperationRemoveBlackLeaf::RightRedABlackBRedCleaf => {
                            self.remove_black_leaf_2_1_1_2_balancing(&next);
                            if !removed {
                                removed = self.remove_leaf(node_x);
                            }
                            break;
                        }
                        OperationRemoveBlackLeaf::LeftRedABlackBleaf => {
                            self.remove_black_leaf_2_1_2_1_balancing(&next);
                            if !removed {
                                removed = self.remove_leaf(node_x);
                            }
                            break;
                        }
                        OperationRemoveBlackLeaf::RightRedABlackBleaf => {
                            self.remove_black_leaf_2_1_2_2_balancing(&next);
                            if !removed {
                                removed = self.remove_leaf(node_x);
                            }
                            break;
                        }
                        OperationRemoveBlackLeaf::BlackARedBWithBlackChildrenRightHaveRedLeaf => {
                            self.remove_black_leaf_2_2_1_balancing(&next);
                            if !removed {
                                removed = self.remove_leaf(node_x);
                            }
                            break;
                        }
                        OperationRemoveBlackLeaf::BlackARedBWithBlackChildrenLeaf => {
                            self.remove_black_leaf_2_2_2_balancing(&next);
                            if !removed {
                                removed = self.remove_leaf(node_x);
                            }
                            break;
                        }
                        OperationRemoveBlackLeaf::BlackALeftBlackBRedDleaf => {
                            self.remove_black_leaf_2_3_1_1_balancing(&next);
                            if !removed {
                                removed = self.remove_leaf(node_x);
                            }
                            break;
                        }
                        OperationRemoveBlackLeaf::BlackARightBlackBRedDleaf => {
                            self.remove_black_leaf_2_3_1_2_balancing(&next);
                            if !removed {
                                removed = self.remove_leaf(node_x);
                            }
                            break;
                        }
                        OperationRemoveBlackLeaf::BlackALeftBlackBleaf => {
                            next = self.remove_black_leaf_2_3_2_1_balancing(&next);
                            if !removed {
                                removed = self.remove_leaf(node_x);
                            }
                        }
                        OperationRemoveBlackLeaf::BlackARightBlackBleaf => {
                            next = self.remove_black_leaf_2_3_2_2_balancing(&next);
                            if !removed {
                                removed = self.remove_leaf(node_x);
                            }
                        }
                        OperationRemoveBlackLeaf::Root => {
                            if !removed {
                                // We get here when the node_x being deleted is root
                                if let Some(fixed_head) = self.fixed_head {
                                    (*fixed_head.as_ptr()).left = None;
                                }
                                let _ = Box::from_raw(node_x.as_ptr());
                                removed = true;
                            }
                            break;
                        }
                        OperationRemoveBlackLeaf::Unimplemented => {
                            panic!("unimplemented black leaf removal case");
                        }
                    }
                }
                removed
            }
        }

        /// 2.1.1.1 remove black X
        ///
        /// ```text
        ///             P            P
        ///            //           //
        ///            A            B
        ///           / \          / \
        ///          B   X   =>   C   A
        ///         //\              /
        ///        C   D            D
        /// ```
        unsafe fn remove_black_leaf_2_1_1_1_balancing(&mut self, node_x: &NonNull<Node<T>>) {
            unsafe {
                let node_a = (*node_x.as_ptr()).parent.unwrap();
                let node_b = (*node_a.as_ptr()).left.unwrap();
                let node_d = (*node_b.as_ptr()).right;
                (*node_b.as_ptr()).parent = (*node_a.as_ptr()).parent;
                if let Some(ref mut parent) = (*node_b.as_ptr()).parent {
                    (*parent.as_ptr()).left = Some(node_b);
                }
                (*node_a.as_ptr()).left = node_d;
                if let Some(n_d) = node_d {
                    (*n_d.as_ptr()).parent = Some(node_a);
                }
                (*node_a.as_ptr()).is_red = false;
                (*node_b.as_ptr()).right = Some(node_a);
                (*node_a.as_ptr()).parent = Some(node_b);
                (*node_b.as_ptr()).is_red = true;
                if let Some(node_c) = (*node_b.as_ptr()).left {
                    (*node_c.as_ptr()).is_red = false;
                }
            }
        }

        /// 2.1.1.2 remove black X
        ///
        /// ```text
        ///             P              P
        ///            //             //
        ///           A               C
        ///          / \            /   \
        ///         X   B   =>     A     B
        ///            //           \   /
        ///            C             E D
        ///           / \
        ///          E   D
        /// ```
        unsafe fn remove_black_leaf_2_1_1_2_balancing(&mut self, node_x: &NonNull<Node<T>>) {
            unsafe {
                let node_a = (*node_x.as_ptr()).parent.unwrap();
                (*node_a.as_ptr()).is_red = false;
                let parent: NonNull<Node<T>> = (*node_a.as_ptr()).parent.unwrap();
                let node_b = (*node_a.as_ptr()).right.unwrap();
                let node_c = (*node_b.as_ptr()).left.unwrap();
                let node_e = (*node_c.as_ptr()).left;
                let node_d = (*node_c.as_ptr()).right;
                (*parent.as_ptr()).left = Some(node_c);
                (*node_c.as_ptr()).parent = Some(parent);
                (*node_b.as_ptr()).left = node_d;
                if let Some(n_d) = node_d {
                    (*n_d.as_ptr()).parent = Some(node_b);
                }
                (*node_c.as_ptr()).right = Some(node_b);
                (*node_b.as_ptr()).parent = Some(node_c);
                (*node_c.as_ptr()).left = Some(node_a);
                (*node_a.as_ptr()).parent = Some(node_c);
                (*node_a.as_ptr()).right = node_e;
                if let Some(n_e) = node_e {
                    (*n_e.as_ptr()).parent = Some(node_a);
                }
            }
        }

        /// 2.1.2.1 remove black X
        ///
        /// ```text
        ///              P            P
        ///             //           /
        ///            A            A
        ///           / \          //
        ///          B   X   =>   B
        ///
        /// ```
        unsafe fn remove_black_leaf_2_1_2_1_balancing(&mut self, node_x: &NonNull<Node<T>>) {
            unsafe {
                let node_a = (*node_x.as_ptr()).parent.unwrap();
                (*node_a.as_ptr()).is_red = false;
                if let Some(node_b) = (*node_a.as_ptr()).left {
                    (*node_b.as_ptr()).is_red = true;
                }
            }
        }

        /// 2.1.2.2 remove black X
        ///
        /// ```text
        ///             P            P
        ///            //           /
        ///            A     =>     B
        ///           / \          //\
        ///          X   B        A   D
        ///             / \        \
        ///            C   D        C
        /// ```
        unsafe fn remove_black_leaf_2_1_2_2_balancing(&mut self, node_x: &NonNull<Node<T>>) {
            unsafe {
                let node_a = (*node_x.as_ptr()).parent.unwrap();
                let node_b = (*node_a.as_ptr()).right.unwrap();
                let node_c = (*node_b.as_ptr()).left;
                let node_d = (*node_b.as_ptr()).right;
                let parent = (*node_a.as_ptr()).parent.unwrap();
                if let Some(left) = (*parent.as_ptr()).left {
                    if std::ptr::eq(node_a.as_ptr(), left.as_ptr()) {
                        (*parent.as_ptr()).left = Some(node_b);
                    } else {
                        (*parent.as_ptr()).right = Some(node_b);
                    }
                } else {
                    (*parent.as_ptr()).right = Some(node_b);
                }
                (*node_b.as_ptr()).parent = Some(parent);
                (*node_b.as_ptr()).left = Some(node_a);
                (*node_b.as_ptr()).right = node_d;
                if let Some(n_d) = node_d {
                    (*n_d.as_ptr()).parent = Some(node_b);
                }
                (*node_a.as_ptr()).parent = Some(node_b);
                (*node_a.as_ptr()).right = node_c;
                if let Some(n_c) = node_c {
                    (*n_c.as_ptr()).parent = Some(node_a);
                }
            }
        }

        /// 2.2.1 remove black X
        ///
        /// ```text
        ///            A               C
        ///          // \            //  \
        ///         B    X    =>    B     A
        ///          \               \   /
        ///           C               D E
        ///           //\
        ///          D   E
        /// ```
        unsafe fn remove_black_leaf_2_2_1_balancing(&mut self, node_x: &NonNull<Node<T>>) {
            unsafe {
                let n_a = node_x.as_ref().parent.unwrap();
                if n_a.as_ref().parent.is_some() {
                    let parent = (*n_a.as_ptr()).parent.unwrap();
                    let mut node_a_from_left = false;
                    if let Some(ref mut node_a) = (*parent.as_ptr()).left
                        && std::ptr::eq(node_a.as_ptr(), n_a.as_ptr())
                    {
                        node_a_from_left = true;
                    }
                    let node_a = if node_a_from_left {
                        (*parent.as_ptr()).left.unwrap()
                    } else {
                        (*parent.as_ptr()).right.unwrap()
                    };
                    let node_b = (*node_a.as_ptr()).left.unwrap();
                    let node_c = (*node_b.as_ptr()).right.unwrap();
                    let node_e = (*node_c.as_ptr()).right;
                    let node_d = (*node_c.as_ptr()).left.unwrap();
                    (*node_b.as_ptr()).right = Some(node_d);
                    (*node_b.as_ptr()).parent = Some(node_c);
                    (*node_d.as_ptr()).parent = Some(node_b);
                    (*node_a.as_ptr()).parent = Some(node_c);
                    (*node_a.as_ptr()).left = node_e;
                    if let Some(n_e) = node_e {
                        (*n_e.as_ptr()).parent = Some(node_a);
                    }
                    (*node_d.as_ptr()).is_red = false;
                    (*node_c.as_ptr()).left = Some(node_b);
                    (*node_c.as_ptr()).right = Some(node_a);
                    (*node_c.as_ptr()).parent = Some(parent);
                    if node_a_from_left {
                        (*parent.as_ptr()).left = Some(node_c);
                    } else {
                        (*parent.as_ptr()).right = Some(node_c);
                    }
                } else {
                    // node_a is root
                    if let Some(fixed_head) = self.fixed_head {
                        let node_a = (*fixed_head.as_ptr()).left.unwrap();
                        let node_b = (*node_a.as_ptr()).left.unwrap();
                        let node_c = (*node_b.as_ptr()).right.unwrap();
                        let node_e = (*node_c.as_ptr()).right;
                        let node_d = (*node_c.as_ptr()).left.unwrap();
                        (*node_b.as_ptr()).right = Some(node_d);
                        (*node_b.as_ptr()).parent = Some(node_c);
                        (*node_d.as_ptr()).parent = Some(node_b);
                        (*node_a.as_ptr()).parent = Some(node_c);
                        (*node_a.as_ptr()).left = node_e;
                        if let Some(n_e) = node_e {
                            (*n_e.as_ptr()).parent = Some(node_a);
                        }
                        (*node_d.as_ptr()).is_red = false;
                        (*node_c.as_ptr()).left = Some(node_b);
                        (*node_c.as_ptr()).right = Some(node_a);
                        (*node_c.as_ptr()).parent = None;
                        (*fixed_head.as_ptr()).left = Some(node_c);
                    }
                }
            }
        }

        /// 2.2.2 remove black X
        ///
        /// ```text
        ///    A            B
        ///  // \            \
        ///  B    X   =>       A
        ///  \               //
        ///   C             C
        /// ```
        unsafe fn remove_black_leaf_2_2_2_balancing(&mut self, node_x: &NonNull<Node<T>>) {
            unsafe {
                let n_a = node_x.as_ref().parent.unwrap();
                if n_a.as_ref().parent.is_some() {
                    let parent = (*n_a.as_ptr()).parent.unwrap();
                    let mut node_a_from_left = false;
                    if let Some(ref mut node_a) = (*parent.as_ptr()).left
                        && std::ptr::eq(node_a.as_ptr(), n_a.as_ptr())
                    {
                        node_a_from_left = true;
                    }
                    let node_a = if node_a_from_left {
                        (*parent.as_ptr()).left.unwrap()
                    } else {
                        (*parent.as_ptr()).right.unwrap()
                    };
                    let node_b = (*node_a.as_ptr()).left.unwrap();
                    (*node_b.as_ptr()).is_red = false;
                    (*node_b.as_ptr()).parent = Some(parent);
                    let node_c = (*node_b.as_ptr()).right.unwrap();
                    (*node_c.as_ptr()).is_red = true;
                    (*node_b.as_ptr()).right = Some(node_a);
                    (*node_a.as_ptr()).left = Some(node_c);
                    (*node_c.as_ptr()).parent = Some(node_a);
                    (*node_a.as_ptr()).parent = Some(node_b);
                    if node_a_from_left {
                        (*parent.as_ptr()).left = Some(node_b);
                    } else {
                        (*parent.as_ptr()).right = Some(node_b);
                    }
                } else {
                    // node_a is root
                    if let Some(fixed_head) = self.fixed_head {
                        let node_a = (*fixed_head.as_ptr()).left.unwrap();
                        let node_b = (*node_a.as_ptr()).left.unwrap();
                        (*node_b.as_ptr()).is_red = false;
                        (*node_b.as_ptr()).parent = None;
                        let node_c = (*node_b.as_ptr()).right.unwrap();
                        (*node_c.as_ptr()).is_red = true;
                        (*node_b.as_ptr()).right = Some(node_a);
                        (*node_a.as_ptr()).left = Some(node_c);
                        (*node_c.as_ptr()).parent = Some(node_a);
                        (*node_a.as_ptr()).parent = Some(node_b);
                        (*fixed_head.as_ptr()).left = Some(node_b);
                    }
                }
            }
        }

        /// 2.3.1.1 remove black X
        ///
        /// ```text
        ///      A           B
        ///     / \         / \
        ///    B   X  =>   D   A
        ///   // \            /
        ///  D    C          C
        /// ```
        unsafe fn remove_black_leaf_2_3_1_1_balancing(&mut self, node_x: &NonNull<Node<T>>) {
            unsafe {
                let node_a = (*node_x.as_ptr()).parent.unwrap();
                if let Some(parent) = (*node_a.as_ptr()).parent {
                    let node_b = (*node_a.as_ptr()).left.unwrap();
                    let node_c = (*node_b.as_ptr()).right;
                    let node_d = (*node_b.as_ptr()).left.unwrap();
                    (*node_d.as_ptr()).is_red = false;
                    let mut node_a_from_left = false;
                    if let Some(p_node_a) = (*parent.as_ptr()).left
                        && std::ptr::eq(p_node_a.as_ptr(), node_a.as_ptr())
                    {
                        node_a_from_left = true;
                    }
                    (*node_b.as_ptr()).parent = Some(parent);
                    (*node_b.as_ptr()).right = Some(node_a);
                    (*node_a.as_ptr()).parent = Some(node_b);
                    (*node_a.as_ptr()).left = node_c;
                    if let Some(n_c) = node_c {
                        (*n_c.as_ptr()).parent = Some(node_a);
                    }
                    if node_a_from_left {
                        (*parent.as_ptr()).left = Some(node_b);
                    } else {
                        (*parent.as_ptr()).right = Some(node_b);
                    }
                } else {
                    // node_a is root
                    if let Some(fixed_head) = self.fixed_head {
                        let node_a = (*fixed_head.as_ptr()).left.unwrap();
                        let node_b = (*node_a.as_ptr()).left.unwrap();
                        let node_c = (*node_b.as_ptr()).right;
                        let node_d = (*node_b.as_ptr()).left.unwrap();
                        (*node_d.as_ptr()).is_red = false;
                        let node_b = (*node_a.as_ptr()).left.unwrap();
                        (*node_b.as_ptr()).parent = None;
                        (*node_b.as_ptr()).right = Some(node_a);
                        (*node_a.as_ptr()).parent = Some(node_b);
                        (*node_a.as_ptr()).left = node_c;
                        if let Some(n_c) = node_c {
                            (*n_c.as_ptr()).parent = Some(node_a);
                        }
                        (*fixed_head.as_ptr()).left = Some(node_b);
                    }
                }
            }
        }

        /// 2.3.1.2 remove black X
        ///
        /// ```text
        ///
        ///      A            D
        ///     / \         /   \
        ///    X   B  =>   A     B
        ///       //        \   /
        ///       D          C E
        ///      / \
        ///     C   E
        /// ```
        unsafe fn remove_black_leaf_2_3_1_2_balancing(&mut self, node_x: &NonNull<Node<T>>) {
            unsafe {
                let node_a = (*node_x.as_ptr()).parent.unwrap();
                if let Some(parent) = (*node_a.as_ptr()).parent {
                    let node_b = (*node_a.as_ptr()).right.unwrap();
                    let node_d = (*node_b.as_ptr()).left.unwrap();
                    let node_c = (*node_d.as_ptr()).left;
                    let node_e = (*node_d.as_ptr()).right;
                    (*node_d.as_ptr()).is_red = false;
                    let mut node_a_from_left = false;
                    if let Some(p_node_a) = (*parent.as_ptr()).left
                        && std::ptr::eq(p_node_a.as_ptr(), node_a.as_ptr())
                    {
                        node_a_from_left = true;
                    }
                    (*node_d.as_ptr()).parent = Some(parent);
                    (*node_d.as_ptr()).left = Some(node_a);
                    (*node_d.as_ptr()).right = Some(node_b);
                    (*node_a.as_ptr()).parent = Some(node_d);
                    (*node_b.as_ptr()).parent = Some(node_d);
                    (*node_a.as_ptr()).right = node_c;
                    if let Some(n_c) = node_c {
                        (*n_c.as_ptr()).parent = Some(node_a);
                    }
                    (*node_b.as_ptr()).left = node_e;
                    if let Some(n_e) = node_e {
                        (*n_e.as_ptr()).parent = Some(node_b);
                    }
                    if node_a_from_left {
                        (*parent.as_ptr()).left = Some(node_d);
                    } else {
                        (*parent.as_ptr()).right = Some(node_d);
                    }
                } else {
                    // node_a is root
                    if let Some(fixed_head) = self.fixed_head {
                        let node_a = (*fixed_head.as_ptr()).left.unwrap();
                        let node_b = (*node_a.as_ptr()).right.unwrap();
                        let node_d = (*node_b.as_ptr()).left.unwrap();
                        let node_c = (*node_d.as_ptr()).left;
                        let node_e = (*node_d.as_ptr()).right;
                        (*node_d.as_ptr()).is_red = false;
                        (*node_d.as_ptr()).parent = None;
                        (*node_d.as_ptr()).left = Some(node_a);
                        (*node_d.as_ptr()).right = Some(node_b);
                        (*node_a.as_ptr()).parent = Some(node_d);
                        (*node_b.as_ptr()).parent = Some(node_d);
                        (*node_a.as_ptr()).right = node_c;
                        if let Some(n_c) = node_c {
                            (*n_c.as_ptr()).parent = Some(node_a);
                        }
                        (*node_b.as_ptr()).left = node_e;
                        if let Some(n_e) = node_e {
                            (*n_e.as_ptr()).parent = Some(node_b);
                        }
                        (*fixed_head.as_ptr()).left = Some(node_d);
                    }
                }
            }
        }

        /// 2.3.2.1 remove black X
        ///
        /// ```text
        ///            P           P
        ///            |           |
        ///            A           A
        ///           / \   =>    //   => next check node A
        ///          B   X       B
        ///
        /// ```
        unsafe fn remove_black_leaf_2_3_2_1_balancing(
            &mut self,
            node_x: &NonNull<Node<T>>,
        ) -> NonNull<Node<T>> {
            unsafe {
                let node_a = (*node_x.as_ptr()).parent.unwrap();
                let node_b = (*node_a.as_ptr()).left.unwrap();
                (*node_b.as_ptr()).is_red = true;
                node_a
            }
        }

        /// 2.3.2.2 remove black X
        ///
        /// ```text
        ///              A            B
        ///             / \   =>    // \    =>  next check node B
        ///            X   B       A    D
        ///               / \       \
        ///              C   D       C
        /// ```
        unsafe fn remove_black_leaf_2_3_2_2_balancing(
            &mut self,
            node_x: &NonNull<Node<T>>,
        ) -> NonNull<Node<T>> {
            unsafe {
                let node_a = (*node_x.as_ptr()).parent.unwrap();
                (*node_a.as_ptr()).is_red = true;
                if let Some(parent) = (*node_a.as_ptr()).parent {
                    let node_b = (*node_a.as_ptr()).right.unwrap();
                    let node_c = (*node_b.as_ptr()).left;
                    let node_d = (*node_b.as_ptr()).right;
                    let mut node_a_from_left = false;
                    if let Some(p_node_a) = (*parent.as_ptr()).left
                        && std::ptr::eq(p_node_a.as_ptr(), node_a.as_ptr())
                    {
                        node_a_from_left = true;
                    }
                    (*node_b.as_ptr()).parent = Some(parent);
                    (*node_a.as_ptr()).right = node_c;
                    if let Some(n_c) = node_c {
                        (*n_c.as_ptr()).parent = Some(node_a);
                    }
                    (*node_b.as_ptr()).right = node_d;
                    if let Some(n_d) = node_d {
                        (*n_d.as_ptr()).parent = Some(node_b);
                    }
                    (*node_b.as_ptr()).left = Some(node_a);
                    (*node_a.as_ptr()).parent = Some(node_b);
                    if node_a_from_left {
                        (*parent.as_ptr()).left = Some(node_b);
                    } else {
                        (*parent.as_ptr()).right = Some(node_b);
                    }
                    node_b
                } else {
                    // node_a is root
                    if let Some(fixed_head) = self.fixed_head {
                        let node_a = (*fixed_head.as_ptr()).left.unwrap();
                        (*node_a.as_ptr()).is_red = true;
                        let node_b = (*node_a.as_ptr()).right.unwrap();
                        let node_c = (*node_b.as_ptr()).left;
                        let node_d = (*node_b.as_ptr()).right;
                        (*node_b.as_ptr()).parent = None;
                        (*node_a.as_ptr()).right = node_c;
                        if let Some(n_c) = node_c {
                            (*n_c.as_ptr()).parent = Some(node_a);
                        }
                        (*node_b.as_ptr()).right = node_d;
                        if let Some(n_d) = node_d {
                            (*n_d.as_ptr()).parent = Some(node_b);
                        }
                        (*node_b.as_ptr()).left = Some(node_a);
                        (*node_a.as_ptr()).parent = Some(node_b);
                        (*fixed_head.as_ptr()).left = Some(node_b);
                    }
                    self.get_root().unwrap()
                }
            }
        }

        unsafe fn remove_node(&mut self, node: NonNull<Node<T>>) -> bool {
            unsafe {
                match self.operation_remove(&node) {
                    OperationRemove::RedLeaf => {
                        // option 1.0.0
                        //
                        // ```text
                        //
                        //  P
                        //  ||
                        //  X
                        //
                        // ```

                        self.remove_leaf(node)
                    }
                    OperationRemove::BlackLeaf => {
                        // option 2 black leaf
                        self.remove_black_leaf(node)
                    }
                    OperationRemove::NodeWithChildren => {
                        // option 4.0.0
                        // ```text
                        //        X
                        //     /      \
                        //    L        R
                        //   / \      / \
                        // ...  C    L
                        //          /
                        //         C
                        //        ...
                        //
                        // ```
                        /*
                            The node has two children.

                            We take the predecessor:
                            the maximal node of the left subtree.

                            We only change the values,
                            then we delete the predecessor as a regular node.
                        */

                        let predecessor = self.find_max(node.as_ref().left.unwrap());

                        std::mem::swap(
                            &mut (*node.as_ptr()).value,
                            &mut (*predecessor.as_ptr()).value,
                        );

                        self.remove_node(predecessor)
                    }
                    OperationRemove::BlackNodeWithRedLeaf => {
                        // option 3.0.0
                        // ```text
                        //  X
                        // //
                        // A
                        // ```
                        let red_left = (*node.as_ptr()).left.unwrap();
                        std::mem::swap(&mut (*node.as_ptr()).value, &mut (*red_left.as_ptr()).value);
                        self.remove_leaf(red_left)
                    }
                }
            }
        }

        unsafe fn find_max(&self, node: NonNull<Node<T>>) -> NonNull<Node<T>> {
            unsafe {
                if node.as_ref().right.is_some() {
                    self.find_max(node.as_ref().right.unwrap())
                } else {
                    node
                }
            }
        }

        unsafe fn find_min(&self, node: NonNull<Node<T>>) -> NonNull<Node<T>> {
            unsafe {
                if node.as_ref().left.is_some() {
                    self.find_min(node.as_ref().left.unwrap())
                } else {
                    node
                }
            }
        }

        unsafe fn remove_leaf(&mut self, node: NonNull<Node<T>>) -> bool {
            unsafe {
                if let Some(mut parent) = node.as_ref().parent {
                    if let Some(ref mut left) = parent.as_mut().left {
                        if std::ptr::eq(left.as_ptr(), node.as_ptr()) {
                            parent.as_mut().left = None;
                        } else {
                            parent.as_mut().right = None;
                        }
                    } else {
                        parent.as_mut().right = None;
                    }
                }
                let _ = Box::from_raw(node.as_ptr());
                true
            }
        }

        unsafe fn remove_tree(&mut self, node: Link<T>) {
            unsafe {
                if let Some(node) = node {
                    self.remove_tree(node.as_ref().left);
                    self.remove_tree(node.as_ref().right);
                    if self.remove_leaf(node) {
                        assert!(self.count > 0);
                        self.count -= 1;
                    }
                }
            }
        }

        /// Rotate left and Rotate right and Flip color
        ///
        /// ```text
        ///             |              ||
        ///             P              C
        ///           //  \          /   \
        ///          A     F        A     P
        ///         / \\      =>   / \   / \
        ///        B   C          B   E D   F
        ///            / \
        ///           E   D
        /// ```
        unsafe fn rotate_left_right_flip_color(&mut self, node_a: NonNull<Node<T>>) -> Link<T> {
            unsafe {
                let node_p = (*node_a.as_ptr()).parent.unwrap();
                if let Some(parent) = (*node_p.as_ptr()).parent {
                    let mut node_p_from_left = false;
                    if let Some(p_node_p) = (*parent.as_ptr()).left
                        && std::ptr::eq(p_node_p.as_ptr(), node_p.as_ptr())
                    {
                        node_p_from_left = true;
                    }
                    let node_p = if node_p_from_left {
                        (*parent.as_ptr()).left.unwrap()
                    } else {
                        (*parent.as_ptr()).right.unwrap()
                    };
                    (*node_a.as_ptr()).is_red = false;
                    let node_c = (*node_a.as_ptr()).right.unwrap();
                    (*node_c.as_ptr()).is_red = false;
                    let node_e = (*node_c.as_ptr()).left;
                    let node_d = (*node_c.as_ptr()).right;
                    (*node_c.as_ptr()).is_red = true;
                    (*node_c.as_ptr()).parent = Some(parent);
                    (*node_c.as_ptr()).left = Some(node_a);
                    (*node_a.as_ptr()).parent = Some(node_c);
                    (*node_c.as_ptr()).right = Some(node_p);
                    (*node_p.as_ptr()).parent = Some(node_c);
                    (*node_p.as_ptr()).left = node_d;
                    if let Some(n_d) = node_d {
                        (*n_d.as_ptr()).parent = Some(node_p);
                    }
                    (*node_a.as_ptr()).right = node_e;
                    if let Some(n_e) = node_e {
                        (*n_e.as_ptr()).parent = Some(node_a);
                    }
                    if node_p_from_left {
                        (*parent.as_ptr()).left = Some(node_c);
                    } else {
                        (*parent.as_ptr()).right = Some(node_c);
                    }
                    Some(parent)
                } else {
                    let fixed_head = self.fixed_head.unwrap();
                    let node_p = (*fixed_head.as_ptr()).left.unwrap();

                    let node_a = (*node_p.as_ptr()).left.unwrap();
                    (*node_a.as_ptr()).is_red = false;
                    let node_c = (*node_a.as_ptr()).right.unwrap();
                    (*node_c.as_ptr()).is_red = false;
                    let node_e = (*node_c.as_ptr()).left;
                    let node_d = (*node_c.as_ptr()).right;
                    (*node_c.as_ptr()).is_red = false;
                    (*node_c.as_ptr()).parent = None;
                    (*node_c.as_ptr()).left = Some(node_a);
                    (*node_a.as_ptr()).parent = Some(node_c);
                    (*node_c.as_ptr()).right = Some(node_p);
                    (*node_p.as_ptr()).parent = Some(node_c);
                    (*node_p.as_ptr()).left = node_d;
                    if let Some(n_d) = node_d {
                        (*n_d.as_ptr()).parent = Some(node_p);
                    }
                    (*node_a.as_ptr()).right = node_e;
                    if let Some(n_e) = node_e {
                        (*n_e.as_ptr()).parent = Some(node_a);
                    }
                    (*fixed_head.as_ptr()).left = Some(node_c);
                    (*fixed_head.as_ptr()).left
                }
            }
        }

        /// Rotate left without parent
        /// A is root
        ///
        /// ```text
        ///             A                  C
        ///            / \\               //\
        ///           B    C     =>      A   D
        ///               / \           / \
        ///              E   D         B   E
        ///
        /// ```
        ///
        /// Rotate left  with parent
        ///
        /// ```text
        ///             P                  P
        ///             |                  |
        ///             A                  C
        ///            / \\               //\
        ///           B    C     =>      A   D
        ///               / \           / \
        ///              E   D         B   E
        ///
        /// ```
        unsafe fn rotate_left(&mut self, node_a: NonNull<Node<T>>) -> Link<T> {
            unsafe {
                if let Some(parent) = (*node_a.as_ptr()).parent {
                    let mut node_a_from_left = false;
                    if let Some(p_node_a) = (*parent.as_ptr()).left
                        && std::ptr::eq(p_node_a.as_ptr(), node_a.as_ptr())
                    {
                        node_a_from_left = true;
                    }
                    let node_a = if node_a_from_left {
                        (*parent.as_ptr()).left.unwrap()
                    } else {
                        (*parent.as_ptr()).right.unwrap()
                    };
                    let node_c = (*node_a.as_ptr()).right.unwrap();
                    (*node_c.as_ptr()).is_red = (*node_a.as_ptr()).is_red;
                    (*node_a.as_ptr()).is_red = true;
                    (*node_c.as_ptr()).parent = Some(parent);
                    if node_c.as_ref().left.is_some() {
                        let mut node_e = (*node_c.as_ptr()).left;
                        if let Some(e) = &mut node_e {
                            (*e.as_ptr()).parent = Some(node_a);
                        }
                        (*node_a.as_ptr()).right = node_e;
                    } else {
                        (*node_a.as_ptr()).right = None;
                    }
                    (*node_c.as_ptr()).left = Some(node_a);
                    (*node_a.as_ptr()).parent = Some(node_c);
                    if node_a_from_left {
                        (*parent.as_ptr()).left = Some(node_c);
                        (*parent.as_ptr()).left
                    } else {
                        (*parent.as_ptr()).right = Some(node_c);
                        (*parent.as_ptr()).right
                    }
                } else {
                    let fixed_head = self.fixed_head.unwrap();
                    let node_a = (*fixed_head.as_ptr()).left.unwrap();
                    let node_c = (*node_a.as_ptr()).right.unwrap();
                    (*node_c.as_ptr()).is_red = (*node_a.as_ptr()).is_red;
                    (*node_a.as_ptr()).is_red = true;
                    if node_c.as_ref().left.is_some() {
                        let mut node_e = (*node_c.as_ptr()).left;
                        if let Some(e) = &mut node_e {
                            (*e.as_ptr()).parent = Some(node_a);
                        }
                        (*node_a.as_ptr()).right = node_e;
                    } else {
                        (*node_a.as_ptr()).right = None;
                    }
                    (*node_c.as_ptr()).left = Some(node_a);
                    (*node_a.as_ptr()).parent = Some(node_c);
                    (*node_c.as_ptr()).parent = None;
                    (*fixed_head.as_ptr()).left = Some(node_c);
                    (*fixed_head.as_ptr()).left
                }
            }
        }

        /// Rotate right without parent
        /// A is root
        ///
        /// ```text
        ///              A               B
        ///            // \             / \\
        ///            B   C     =>    E    A
        ///          // \                  / \
        ///         E    D                D   C
        /// ```
        ///
        /// Rotate right with parent
        ///
        /// ```text
        ///                P               P
        ///                |               |
        ///                A               B
        ///              // \             / \\
        ///              B   C     =>    E    A
        ///            // \                  / \
        ///           E    D                D   C
        ///
        /// ```
        unsafe fn rotate_right(&mut self, node_a: NonNull<Node<T>>) -> Link<T> {
            unsafe {
                if let Some(parent) = (*node_a.as_ptr()).parent {
                    let mut node_a_from_left = false;
                    if let Some(p_node_a) = (*parent.as_ptr()).left
                        && std::ptr::eq(p_node_a.as_ptr(), node_a.as_ptr())
                    {
                        node_a_from_left = true;
                    }
                    let node_a = if node_a_from_left {
                        (*parent.as_ptr()).left.unwrap()
                    } else {
                        (*parent.as_ptr()).right.unwrap()
                    };
                    let node_b = (*node_a.as_ptr()).left.unwrap();
                    (*node_b.as_ptr()).parent = Some(parent);
                    (*node_b.as_ptr()).is_red = (*node_a.as_ptr()).is_red;
                    (*node_a.as_ptr()).is_red = true;
                    if node_b.as_ref().right.is_some() {
                        let mut node_d = (*node_b.as_ptr()).right;
                        if let Some(d) = &mut node_d {
                            (*d.as_ptr()).parent = Some(node_a);
                        }
                        (*node_a.as_ptr()).left = node_d;
                    } else {
                        (*node_a.as_ptr()).left = None;
                    }
                    (*node_b.as_ptr()).right = Some(node_a);
                    (*node_a.as_ptr()).parent = Some(node_b);
                    if node_a_from_left {
                        (*parent.as_ptr()).left = Some(node_b);
                        (*parent.as_ptr()).left
                    } else {
                        (*parent.as_ptr()).right = Some(node_b);
                        (*parent.as_ptr()).right
                    }
                } else {
                    let fixed_head = self.fixed_head.unwrap();
                    let node_a = (*fixed_head.as_ptr()).left.unwrap();
                    let node_b = (*node_a.as_ptr()).left.unwrap();
                    (*node_b.as_ptr()).is_red = (*node_a.as_ptr()).is_red;
                    (*node_a.as_ptr()).is_red = true;
                    if node_b.as_ref().right.is_some() {
                        let mut node_d = (*node_b.as_ptr()).right;
                        if let Some(d) = &mut node_d {
                            (*d.as_ptr()).parent = Some(node_a);
                        }
                        (*node_a.as_ptr()).left = node_d;
                    } else {
                        (*node_a.as_ptr()).left = None;
                    }
                    (*node_b.as_ptr()).right = Some(node_a);
                    (*node_a.as_ptr()).parent = Some(node_b);
                    (*node_b.as_ptr()).parent = None;
                    (*fixed_head.as_ptr()).left = Some(node_b);
                    (*fixed_head.as_ptr()).left
                }
            }
        }

        /// Flip color
        ///
        /// ```text
        ///             |          ||
        ///             A    =>    A
        ///           // \\       / \
        ///          B    C      B   C
        ///
        /// ```
        unsafe fn flip_colors(&mut self, mut node: NonNull<Node<T>>) {
            unsafe {
                if node.as_ref().left.is_some() && node.as_ref().right.is_some() {
                    if let Some(ref mut left) = node.as_mut().left {
                        left.as_mut().is_red = false;
                    }
                    if let Some(ref mut right) = node.as_mut().right {
                        right.as_mut().is_red = false;
                    }
                    node.as_mut().is_red = true;

                    if (*node.as_ptr()).parent.is_none() {
                        node.as_mut().is_red = false;
                    }
                }
            }
        }

        unsafe fn check_put_balancing(&mut self, node: NonNull<Node<T>>) -> OperationPut {
            unsafe {
                if node.as_ref().right.is_some() {
                    let r = node.as_ref().right.unwrap();
                    if node.as_ref().left.is_some() && !node.as_ref().is_red {
                        let l = node.as_ref().left.unwrap();
                        if l.as_ref().is_red && r.as_ref().is_red {
                            return OperationPut::FlipColors;
                        }
                    }
                    if r.as_ref().is_red {
                        return OperationPut::Left;
                    }
                }
                if node.as_ref().is_red && node.as_ref().left.is_some() {
                    let l = node.as_ref().left.unwrap();
                    if l.as_ref().is_red {
                        return OperationPut::Right;
                    }
                }
                OperationPut::Nothing
            }
        }

        unsafe fn put_balancing(&mut self, next: NonNull<Node<T>>) {
            unsafe {
                let mut next = next;
                loop {
                    match self.check_put_balancing(next) {
                        OperationPut::Left => {
                            if (*next.as_ptr()).is_red {
                                if let Some(n) = self.rotate_left_right_flip_color(next) {
                                    next = n;
                                }
                            } else {
                                if let Some(n) = self.rotate_left(next) {
                                    next = n;
                                }
                            }
                        }
                        OperationPut::Right => {
                            if (*next.as_ptr()).parent.is_some() {
                                let node_a = (*next.as_ptr()).parent.unwrap();
                                if let Some(n) = self.rotate_right(node_a) {
                                    next = n;
                                }
                            }
                        }
                        OperationPut::FlipColors => {
                            self.flip_colors(next);
                            if (*next.as_ptr()).parent.is_some() {
                                next = (*next.as_ptr()).parent.unwrap();
                            } else {
                                break;
                            }
                        }
                        OperationPut::Nothing => {
                            break;
                        }
                    }
                }
            }
        }

        unsafe fn find_put_parent_candidate(&mut self, parent: Link<T>, elem: &T) -> Link<T> {
            unsafe {
                if let Some(parent) = parent {
                    match elem.cmp(&parent.as_ref().value) {
                        Ordering::Equal => None,
                        Ordering::Less => {
                            if (*parent.as_ptr()).left.is_some() {
                                self.find_put_parent_candidate((*parent.as_ptr()).left, elem)
                            } else {
                                Some(parent)
                            }
                        }
                        Ordering::Greater => {
                            if (*parent.as_ptr()).right.is_some() {
                                self.find_put_parent_candidate((*parent.as_ptr()).right, elem)
                            } else {
                                Some(parent)
                            }
                        }
                    }
                } else {
                    parent
                }
            }
        }

        unsafe fn attach_node(&mut self, parent: NonNull<Node<T>>, elem: T) -> bool {
            unsafe {
                match elem.cmp(&parent.as_ref().value) {
                    Ordering::Equal => false,
                    Ordering::Less => {
                        (*parent.as_ptr()).left = Node::new_red(elem, parent);
                        true
                    }
                    Ordering::Greater => {
                        (*parent.as_ptr()).right = Node::new_red(elem, parent);
                        true
                    }
                }
            }
        }

        /// Returns the default iterator over the tree.
        ///
        /// The default traversal is in-order (`LNR`), so values are yielded in
        /// ascending order.
        ///
        /// # Examples
        ///
        /// ```
        /// use rbtree_extended::RedBlackTree;
        ///
        /// let mut tree = RedBlackTree::new();
        /// tree.extend([3, 1, 2]);
        ///
        /// let values: Vec<i32> = tree.iter().copied().collect();
        ///
        /// assert_eq!(values, vec![1, 2, 3]);
        /// ```
        pub fn iter(&self) -> IterInOrder<'_, T> {
            self.iter_inorder()
        }

        /// Returns an in-order (`LNR`) iterator.
        ///
        /// Values are yielded in ascending order.
        ///
        /// # Examples
        ///
        /// ```
        /// use rbtree_extended::RedBlackTree;
        ///
        /// let mut tree = RedBlackTree::new();
        /// tree.extend([3, 1, 2]);
        ///
        /// assert_eq!(
        ///     tree.iter_inorder().copied().collect::<Vec<_>>(),
        ///     vec![1, 2, 3]
        /// );
        /// ```
        pub fn iter_inorder(&self) -> IterInOrder<'_, T> {
            unsafe { IterInOrder::new(leftmost_child_in_order(self.get_root()), self.count) }
        }

        /// Returns a reverse in-order (`RNL`) iterator.
        ///
        /// Values are yielded in descending order.
        ///
        /// # Examples
        ///
        /// ```
        /// use rbtree_extended::RedBlackTree;
        ///
        /// let mut tree = RedBlackTree::new();
        /// tree.extend([3, 1, 2]);
        ///
        /// assert_eq!(
        ///     tree.iter_reverse_inorder().copied().collect::<Vec<_>>(),
        ///     vec![3, 2, 1]
        /// );
        /// ```
        pub fn iter_reverse_inorder(&self) -> IterReverseInOrder<'_, T> {
            unsafe {
                IterReverseInOrder::new(
                    rightmost_child_reverse_in_order(self.get_root()),
                    self.count,
                )
            }
        }

        /// Returns a pre-order (`NLR`) iterator.
        ///
        /// The current node is visited before its left and right subtrees.
        ///
        /// # Examples
        ///
        /// ```
        /// use rbtree_extended::RedBlackTree;
        ///
        /// let mut tree = RedBlackTree::new();
        /// tree.extend([4, 2, 6, 1, 3, 5, 7]);
        ///
        /// let values: Vec<i32> = tree.iter_preorder().copied().collect();
        ///
        /// assert_eq!(values.len(), tree.len());
        /// ```
        pub fn iter_preorder(&self) -> IterPreOrder<'_, T> {
            IterPreOrder::new(self.get_root(), self.count)
        }

        /// Returns a post-order (`LRN`) iterator.
        ///
        /// The left and right subtrees are visited before the current node.
        ///
        /// # Examples
        ///
        /// ```
        /// use rbtree_extended::RedBlackTree;
        ///
        /// let mut tree = RedBlackTree::new();
        /// tree.extend([4, 2, 6, 1, 3, 5, 7]);
        ///
        /// let values: Vec<i32> = tree.iter_postorder().copied().collect();
        ///
        /// assert_eq!(values.len(), tree.len());
        /// ```
        pub fn iter_postorder(&self) -> IterPostOrder<'_, T> {
            unsafe {
                let first = self.get_root().map(|root| leaf_post_order(root));
                IterPostOrder::new(first, self.count)
            }
        }

        /// Returns all values in level-order traversal.
        ///
        /// Nodes are visited level by level, starting from the root.
        ///
        /// # Examples
        ///
        /// ```
        /// use rbtree_extended::RedBlackTree;
        ///
        /// let mut tree = RedBlackTree::new();
        /// tree.extend([4, 2, 6, 1, 3, 5, 7]);
        ///
        /// let values = tree.iter_level_order();
        ///
        /// assert_eq!(values.len(), tree.len());
        /// ```
        pub fn iter_level_order(&self) -> Vec<&T> {
            let mut ret: Vec<&T> = vec![];
            if let Some(root) = self.get_root() {
                iter_level_order_impl(root, &mut ret);
            }
            ret
        }
    }

    impl<'a, T: Ord + Default> std::iter::IntoIterator for &'a RedBlackTree<T> {
        type IntoIter = IterInOrder<'a, T>;
        type Item = &'a T;

        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }

    impl<T: Ord + Default> Drop for RedBlackTree<T> {
        fn drop(&mut self) {
            unsafe {
                if let Some(fixed_head) = self.fixed_head {
                    self.remove_tree((*fixed_head.as_ptr()).left);
                    let _ = Box::from_raw(fixed_head.as_ptr());
                }
                self.fixed_head = None;
            }
        }
    }

    fn find_node<T: Ord>(fromnode: Link<T>, value: &T) -> Link<T> {
        unsafe {
            if let Some(fromnode) = fromnode {
                match value.cmp(&(*fromnode.as_ptr()).value) {
                    Ordering::Equal => Some(fromnode),
                    Ordering::Less => find_node((*fromnode.as_ptr()).left, value),
                    Ordering::Greater => find_node((*fromnode.as_ptr()).right, value),
                }
            } else {
                fromnode
            }
        }
    }

    fn helper_display_tree<T: Display>(node: NonNull<Node<T>>) -> String {
        unsafe {
            let mut s: String = "".into();
            let color = if (*node.as_ptr()).is_red {
                "[color=\"red\"]"
            } else {
                "[color=\"black\"]"
            };

            if let Some(left) = (*node.as_ptr()).left {
                s.push_str(&format!(
                    "\t{n1}->{n2} {color1}; {n1} {color1}; {n2} {color2};\n",
                    n1 = (*node.as_ptr()).value,
                    n2 = (*left.as_ptr()).value,
                    color1 = color,
                    color2 = if (*left.as_ptr()).is_red {
                        "[color=\"red\"]"
                    } else {
                        "[color=\"black\"]"
                    },
                ));
                s.push_str(&helper_display_tree(left));
            } else if (*node.as_ptr()).right.is_some() {
                s.push_str(&format!(
                    "\t{n1}->node_null_{n1} [color=\"grey\"]; {n1} {color1};\n",
                    n1 = (*node.as_ptr()).value,
                    color1 = color
                ));
                s.push_str(&format!(
                    "\tnode_null_{n1}[label=\"null\"]\n",
                    n1 = (*node.as_ptr()).value
                ));
            }

            if let Some(right) = (*node.as_ptr()).right {
                s.push_str(&format!(
                    "\t{n1}->{n2} {color1}; {n1} {color1}; {n2} {color2};\n",
                    n1 = (*node.as_ptr()).value,
                    n2 = (*right.as_ptr()).value,
                    color2 = if (*right.as_ptr()).is_red {
                        "[color=\"red\"]"
                    } else {
                        "[color=\"black\"]"
                    },
                    color1 = color
                ));
                s.push_str(&helper_display_tree(right));
            } else {
                s.push_str(&format!(
                    "\t{n1}->node_null_{n1} [color=\"grey\"]; {n1} {color1};\n",
                    n1 = (*node.as_ptr()).value,
                    color1 = color
                ));
                s.push_str(&format!(
                    "\tnode_null_{}[label=\"null\"]\n",
                    (*node.as_ptr()).value
                ));
            }
            s
        }
    }

    impl<T: Ord + Default> Default for RedBlackTree<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T: Ord + Default + Clone> Clone for RedBlackTree<T> {
        fn clone(&self) -> Self {
            let mut new_list = Self::new();
            for value in self {
                new_list.insert(value.clone());
            }
            new_list
        }
    }

    impl<T: Ord + Default> Extend<T> for RedBlackTree<T> {
        fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
            for value in iter {
                self.insert(value);
            }
        }
    }

    impl<T: Ord + Default> FromIterator<T> for RedBlackTree<T> {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            let mut list = Self::new();
            list.extend(iter);
            list
        }
    }

    impl<T: Ord + Default + Debug> Debug for RedBlackTree<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_list().entries(self).finish()
        }
    }

    impl<T: Ord + Default> PartialEq for RedBlackTree<T> {
        fn eq(&self, other: &Self) -> bool {
            self.len() == other.len() && self.iter_inorder().eq(other.iter_inorder())
        }
    }

    impl<T: Ord + Default> Eq for RedBlackTree<T> {}

    impl<T: Ord + Default> PartialOrd for RedBlackTree<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl<T: Ord + Default> Ord for RedBlackTree<T> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.iter_inorder().cmp(other.iter_inorder())
        }
    }

    impl<T: Ord + Default + std::hash::Hash> std::hash::Hash for RedBlackTree<T> {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.len().hash(state);
            for item in self {
                item.hash(state);
            }
        }
    }
}