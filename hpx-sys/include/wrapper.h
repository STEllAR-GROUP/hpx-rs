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

inline void hpx_copy(const rust::Vec<int32_t>& src, rust::Vec<int32_t>& dest) {
    std::vector<int32_t> cpp_src(src.begin(), src.end());
    std::vector<int32_t> cpp_dest(dest.size());

    hpx::copy(hpx::execution::par, cpp_src.begin(), cpp_src.end(), cpp_dest.begin());

    dest.clear();
    dest.reserve(cpp_dest.size());
    for (const auto& item : cpp_dest) {
        dest.push_back(item);
    }
}

inline void hpx_copy_n(const rust::Vec<int32_t>& src, size_t count, rust::Vec<int32_t>& dest) {
    std::vector<int32_t> cpp_src(src.begin(), src.end());
    std::vector<int32_t> cpp_dest(count);

    hpx::copy_n(hpx::execution::par, cpp_src.begin(), count, cpp_dest.begin());

    dest.clear();
    dest.reserve(cpp_dest.size());
    for (const auto& item : cpp_dest) {
        dest.push_back(item);
    }
}

inline void hpx_copy_if(const rust::Vec<int32_t>& src, rust::Vec<int32_t>& dest, 
                        rust::Fn<bool(int32_t)> pred) {
    std::vector<int32_t> cpp_src(src.begin(), src.end());
    std::vector<int32_t> cpp_dest(cpp_src.size()); 

    auto result = hpx::copy_if(hpx::execution::par, 
                 cpp_src.begin(), cpp_src.end(), 
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

inline void hpx_fill(rust::Vec<int32_t>& src, int32_t value) {
    std::vector<int32_t> cpp_vec(src.begin(), src.end());
    
    hpx::fill(hpx::execution::par, cpp_vec.begin(), cpp_vec.end(), value);
    
    src.clear();
    src.reserve(cpp_vec.size());
    for (const auto& item : cpp_vec) {
        src.push_back(item);
    }
}

inline int64_t hpx_find(const rust::Vec<int32_t>& src, int32_t value) {
    std::vector<int32_t> cpp_vec(src.begin(), src.end());
    
    auto result = hpx::find(hpx::execution::par,
                            cpp_vec.begin(),
                            cpp_vec.end(),
                            value);
    
    if (result != cpp_vec.end()) {
        return static_cast<int64_t>(std::distance(cpp_vec.begin(), result));
    }
    return -1;
}

inline void hpx_sort(rust::Vec<int32_t>& src) {
    std::vector<int32_t> cpp_vec(src.begin(), src.end());
    
    hpx::sort(hpx::execution::par, cpp_vec.begin(), cpp_vec.end());
    
    src.clear();
    src.reserve(cpp_vec.size());
    for (const auto& item : cpp_vec) {
        src.push_back(item);
    }
}

inline void hpx_sort_comp(rust::Vec<int32_t>& src, rust::Fn<bool(int32_t, int32_t)> comp) {
    std::vector<int32_t> cpp_vec(src.begin(), src.end());
    
    hpx::sort(hpx::execution::par, cpp_vec.begin(), cpp_vec.end(),
        [&](int32_t a, int32_t b) { return comp(a, b); });
    
    src.clear();
    src.reserve(cpp_vec.size());
    for (const auto& item : cpp_vec) {
        src.push_back(item);
    }
}
