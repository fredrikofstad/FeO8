<p align="center">
    <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://github.com/fredrikofstad/FeO8/blob/master/res/logo_dark.png?raw=true">
    <source media="(prefers-color-scheme: light)" srcset="https://github.com/fredrikofstad/FeO8/blob/master/res/logo_light.png?raw=true">
    <img alt="FeO8 logo" width="400" src="https://github.com/fredrikofstad/FeO8/blob/master/res/logo_light.png?raw=true">
    </picture>
</p>

## FeO8 - Chip-8 Emulator written in Rust

Currently contains around 30 cpu instructions as well as the basic components of chip-8's specifications.

### Specifications

Memory: Direct access to 4kb of RAM

Display: 64 x 32px monochrome

Program Counter (16bit)- current instruction in memory

Index Register (16bit)- points to memory addresses

16 general purpose registers(8bit) - registers 0 - F, 
    F register used for flags (like carry)
    
Stack: An array of 16 addresses (16bit) for storing subroutine locations and current instruction location

Delay Timer (8bit) - decremented 60Hz until 0 is reached

Sound Timer (8bit) - decremented 60Hz used to time beeps
