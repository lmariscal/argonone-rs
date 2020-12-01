# ArgonONE-rs

This is my implementation to manage the fan in the ArgonONE case. It was created primarily because RPi.GPIO was not recognizing the aarch64 OS as a RaspberryPi, and all my attempts to fix it failed.

At the moment only the fan manager is written as it is the only feature I currently need. No config file is provided but the min-max temperatures for the fan speed are in easily modifiable constants. The way the fan speed is selected is via a linear value, min being 10 and max being 100.

Default min temperature is 51 and max temperature is 69.

I don't plan on maintaining this project for public consumption, but the code is under the zlib licence so you can get your hands dirty in my lengthy 50 lines of Rust code. PRs are welcome, or just fork it.

PD: First time writing something in Rust.