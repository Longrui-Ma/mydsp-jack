# Rust Audio Processing @ INSA Lyon -- mydsp Repository
This library is developed with the aim of helping to set up a [5th year course option](https://embaudio.grame.fr/) 
around audio in Rust 
([@INSA Lyon - Département Télécommunications, Services et Usages](https://telecom.insa-lyon.fr/) 
&& [@Emeraude Research Team](https://team.inria.fr/emeraude/)).

The crate `mydsp-jack` written in Rust reproduces the functionalities found in 
[the original C++ `mydsp` library](https://github.com/grame-cncm/embaudio/tree/master/examples/teensy/libraries/mydsp).

# From old to new one (in process)
Modules to be connected in a sequential manner rather than nested <https://github.com/Longrui-Ma/mydsp-jack-old>.

# How to use 
`mydsp-jack` with [jack bindings for rust](https://github.com/RustAudio/rust-jack) should be added as dependencies
in another binary crate.

# Notes
## `impl ProcessHandler for Patch` in user's binary crate
Because we want the modules to be connected in a sequential manner rather than [nested](https://github.com/Longrui-Ma/mydsp-jack-old), 
`impl ProcessHandler for Patch` (`ProcessHandler` trait defined in `jack-rust`) 
cannot be implemented in the `mydsp-jack` library crate, 
it can only be implemented in the user's binary crate by defining a new struct `Patch`. (**orphan rule**) 
So the final result is that `mydsp-jack` is completely independent of `jack-rust`.