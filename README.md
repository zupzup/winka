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

* check double check of enum in godbolt
* don't use booleans, use enum types as arguments, or Option
* Create a separate text input field
    * if it's active and text is input, it shows in the text field
    * Create a blinking cursor within it
        * https://github.com/emilk/egui/blob/master/crates/egui/src/widgets/text_edit/builder.rs
        * https://github.com/emilk/egui/blob/master/crates/epaint/src/text/cursor.rs
* Create "success" notification on clicking submit with the entered text
* Refactor fontsystem to use global (once cell, arc, lock)
* only load 1 custom font
    * https://github.com/iced-rs/iced/blob/3013463baa71504488a20436beb3db87ecb66df0/graphics/src/text.rs

