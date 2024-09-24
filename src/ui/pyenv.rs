use std::sync::LazyLock;

use pyo3::{
    ffi::{c_str, PyImport_AddModule, PyModule_GetDict},
    prepare_freethreaded_python,
    types::{PyDict, PyDictMethods}, Py, Python,
};

use crate::err;

pub struct PyEnv {
    globals: Py<PyDict>,
    locals: Py<PyDict>,
}

impl Default for PyEnv {
    fn default() -> Self {
        static GLOBALS: LazyLock<Py<PyDict>> = LazyLock::new(|| {
            prepare_freethreaded_python();
            Python::with_gil(|py| unsafe {
                let mptr = PyImport_AddModule(c_str!("__main__").as_ptr());
                if mptr.is_null() {
                    panic!("Failed to get __main__ module");
                }
                let globals = PyModule_GetDict(mptr);
                Py::from_owned_ptr(py, globals)
            })
        });
        prepare_freethreaded_python();
        Python::with_gil(|py| {
            let globals = GLOBALS.clone_ref(py);
            globals.bind(py).set_item("__virt__", 1).unwrap(); // Force to copy globals dict, otherwise drop one PyEnv will affect others
            let locals = unsafe {
                let mptr = globals.as_ptr();
                Py::from_owned_ptr(py, mptr)
            };
            Self { globals, locals }
        })
    }
}

impl PyEnv {
    pub fn run_code(&mut self, code: &str) {
        Python::with_gil(|py| {
            let globals = self.globals.bind(py);
            let locals = self.locals.bind(py);
            let e = py.run_bound(code, Some(globals), Some(locals));
            if let Err(e) = e {
                err!("Run code error: {}", e);
            }
        });
    }
}
