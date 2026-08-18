pub mod cpu;
pub mod consts;

pub mod lib { 
    pub use crate::cpu::*;
    pub use crate::consts::*;
}