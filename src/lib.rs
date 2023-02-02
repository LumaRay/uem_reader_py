use std::sync::{Arc, Mutex};

use pyo3::{prelude::*, exceptions::PyValueError};

use lazy_static::*;

#[allow(unused_imports)]
use uem_reader::{
    reader::{
        UemReader,
        UemReaderInternalTrait,
        usb::find_usb_readers as fur
    },
    commands::{
        UemCommandsTrait,
        reader::UemCommandsReaderTrait, 
        cards::{
            UemCommandsCardsTrait,
            UemActivateParameters,
            mifare::{
                UemCommandsCardsMifareTrait,
                classic::UemCommandsCardsMifareClassicTrait
            },
        },
    },
};

//static mut rdrs: Arc<Mutex<Option<Vec<Vec<UemReader>>>>> = Arc::new(Mutex::new(None));
lazy_static! {
    static ref UEM_READER_LISTS: Arc<Mutex<Vec<Vec<UemReader>>>> = Arc::new(Mutex::new(vec![]));
//    static ref my_mutex: Mutex<i32> = Mutex::new(0i32);
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyclass]
#[allow(dead_code)]
pub struct PyUemReader {
    lists_idx: usize,
    list_idx: usize,
}

#[pymethods]
impl PyUemReader {
    //fn get_reader_locked(&self) -> Result<UemReader, > {
    //    unsafe{uem_reader_lists[self.lists_idx][self.list_idx].lock()}
    //}
    pub fn open(&self) -> PyResult<()> {
        // let reader = unsafe{uem_reader_lists[self.lists_idx][self.list_idx].lock()};
        let rawrdrs = UEM_READER_LISTS.lock().unwrap();
        let reader = rawrdrs[self.lists_idx][self.list_idx].lock();
        if reader.is_err() { return Err(PyValueError::new_err("Failed to find the reader.")); }
        let mut reader = reader.unwrap();
        reader.open().map_err(|e| {PyValueError::new_err(format!("{e}. Please check device permissions."))})?;
        Ok(())
    }

    pub fn close(&self) -> PyResult<()> {
        // let reader = unsafe{uem_reader_lists[self.lists_idx][self.list_idx].lock()};
        let rawrdrs = UEM_READER_LISTS.lock().unwrap();
        let reader = rawrdrs[self.lists_idx][self.list_idx].lock();
        if reader.is_err() { return Err(PyValueError::new_err("Failed to find the reader.")); }
        let mut reader = reader.unwrap();
        reader.close().map_err(|e| {PyValueError::new_err(format!("{e}. Please check device permissions."))})?;
        Ok(())
    }

    pub fn send(&self, command: Vec<u8>) -> PyResult<Vec<u8>> {
        // let reader = unsafe{uem_reader_lists[self.lists_idx][self.list_idx].lock()};
        let rawrdrs = UEM_READER_LISTS.lock().unwrap();
        let reader = rawrdrs[self.lists_idx][self.list_idx].lock();
        if reader.is_err() { return Err(PyValueError::new_err("Failed to find the reader.")); }
        let mut reader = reader.unwrap();
        let res = reader.send(&command).map_err(|e| {PyValueError::new_err(format!("{e}."))})?;
        Ok(res)
    }
}

#[pyfunction]
fn find_usb_readers() -> PyResult<Vec<PyUemReader>> {
    Python::with_gil(|_py| {
        // Search system for USB readers
        let uem_readers = fur();

        // Quit if no readers found
        if uem_readers.is_empty() {
            return Ok(Vec::new());
        }
        
        // {
            let mut rawrdrs = UEM_READER_LISTS.lock().unwrap();
        //     rawrdrs.push(uem_readers);
        // }

        // //let uem_reader = uem_readers.get_mut(0);
        // let mut uem_reader = rdrs.lock().unwrap();
        // let mut uem_reader = uem_reader.get_mut(0).unwrap().get_mut(0).unwrap();

        let mut py_readers = Vec::<PyUemReader>::new();
        // let lists_idx = unsafe{uem_reader_lists.len()};
        let lists_idx = rawrdrs.len();
        for list_idx in 0..uem_readers.len() {
            py_readers.push(PyUemReader{lists_idx, list_idx});
        }
        
        // unsafe {
        //     uem_reader_lists.push(uem_readers);
        rawrdrs.push(uem_readers);
        // }

        Ok(py_readers)
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn uem_reader_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(find_usb_readers, m)?)?;
    Ok(())
}

/*#[pyfunction]
fn my_function() -> PyResult<MyStruct> {
    Python::with_gil(|_py| {
        //Py::new(py, MyStruct {})
        Ok(MyStruct{})
    })
}*/
