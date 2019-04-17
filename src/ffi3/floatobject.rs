use crate::ffi3::object::*;
use std::os::raw::{c_double, c_int};

#[cfg(not(Py_LIMITED_API))]
#[repr(C)]
pub struct PyFloatObject {
    pub ob_base: PyObject,
    pub ob_fval: c_double,
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub static mut PyFloat_Type: PyTypeObject;
}

#[inline]
pub unsafe fn PyFloat_Check(op: *mut PyObject) -> c_int {
    PyObject_TypeCheck(op, &mut PyFloat_Type)
}

#[inline]
pub unsafe fn PyFloat_CheckExact(op: *mut PyObject) -> c_int {
    (Py_TYPE(op) == &mut PyFloat_Type) as c_int
}

#[cfg(not(Py_LIMITED_API))]
#[inline]
pub unsafe fn PyFloat_AS_DOUBLE(op: *mut PyObject) -> c_double {
    (*(op as *mut PyFloatObject)).ob_fval
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub fn PyFloat_GetMax() -> c_double;
    pub fn PyFloat_GetMin() -> c_double;
    pub fn PyFloat_GetInfo() -> *mut PyObject;
    pub fn PyFloat_FromString(arg1: *mut PyObject) -> *mut PyObject;
    pub fn PyFloat_FromDouble(arg1: c_double) -> *mut PyObject;
    pub fn PyFloat_AsDouble(arg1: *mut PyObject) -> c_double;
}
