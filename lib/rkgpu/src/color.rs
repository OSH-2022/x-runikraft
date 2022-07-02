// SPDX-License-Identifier: BSD-3-Clause
// rkgpu/color.rs

// Authors:  郭耸霄 <logname@mail.ustc.edu.cn>

// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in the
//    documentation and/or other materials provided with the distribution.
// 3. Neither the name of the copyright holder nor the names of its
//    contributors may be used to endorse or promote products derived from
//    this software without specific prior written permission.
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.
#[derive(Clone, Copy)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Color {
            red,
            green,
            blue,
        }
    }
}

pub const ALICE_BLUE: Color = Color::new(240, 248, 255);
pub const ANTIQUE_WHITE: Color = Color::new(250, 235, 215);
pub const AQUA: Color = Color::new(0, 255, 255);
pub const AQUAMARINE: Color = Color::new(127, 255, 212);
pub const AZURE: Color = Color::new(240, 255, 255);
pub const BEIGE: Color = Color::new(245, 245, 220);
pub const BISQUE: Color = Color::new(255, 228, 196);
pub const BLACK: Color = Color::new(0, 0, 0);
pub const BLANCHED_ALMOND: Color = Color::new(255, 235, 205);
pub const BLUE: Color = Color::new(0, 0, 255);
pub const BLUE_VIOLET: Color = Color::new(138, 43, 226);
pub const BROWN: Color = Color::new(165, 42, 42);
pub const BURLY_WOOD: Color = Color::new(222, 184, 135);
pub const CADET_BLUE: Color = Color::new(95, 158, 160);
pub const CHARTREUSE: Color = Color::new(127, 255, 0);
pub const CHOCOLATE: Color = Color::new(210, 105, 30);
pub const CORAL: Color = Color::new(255, 127, 80);
pub const CORNFLOWER_BLUE: Color = Color::new(100, 149, 237);
pub const CORNSILK: Color = Color::new(255, 248, 220);
pub const CRIMSON: Color = Color::new(220, 20, 60);
pub const CYAN: Color = Color::new(0, 255, 255);
pub const DARK_BLUE: Color = Color::new(0, 0, 139);
pub const DARK_CYAN: Color = Color::new(0, 139, 139);
pub const DARK_GOLDEN_ROD: Color = Color::new(184, 134, 11);
pub const DARK_GRAY: Color = Color::new(169, 169, 169);
pub const DARK_GREEN: Color = Color::new(0, 100, 0);
pub const DARK_KHAKI: Color = Color::new(189, 183, 107);
pub const DARK_MAGENTA: Color = Color::new(139, 0, 139);
pub const DARK_OLIVE_GREEN: Color = Color::new(85, 107, 47);
pub const DARK_ORANGE: Color = Color::new(255, 140, 0);
pub const DARK_ORCHID: Color = Color::new(153, 50, 204);
pub const DARK_RED: Color = Color::new(139, 0, 0);
pub const DARK_SALMON: Color = Color::new(233, 150, 122);
pub const DARK_SEA_GREEN: Color = Color::new(143, 188, 143);
pub const DARK_SLATE_BLUE: Color = Color::new(72, 61, 139);
pub const DARK_SLATE_GRAY: Color = Color::new(47, 79, 79);
pub const DARK_TURQUOISE: Color = Color::new(0, 206, 209);
pub const DARK_VIOLET: Color = Color::new(148, 0, 211);
pub const DEEP_PINK: Color = Color::new(255, 20, 147);
pub const DEEP_SKY_BLUE: Color = Color::new(0, 191, 255);
pub const DIM_GRAY: Color = Color::new(105, 105, 105);
pub const DODGER_BLUE: Color = Color::new(30, 144, 255);
pub const FIRE_BRICK: Color = Color::new(178, 34, 34);
pub const FLORAL_WHITE: Color = Color::new(255, 250, 240);
pub const FOREST_GREEN: Color = Color::new(34, 139, 34);
pub const FUCHSIA: Color = Color::new(255, 0, 255);
pub const GAINSBORO: Color = Color::new(220, 220, 220);
pub const GHOST_WHITE: Color = Color::new(248, 248, 255);
pub const GOLD: Color = Color::new(255, 215, 0);
pub const GOLDEN_ROD: Color = Color::new(218, 165, 32);
pub const GRAY: Color = Color::new(128, 128, 128);
pub const GREEN: Color = Color::new(0, 128, 0);
pub const GREEN_YELLOW: Color = Color::new(173, 255, 47);
pub const HONEY_DEW: Color = Color::new(240, 255, 240);
pub const HOT_PINK: Color = Color::new(255, 105, 180);
pub const INDIAN_RED: Color = Color::new(205, 92, 92);
pub const INDIGO: Color = Color::new(75, 0, 130);
pub const IVORY: Color = Color::new(255, 255, 240);
pub const KHAKI: Color = Color::new(240, 230, 140);
pub const LAVENDER: Color = Color::new(230, 230, 250);
pub const LAVENDER_BLUSH: Color = Color::new(255, 240, 245);
pub const LAWN_GREEN: Color = Color::new(124, 252, 0);
pub const LEMON_CHIFFON: Color = Color::new(255, 250, 205);
pub const LIGHT_BLUE: Color = Color::new(173, 216, 230);
pub const LIGHT_CORAL: Color = Color::new(240, 128, 128);
pub const LIGHT_CYAN: Color = Color::new(224, 255, 255);
pub const LIGHT_GOLDEN_ROD_YELLOW: Color = Color::new(250, 250, 210);
pub const LIGHT_GRAY: Color = Color::new(211, 211, 211);
pub const LIGHT_GREEN: Color = Color::new(144, 238, 144);
pub const LIGHT_PINK: Color = Color::new(255, 182, 193);
pub const LIGHT_SALMON: Color = Color::new(255, 160, 122);
pub const LIGHT_SEA_GREEN: Color = Color::new(32, 178, 170);
pub const LIGHT_SKY_BLUE: Color = Color::new(135, 206, 250);
pub const LIGHT_SLATE_GRAY: Color = Color::new(119, 136, 153);
pub const LIGHT_STEEL_BLUE: Color = Color::new(176, 196, 222);
pub const LIGHT_YELLOW: Color = Color::new(255, 255, 224);
pub const LIME: Color = Color::new(0, 255, 0);
pub const LIME_GREEN: Color = Color::new(50, 205, 50);
pub const LINEN: Color = Color::new(250, 240, 230);
pub const MAGENTA: Color = Color::new(255, 0, 255);
pub const MAROON: Color = Color::new(128, 0, 0);
pub const MEDIUM_AQUAMARINE: Color = Color::new(102, 205, 170);
pub const MEDIUM_BLUE: Color = Color::new(0, 0, 205);
pub const MEDIUM_ORCHID: Color = Color::new(186, 85, 211);
pub const MEDIUM_PURPLE: Color = Color::new(147, 112, 219);
pub const MEDIUM_SEA_GREEN: Color = Color::new(60, 179, 113);
pub const MEDIUM_SLATE_BLUE: Color = Color::new(123, 104, 238);
pub const MEDIUM_SPRING_GREEN: Color = Color::new(0, 250, 154);
pub const MEDIUM_TURQUOISE: Color = Color::new(72, 209, 204);
pub const MEDIUM_VIOLET_RED: Color = Color::new(199, 21, 133);
pub const MIDNIGHT_BLUE: Color = Color::new(25, 25, 112);
pub const MINT_CREAM: Color = Color::new(245, 255, 250);
pub const MISTY_ROSE: Color = Color::new(255, 228, 225);
pub const MOCCASIN: Color = Color::new(255, 228, 181);
pub const NAVAJO_WHITE: Color = Color::new(255, 222, 173);
pub const NAVY: Color = Color::new(0, 0, 128);
pub const OLD_LACE: Color = Color::new(253, 245, 230);
pub const OLIVE: Color = Color::new(128, 128, 0);
pub const OLIVE_DRAB: Color = Color::new(107, 142, 35);
pub const ORANGE: Color = Color::new(255, 165, 0);
pub const ORANGE_RED: Color = Color::new(255, 69, 0);
pub const ORCHID: Color = Color::new(218, 112, 214);
pub const PALE_GOLDEN_ROD: Color = Color::new(238, 232, 170);
pub const PALE_GREEN: Color = Color::new(152, 251, 152);
pub const PALE_TURQUOISE: Color = Color::new(175, 238, 238);
pub const PALE_VIOLET_RED: Color = Color::new(219, 112, 147);
pub const PAPAYA_WHIP: Color = Color::new(255, 239, 213);
pub const PEACH_PUFF: Color = Color::new(255, 218, 185);
pub const PERU: Color = Color::new(205, 133, 63);
pub const PINK: Color = Color::new(255, 192, 203);
pub const PLUM: Color = Color::new(221, 160, 221);
pub const POWDER_BLUE: Color = Color::new(176, 224, 230);
pub const PURPLE: Color = Color::new(128, 0, 128);
pub const REBECCA_PURPLE: Color = Color::new(102, 51, 153);
pub const RED: Color = Color::new(255, 0, 0);
pub const ROSY_BROWN: Color = Color::new(188, 143, 143);
pub const ROYAL_BLUE: Color = Color::new(65, 105, 225);
pub const SADDLE_BROWN: Color = Color::new(139, 69, 19);
pub const SALMON: Color = Color::new(250, 128, 114);
pub const SANDY_BROWN: Color = Color::new(244, 164, 96);
pub const SEA_GREEN: Color = Color::new(46, 139, 87);
pub const SEA_SHELL: Color = Color::new(255, 245, 238);
pub const SIENNA: Color = Color::new(160, 82, 45);
pub const SILVER: Color = Color::new(192, 192, 192);
pub const SKY_BLUE: Color = Color::new(135, 206, 235);
pub const SLATE_BLUE: Color = Color::new(106, 90, 205);
pub const SLATE_GRAY: Color = Color::new(112, 128, 144);
pub const SNOW: Color = Color::new(255, 250, 250);
pub const SPRING_GREEN: Color = Color::new(0, 255, 127);
pub const STEEL_BLUE: Color = Color::new(70, 130, 180);
pub const TAN: Color = Color::new(210, 180, 140);
pub const TEAL: Color = Color::new(0, 128, 128);
pub const THISTLE: Color = Color::new(216, 191, 216);
pub const TOMATO: Color = Color::new(255, 99, 71);
pub const TURQUOISE: Color = Color::new(64, 224, 208);
pub const VIOLET: Color = Color::new(238, 130, 238);
pub const WHEAT: Color = Color::new(245, 222, 179);
pub const WHITE: Color = Color::new(255, 255, 255);
pub const WHITE_SMOKE: Color = Color::new(245, 245, 245);
pub const YELLOW: Color = Color::new(255, 255, 0);
pub const YELLOW_GREEN: Color = Color::new(154, 205, 50);
