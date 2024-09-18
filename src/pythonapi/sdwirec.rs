use pyo3::{exceptions::PyRuntimeError, pyclass, pymethods, PyResult};
use serde::Deserialize;

use crate::{devhost::sdwirec::{SdwirecChooser, SdwirecProd, SdwirecStat}, info};

#[derive(Deserialize)]
pub struct SdWirecConf {
    pub id: Option<u16>,
    pub serial: Option<String>,
    pub vendor: Option<u16>,
    pub product: Option<u16>,
}

#[pyclass]
pub struct SdWirec {
    pub inner: SdwirecProd,
}

#[pymethods]
impl SdWirec {
    #[new]
    fn py_new(conf: &str) -> PyResult<Self> {
        info!("SdWireC got config: {}", conf);
        let conf: SdWirecConf = toml::from_str(conf).unwrap();
        let chooser = if let Some(id) = conf.id {
            SdwirecChooser::Id(id)
        } else if let Some(serial) = conf.serial {
            SdwirecChooser::Serial(serial)
        } else if let Some(vendor) = conf.vendor {
            SdwirecChooser::Vendor(vendor)
        } else if let Some(product) = conf.product {
            SdwirecChooser::Product(product)
        } else {
            return Err(PyRuntimeError::new_err("Invalid chooser"));
        };
        Ok(SdWirec {
            inner: SdwirecProd::new(chooser),
        })
    }

    fn get_stat(&self) -> PyResult<String> {
        let res = self
            .inner
            .get_stat()
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        let res = match res {
            SdwirecStat::TS => "TS",
            SdwirecStat::DUT => "DUT",
        };
        Ok(res.to_string())
    }
    fn to_ts(&self) -> PyResult<()> {
        self.inner
            .to_ts()
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
    fn to_dut(&self) -> PyResult<()> {
        self.inner
            .to_dut()
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
    fn set_to(&self, stat: &str) -> PyResult<()> {
        let stat = match stat {
            "TS" => SdwirecStat::TS,
            "DUT" => SdwirecStat::DUT,
            _ => return Err(PyRuntimeError::new_err("Invalid stat")),
        };
        self.inner
            .set_to(&stat)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
}
