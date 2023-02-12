// SPDX-License-Identifier: GPL-2.0

//! rust scull driver, to learn how to develop kernel modules in rust

use kernel::prelude::*;

module! { 
    type: RustScull,
    name: "rust_scull",
    author: "Sarbojit Ganguly",
    description: "A basic rust driver",
    license: "GPL",
}

struct RustScull;

impl kernel::Module for RustScull {
    fn init(name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hello Rust World\n");
        Ok(RustScull)
    }
}
