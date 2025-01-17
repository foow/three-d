//!
//! A collection of light types.
//! Currently implemented light types are ambient light, directional light, spot light and point light.
//! Directional and spot lights can cast shadows.
//!

#[doc(hidden)]
pub mod directional_light;
#[doc(inline)]
pub use crate::directional_light::*;

#[doc(hidden)]
pub mod spot_light;
#[doc(inline)]
pub use crate::spot_light::*;

#[doc(hidden)]
pub mod point_light;
#[doc(inline)]
pub use crate::point_light::*;

#[doc(hidden)]
pub mod ambient_light;
#[doc(inline)]
pub use crate::ambient_light::*;
