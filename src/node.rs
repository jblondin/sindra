//! Abstract syntax tree node.

use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;

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
    pub annotation: Rc<RefCell<T::Annotation>>,
}
impl<T: Annotated> Node<T> {
    /// Creates a new tree node with default annotation.
    pub fn new(item: T) -> Node<T> {
        Node {
            item: item,
            annotation: Rc::new(RefCell::new(T::Annotation::default())),
        }
    }
}
