use std::convert::TryInto;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use xmas_elf::program;

use crate::{Memory, Registers, MEM_START, PC};
