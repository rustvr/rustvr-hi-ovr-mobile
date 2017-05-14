//  Copyright 2017 RustVR
//
//  This Source Code Form is subject to the terms of the Mozilla Public
//  License, v. 2.0. If a copy of the MPL was not distributed with this
//  file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate rustvr_core;

pub struct PluginHiOvrMobile {
}

impl rustvr_core::Plugin for PluginHiOvrMobile {
}

impl rustvr_core::PluginHi for PluginHiOvrMobile {
}

pub fn make_plugin() -> PluginHiOvrMobile {
    PluginHiOvrMobile {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
