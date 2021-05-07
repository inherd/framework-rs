#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate maplit;

#[macro_use]
extern crate serde_derive;
extern crate regex;
extern crate serde;

pub use framework_detector::FrameworkDetector;
pub use framework_detector::FrameworkContainer;
pub use framework_detector::DetectFramework;
pub use framework_detector::SourceFile;

pub mod dependency;
pub mod facet;
pub mod framework_detector;
pub mod lang;
pub mod frameworks;
