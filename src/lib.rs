use std::{ffi::CString, fmt::format, path::PathBuf};
use anyhow::{Result, anyhow};

mod modules {
    pub mod dir_stat;
}

#[unsafe(no_mangle)]
pub extern "C" fn create_report(target_dir: *const i8) -> *mut i8 {
    let target_dir = convert_to_string(target_dir);
    if target_dir.is_err() {
        let return_value=format!("Error:{:?}", target_dir.err());
        return get_output_c_string(return_value).into_raw();
        
    }
    let target_dir = target_dir.unwrap();
    let res = modules::dir_stat::create_report_for_dir(target_dir.as_str());
    return match res {
        Ok(rep) => get_output_c_string(rep).into_raw(),
        Err(e) => return get_output_c_string(format!("Error:{:?}", e)).into_raw(),
    };
}

#[unsafe(no_mangle)]
pub extern "C" fn directory_summary(report_path: *const i8, target_dir: *const i8) -> *mut i8 {
    let target_dir = convert_to_string(target_dir);
    let report_path = convert_to_string(report_path);
    if target_dir.is_err() || report_path.is_err() {
        let err=format!("Error: Invalid target directory or report path");
        return get_output_c_string(err).into_raw();
    }
    let target_dir = target_dir.unwrap();
    let report_path = report_path.unwrap();
    let res = modules::dir_stat::directory_summary(report_path.as_str(), target_dir.as_str());
    return match res {
        Ok((total_size, total_files)) => {
            let value=format!("total files={} and size={}", total_files, total_size);
            return get_output_c_string(value).into_raw();
        }
        Err(e) => get_output_c_string(format!("Error: {}", e)).into_raw(),
    };
}

#[unsafe(no_mangle)]
pub extern "C" fn summary(report_path: *const i8) -> *mut i8 {
    let report_path = convert_to_string(report_path);
    if report_path.is_err() {
        let value= format!("Error: Invalid report path");
        return get_output_c_string(value).into_raw();
    }
    let report_path = report_path.unwrap();
    let report_path = PathBuf::from(report_path);
    let res = modules::dir_stat::summary(&report_path);
    return match res {
        Ok(rep) => get_output_c_string(rep).into_raw(),
        Err(e) => get_output_c_string(format!("Error: {}", e)).into_raw(),
    };
}

fn is_valid_ptr(ptr: *const i8) -> bool {
    if ptr.is_null() {
        return false;
    }
    return true;
}

fn convert_to_string(value_ptr: *const i8) -> Result<String> {
    if !is_valid_ptr(value_ptr) {
        return Err(anyhow!("string pointer is null or invalid"));
    }
    let c_string = unsafe { std::ffi::CStr::from_ptr(value_ptr) };
    let value = c_string.to_string_lossy().to_string();
    return Ok(value);
}


fn get_output_c_string(value: String)-> CString{
    let res=CString::new(value);
    if res.is_err(){
       let value= CString::new("The output was null string").unwrap();
       return value;
    }
    let value=res.unwrap();
    return value
}