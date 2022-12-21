/*
 * TODO: Copyright
*/
// cxx-async/include/rust/cxx_async_seastar.h

#ifndef RUST_CXX_ASYNC_SEASTAR_H
#define RUST_CXX_ASYNC_SEASTAR_H

#include "rust/cxx_async.h"
#include <seastar/core/future.hh>
#include <seastar/core/std-coroutine.hh>

template<typename... T>
struct awaiter {
    seastar::future<T...> _future;
public:
    explicit awaiter(seastar::future<T...>&& f) noexcept : _future(std::move(f)) { }

    awaiter(const awaiter&) = delete;
    awaiter(awaiter&&) = delete;

    bool await_ready() const noexcept {
        return _future.available() && !seastar::need_preempt();
    }

    template<typename U>
    void await_suspend(SEASTAR_INTERNAL_COROUTINE_NAMESPACE::coroutine_handle<U> hndl) noexcept {
        _future = _future.then([hndl = std::move(hndl)] {
            hndl.resume();
        });
    }

    std::tuple<T...> await_resume() { return _future.get(); }
};

template<typename T>
struct awaiter<T> {
    seastar::future<T> _future;
public:
    explicit awaiter(seastar::future<T>&& f) noexcept : _future(std::move(f)) { }

    awaiter(const awaiter&) = delete;
    awaiter(awaiter&&) = delete;

    bool await_ready() const noexcept {
        return _future.available() && !seastar::need_preempt();
    }

    template<typename U>
    void await_suspend(SEASTAR_INTERNAL_COROUTINE_NAMESPACE::coroutine_handle<U> hndl) noexcept {
        _future = _future.then([hndl = std::move(hndl)] {
            hndl.resume();
        });
    }

    T await_resume() { return _future.get0(); }
};

template<>
struct awaiter<> {
    seastar::future<> _future;
public:
    explicit awaiter(seastar::future<>&& f) noexcept : _future(std::move(f)) { }

    awaiter(const awaiter&) = delete;
    awaiter(awaiter&&) = delete;

    bool await_ready() const noexcept {
        return _future.available() && !seastar::need_preempt();
    }

    template<typename U>
    void await_suspend(SEASTAR_INTERNAL_COROUTINE_NAMESPACE::coroutine_handle<U> hndl) noexcept {
        _future = _future.then([hndl = std::move(hndl)] {
            hndl.resume();
        });
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
    return awaiter<T>(std::move(future));
  }
};

} // namespace async
} // namespace rust

#endif // RUST_CXX_ASYNC_SEASTAR_H
