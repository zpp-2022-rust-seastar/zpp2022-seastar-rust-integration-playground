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
 * Copyright (C) 2019 ScyllaDB Ltd.
 */

#pragma once

#include <seastar/core/future.hh>

#ifndef SEASTAR_COROUTINES_ENABLED
#error Coroutines support disabled.
#endif

#include <seastar/core/std-coroutine.hh>

template<bool CheckPreempt = false, typename... T>
struct awaiter {
    seastar::future<T...> _future;
public:
    explicit awaiter(seastar::future<T...>&& f) noexcept : _future(std::move(f)) { }

    awaiter(const awaiter&) = delete;
    awaiter(awaiter&&) = delete;

    bool await_ready() const noexcept {
        return _future.available() && !need_preempt();
    }

    template<typename U>
    void await_suspend(SEASTAR_INTERNAL_COROUTINE_NAMESPACE::coroutine_handle<U> hndl) noexcept {
        if (!_future.available()) {
            _future.set_coroutine(hndl.promise());
        }
    }

    std::tuple<T...> await_resume() { return _future.get(); }
};

template<bool CheckPreempt, typename T>
struct awaiter<false, T> {
    seastar::future<T> _future;
public:
    explicit awaiter(seastar::future<T>&& f) noexcept : _future(std::move(f)) { }

    awaiter(const awaiter&) = delete;
    awaiter(awaiter&&) = delete;

    bool await_ready() const noexcept {
        return _future.available() && !need_preempt();
    }

    template<typename U>
    void await_suspend(SEASTAR_INTERNAL_COROUTINE_NAMESPACE::coroutine_handle<U> hndl) noexcept {
        if (!_future.available()) {
            _future.set_coroutine(hndl.promise());
        }
    }

    T await_resume() { return _future.get0(); }
};

template<>
struct awaiter<false> {
    seastar::future<> _future;
public:
    explicit awaiter(seastar::future<>&& f) noexcept : _future(std::move(f)) { }

    awaiter(const awaiter&) = delete;
    awaiter(awaiter&&) = delete;

    bool await_ready() const noexcept {
        return _future.available() && !need_preempt();
    }

    template<typename U>
    void await_suspend(SEASTAR_INTERNAL_COROUTINE_NAMESPACE::coroutine_handle<U> hndl) noexcept {
        if (!_future.available()) {
            _future.set_coroutine(hndl.promise());
        }
    }

    void await_resume() { _future.get(); }
};