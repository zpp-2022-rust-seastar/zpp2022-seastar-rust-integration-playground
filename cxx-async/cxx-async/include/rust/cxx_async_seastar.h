/*
 * This file is open source software, licensed to you under the terms
 * of the Apache License, Version 2.0 (the "License").  See the NOTICE file
 * distributed with this work for additional information regarding copyright
 * ownership.  You may not use this file except in compliance with the License.
 *
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */
/*
 * Copyright (C) 2015 Cloudius Systems, Ltd.
 */

// cxx-async/include/rust/cxx_async_seastar.h

#ifndef RUST_CXX_ASYNC_SEASTAR_H
#define RUST_CXX_ASYNC_SEASTAR_H

#include "rust/cxx_async.h"
#include <seastar/core/future.hh>
#include <seastar/core/std-coroutine.hh>
#include <seastar/core/make_task.hh>

template<typename... T>
struct cxx_awaiter {
    seastar::future<T...> _future;
public:
    explicit cxx_awaiter(seastar::future<T...>&& f) noexcept : _future(std::move(f)) { }

    cxx_awaiter(const cxx_awaiter&) = delete;
    cxx_awaiter(cxx_awaiter&&) = delete;

    bool await_ready() const noexcept { return _future.available(); }

    template<typename U>
    void await_suspend(SEASTAR_INTERNAL_COROUTINE_NAMESPACE::coroutine_handle<U> hndl) noexcept {
        seastar::task* t = seastar::make_task([hndl = std::move(hndl)] {
            hndl.resume();
        });
        
        if (!_future.available()) {
            _future.set_coroutine(*t);
        } else {
            schedule(t);
        }
    }

    std::tuple<T...> await_resume() { return _future.get(); }
};

template<typename T>
struct cxx_awaiter<T> {
    seastar::future<T> _future;
public:
    explicit cxx_awaiter(seastar::future<T>&& f) noexcept : _future(std::move(f)) { }

    cxx_awaiter(const cxx_awaiter&) = delete;
    cxx_awaiter(cxx_awaiter&&) = delete;

    bool await_ready() const noexcept { return _future.available(); }

    template<typename U>
    void await_suspend(SEASTAR_INTERNAL_COROUTINE_NAMESPACE::coroutine_handle<U> hndl) noexcept {
        seastar::task* t = seastar::make_task([hndl = std::move(hndl)] {
            hndl.resume();
        });
        
        if (!_future.available()) {
            _future.set_coroutine(*t);
        } else {
            schedule(t);
        }
    }

    T await_resume() { return _future.get0(); }
};

template<>
struct cxx_awaiter<> {
    seastar::future<> _future;
public:
    explicit cxx_awaiter(seastar::future<>&& f) noexcept : _future(std::move(f)) { }

    cxx_awaiter(const cxx_awaiter&) = delete;
    cxx_awaiter(cxx_awaiter&&) = delete;

    bool await_ready() const noexcept { return _future.available(); }

    template<typename U>
    void await_suspend(SEASTAR_INTERNAL_COROUTINE_NAMESPACE::coroutine_handle<U> hndl) noexcept {
        seastar::task* t = seastar::make_task([hndl = std::move(hndl)] {
            hndl.resume();
        });
        
        if (!_future.available()) {
            _future.set_coroutine(*t);
        } else {
            schedule(t);
        }
    }

    void await_resume() { _future.get(); }
};

namespace rust {
namespace async {

template<typename T, typename Future>
class AwaitTransformer<
    seastar::future<T>,
    Future,
    void> {
  AwaitTransformer() = delete;

 public:
  static auto await_transform(
      RustPromiseBase<Future>& promise,
      seastar::future<T>&& future) noexcept {
    return cxx_awaiter<T>(std::move(future));
  }
};

} // namespace async
} // namespace rust

#endif // RUST_CXX_ASYNC_SEASTAR_H
