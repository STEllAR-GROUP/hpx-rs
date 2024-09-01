#pragma once

#include <hpx/hpx_init.hpp>
#include <hpx/algorithm.hpp>
#include <iostream>
#include <cstdint>
#include <vector>

#include "rust/cxx.h"

inline std::int32_t init(rust::Fn<int(int, char **)> rust_fn, int argc, char **argv) {
	return hpx::init(
		[&](int argc, char **argv) {
		return rust_fn(argc, argv);
        },
        argc, argv);
}

inline std::int32_t finalize_with_timeout(double shutdown_timeout, double localwait) {
	return hpx::finalize(shutdown_timeout, localwait);
}

inline void terminate() {
	return hpx::terminate();
}

inline std::int32_t disconnect() {
	return hpx::disconnect();
}

inline std::int32_t disconnect_with_timeout(double shutdown_timeout, double localwait) {
	return hpx::disconnect(shutdown_timeout, localwait);
}

inline std::int32_t finalize() { return hpx::finalize(); }

inline void hpx_copy(rust::Slice<const int32_t> src, rust::Slice<int32_t> dest) {
    hpx::copy(hpx::execution::par, src.begin(), src.end(), dest.begin());
}

inline void hpx_copy_n(rust::Slice<const int32_t> src, size_t count, rust::Slice<int32_t> dest) {
    hpx::copy_n(hpx::execution::par, src.begin(), count, dest.begin());
}

inline void hpx_copy_if(const rust::Vec<int32_t>& src, rust::Vec<int32_t>& dest, 
                        rust::Fn<bool(int32_t)> pred) {
    std::vector<int32_t> cpp_dest(src.size());

    auto result = hpx::copy_if(hpx::execution::par, 
                 src.begin(), src.end(), 
                 cpp_dest.begin(),
                 [&](int32_t value) { return pred(value); });

    cpp_dest.resize(std::distance(cpp_dest.begin(), result));

    dest.clear();
    dest.reserve(cpp_dest.size());
    for (const auto& item : cpp_dest) {
        dest.push_back(item);
    }
}

inline std::int64_t hpx_count(const rust::Vec<int32_t>& vec, int32_t value) {
    return hpx::count(hpx::execution::par, vec.begin(), vec.end(), value);
}


inline int64_t hpx_count_if(const rust::Vec<int32_t>& vec, rust::Fn<bool(int32_t)> pred) {
    std::vector<int32_t> cpp_vec(vec.begin(), vec.end());
    
    auto result = hpx::count_if(hpx::execution::par,
                                cpp_vec.begin(),
                                cpp_vec.end(),
                                [&](int32_t value) { return pred(value); });
    
    return static_cast<int64_t>(result);
}

inline bool hpx_ends_with(rust::Slice<const int32_t> src, 
                          rust::Slice<const int32_t> dest) {
    return hpx::ends_with(hpx::execution::par,
                          src.begin(), src.end(),
                          dest.begin(), dest.end(),
                          std::equal_to<int32_t>());
}

inline bool hpx_equal(rust::Slice<const int32_t> src, rust::Slice<const int32_t> dest) {
    return hpx::equal(
        hpx::execution::par,
        src.begin(), src.end(),
        dest.begin(), dest.end()
    );
}

inline void hpx_fill(rust::Slice<int32_t> src, int32_t value) {
    hpx::fill(hpx::execution::par, src.begin(), src.end(), value);
}

inline int64_t hpx_find(rust::Slice<const int32_t> src, int32_t value) {
    auto result = hpx::find(hpx::execution::par,
                            src.begin(),
                            src.end(),
                            value);
    
    if (result != src.end()) {
        return static_cast<int64_t>(std::distance(src.begin(), result));
    }
    return -1;
}

inline void hpx_sort(rust::Slice<int32_t> src) {
    hpx::sort(hpx::execution::par, src.begin(), src.end());
}

inline void hpx_sort_comp(rust::Vec<int32_t>& src, rust::Fn<bool(int32_t, int32_t)> comp) {
    hpx::sort(hpx::execution::par, src.begin(), src.end(),
        [&](int32_t a, int32_t b) { return comp(a, b); });
}

inline void hpx_merge(rust::Slice<const int32_t> src1, 
                      rust::Slice<const int32_t> src2, 
                      rust::Vec<int32_t>& dest) {
    dest.clear();
    dest.reserve(src1.size() + src2.size());
    
    for (size_t i = 0; i < src1.size() + src2.size(); ++i) {
        dest.push_back(0);
    }

    hpx::merge(hpx::execution::par,
               src1.begin(), src1.end(),
               src2.begin(), src2.end(),
               dest.begin());
}

inline void hpx_partial_sort(rust::Vec<int32_t>& src, size_t last) {
    if (last > src.size()) {
        last = src.size();
    }
    
    hpx::partial_sort(hpx::execution::par, 
                      src.begin(), 
                      src.begin() + last, 
                      src.end());
}

inline void hpx_partial_sort_comp(rust::Vec<int32_t>& src, size_t last, 
                                  rust::Fn<bool(int32_t, int32_t)> comp) {
    if (last > src.size()) {
        last = src.size();
    }
    
    hpx::partial_sort(hpx::execution::par, 
                      src.begin(), 
                      src.begin() + last, 
                      src.end(),
                      [&](int32_t a, int32_t b) { return comp(a, b); });
}
