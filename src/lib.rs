use rand::RngCore;

pub const KB: usize = 1024;
pub const MB: usize = 1024 * 1024;
pub const GB: usize = 1024 * 1024 * 1024;

/// An iterator over a range of byte size that doubles on every iteration and stops
/// before exceeding the limit which was provided during the initialization.
pub struct DoublingSizeIterator {
    current: usize,
    end: usize,
}

impl DoublingSizeIterator {
    /// Create a new iterator.
    ///
    /// # Panics
    ///
    /// If `from` is equal to zero.
    pub fn new(from: usize, to: usize) -> Self {
        assert!(from > 0, "From value cannot be zero.");
        Self {
            current: from,
            end: to,
        }
    }
}

impl Iterator for DoublingSizeIterator {
    type Item = (usize, String);

    fn next(&mut self) -> Option<Self::Item> {
        let size = self.current;
        if size > self.end {
            None
        } else {
            self.current *= 2;
            let label = humansize::format_size(size, humansize::BINARY);
            Some((size, label))
        }
    }
}

/// Return a [`DoublingSizeIterator`] that goes from 1B until the provided value in bytes.
pub fn size(until: usize) -> DoublingSizeIterator {
    DoublingSizeIterator::new(1, until)
}

/// Generate a vector with the provided size.
#[allow(clippy::uninit_vec)]
pub fn random_vec(size: usize) -> Vec<u8> {
    let mut vec = Vec::with_capacity(size);
    unsafe {
        vec.set_len(size);
    }
    rand_core::OsRng.fill_bytes(vec.as_mut_slice());
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_vec() {
        assert_eq!(random_vec(0).len(), 0);
        assert_eq!(random_vec(GB).len(), GB);
    }

    #[test]
    fn test_doubling_size_iter() {
        let v = DoublingSizeIterator::new(1, GB).collect::<Vec<_>>();
        assert_eq!(v.len(), 31);
        assert_eq!(v[10], (KB, "1 KiB".to_owned()));
        assert_eq!(v[20], (MB, "1 MiB".to_owned()));
        assert_eq!(v[30], (GB, "1 GiB".to_owned()));
        assert_eq!(v[9], (KB / 2, "512 B".to_owned()));
        assert_eq!(v[19], (MB / 2, "512 KiB".to_owned()));
        assert_eq!(v[29], (GB / 2, "512 MiB".to_owned()));
    }

    #[test]
    #[should_panic]
    fn test_doubling_size_from_zero_should_panic() {
        DoublingSizeIterator::new(0, GB);
    }

    #[test]
    fn test_size() {
        let expected = DoublingSizeIterator::new(1, GB).collect::<Vec<_>>();
        let actual = size(GB).collect::<Vec<_>>();
        assert_eq!(expected, actual);
    }
}
