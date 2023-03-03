use rand::RngCore;

pub const KB: usize = 1024;
pub const MB: usize = 1024 * 1024;
pub const GB: usize = 1024 * 1024 * 1024;

/// An iterator over a range of byte size that doubles on every iteration and stops
/// before exceeding the limit which was provided during the initialization.
pub struct SizeIterator {
    current: usize,
    end: usize,
    multiplier: usize,
}

impl SizeIterator {
    /// Create a new size iterator with a fixed multiplier of 2.
    ///
    /// # Panics
    ///
    /// If `from` is equal to zero.
    pub fn new(from: usize, to: usize) -> Self {
        assert!(from > 0, "From value cannot be zero.");
        Self {
            current: from,
            end: to,
            multiplier: 2,
        }
    }

    /// Set the multiplier of the iterator.
    pub fn with_multiplier(self, multiplier: usize) -> Self {
        assert!(multiplier > 1, "Multiplier must be greater than 1.");
        Self { multiplier, ..self }
    }
}

impl Iterator for SizeIterator {
    type Item = (usize, String);

    fn next(&mut self) -> Option<Self::Item> {
        let size = self.current;
        if size > self.end {
            None
        } else {
            self.current *= self.multiplier;
            let label = humansize::format_size(size, humansize::BINARY);
            Some((size, label))
        }
    }
}

/// Return a [`SizeIterator`] that goes from 1B until the provided value in bytes but takes
/// 16x steps until 1KB, 8x steps until 1MB, 4x steps until 1GB and 2x steps for anything left.
///
/// It takes 13 steps to iterate from 1B to 1GB.
pub fn size(until: usize) -> impl Iterator<Item = (usize, String)> {
    let kb_step = until.min(KB);
    let mb_step = until.min(MB);
    let gb_step = until.min(GB);

    let kb = SizeIterator::new(1, kb_step - 1).with_multiplier(16);
    let mb = SizeIterator::new(kb_step, mb_step - 1).with_multiplier(8);
    let gb = SizeIterator::new(mb_step, gb_step - 1).with_multiplier(4);
    let other = SizeIterator::new(gb_step, until);

    kb.chain(mb).chain(gb).chain(other)
}

/// Generate a vector with the provided size.
pub fn random_vec(size: usize) -> Vec<u8> {
    let mut vec = mk_vec(size);
    fastrand::Rng::new().fill(vec.as_mut_slice());
    vec
}

/// Create an vector with the given size that has the default data from the alloc in.
#[allow(clippy::uninit_vec)]
pub fn mk_vec(size: usize) -> Vec<u8> {
    let mut vec = Vec::with_capacity(size);
    unsafe {
        vec.set_len(size);
    }
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
        let v = SizeIterator::new(1, GB).collect::<Vec<_>>();
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
        SizeIterator::new(0, GB);
    }
}
