//!
//! A collection of objects that can be rendered, for example a mesh.
//!

#[doc(hidden)]
pub mod mesh;
#[doc(inline)]
pub use crate::mesh::*;

#[doc(hidden)]
pub mod instanced_mesh;
#[doc(inline)]
pub use crate::instanced_mesh::*;

#[doc(hidden)]
pub mod skybox;
#[doc(inline)]
pub use crate::skybox::*;

#[doc(hidden)]
pub mod imposters;
#[doc(inline)]
pub use crate::imposters::*;

#[doc(hidden)]
pub mod particles;
#[doc(inline)]
pub use crate::particles::*;

#[doc(hidden)]
pub mod axes;
#[doc(inline)]
pub use crate::axes::*;
