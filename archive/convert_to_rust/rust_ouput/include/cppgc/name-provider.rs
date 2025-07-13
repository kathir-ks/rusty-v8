// Converted from V8 C++ source files:
// Header: name-provider.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod cppgc {

/**
 * NameProvider allows for providing a human-readable name for garbage-collected
 * objects.
 *
 * There's two cases of names to distinguish:
 * a. Explicitly specified names via using NameProvider. Such names are always
 *    preserved in the system.
 * b. Internal names that Oilpan infers from a C++ type on the class hierarchy
 *    of the object. This is not necessarily the type of the actually
 *    instantiated object.
 *
 * Depending on the build configuration, Oilpan may hide names, i.e., represent
 * them with kHiddenName, of case b. to avoid exposing internal details.
 */
pub trait NameProvider {
  /**
   * Name that is used when hiding internals.
   */
  const kHiddenName: &'static str = "InternalNode";

  /**
   * Name that is used in case compiler support is missing for composing a name
   * from C++ types.
   */
  const kNoNameDeducible: &'static str = "<No name>";

  /**
   * Indicating whether the build supports extracting C++ names as object names.
   *
   * @returns true if C++ names should be hidden and represented by kHiddenName.
   */
  fn supports_cpp_class_names_as_object_names() -> bool {
    #[cfg(feature = "cppgc_supports_object_names")]
    {
      true
    }
    #[cfg(not(feature = "cppgc_supports_object_names"))]
    {
      false
    }
  }

  /**
   * Specifies a name for the garbage-collected object. Such names will never
   * be hidden, as they are explicitly specified by the user of this API.
   *
   * Implementations of this function must not allocate garbage-collected
   * objects or otherwise modify the cppgc heap.
   *
   * V8 may call this function while generating a heap snapshot or at other
   * times. If V8 is currently generating a heap snapshot (according to
   * HeapProfiler::IsTakingSnapshot), then the returned string must stay alive
   * until the snapshot generation has completed. Otherwise, the returned string
   * must stay alive forever. If you need a place to store a temporary string
   * during snapshot generation, use HeapProfiler::CopyNameForHeapSnapshot.
   *
   * @returns a human readable name for the object.
   */
  fn get_human_readable_name(&self) -> &'static str;
}

} // namespace cppgc
