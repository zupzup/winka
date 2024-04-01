# winka

## WGPU

* https://github.com/linebender/vello
* Text
    * `https://github.com/grovesNL/glyphon`
        * https://github.com/gfx-rs/wgpu/wiki/Encapsulating-Graphics-Work
    * `https://github.com/Blatko1/wgpu-text`
* https://sotrh.github.io/learn-wgpu/beginner/tutorial1-window/#boring-i-know
* `https://www.youtube.com/playlist?list=PLWtPciJ1UMuBs_3G-jFrMJnM5ZMKgl37H` - videos
* example: `https://github.com/gfx-rs/wgpu/blob/trunk/examples/src/hello_triangle/mod.rs`
* https://github.com/raphamorim/rio
* https://github.com/LelsersLasers/UnderwaterWorld
* https://github.com/rofrol/awesome-wgpu
* WGPU 2d Engine https://github.com/darthdeus/comfy
* https://github.com/lapce/lapce - built using wgpu
    * via https://github.com/lapce/floem
    * https://github.com/lapce/floem/blob/main/vger/src/lib.rs
* https://github.com/emilk/egui - built using wgpu
    * via https://github.com/emilk/egui/blob/master/crates/egui-wgpu/src/lib.rs
    * https://github.com/emilk/egui/blob/master/crates/egui-wgpu/src/lib.rs

## TODO

* Create a separate text input field
    * scroll field to the right on entering text
* refactor clean up blinking logic
* Create "success" notification on clicking submit with the entered text
* Refactor fontsystem to use global (once cell, arc, lock)
* refactor to not use booleans as fields and arguments, use enum types as arguments, or Option
