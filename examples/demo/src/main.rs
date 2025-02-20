#![no_std]
#![no_main]
#![deny(unsafe_code)]

use linker_sections::init_sections;
use {defmt_rtt as _, panic_probe as _};

const INITIAL_VALUE: u32 = 0xDEAD_BEEF;

#[allow(unsafe_code)]
// SAFETY:
// - Using static mut just to force compiler not to optimize it out in
//   this simple example
// - linker section gets initialized because of using `linker_sections`
#[unsafe(link_section = ".custom_data_a")]
static mut STATIC_ARRAY_A: u32 = INITIAL_VALUE;

#[allow(unsafe_code)]
// SAFETY:
// - Using static mut just to force compiler not to optimize it out in
//   this simple example
// - linker section gets initialized because of using `linker_sections`
#[unsafe(link_section = ".custom_data_b")]
static mut STATIC_ARRAY_B: [u32; 256] = [INITIAL_VALUE; 256];

#[cortex_m_rt::pre_init]
unsafe fn pre_init() {
    init_sections!(custom_data_a, custom_data_b);
}

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("main started");

    #[allow(unsafe_code)]
    // SAFETY: This is the only place accessing that static mut variable
    unsafe {
        // Check whether ARRAYs got initialized
        defmt::assert_eq!(STATIC_ARRAY_A, INITIAL_VALUE);
        defmt::assert_eq!(STATIC_ARRAY_B, [INITIAL_VALUE; 256]);
    }

    // We have not paniced on assert
    defmt::info!("asserts ok");

    // End in an infinite loop
    #[allow(clippy::empty_loop)]
    loop {}
}
