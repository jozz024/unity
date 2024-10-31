use std::{
    alloc::Layout,
    ops::{Index, IndexMut, Range},
    ptr::null,
};

#[derive(Debug)]
#[repr(C)]
pub struct CppVector<T> {
    start: *mut T,
    end: *mut T,
    eos: *mut T,
}

impl<T> Default for CppVector<T> {
    fn default() -> Self {
        CppVector::new()
    }
}

impl<T> CppVector<T> {
    unsafe fn realloc(&mut self) {
        let current_capacity = self.eos.offset_from(self.start) as usize;
        let current_len = self.end.offset_from(self.start) as usize;
        let layout = Layout::from_size_align(current_capacity * 2 * std::mem::size_of::<T>(), 1).unwrap();
        let (new_start, new_eos) = {
            let start = std::alloc::alloc(layout) as *mut T;
            (start, start.add(current_capacity * 2))
        };
        std::ptr::copy_nonoverlapping(self.start, new_start, current_len);
        std::alloc::dealloc(
            self.start as _,
            Layout::from_size_align(current_capacity * std::mem::size_of::<T>(), 1).unwrap(),
        );
        self.start = new_start;
        self.end = self.start.add(current_len);
        self.eos = new_eos;
    }

    pub fn new() -> Self {
        Self {
            start: null::<T>() as _,
            end: null::<T>() as _,
            eos: null::<T>() as _,
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        let layout = Layout::from_size_align(cap * std::mem::size_of::<T>(), 1).unwrap();
        let (start, eos) = unsafe {
            let start = std::alloc::alloc(layout) as *mut T;
            (start, start.add(cap))
        };
        Self { start, end: start, eos }
    }

    pub fn push(&mut self, val: T) {
        unsafe {
            if self.end.add(1) > self.eos {
                self.realloc();
            }
            *self.end = val;
            self.end = self.end.add(1);
        }
    }

    pub fn reserve(&mut self, additional: usize) {
        unsafe {
            if self.end.add(additional) > self.eos {
                self.realloc();
            }
        }
    }

    pub fn iter(&self) -> CppVectorIterator<T> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> CppVectorIteratorMut<T> {
        self.into_iter()
    }

    pub fn len(&self) -> usize {
        ((self.end as usize) - (self.start as usize)) / std::mem::size_of::<T>()
    }

    pub fn as_ptr(&self) -> *const T {
        self.start
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.start
    }

    pub fn as_slice(&self) -> &[T] {
        unsafe {
            let len = self.end.offset_from(self.start) as usize;
            std::slice::from_raw_parts(self.start, len)
        }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe {
            let len = self.end.offset_from(self.start) as usize;
            std::slice::from_raw_parts_mut(self.start, len)
        }
    }

    pub fn extend_from_slice(&mut self, slice: &[T])
    where
        T: Copy + Clone,
    {
        unsafe {
            if self.end.add(slice.len()) > self.eos {
                self.realloc();
                self.extend_from_slice(slice);
            } else {
                std::ptr::copy_nonoverlapping(slice.as_ptr(), self.end, slice.len());
                self.end = self.end.add(slice.len());
            }
        }
    }
}

impl<T: Copy + Clone> CppVector<T> {
    pub fn from_slice(slice: &[T]) -> Self {
        let layout = Layout::from_size_align(slice.len() * std::mem::size_of::<T>(), 1).unwrap();
        let (start, eos) = unsafe {
            let start = std::alloc::alloc(layout) as *mut T;
            (start, start.add(slice.len()))
        };
        let new_slice = unsafe { std::slice::from_raw_parts_mut(start, slice.len()) };
        new_slice.copy_from_slice(slice);
        Self { start, end: eos, eos }
    }
}

impl<T: Clone> CppVector<T> {
    pub fn clone_from_slice(slice: &[T]) -> Self {
        let layout = Layout::from_size_align(slice.len() * std::mem::size_of::<T>(), 1).unwrap();
        let (start, eos) = unsafe {
            let start = std::alloc::alloc(layout) as *mut T;
            (start, start.add(slice.len()))
        };
        let new_slice = unsafe { std::slice::from_raw_parts_mut(start, slice.len()) };
        new_slice.clone_from_slice(slice);
        Self { start, end: eos, eos }
    }
}

impl<T> Index<usize> for CppVector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl<T> Index<Range<usize>> for CppVector<T> {
    type Output = [T];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl<T> IndexMut<usize> for CppVector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut_slice()[index]
    }
}

impl<T> IndexMut<Range<usize>> for CppVector<T> {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        &mut self.as_mut_slice()[index]
    }
}

impl<'a, T> IntoIterator for &'a CppVector<T> {
    type IntoIter = CppVectorIterator<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        CppVectorIterator { vector: self, index: 0 }
    }
}

impl<'a, T> IntoIterator for &'a mut CppVector<T> {
    type IntoIter = CppVectorIteratorMut<'a, T>;
    type Item = &'a mut T;

    fn into_iter(self) -> Self::IntoIter {
        CppVectorIteratorMut { vector: self, index: 0 }
    }
}

pub struct CppVectorIterator<'a, T> {
    vector: &'a CppVector<T>,
    index: isize,
}

impl<'a, T> Iterator for CppVectorIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        unsafe {
            if self.vector.start.offset(self.index) != self.vector.end {
                self.index += 1;
                Some(&*self.vector.start.offset(self.index - 1))
            } else {
                None
            }
        }
    }
}

impl<'a, T> DoubleEndedIterator for CppVectorIterator<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.vector.end.offset(self.index) != self.vector.start {
                self.index -= 1;
                Some(&*self.vector.end.offset(self.index))
            } else {
                None
            }
        }
    }
}

pub struct CppVectorIteratorMut<'a, T> {
    vector: &'a mut CppVector<T>,
    index: isize,
}

impl<'a, T> Iterator for CppVectorIteratorMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        unsafe {
            if self.vector.start.offset(self.index) != self.vector.end {
                self.index += 1;
                Some(&mut *self.vector.start.offset(self.index - 1))
            } else {
                None
            }
        }
    }
}

impl<'a, T> DoubleEndedIterator for CppVectorIteratorMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.vector.end.offset(self.index) != self.vector.start {
                self.index -= 1;
                Some(&mut *self.vector.end.offset(self.index))
            } else {
                None
            }
        }
    }
}