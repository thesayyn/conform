use autocxx::prelude::*; // use all the main autocxx functions

include_cpp! {
    #include "gen.h"
    safety!(unsafe)
    generate!("extract_suite")
    generate!("Case")
}

pub use ffi::*;