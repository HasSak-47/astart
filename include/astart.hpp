#ifndef ASTART_HPP
#define ASTART_HPP

#include <cstddef>

extern "C"{
    struct FFI { char bytes[48]; };
    struct FFI_AStart { char bytes[78]; };
    
    FFI_AStart new_world(FFI world) ;
    
    void start(FFI_AStart* start);
    bool next(FFI_AStart * start, size_t* val);
    void reset(FFI_AStart* start);
    
    FFI new_ffi(
        void* obj,
        void* start,
        void* get_neightbors,
        void* heuristic,
        void* is_end,
        void* reset
    );
}

#endif // !ASTART_HPP
