use pyo3::{create_exception, exceptions::PyException};

create_exception!(akatsuki_pp_py, ArgsError, PyException);
create_exception!(akatsuki_pp_py, ParseError, PyException);
create_exception!(akatsuki_pp_py, ConvertError, PyException);
