/// Allows us to make numbers larger by some fixed amount
#[repr(C)]
pub struct MagicAdder {
    amount: u32,
}

impl MagicAdder {
    /// Create a new [`MagicAdder`]
    pub fn new(amount: u32) -> MagicAdder {
        MagicAdder { amount }
    }

    /// Process a value using this [`MagicAdder`]
    pub fn process_value(&self, value: u32) -> u32 {
        self.amount + value
    }
}

/// Create a new [`MagicAdder`], on the stack
#[no_mangle]
pub extern "C" fn magicadder_new(amount: u32) -> MagicAdder {
    MagicAdder::new(amount)
}

/// Use a [`MagicAdder`] to process a number
#[no_mangle]
pub extern "C" fn magicadder_process_value(adder: *const MagicAdder, value: u32) -> u32 {
    if let Some(ma) = unsafe { adder.as_ref() } {
        ma.process_value(value)
    } else {
        0
    }
}

/// Heap allocate a new [`MagicAdder`]
#[no_mangle]
pub extern "C" fn magicadder_allocate(amount: u32) -> Box<MagicAdder> {
    Box::new(MagicAdder::new(amount))
}

/// Destroy a [`MagicAdder`] that was created with `magicadder_allocate`
#[no_mangle]
pub extern "C" fn magicadder_free(_adder: Option<Box<MagicAdder>>) {
    // dropping the box frees it, and Option allows it to be nullptr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ma = MagicAdder::new(5);
        assert_eq!(6, ma.process_value(1));
        assert_eq!(10, ma.process_value(5));
    }
}
