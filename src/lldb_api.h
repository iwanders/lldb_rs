
#include <lldb/API/LLDB.h>

// https://github.com/rust-lang/rust-bindgen/issues/1509

/// <div rustbindgen replaces="std::shared_ptr"></div>
template<typename T>
class simple_shared_ptr {
 public:
  T* ptr;
  void* count;
};

/// <div rustbindgen replaces="std::unique_ptr"></div>
template<typename T>
class simple_unique_ptr {
 public:
  T* ptr;
};

/// <div rustbindgen replaces="std::weak_ptr"></div>
template<typename T>
class simple_weak_ptr {
 public:
  T* ptr;
  void* count;
};


static_assert(sizeof(simple_shared_ptr<int>) == sizeof(std::shared_ptr<int>), "");
static_assert(alignof(simple_shared_ptr<int>) == alignof(std::shared_ptr<int>), "");

static_assert(sizeof(simple_unique_ptr<int>) == sizeof(std::unique_ptr<int>), "");
static_assert(alignof(simple_unique_ptr<int>) == alignof(std::unique_ptr<int>), "");

static_assert(sizeof(simple_weak_ptr<int>) == sizeof(std::weak_ptr<int>), "");
static_assert(alignof(simple_weak_ptr<int>) == alignof(std::weak_ptr<int>), "");
