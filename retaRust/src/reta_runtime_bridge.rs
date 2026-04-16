use std::cell::RefCell;

use crate::reta_program_types::RetaRuntime;

std::thread_local! {
    static RUNTIME_OVERRIDE: RefCell<Option<RetaRuntime>> = RefCell::new(None);
}

pub fn with_runtime_override<T>(runtime: Option<RetaRuntime>, f: impl FnOnce() -> T) -> T {
    RUNTIME_OVERRIDE.with(|slot| {
        let previous = slot.replace(runtime);
        let result = f();
        let _ = slot.replace(previous);
        result
    })
}

pub fn terminal_width_override() -> Option<i64> {
    RUNTIME_OVERRIDE.with(|slot| {
        slot.borrow()
            .as_ref()
            .and_then(|runtime| runtime.terminal_width)
            .map(|width| width as i64)
            .filter(|width| *width > 0)
    })
}
