pub trait BitOps {
    /// Returns the bit on the specified position.
    ///
    fn get_bit(&self, n: u8) -> bool;

    /// Sets a bit on the specified position.
    ///
    fn set_bit(&self, n: usize, value: bool) -> u8;

    /// Sets the least significant bit of a number according to the passed value.
    ///
    fn set_lsb(&self, value: bool) -> Self;

    /// Returns the least significant bit.
    ///
    fn get_lsb(&self) -> bool;
}

pub trait BitBuffer {
    /// Returns a vector of bools representing the single bits of self.
    ///
    fn to_bit_buffer(&self) -> Vec<bool>;
}

impl BitBuffer for u8 {
    fn to_bit_buffer(&self) -> Vec<bool> {
        let mut result = vec![];

        for i in 0..=7 {
            result.push(self.get_bit(i));
        }

        result
    }
}

impl BitOps for u8 {
    fn get_bit(&self, n: u8) -> bool {
        if n > 7 {
            panic!("Overflow detected while using `get_bit`: Tried to get the {n}th bit.");
        }
        let result = *self >> n & 1;

        result == 1
    }

    fn set_bit(&self, n: usize, value: bool) -> u8 {
        let mut result = *self;

        if n > 7 {
            panic!("Overflow detected while using `set_bit`: Tried to set the {n}th bit.");
        }

        if value {
            result |= 1 << n;
        } else {
            result &= !(1 << n);
        }
        result
    }

    fn set_lsb(&self, value: bool) -> u8 {
        let mut result = *self;
        if value {
            result |= 0b0000_0001;
            return result;
        }
        result &= 0b1111_1110;
        result
    }

    fn get_lsb(&self) -> bool {
        self % 2 != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_lsb_ut() {
        assert_eq!(1, 0.set_lsb(true));
        assert_eq!(0, 0.set_lsb(false));
        assert_eq!(0, 1.set_lsb(false));
        assert_eq!(1, 1.set_lsb(true));

        assert_eq!(254, 255.set_lsb(false));
        assert_eq!(255, 255.set_lsb(true));
    }

    #[test]
    fn get_lsb_ut() {
        assert!(1.get_lsb());
        assert!(!0.get_lsb());
        assert!(11.get_lsb());
        assert!(!22.get_lsb());
    }

    #[test]
    fn set_bit_ut() {
        // Test setting the last bit
        assert_eq!(0b1000_0000, 0b0000_0000.set_bit(7, true));
        assert_eq!(0b0000_0000, 0b1000_0000.set_bit(7, false));
        assert_eq!(0b1000_0000, 0b1000_0000.set_bit(7, true));
        assert_eq!(0b0000_0000, 0b0000_0000.set_bit(7, false));

        // Test setting first bits
        assert_eq!(0b0000_0001, 0b0000_0000.set_bit(0, true));
        assert_eq!(0b0000_0000, 0b0000_0001.set_bit(0, false));
        assert_eq!(0b0000_0001, 0b0000_0001.set_bit(0, true));
        assert_eq!(0b0000_0000, 0b0000_0000.set_bit(0, false));

        // Test setting some random bits
        assert_eq!(0b0000_1000, 0b0000_0000.set_bit(3, true));
        assert_eq!(0b0110_1000, 0b0100_1000.set_bit(5, true));
        assert_eq!(0b0000_0000, 0b0000_1000.set_bit(3, false));
        assert_eq!(0b1100_1011, 0b1100_1111.set_bit(2, false));
    }

    #[test]
    fn get_bit_ut() {
        assert!(0b0000_0001.get_bit(0));
        assert!(0b0000_0010.get_bit(1));
        assert!(0b0000_0100.get_bit(2));
        assert!(0b0000_1000.get_bit(3));
        assert!(0b0001_0000.get_bit(4));
        assert!(0b0010_0000.get_bit(5));
        assert!(0b0100_0000.get_bit(6));
        assert!(0b1000_0000.get_bit(7));
    }

    #[test]
    fn get_bit_panic_ut() {
        for i in 8..=255 {
            assert!(std::panic::catch_unwind(|| 0b0000_0000.get_bit(i)).is_err());
        }
    }

    #[test]
    fn set_bit_panic_ut() {
        for i in 8..=255 {
            assert!(std::panic::catch_unwind(|| 0b0000_0000.set_bit(i, false)).is_err());
        }
    }
}
