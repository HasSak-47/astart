use std::{any::Any, ffi::c_void};

use crate::{AStart, Node, World};

type Start         = unsafe extern "C" fn (*mut c_void) -> usize;
type GetNeightbors = unsafe extern "C" fn (*mut c_void, usize, *mut f64) -> bool;
type Heuristic     = unsafe extern "C" fn (*mut c_void, usize) -> f64;
type IsEnd         = unsafe extern "C" fn (*mut c_void, usize) -> bool;
type Reset         = unsafe extern "C" fn (*mut c_void);

#[derive(Debug)]
#[repr(C)]
pub struct FFI{
    obj           : *mut c_void,
    start         : Start,
    get_neightbors: GetNeightbors,
    heuristic     : Heuristic,
    is_end        : IsEnd,
    reset         : Reset,
}

impl World<usize> for FFI {
    fn is_end(&self, n: usize) -> bool {
        unsafe{
            (self.is_end)(self.obj, n)
        }
    }

    fn get_start(&self) -> usize {
        unsafe{
            (self.start)(self.obj)
        }
    }

    fn heuristic(&self, n: usize) -> f64 {
        unsafe{
            (self.heuristic)(self.obj, n)
        }
    }

    fn get_neightbors(&mut self, n: usize) -> Vec<crate::Neightbor<usize>> {
        let mut v = Vec::new();
        unsafe{
            let mut val : usize = 0;
            let mut dist: f64 = 0.;
            while (self.get_neightbors)(self.obj, n, &mut dist) {
                v.push(crate::Neightbor { distance: dist, ident: val});
            }
            (self.reset)(self.obj);
        }

        return v;
    }
}

#[repr(C)]
pub struct FFAStart {
    astart: AStart<usize, FFI>,
    result: Vec<usize>,
    index : usize,
}


#[no_mangle]
pub unsafe extern "C" fn new_ffi(
    obj           : *mut c_void,
    start         : Start,
    get_neightbors: GetNeightbors,
    heuristic     : Heuristic,
    is_end        : IsEnd,
    reset         : Reset,
) -> FFI {
    FFI {
        obj,
        start,
        get_neightbors,
        heuristic,
        is_end,
        reset,
    }
}

#[no_mangle]
pub unsafe extern "C" fn new_world(world: FFI) -> FFAStart{
    let s = FFAStart{
        astart: AStart::new(world),
        result: Vec::new(),
        index : 0
    };
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn start(start : *mut FFAStart) {
    unsafe{
        (*start).result = (*start).astart.start();
    }
}
#[no_mangle]
pub unsafe extern "C" fn next(start : *mut FFAStart, val: *mut usize) -> bool {
    unsafe{
        if (*start).index == (*start).result.len(){
            return false;
        }

        *val = (*start).result[(*start).index];
        (*start).index += 1;
    }

    return true;
}

#[no_mangle]
pub unsafe extern "C" fn step(start : *mut FFAStart) {
    unsafe{
        (*start).astart.step();
    }
}

#[no_mangle]
pub unsafe extern "C" fn explored_ptr(start : *mut FFAStart) -> *const usize{
    unsafe{
        (*start).astart.explored.as_ptr()
    }
}

#[no_mangle]
pub unsafe extern "C" fn explored_len(start : *mut FFAStart) -> usize{
    unsafe{
        (*start).astart.explored.len()
    }
}

#[no_mangle]
pub unsafe extern "C" fn candidate(start : *mut FFAStart) -> usize{
    unsafe{
        (*start).astart.candidates.last().unwrap_unchecked().ident
    }
}

#[no_mangle]
pub unsafe extern "C" fn reset(start : *mut FFAStart) {
    unsafe{
        (*start).index = 0;
    }
}
