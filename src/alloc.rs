use bumpalo::{
    collections::Vec as BumpVec,
    Bump,
};
use std::mem;

#[must_use]
pub(crate) fn allocate<T>(allocator: &Bump, val: T) -> &mut T {
    debug_assert!(!mem::needs_drop::<T>());
    allocator.alloc(val)
}

#[must_use]
pub(crate) fn allocate_slice<'alloc, T>(allocator: &'alloc Bump, src: &[T]) -> &'alloc [T]
where
    T: Copy,
{
    debug_assert!(!mem::needs_drop::<T>());
    allocator.alloc_slice_copy(src)
}

#[must_use]
pub(crate) fn new_vec<T>(allocator: &Bump) -> BumpVec<'_, T> {
    debug_assert!(!mem::needs_drop::<T>());
    BumpVec::new_in(allocator)
}
