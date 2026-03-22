use std::cell::RefCell;
use std::env;
use std::rc::Rc;

use crate::py_runtime::*;

/* =========================================
 * Python: import tables, types, utils, i18n
 * → in Rust: explizite Modul-Init-Funktionen
 * ========================================= */

fn import_tables(frame: &mut PyFrame) {
    // PLACEHOLDER: tables.py wird hier später
    // 1:1 in Runtime-Funktionen übersetzt
}

fn import_types(frame: &mut PyFrame) {
    // PLACEHOLDER
}

fn import_utils(frame: &mut PyFrame) {
    // PLACEHOLDER
}

fn import_i18n(frame: &mut PyFrame) {
    // PLACEHOLDER
}

/* =========================================
 * Python: def main(): ...
 * ========================================= */

fn py_main(_args: Vec<PyValue>, frame: &mut PyFrame) -> Result<PyValue, PyValue> {
    // Python-Semantik:
    // alles darf Exception werfen
    // Rückgabe kann None sein

    // >>> HIER kommt später der 1:1-Code aus center.py rein <<<

    Ok(PyValue::None)
}

/* =========================================
 * Python: if __name__ == "__main__"
 * ========================================= */

pub fn run_center_module() -> Result<PyValue, PyValue> {
    let globals = Rc::new(RefCell::new(PyDict::new()));
    let locals = Rc::new(RefCell::new(PyDict::new()));
    let mut frame = PyFrame { globals, locals };

    /* -----------------------------
     * sys.argv exakt wie Python
     * ----------------------------- */
    let argv: Vec<PyValue> = env::args()
        .map(|s| PyValue::Str(s))
        .collect();

    frame
        .globals
        .borrow_mut()
        .set("argv".to_string(), PyValue::List(Rc::new(RefCell::new(argv))));

    /* -----------------------------
     * Import-Side-Effects
     * ----------------------------- */
    import_tables(&mut frame);
    import_types(&mut frame);
    import_utils(&mut frame);
    import_i18n(&mut frame);

    /* -----------------------------
     * main-Funktion registrieren
     * ----------------------------- */
    let main_fn = PyFunction {
        name: "main".to_string(),
        func: py_main,
    };

    frame.globals.borrow_mut().set(
        "main".to_string(),
        PyValue::Function(Rc::new(main_fn)),
    );

    /* -----------------------------
     * Python-Aufruf: main()
     * ----------------------------- */
    let f = frame.globals.borrow().get("main");
    match f {
        PyValue::Function(func) => func.call(Vec::new(), &mut frame),
        _ => Err(PyValue::Exception(Rc::new(PyException {
            name: "RuntimeError".to_string(),
            message: "main is not callable".to_string(),
        }))),
    }
}

/* =========================================
 * Rust main → Python main
 * ========================================= */

fn main() {
    match run_center_module() {
        Ok(_) => {}
        Err(PyValue::Exception(e)) => {
            eprintln!("Uncaught exception: {:?}", e);
            std::process::exit(1);
        }
        Err(_) => {
            eprintln!("Unknown error");
            std::process::exit(1);
        }
    }
}
