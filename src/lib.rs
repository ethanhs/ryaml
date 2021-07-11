use std::io::{Read, Write};

use pyo3::create_exception;
use pyo3::types::PyList;
use pyo3::wrap_pyfunction;
use pyo3::Python;
use pyo3::{exceptions::PyValueError, prelude::*};
use pyo3_file::PyFileLikeObject;
use pythonize::{depythonize, pythonize};
use serde::Deserialize;
use serde_yaml::Value;

create_exception!(ryaml, InvalidYamlError, PyValueError);

fn read_file(file: PyObject) -> PyResult<String> {
    match PyFileLikeObject::with_requirements(file, true, false, false) {
        Ok(mut f) => {
            let mut str = String::new();
            f.read_to_string(&mut str)?;
            Ok(str)
        }
        Err(_) => Err(PyValueError::new_err(
            "Argument 1 not a readable file-like object.",
        )),
    }
}

fn write_file(file: PyObject, str: String) -> PyResult<()> {
    match PyFileLikeObject::with_requirements(file, false, true, false) {
        Ok(mut f) => Ok(f.write_all(str.as_bytes())?),
        Err(_) => Err(PyValueError::new_err(
            "Argument 1 not a writable file-like object.",
        )),
    }
}

fn deserialize_yaml(str: String) -> PyResult<Value> {
    match serde_yaml::from_str(&str) {
        Ok(val) => Ok(val),
        Err(err) => Err(InvalidYamlError::new_err(err.to_string())),
    }
}

fn deserialize_all_yaml(str: String) -> PyResult<Vec<Value>> {
    let mut documents = vec![];
    for document in serde_yaml::Deserializer::from_str(&str) {
        match Value::deserialize(document) {
            Ok(value) => documents.push(value),
            Err(e) => return Err(InvalidYamlError::new_err(e.to_string())),
        }
    }
    Ok(documents)
}

fn serialize_yaml(yaml: &Value) -> PyResult<String> {
    match serde_yaml::to_string(&yaml) {
        Ok(s) => Ok(s),
        Err(err) => Err(PyValueError::new_err(err.to_string())),
    }
}

fn yaml_to_pyobject(py: Python, yaml: &Value) -> PyResult<PyObject> {
    match pythonize(py, yaml) {
        Ok(obj) => Ok(obj),
        Err(err) => Err(PyValueError::new_err(err.to_string())),
    }
}

fn pyobject_to_yaml(obj: &PyAny) -> PyResult<Value> {
    match depythonize(obj) {
        Ok(obj) => Ok(obj),
        Err(err) => Err(PyValueError::new_err(err.to_string())),
    }
}

#[pyfunction]
fn load(py: Python, file: PyObject) -> PyResult<PyObject> {
    let str = read_file(file)?;
    loads(py, str)
}

#[pyfunction]
fn load_all(py: Python, file: PyObject) -> PyResult<PyObject> {
    let str = read_file(file)?;
    loads_all(py, str)
}

#[pyfunction]
fn dump(file: PyObject, obj: &PyAny) -> PyResult<()> {
    let str = dumps(obj)?;
    write_file(file, str)
}

#[pyfunction]
fn loads(py: Python, str: String) -> PyResult<PyObject> {
    if str.is_empty() {
        Ok(Python::None(py))
    } else {
        let value = deserialize_yaml(str)?;
        yaml_to_pyobject(py, &value)
    }
}

#[pyfunction]
fn loads_all(py: Python, str: String) -> PyResult<PyObject> {
    if str.is_empty() {
        Ok(Python::None(py))
    } else {
        let documents = deserialize_all_yaml(str)?;
        let mut pydocs = vec![];
        for doc in documents {
            pydocs.push(yaml_to_pyobject(py, &doc)?);
        }
        Ok(PyList::new(py, pydocs).to_object(py))
    }
}

#[pyfunction]
fn dumps(obj: &PyAny) -> PyResult<String> {
    let yaml = pyobject_to_yaml(obj)?;
    serialize_yaml(&yaml)
}

#[pymodule]
fn ryaml(py: Python, m: &PyModule) -> PyResult<()> {
    m.add("InvalidYamlError", py.get_type::<InvalidYamlError>())?;
    m.add_function(wrap_pyfunction!(load, m)?)?;
    m.add_function(wrap_pyfunction!(load_all, m)?)?;
    m.add_function(wrap_pyfunction!(loads, m)?)?;
    m.add_function(wrap_pyfunction!(loads_all, m)?)?;
    m.add_function(wrap_pyfunction!(dump, m)?)?;
    m.add_function(wrap_pyfunction!(dumps, m)?)?;

    Ok(())
}
