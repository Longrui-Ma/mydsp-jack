/*! This library is developed with the aim of helping to set up a [5th year course option](https://embaudio.grame.fr/) 
 * around audio in Rust 
 * ([@INSA Lyon - Département Télécommunications, Services et Usages](https://telecom.insa-lyon.fr/) 
 * && [@Emeraude Research Team](https://team.inria.fr/emeraude/)).
 * 
 * The crate `mydsp-jack` written in Rust reproduces the functionalities found in 
 * [the original C++ `mydsp` library](https://github.com/grame-cncm/embaudio/tree/master/examples/teensy/libraries/mydsp).
 * 
 * # How to use 
 * `mydsp-jack` with [jack bindings for rust](https://github.com/RustAudio/rust-jack) should be added as dependencies
 * in another binary crate.
 * 
 * # Notes
 * ## `impl ProcessHandler for Patch` in user's binary crate
 * Because we want the modules to be connected in a sequential manner rather than [nested](https://github.com/Longrui-Ma/mydsp-jack-old), 
 * `impl ProcessHandler for Patch` (`ProcessHandler` trait defined in `jack-rust`) 
 * cannot be implemented in the `mydsp-jack` library crate, 
 * it can only be implemented in the user's binary crate by defining a new struct `Patch`. (**orphan rule**) 
 * So the final result is that `mydsp-jack` is completely independent of `jack-rust`.
 * 
 * ## #[derive(Debug)]
 * Structs implant `#[derive(Debug)]`, to use `std::fmt formatting` traits, derive from Debug trait.
 * 
 * Usage: `println?("{:?}", all_type)` or `println?("{:#?}", all_type)` to display any type without impl `fmt::Display` manually.
 * 
 * # TODO:
 * 1. finir + doc + autotest
 * 2. test
 * 2.5. jack block connect with code
 * 3. graph
 * 4. <https://github.com/orottier/web-audio-api-rs>
 * 5. <https://github.com/RustAudio/dasp>  
 * Parameters of f32 type take integers `i32` or `u32` as well. (use enum() instead?) (more info in echo.rs)
*/

// In each module, just: `use crate::import::*;` to add crates and modules used in all `mydsp-jack`'s modules.
// pub mod import{
// 	pub(crate) use
// 	// {
//         jack::{Port, AudioIn, AudioOut, Client, Control, ProcessHandler, ProcessScope};
//         // std     :: error::Error,
// 	// };
// }

pub mod dummy;
pub mod gain;
#[doc(alias = "PWM")]
pub mod pwm;
pub mod noise;
#[doc(alias = "sinetable")]
#[doc(alias = "table")]
pub mod sine_table;
#[doc(alias = "phase")]
pub mod phasor;
#[doc(alias = "sinewave")]
pub mod sine;
pub mod delay;
pub mod echo;
#[doc(alias = "varible_delay")]
#[doc(alias = "delay_varible")]
pub mod delay_var;
pub mod smooth;
pub mod one_zero;
pub mod distortion;
pub mod am;
pub mod fm;
// pub mod flanger;
// pub mod ks;

pub trait AudioComponent: Send + Sync{ 
    fn tick(&mut self, in_frame: f32) -> f32;
}

pub fn multiply(signals: &[f32]) -> f32 {
    signals.iter().product()
}

pub fn add(signals: &[f32]) -> f32 {
    signals.iter().sum()
}

pub fn get_type<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

pub fn print_type<T>(_: &T) { 
    println!("{:?}", std::any::type_name::<T>());
}
