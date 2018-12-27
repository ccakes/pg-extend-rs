// Copyright 2018 Benjamin Fry <benjaminfry@me.com>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(safe_packed_borrows)]
#![allow(clippy::useless_transmute)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::approx_constant)]
#![allow(clippy::transmute_ptr_to_ptr)]
#![allow(clippy::new_without_default_derive)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::unreadable_literal)]

include!(concat!(env!("OUT_DIR"), "/postgres.rs"));
