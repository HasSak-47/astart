/*
use std::{any::Any, ffi::c_void};

use crate::{AStart, NodeIdentifier, World};

#[derive(Debug)]
#[repr(C)]
pub struct FFI{
    obj           : *mut c_void,
    start         : unsafe extern "C" fn (*mut c_void) -> usize,
    get_neightbors: unsafe extern "C" fn (*mut c_void, usize, *mut f64) -> bool,
    heuristic     : unsafe extern "C" fn (*mut c_void, usize) -> f64,
    is_end        : unsafe extern "C" fn (*mut c_void, usize) -> bool,
    reset         : unsafe extern "C" fn (*mut c_void),
}

impl NodeIdentifier for usize { }

pub unsafe extern "C" fn new_ffi(
    obj: *mut c_void,
    start         : unsafe extern "C" fn (*mut c_void) -> usize,
    get_neightbors: unsafe extern "C" fn (*mut c_void, *mut usize, *mut f64) -> bool,
    heuristic     : unsafe extern "C" fn (*mut c_void, usize) -> f64,
    is_end        : unsafe extern "C" fn (*mut c_void, usize) -> bool,
    reset         : unsafe extern "C" fn (*mut c_void),
) -> FFI{ 
    FFI {
        obj,
        start, get_neightbors, heuristic,
        is_end,
        reset,
    }
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
pub struct FFI_AStart (
    AStart<usize, FFI>,
    Vec<usize>,
    usize,
);


pub extern "C" fn new_world(world: FFI) -> FFI_AStart{
    let s = FFI_AStart( AStart::new(world), Vec::new(), 0);
    return s;
}

pub extern "C" fn start(start : *mut FFI_AStart) {
    unsafe{
        (*start).1 = (*start).0.start();
    }
}

pub extern "C" fn next(start : *mut FFI_AStart, val: *mut usize) -> bool {
    unsafe{
        if (*start).2 == (*start).1.len(){
            return false;
        }

        *val = (*start).1[(*start).2];
        (*start).2 += 1;
    }

    return true;
}
*/
