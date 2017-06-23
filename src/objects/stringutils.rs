use std::borrow::Cow;

use err::PyResult;
use pointer::PyObject;
use objects::{PyInstance, PyString};
use objectprotocol::ObjectProtocol;
use python::Python;
use conversion::{ToPyObject, IntoPyObject, RefFromPyObject};

/// Converts Rust `str` to Python object.
/// See `PyString::new` for details on the conversion.
impl ToPyObject for str {
    #[inline]
    fn to_object(&self, py: Python) -> PyObject {
        PyString::new(py, self).into()
    }
}
impl<'a> IntoPyObject for &'a str {
    #[inline]
    fn into_object(self, py: Python) -> PyObject {
        PyString::new(py, self).into()
    }
}

/// Converts Rust `Cow<str>` to Python object.
/// See `PyString::new` for details on the conversion.
impl<'a> ToPyObject for Cow<'a, str> {
    #[inline]
    fn to_object(&self, py: Python) -> PyObject {
        PyString::new(py, self).into()
    }
}

/// Converts Rust `String` to Python object.
/// See `PyString::new` for details on the conversion.
impl ToPyObject for String {
    #[inline]
    fn to_object(&self, py: Python) -> PyObject {
        PyString::new(py, self).into()
    }
}
impl IntoPyObject for String {
    #[inline]
    fn into_object(self, py: Python) -> PyObject {
        PyString::new(py, &self).into()
    }
}
impl<'a> IntoPyObject for &'a String {
    #[inline]
    fn into_object(self, py: Python) -> PyObject {
        PyString::new(py, self).into()
    }
}

/// Allows extracting strings from Python objects.
/// Accepts Python `str` and `unicode` objects.
impl<'source> ::FromPyObject<'source> for Cow<'source, str>
{
    fn extract(ob: &'source PyInstance) -> PyResult<Self>
    {
        try!(ob.cast_as::<PyString>()).to_string()
    }
}

/// Allows extracting strings from Python objects.
/// Accepts Python `str` and `unicode` objects.
pyobject_extract!(py, obj to String => {
    let s = try!(obj.cast_as::<PyString>());
    s.to_string().map(Cow::into_owned)
});

impl RefFromPyObject for str {
    fn with_extracted<F, R>(obj: &PyInstance, f: F) -> PyResult<R>
        where F: FnOnce(&str) -> R
    {
        let s = try!(obj.extract::<Cow<str>>());
        Ok(f(&s))
    }
}
