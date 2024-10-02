use std::sync::LazyLock;

use pyo3::{types::PyAnyMethods, Python};

use crate::util::util::rand_string;

pub fn get_main_virt() -> &'static str {
    static VIRT: LazyLock<String> =
        LazyLock::new(|| String::from("tester") + &String::from_utf8_lossy(&rand_string(8)));
    VIRT.as_str()
}

pub fn get_sub_virt() -> String {
    Python::with_gil(|py| {
        // virt is in python's __virt__ variable
        let globals = py.eval_bound("globals()", None, None).unwrap();
        let virt = globals.get_item("__virt__");
        if let Ok(virt) = virt {
            virt.extract().unwrap()
        } else {
            "NoVirt".to_string()
        }
    })
}
