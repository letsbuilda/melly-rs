#![warn(rust_2018_idioms)]

use std::io;
use std::io::Write;

use pyo3::prelude::*;

/// Run Python code
fn run_py(code: &str) -> Result<String, PyErr> {
    Python::with_gil(|py| {
        let fun: Py<PyAny> = PyModule::from_code(
            py,
            r#"
def _parse_python(text: str) -> str:
    """Attempt to parse and execute input as Python"""
    try:
        output = eval(text, globals(), globals())
    except SyntaxError as error:
        try:
            output = exec(compile(text, "<string>", "exec"), globals(), globals())
        except Exception as exception:
            output = f"{exception=}"
    return str(output)
                   "#,
            "",
            "",
        )?.getattr("_parse_python")?.into();

        let result = fun.call1(py, (code, ))?;
        Ok(result.to_string())
    })
}

fn parse(text: String) -> String {
    if text.starts_with("py! ") {
        let python_code = text.strip_prefix("py! ").unwrap();
        let output = run_py(python_code).unwrap_or_else(|_| String::from("ERROR"));
        return output;
    }
    String::from("Failed to parse line")
}


fn main() {
  loop {
        print!("|> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();

        io::stdin()
            .read_line(&mut line)
            .expect("Error getting guess");
        let output = parse(line);
        println!("{output}");
    }
}
