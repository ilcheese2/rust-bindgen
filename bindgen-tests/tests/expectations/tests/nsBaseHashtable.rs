#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// If Bindgen could only determine the size and alignment of a
/// type, it is represented like this.
#[derive(PartialEq, Copy, Clone, Debug, Hash)]
#[repr(C)]
pub struct __BindgenOpaqueArray<T: Copy, const N: usize>(pub [T; N]);
impl<T: Copy + Default, const N: usize> Default for __BindgenOpaqueArray<T, N> {
    fn default() -> Self {
        Self([<T as Default>::default(); N])
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct nsBaseHashtableET {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct nsTHashtable {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nsBaseHashtable {
    pub _address: u8,
}
pub type nsBaseHashtable_KeyType = __BindgenOpaqueArray<u8, 0usize>;
pub type nsBaseHashtable_EntryType = nsBaseHashtableET;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nsBaseHashtable_LookupResult {
    pub mEntry: *mut nsBaseHashtable_EntryType,
    pub mTable: *mut nsBaseHashtable,
}
impl Default for nsBaseHashtable_LookupResult {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct nsBaseHashtable_EntryPtr {
    pub mEntry: *mut nsBaseHashtable_EntryType,
    pub mExistingEntry: bool,
}
impl Default for nsBaseHashtable_EntryPtr {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for nsBaseHashtable {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
