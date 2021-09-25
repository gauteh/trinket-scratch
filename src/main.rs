#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(not(test))]
extern crate panic_halt;

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn main() {
    _main_()
}

pub fn _main_() {
    let i: u8 = 0;
    loop {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
