/*cargo build --target x86_64-rusk.json
  cargo bootimage
  qemu-system-x86_64 -drive format=raw,file=target/bootimage-rusk_os.bin
*/
#![no_std] //disable link to standard library
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rusk_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use rusk_os::println;
use rusk_os::print;
use core::panic::PanicInfo;
pub mod frames;
use frames::ASCII_AMFOSS_top;
use frames::ASCII_AMFOSS_bottom;

#[no_mangle]
pub extern "C" fn _start() -> ! {
print(ASCII_AMFOSS_top);
print!("{}",ASCII_AMFOSS_bottom);


  rusk_os::init()

  #[cfg(test)]
  test_main();

  rusk_os::hlt_loop();
}

fn add_space_before_lines(input: &[u8]) -> [u8; 256] {
  let mut result = [0u8; 256];
  let mut input_index = 0;
  let mut result_index = 0;

  while input[input_index] != 0 {
      if input[input_index] == b'\n' {

          result[result_index] = b' ';
          result_index += 1;
      }

      result[result_index] = input[input_index];
      result_index += 1;
      input_index += 1;
  }

  result
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rusk_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rusk_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rusk_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
/*
It is also possible to write it to a USB stick and boot it on a real machine, 
but be careful to choose the correct device name, 
because everything on that device is overwritten:
---
> dd if=target/x86_64-rusk/debug/bootimage-rusk_os.bin of=/dev/sdX && sync
---
Where sdX is the device name of your USB stick. 
 */


