pub mod registers;

/// Simple Timer driver.
///
/// If you require an [embedded_hal::delay::DelayNs] implementation, the [DelayTimer] provides
/// this functionality.
pub struct Timer {
    regs: registers::MmioRegisters<'static>,
}

impl Timer {
    /// Create a new timer driver from a given peripheral instance block.
    #[inline]
    pub fn new(regs: registers::MmioRegisters<'static>) -> Self {
        Self { regs }
    }

    /// Set the reload frequency from the given system clock and target frequency.
    #[inline]
    pub fn set_frequency(&mut self, sys_clk_hz: u32, freq_hz: u32) {
        self.write_reload(sys_clk_hz / freq_hz);
    }

    /// Write the reload value.
    ///
    /// This does not overwrite to the current value register.
    #[inline]
    pub fn write_reload(&mut self, value: u32) {
        self.regs.write_reload(value);
    }

    /// Overwrite the current timer value.
    #[inline]
    pub fn write_value(&mut self, value: u32) {
        self.regs.write_value(value);
    }

    /// Clear the interrupt flag.
    ///
    /// NOTE: This might not be useful because the interrupt is not latched.
    #[inline]
    pub fn clear_interrupt(&mut self) {
        self.regs.write_interrupt(
            registers::Interrupt::builder()
                .with_interrupt_bit(true)
                .build(),
        );
    }

    /// Enable the timer.
    #[inline]
    pub fn enable(&mut self) {
        self.regs.modify_control(|c| c.with_enable(true));
    }

    /// This does NOT enable the interrupt in the NVIC.
    #[inline]
    pub fn enable_interrupt(&mut self) {
        self.regs.modify_control(|c| c.with_interrupt_enable(true));
    }

    /// Read the timer value.
    #[inline]
    pub fn read(&self) -> u32 {
        self.regs.read_value()
    }

    /// Disable the timer.
    #[inline]
    pub fn disable(&mut self) {
        self.regs.modify_control(|c| c.with_enable(false));
    }
}

/// Delay timer which implements the [embedded_hal::delay::DelayNs] trait.
pub struct DelayTimer {
    pub timer: Timer,
    sys_clk_hz: u32,
}

impl DelayTimer {
    pub fn new(timer: Timer, sys_clk_hz: u32) -> Self {
        Self { timer, sys_clk_hz }
    }
}

impl embedded_hal::delay::DelayNs for DelayTimer {
    fn delay_ns(&mut self, ns: u32) {
        let mut remaining_ticks =
            (ns as u64).saturating_mul(self.sys_clk_hz as u64) / 1_000_000_000u64;
        if remaining_ticks == 0 {
            return;
        }
        // Setup timer once
        self.timer.disable();
        // Not strictly necessary, but enabling the timer with reload 0 leads to a warning
        // which mentions that the time was diasbled..
        self.timer.write_reload(u32::MAX);
        self.timer.enable();

        while remaining_ticks > 0 {
            let wait_ticks = core::cmp::min(remaining_ticks as u32, u32::MAX / 2);
            let threshold = u32::MAX - wait_ticks;
            self.timer.write_value(u32::MAX);

            // I would prefer to use the interrupt bit, but it is not latched..
            while self.timer.read() > threshold {}
            remaining_ticks -= wait_ticks as u64;
        }
    }
}
