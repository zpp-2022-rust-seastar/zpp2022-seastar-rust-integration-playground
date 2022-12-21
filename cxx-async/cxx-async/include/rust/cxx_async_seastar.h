/*
 * TODO: Copyright
*/
// cxx-async/include/rust/cxx_async_seastar.h

#ifndef RUST_CXX_ASYNC_SEASTAR_H
#define RUST_CXX_ASYNC_SEASTAR_H

#include "rust/cxx_async.h"
#include "awaiter.hh"
#include <seastar/core/future.hh>

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
    return awaiter<false, T>(std::move(future));
  }
};


} // namespace async
} // namespace rust

#endif // RUST_CXX_ASYNC_SEASTAR_H
