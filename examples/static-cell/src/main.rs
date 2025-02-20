#![no_std]
#![no_main]
#![deny(unsafe_code)]

use linker_sections::init_sections;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

const INITIAL_VALUE: u32 = 0xDEAD_BEEF;

#[allow(unsafe_code)]
// SAFETY:
// - Using static mut just to force compiler not to optimize it out in
//   this simple example
// - linker section gets initialized because of using `linker_sections`
#[unsafe(link_section = ".custom_data")]
static STATIC_ARRAY_A: StaticCell<u32> = StaticCell::new();

#[allow(unsafe_code)]
// SAFETY:
// - Using static mut just to force compiler not to optimize it out in
//   this simple example
// - linker section gets initialized because of using `linker_sections`
#[unsafe(link_section = ".custom_data")]
static STATIC_ARRAY_B: StaticCell<[u32; 256]> = StaticCell::new();

#[cortex_m_rt::pre_init]
unsafe fn pre_init() {
    init_sections!(custom_data);
}

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("main started");

    let array_a = STATIC_ARRAY_A.init_with(|| INITIAL_VALUE);
    let array_b = STATIC_ARRAY_B.init_with(|| [INITIAL_VALUE; 256]);

    // Check whether ARRAYs got initialized
    defmt::assert_eq!(array_a, &INITIAL_VALUE);
    defmt::assert_eq!(array_b, &[INITIAL_VALUE; 256]);

    // We have not paniced on assert
    defmt::info!("asserts ok");

    // End in an infinite loop
    #[allow(clippy::empty_loop)]
    loop {}
}
