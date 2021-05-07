#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub use framework_detector::DetectFramework;
pub use framework_detector::FrameworkContainer;
pub use framework_detector::FrameworkDetector;
pub use framework_detector::SourceFile;
pub use frameworks::Framework;

pub mod dependency;
pub mod facet;
pub mod framework_detector;
pub mod frameworks;
pub mod lang;
