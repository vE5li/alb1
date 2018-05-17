pub mod interface;
pub use self::interface::interface_peripherals_base;

// get the address of a peripheral based on it's offet so the peripherals base
macro_rules! peripheral {
    (gpio) => ($crate::peripherals::interface_peripherals_base + 0x200000);
    (mailbox0) => ($crate::peripherals::interface_peripherals_base + 0xb880);
    (mailbox1) => ($crate::peripherals::interface_peripherals_base + 0xb8a0);
    (undefined) => (panic!("undefined peripheral {}", undefined));
}

#[macro_use]
pub mod logger;
pub mod input;
pub mod mailman;
pub mod gpio;

// initialize all peripherals
pub fn initialize() {
    // turn off the act led
    mailman::push_tag(&[0x38041, 8, 0, 130, 0]);
    mailman::pop_tag();

    // set physical pin 40 as output
    gpio::set_function(21, gpio::Function::Output);
}

// temporary function
pub fn idle() -> ! {
    use self::input::{ InputEvent, InputMode };
    loop {
        let event = InputEvent::from_raw(unsafe { interface::read_event() });
        match event.tupled() {
            (InputMode::Setting, b'q') => unsafe { interface::reboot() },
            (InputMode::Action, b'o') => self::gpio::set_state(21, false),
            (InputMode::Action, b'p') => self::gpio::set_state(21, true),
            _ => {
                if let Some(character) = event.ascii() {
                    log!("key pressed {}", character);
                }
            },
        }
    }
}