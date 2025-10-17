pub mod registers;

pub struct Timer {
    regs: registers::MmioRegisters<'static>,
}

impl Timer {
    #[inline]
    pub fn new(regs: registers::MmioRegisters<'static>) -> Self {
        Self { regs }
    }

    #[inline]
    pub fn set_frequency(&mut self, sys_clk_hz: u32, freq_hz: u32) {
        self.write_reload(sys_clk_hz / freq_hz);
    }

    #[inline]
    pub fn write_reload(&mut self, value: u32) {
        self.regs.write_reload(value);
    }

    #[inline]
    pub fn read_interrupt_bit(&self) -> bool {
        self.regs.read_interrupt().interrupt_bit()
    }

    #[inline]
    pub fn clear_interrupt(&mut self) {
        self.regs.write_interrupt(
            registers::Interrupt::builder()
                .with_interrupt_bit(true)
                .build(),
        );
    }

    #[inline]
    pub fn enable(&mut self) {
        self.regs.modify_control(|c| c.with_enable(true));
    }

    /// This does NOT enable the interrupt in the NVIC.
    #[inline]
    pub fn enable_interrupt(&mut self) {
        self.regs.modify_control(|c| c.with_interrupt_enable(true));
    }

    #[inline]
    pub fn read(&self) -> u32 {
        self.regs.read_value()
    }

    #[inline]
    pub fn disable(&mut self) {
        self.regs.modify_control(|c| c.with_enable(false));
    }
}

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
            let reload = core::cmp::min(remaining_ticks as u32, u32::MAX);
            self.timer.write_reload(reload);

            // I would prefer to use the interrupt bit, but it is not latched..
            let mut last_read = self.timer.read();
            loop {
                let current = self.timer.read();
                if current > last_read {
                    break;
                }
                last_read = current;
            }
            remaining_ticks -= reload as u64;
        }
    }
}
