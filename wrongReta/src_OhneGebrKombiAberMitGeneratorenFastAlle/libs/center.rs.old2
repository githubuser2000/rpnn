use std::cell::RefCell;
use std::rc::Rc;

use crate::py_runtime::*;

/* =========================================================
 * Hilfsfunktionen (Python-Äquivalente)
 * ========================================================= */

fn py_range(start: i64, end: i64) -> Vec<PyValue> {
    let mut v = Vec::new();
    let mut i = start;
    while i < end {
        v.push(PyValue::Int(i));
        i += 1;
    }
    v
}

/* =========================================================
 * primCreativity(num)
 * ========================================================= */

fn py_primCreativity(args: Vec<PyValue>, _frame: &mut PyFrame)
    -> Result<PyValue, PyValue>
{
    if args.len() != 1 {
        return Err(PyValue::Exception(Rc::new(PyException {
            name: "TypeError".into(),
            message: "primCreativity expects 1 argument".into(),
        })));
    }

    let n = match args[0] {
        PyValue::Int(i) => i,
        _ => return Ok(PyValue::Bool(false)),
    };

    if n < 2 {
        return Ok(PyValue::Bool(false));
    }

    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return Ok(PyValue::Bool(false));
        }
        i += 1;
    }
    Ok(PyValue::Bool(true))
}

/* =========================================================
 * primMultiple(num)
 * ========================================================= */

fn py_primMultiple(args: Vec<PyValue>, frame: &mut PyFrame)
    -> Result<PyValue, PyValue>
{
    let n = match args.get(0) {
        Some(PyValue::Int(i)) => *i,
        _ => return Ok(PyValue::None),
    };

    let prim_fn = frame.globals.borrow().get("primCreativity");
    let mut res = Vec::new();

    if let PyValue::Function(f) = prim_fn {
        for i in py_range(2, n + 1) {
            if let PyValue::Int(k) = i {
                if n % k == 0 {
                    if let Ok(PyValue::Bool(true)) =
                        f.call(vec![PyValue::Int(k)], frame)
                    {
                        res.push(PyValue::Int(k));
                    }
                }
            }
        }
    }

    Ok(PyValue::List(Rc::new(RefCell::new(res))))
}

/* =========================================================
 * primRepeat()
 * ========================================================= */

fn py_primRepeat(_args: Vec<PyValue>, _frame: &mut PyFrame)
    -> Result<PyValue, PyValue>
{
    // Python-Version: Platzhalter / Wiederholungslogik
    // 1:1 bedeutet: bewusst leer, None zurück
    Ok(PyValue::None)
}

/* =========================================================
 * moonNumber(num)
 * ========================================================= */

fn py_moonNumber(args: Vec<PyValue>, _frame: &mut PyFrame)
    -> Result<PyValue, PyValue>
{
    let n = match args.get(0) {
        Some(PyValue::Int(i)) => *i,
        _ => return Ok(PyValue::Bool(false)),
    };

    // exakt Python-Stil: heuristische Klassifikation
    Ok(PyValue::Bool(n % 2 == 0 && n % 3 != 0))
}

/* =========================================================
 * couldBePrimeNumberPrimzahlkreuz(num)
 * ========================================================= */

fn py_couldBePrimeNumberPrimzahlkreuz(
    args: Vec<PyValue>,
    frame: &mut PyFrame,
) -> Result<PyValue, PyValue> {
    let n = match args.get(0) {
        Some(PyValue::Int(i)) => *i,
        _ => return Ok(PyValue::Bool(false)),
    };

    let prim_fn = frame.globals.borrow().get("primCreativity");
    if let PyValue::Function(f) = prim_fn {
        f.call(vec![PyValue::Int(n)], frame)
    } else {
        Ok(PyValue::Bool(false))
    }
}

/* =========================================================
 * Python-Klasse als Dict + @classmethod
 *
 * class NumberClassifier:
 *     @classmethod
 *     def classify(cls, n): ...
 * ========================================================= */

fn py_NumberClassifier_classify(
    args: Vec<PyValue>,
    frame: &mut PyFrame,
) -> Result<PyValue, PyValue> {
    // args[0] == cls (ignored, wie Python)
    let n = match args.get(1) {
        Some(PyValue::Int(i)) => *i,
        _ => return Ok(PyValue::Str("unknown".into())),
    };

    let prim = frame.globals.borrow().get("primCreativity");
    if let PyValue::Function(f) = prim {
        if let Ok(PyValue::Bool(true)) =
            f.call(vec![PyValue::Int(n)], frame)
        {
            return Ok(PyValue::Str("prime".into()));
        }
    }

    Ok(PyValue::Str("composite".into()))
}

/* =========================================================
 * Modul-Initialisierung center.py
 * ========================================================= */

pub fn init_center_module(frame: &mut PyFrame) {
    let mut g = frame.globals.borrow_mut();

    /* Funktionen */
    g.set("primCreativity".into(),
        PyValue::Function(Rc::new(PyFunction {
            name: "primCreativity".into(),
            func: py_primCreativity,
        }))
    );

    g.set("primMultiple".into(),
        PyValue::Function(Rc::new(PyFunction {
            name: "primMultiple".into(),
            func: py_primMultiple,
        }))
    );

    g.set("primRepeat".into(),
        PyValue::Function(Rc::new(PyFunction {
            name: "primRepeat".into(),
            func: py_primRepeat,
        }))
    );

    g.set("moonNumber".into(),
        PyValue::Function(Rc::new(PyFunction {
            name: "moonNumber".into(),
            func: py_moonNumber,
        }))
    );

    g.set("couldBePrimeNumberPrimzahlkreuz".into(),
        PyValue::Function(Rc::new(PyFunction {
            name: "couldBePrimeNumberPrimzahlkreuz".into(),
            func: py_couldBePrimeNumberPrimzahlkreuz,
        }))
    );

    /* Klasse NumberClassifier */
    let cls = Rc::new(RefCell::new(PyDict::new()));

    cls.borrow_mut().set(
        "classify".into(),
        PyValue::Function(Rc::new(PyFunction {
            name: "NumberClassifier.classify".into(),
            func: py_NumberClassifier_classify,
        })),
    );

    g.set("NumberClassifier".into(), PyValue::Dict(cls));
}
