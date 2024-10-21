use std::sync::LazyLock;

use pyo3::{
    ffi::{c_str, PyImport_AddModule, PyModule_GetDict},
    prepare_freethreaded_python,
    types::{PyDict, PyDictMethods},
    Py, Python,
};

use crate::{err, info};

pub struct PyEnv {
    globals: Py<PyDict>,
    locals: Py<PyDict>,
}

impl PyEnv {
    pub fn build(virt_info: &str) -> Self {
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
            globals.bind(py).set_item("__virt__", virt_info).unwrap(); // Force to copy globals dict, otherwise drop one PyEnv will affect others
            let locals = unsafe {
                let mptr = globals.as_ptr();
                Py::from_owned_ptr(py, mptr)
            };
            let mut res = Self { globals, locals };
            res.run_code("import tester\ntester.ui.__init_sub_virt__(__virt__)\n");
            res
        })
    }
}

impl PyEnv {
    pub fn run_code(&mut self, code: &str) {
        // Python mat block the main thread... Use async outside plz...
        Python::with_gil(|py| {
            let globals = self.globals.bind(py);
            let locals = self.locals.bind(py);
            info!("Run code: ```\n{}\n```", code);
            let e = py.run_bound(code, Some(globals), Some(locals));
            if let Err(e) = e {
                err!("Run code error: {}", e);
            }
        });
    }
}
