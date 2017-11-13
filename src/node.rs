//! Abstract syntax tree node.

use std::fmt::Debug;
use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};

/// Combination trait required for all annotation types.
pub trait AnnotationType: Default + Debug + Clone + PartialEq {}
impl<A: Default + Debug + Clone + PartialEq> AnnotationType for A {}

/// Trait use to identify the type of annotation used with a type.
pub trait Annotated {
    /// The annotation type.
    type Annotation: AnnotationType;
}

/// Used to specify that a type has no annotation.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct EmptyAnnotation {}

#[macro_export]
macro_rules! annotate {
    ($t:tt, $anno:ty) => {
        impl $crate::node::Annotated for $t {
            type Annotation = $anno;
        }
    };
    ($t:tt) => {
        impl $crate::node::Annotated for $t {
            type Annotation = $crate::node::EmptyAnnotation;
        }
    }
}

/// Abstract syntax tree node structure for heterogeneous tree nodes.
///
/// Adds arbitrary tree node annotation to each node.
#[derive(Debug, Clone, PartialEq)]
pub struct Node<T> where T: Annotated {
    /// Tree node item.
    pub item: T,
    /// Tree node annotation.
    pub annotation: T::Annotation,
}
impl<T: Annotated> Node<T> {
    // /// Retrieves a reference to the tree node.
    // pub fn item(&self) -> &T { &self.item }
    // /// Retrieves a mutable reference to the tree node.
    // pub fn item_mut(&mut self) -> &mut T { &mut self.item }
    // /// Retreieves a reference to the tree node annotation.
    // pub fn annotation(&self) -> &T::Annotation { &self.annotation }
    // /// Retrieves a mutable reference to the tree node annotation.
    // pub fn annotation_mut(&mut self) -> &mut T::Annotation { &mut self.annotation }
    // /// Retrieves a tuple of references to both the tree node and its annotation.
    // pub fn elems(&self) -> (&T, &T::Annotation) {
    //     (&self.item, &self.annotation)
    // }
    // /// Retrieves a tuple of mutable references to both the tree node and its annotation.
    // pub fn elems_mut(&mut self) -> (&mut T, &mut T::Annotation) {
    //     (&mut self.item, &mut self.annotation)
    // }
    /// Creates a new tree node with default annotation.
    pub fn new(item: T) -> Node<T> {
        Node {
            item: item,
            annotation: T::Annotation::default(),
        }
    }
}

/// Wraps a node in a reference-counting pointer.
#[derive(Debug, PartialEq)]
pub struct PNode<T: Annotated> {
    node: Rc<RefCell<Node<T>>>,
}

impl<T: Annotated> PNode<T> {
    /// Creates a new node.
    pub fn new(item: T) -> PNode<T> {
        PNode {
            node: Rc::new(RefCell::new(Node::new(item))),
        }
    }
    /// Get a mutable reference to the underlying node.
    // pub fn get_mut(&mut self) -> ::std::result::Result<&mut Node<T>, String> {
    pub fn borrow_mut<'a>(&'a mut self) -> RefMut<'a, Node<T>> {
        self.node.borrow_mut()
        // Rc::get_mut(&mut self.node).ok_or("invalid AST node linkage".to_string())
    }
    /// Get an immutable reference to the underlying node
    pub fn borrow<'a>(&'a self) -> Ref<'a, Node<T>> {
        self.node.borrow()
    }

    /// Weak reference count of underlying node
    pub fn weak_count(&self) -> usize { Rc::weak_count(&self.node) }
    /// Strong reference count of underlying node
    pub fn strong_count(&self) -> usize { Rc::strong_count(&self.node) }
}

impl<T: Annotated> Clone for PNode<T> {
    fn clone(&self) -> PNode<T> {
        PNode { node: Rc::clone(&self.node) }
    }
}

// impl<T: Annotated> Deref for PNode<T> {
//     type Target = Node<T>;

//     fn deref(&self) -> &Node<T> {
//         self.node.deref().borrow().deref()
//     }
// }
