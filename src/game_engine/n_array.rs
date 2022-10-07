use std::{any::Any, slice::Iter};

pub struct NArray<T> {
    data: Box<[T]>
}

impl<T: Clone> NArray<T> {
    pub fn new(value: T, size: usize) -> NArray<T> {
        let data = vec![value; size];
        NArray { data: data.into_boxed_slice() }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl<Idx, T> std::ops::Index<Idx> for NArray<T>
where
    Idx: std::slice::SliceIndex<[T]>,
    T: std::any::Any,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.data[index]
    }
}

impl<Idx, T> std::ops::IndexMut<Idx> for NArray<T>
where
    Idx: std::slice::SliceIndex<[T]>,
    T: std::any::Any,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.data[index]
    }
}

pub struct ArrIter<T> {
    arr: Box<[T]>,
    i: usize
}

impl<T: Clone> Iterator for ArrIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.arr.len() {
            let r = Some(self.arr[self.i].clone());
            self.i += 1;

            r
        } else {
            None
        }
    }
}

impl<T: Any> From<NArray<T>> for ArrIter<T> {
    fn from(arr: NArray<T>) -> Self {
        ArrIter { arr: arr.data, i: 0 }
    }
}

impl<'a, T> IntoIterator for NArray<T>
where
    T: Clone + 'static
{
        type Item = T;

        type IntoIter = ArrIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.into()
    }
}

impl<'a, T> IntoIterator for &'a NArray<T>
where
    T: Any
{
        type Item = &'a T;

        type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}