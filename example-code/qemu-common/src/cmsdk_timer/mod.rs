//! Timer driver for the CMSDK TIMER

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
    /// NOTE: This does not affect the *current timer value*
    #[inline]
    pub fn write_reload(&mut self, value: u32) {
        self.regs.write_reload(value);
    }

    /// Overwrite the current timer value.
    #[inline]
    pub fn write_value(&mut self, value: u32) {
        self.regs.write_value(value);
    }

    /// Is the interrupt flag set?
    #[inline]
    pub fn interrupt_fired(&self) -> bool {
        self.regs.read_interrupt().interrupt_bit()
    }

    /// Clear the interrupt flag.
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

    /// Control whether the timer interrupt is enabled
    ///
    /// NOTE: You might also need to enable the interrupt in the NVIC
    #[inline]
    pub fn enable_interrupt(&mut self, enabled: bool) {
        self.regs
            .modify_control(|c| c.with_interrupt_enable(enabled));
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
    /// Timer driver structure.
    pub timer: Timer,
    sys_clk_hz: u32,
}

impl DelayTimer {
    /// Create a delay timer from a timer instance and a system clock frequency.
    pub fn new(timer: Timer, sys_clk_hz: u32) -> Self {
        Self { timer, sys_clk_hz }
    }
}

impl embedded_hal::delay::DelayNs for DelayTimer {
    fn delay_ns(&mut self, ns: u32) {
        const MAX_TICKS_PER_LOOP: u32 = u32::MAX;
        const NS_PER_SECOND: u64 = 1_000_000_000u64;

        let mut remaining_ticks =
            (ns as u64).saturating_mul(self.sys_clk_hz as u64) / NS_PER_SECOND;
        self.timer.disable();
        self.timer.enable_interrupt(true);
        while remaining_ticks > 0 {
            // cap to at most u32::MAX ticks per go-around this loop
            let wait_ticks = remaining_ticks.min(u64::from(MAX_TICKS_PER_LOOP)) as u32;
            self.timer.write_value(wait_ticks);
            self.timer.enable();
            while !self.timer.interrupt_fired() {
                core::hint::spin_loop();
            }
            self.timer.disable();
            self.timer.clear_interrupt();
            remaining_ticks -= u64::from(wait_ticks);
        }
        self.timer.enable_interrupt(false);
    }
}
