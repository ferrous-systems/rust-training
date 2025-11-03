//! Register definitions for the CMSDK Timer

#[derive(derive_mmio::Mmio)]
#[repr(C)]
pub struct Registers {
    control: Control,
    value: u32,
    reload: u32,
    interrupt: Interrupt,
    _reserved: [u32; 0x3F0],
    peripheral_id_4: u32,
    peripheral_id_5: u32,
    peripheral_id_6: u32,
    peripheral_id_7: u32,
    peripheral_id_0: u32,
    peripheral_id_1: u32,
    peripheral_id_2: u32,
    peripheral_id_3: u32,
    component_id_0: u32,
    component_id_1: u32,
    component_id_2: u32,
    component_id_3: u32,
}

#[bitbybit::bitfield(u32, defmt_bitfields)]
pub struct Control {
    #[bit(3, rw)]
    interrupt_enable: bool,
    #[bit(2, rw)]
    external_input_as_clock: bool,
    #[bit(1, rw)]
    external_input_as_enable: bool,
    #[bit(0, rw)]
    enable: bool
}

#[bitbybit::bitfield(u32, default = 0x0, defmt_bitfields)]
pub struct Interrupt {
    /// Write 1 to clear.
    #[bit(0, rw)]
    interrupt_bit: bool
}
