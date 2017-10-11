//! Abstract syntax tree node.

/// Abstract syntax tree node structure for heterogeneous tree nodes.
///
/// Adds arbitrary tree node annotation to each node.
#[derive(Debug, Clone, PartialEq)]
pub struct Node<T, A: Default> {
    item: T,
    annotation: A,
}
impl<T, A: Default> Node<T, A> {
    /// Retrieves a reference to the tree node.
    pub fn item(&self) -> &T { &self.item }
    /// Retrieves a mutable reference to the tree node.
    pub fn item_mut(&mut self) -> &mut T { &mut self.item }
    /// Retreieves a reference to the tree node annotation.
    pub fn annotation(&self) -> &A { &self.annotation }
    /// Retrieves a mutable reference to the tree node annotation.
    pub fn annotation_mut(&mut self) -> &mut A { &mut self.annotation }
    /// Retrieves a tuple of references to both the tree node and its annotation.
    pub fn elems(&self) -> (&T, &A) {
        (&self.item, &self.annotation)
    }
    /// Retrieves a tuple of mutable references to both the tree node and its annotation.
    pub fn elems_mut(&mut self) -> (&mut T, &mut A) {
        (&mut self.item, &mut self.annotation)
    }
    /// Creates a new tree node with default annotation.
    pub fn new(item: T) -> Node<T, A> {
        Node {
            item: item,
            annotation: A::default(),
        }
    }
}
