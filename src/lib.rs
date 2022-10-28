struct MyIter<'iter, T> {
    slice: &'iter [T],
}

impl<'iter, T> Iterator for MyIter<'iter, T> {
    type Item = &'iter T;

    fn next(&mut self) -> Option<Self::Item> {
        let (element, tail) = self.slice.split_first()?;
        self.slice = tail;
        Some(element)
    }
}

struct MyIterMut<'iter, T> {
    slice: &'iter mut [T],
}

impl<'iter, T> Iterator for MyIterMut<'iter, T> {
    type Item = &'iter mut T;

    #[allow(clippy::needless_lifetimes)]
    fn next<'next>(&'next mut self) -> Option<Self::Item> {
        // Here is the trick to fix the lifetime issue without unsafe.
        // slice is a &mut &mut [T]
        let slice = &mut self.slice;
        // So we can use std::mem::(take | replace | switch) to get a owned &mut [T]
        let slice2 = std::mem::take(slice);
        let (element, tail) = slice2.split_first_mut()?;
        self.slice = tail;
        Some(element)
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn my_iter_test() {
        let input = vec![1, 2, 3, 4];
        let my_iter = MyIter { slice: &input };

        for (index, value) in my_iter.enumerate() {
            println!("{}->{}", index, value);
            assert_eq!(*value, input[index]);
        }
    }

    #[test]
    fn my_iter_mut_test() {
        let mut input = vec![1, 2, 3, 4];
        let my_iter = MyIterMut { slice: &mut input };
        let mut check = Vec::new();

        for (index, value) in &mut my_iter.enumerate() {
            *value += 1;
            println!("{}->{}", index, value);
            check.push(*value);
        }

        debug_assert_eq!(check, vec![2, 3, 4, 5]);
    }
}
