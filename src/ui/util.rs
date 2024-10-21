use std::sync::LazyLock;

use pyo3::pyfunction;

use crate::{info, util::util::rand_string};

/// To generate a base sock name inside main gui app.
#[doc(hidden)]
pub fn get_main_virt() -> &'static str {
    static VIRT: LazyLock<String> = LazyLock::new(|| {
        let res = String::from("ter") + &rand_string(4);
        info!("A python sub exec created with {}", res);
        res
    });
    VIRT.as_str()
}

// This will only be write once on init time, so no lock is needed.
static mut SUB_VIRT: String = String::new();

#[doc(hidden)]
#[pyfunction]
pub fn __init_sub_virt__(s: &str) {
    unsafe {
        SUB_VIRT = String::from(s);
    }
}

pub fn get_sub_virt() -> String {
    unsafe {
        #[allow(static_mut_refs)]
        return SUB_VIRT.clone();
    }
}
