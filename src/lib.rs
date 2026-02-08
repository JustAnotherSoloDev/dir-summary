use std::{fmt::format, path::PathBuf};

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;


mod modules{
    pub mod dir_stat;
}



#[pyfunction]
fn create_report(target_dir: &str) -> PyResult<String> {
    let res=modules::dir_stat::create_report_for_dir(target_dir);
    return match res {
        Ok(rep)=>Ok(rep),
        Err(e)=>Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error: {}", e)))
    }
}

#[pyfunction]
fn directory_summary(report_path:&str,target_dir: &str,) -> PyResult<String> {
    let res=modules::dir_stat::directory_summary(report_path,target_dir);
    return match res {
        Ok((total_size, total_files))=>Ok(format!("total files={} and size={}", total_files, total_size)),
        Err(e)=>Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error: {}", e)))
    }
}

#[pyfunction]
fn summary(report_path: &str) -> PyResult<String> {
    let report_path=PathBuf::from(report_path);
    let res=modules::dir_stat::summary(&report_path);
    return match res {
        Ok(rep)=>Ok(rep),
        Err(e)=>Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error: {}", e)))
    }
}


#[pymodule(name = "dirSummary")]
mod dirSummary {
    #[pymodule_export]
    use super::create_report;
    #[pymodule_export]
    use super::directory_summary;
    #[pymodule_export]
    use super::summary;
}