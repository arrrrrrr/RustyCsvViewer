
pub mod color_f {
    use ::winapi::um::d2d1::D2D1_COLOR_F;

    pub struct ColorF(u32);

    // Port of the predefined colors in the D2D1::ColorF namespace in d2d1helper.h
    impl ColorF {
        pub const ALICE_BLUE: u32 = 0xF0F8FF;
        pub const ANTIQUE_WHITE: u32 = 0xFAEBD7;
        pub const AQUA: u32 = 0x00FFFF;
        pub const AQUAMARINE: u32 = 0x7FFFD4;
        pub const AZURE: u32 = 0xF0FFFF;
        pub const BEIGE: u32 = 0xF5F5DC;
        pub const BISQUE: u32 = 0xFFE4C4;
        pub const BLACK: u32 = 0x000000;
        pub const BLANCHED_ALMOND: u32 = 0xFFEBCD;
        pub const BLUE: u32 = 0x0000FF;
        pub const BLUE_VIOLET: u32 = 0x8A2BE2;
        pub const BROWN: u32 = 0xA52A2A;
        pub const BURLY_WOOD: u32 = 0xDEB887;
        pub const CADET_BLUE: u32 = 0x5F9EA0;
        pub const CHARTREUSE: u32 = 0x7FFF00;
        pub const CHOCOLATE: u32 = 0xD2691E;
        pub const CORAL: u32 = 0xFF7F50;
        pub const CORNFLOWER_BLUE: u32 = 0x6495ED;
        pub const CORNSILK: u32 = 0xFFF8DC;
        pub const CRIMSON: u32 = 0xDC143C;
        pub const CYAN: u32 = 0x00FFFF;
        pub const DARK_BLUE: u32 = 0x00008B;
        pub const DARK_CYAN: u32 = 0x008B8B;
        pub const DARK_GOLDENROD: u32 = 0xB8860B;
        pub const DARK_GRAY: u32 = 0xA9A9A9;
        pub const DARK_GREEN: u32 = 0x006400;
        pub const DARK_KHAKI: u32 = 0xBDB76B;
        pub const DARK_MAGENTA: u32 = 0x8B008B;
        pub const DARK_OLIVE_GREEN: u32 = 0x556B2F;
        pub const DARK_ORANGE: u32 = 0xFF8C00;
        pub const DARK_ORCHID: u32 = 0x9932CC;
        pub const DARK_RED: u32 = 0x8B0000;
        pub const DARK_SALMON: u32 = 0xE9967A;
        pub const DARK_SEA_GREEN: u32 = 0x8FBC8F;
        pub const DARK_SLATE_BLUE: u32 = 0x483D8B;
        pub const DARK_SLATE_GRAY: u32 = 0x2F4F4F;
        pub const DARK_TURQUOISE: u32 = 0x00CED1;
        pub const DARK_VIOLET: u32 = 0x9400D3;
        pub const DEEP_PINK: u32 = 0xFF1493;
        pub const DEEP_SKY_BLUE: u32 = 0x00BFFF;
        pub const DIM_GRAY: u32 = 0x696969;
        pub const DODGER_BLUE: u32 = 0x1E90FF;
        pub const FIREBRICK: u32 = 0xB22222;
        pub const FLORAL_WHITE: u32 = 0xFFFAF0;
        pub const FOREST_GREEN: u32 = 0x228B22;
        pub const FUCHSIA: u32 = 0xFF00FF;
        pub const GAINSBORO: u32 = 0xDCDCDC;
        pub const GHOST_WHITE: u32 = 0xF8F8FF;
        pub const GOLD: u32 = 0xFFD700;
        pub const GOLDENROD: u32 = 0xDAA520;
        pub const GRAY: u32 = 0x808080;
        pub const GREEN: u32 = 0x008000;
        pub const GREEN_YELLOW: u32 = 0xADFF2F;
        pub const HONEYDEW: u32 = 0xF0FFF0;
        pub const HOT_PINK: u32 = 0xFF69B4;
        pub const INDIAN_RED: u32 = 0xCD5C5C;
        pub const INDIGO: u32 = 0x4B0082;
        pub const IVORY: u32 = 0xFFFFF0;
        pub const KHAKI: u32 = 0xF0E68C;
        pub const LAVENDER: u32 = 0xE6E6FA;
        pub const LAVENDER_BLUSH: u32 = 0xFFF0F5;
        pub const LAWN_GREEN: u32 = 0x7CFC00;
        pub const LEMON_CHIFFON: u32 = 0xFFFACD;
        pub const LIGHT_BLUE: u32 = 0xADD8E6;
        pub const LIGHT_CORAL: u32 = 0xF08080;
        pub const LIGHT_CYAN: u32 = 0xE0FFFF;
        pub const LIGHT_GOLDENROD_YELLOW: u32 = 0xFAFAD2;
        pub const LIGHT_GREEN: u32 = 0x90EE90;
        pub const LIGHT_GRAY: u32 = 0xD3D3D3;
        pub const LIGHT_PINK: u32 = 0xFFB6C1;
        pub const LIGHT_SALMON: u32 = 0xFFA07A;
        pub const LIGHT_SEA_GREEN: u32 = 0x20B2AA;
        pub const LIGHT_SKY_BLUE: u32 = 0x87CEFA;
        pub const LIGHT_SLATE_GRAY: u32 = 0x778899;
        pub const LIGHT_STEEL_BLUE: u32 = 0xB0C4DE;
        pub const LIGHT_YELLOW: u32 = 0xFFFFE0;
        pub const LIME: u32 = 0x00FF00;
        pub const LIME_GREEN: u32 = 0x32CD32;
        pub const LINEN: u32 = 0xFAF0E6;
        pub const MAGENTA: u32 = 0xFF00FF;
        pub const MAROON: u32 = 0x800000;
        pub const MEDIUM_AQUAMARINE: u32 = 0x66CDAA;
        pub const MEDIUM_BLUE: u32 = 0x0000CD;
        pub const MEDIUM_ORCHID: u32 = 0xBA55D3;
        pub const MEDIUM_PURPLE: u32 = 0x9370DB;
        pub const MEDIUM_SEA_GREEN: u32 = 0x3CB371;
        pub const MEDIUM_SLATE_BLUE: u32 = 0x7B68EE;
        pub const MEDIUM_SPRING_GREEN: u32 = 0x00FA9A;
        pub const MEDIUM_TURQUOISE: u32 = 0x48D1CC;
        pub const MEDIUM_VIOLET_RED: u32 = 0xC71585;
        pub const MIDNIGHT_BLUE: u32 = 0x191970;
        pub const MINT_CREAM: u32 = 0xF5FFFA;
        pub const MISTY_ROSE: u32 = 0xFFE4E1;
        pub const MOCCASIN: u32 = 0xFFE4B5;
        pub const NAVAJO_WHITE: u32 = 0xFFDEAD;
        pub const NAVY: u32 = 0x000080;
        pub const OLD_LACE: u32 = 0xFDF5E6;
        pub const OLIVE: u32 = 0x808000;
        pub const OLIVE_DRAB: u32 = 0x6B8E23;
        pub const ORANGE: u32 = 0xFFA500;
        pub const ORANGE_RED: u32 = 0xFF4500;
        pub const ORCHID: u32 = 0xDA70D6;
        pub const PALE_GOLDENROD: u32 = 0xEEE8AA;
        pub const PALE_GREEN: u32 = 0x98FB98;
        pub const PALE_TURQUOISE: u32 = 0xAFEEEE;
        pub const PALE_VIOLET_RED: u32 = 0xDB7093;
        pub const PAPAYA_WHIP: u32 = 0xFFEFD5;
        pub const PEACH_PUFF: u32 = 0xFFDAB9;
        pub const PERU: u32 = 0xCD853F;
        pub const PINK: u32 = 0xFFC0CB;
        pub const PLUM: u32 = 0xDDA0DD;
        pub const POWDER_BLUE: u32 = 0xB0E0E6;
        pub const PURPLE: u32 = 0x800080;
        pub const RED: u32 = 0xFF0000;
        pub const ROSY_BROWN: u32 = 0xBC8F8F;
        pub const ROYAL_BLUE: u32 = 0x4169E1;
        pub const SADDLE_BROWN: u32 = 0x8B4513;
        pub const SALMON: u32 = 0xFA8072;
        pub const SANDY_BROWN: u32 = 0xF4A460;
        pub const SEA_GREEN: u32 = 0x2E8B57;
        pub const SEA_SHELL: u32 = 0xFFF5EE;
        pub const SIENNA: u32 = 0xA0522D;
        pub const SILVER: u32 = 0xC0C0C0;
        pub const SKY_BLUE: u32 = 0x87CEEB;
        pub const SLATE_BLUE: u32 = 0x6A5ACD;
        pub const SLATE_GRAY: u32 = 0x708090;
        pub const SNOW: u32 = 0xFFFAFA;
        pub const SPRING_GREEN: u32 = 0x00FF7F;
        pub const STEEL_BLUE: u32 = 0x4682B4;
        pub const TAN: u32 = 0xD2B48C;
        pub const TEAL: u32 = 0x008080;
        pub const THISTLE: u32 = 0xD8BFD8;
        pub const TOMATO: u32 = 0xFF6347;
        pub const TURQUOISE: u32 = 0x40E0D0;
        pub const VIOLET: u32 = 0xEE82EE;
        pub const WHEAT: u32 = 0xF5DEB3;
        pub const WHITE: u32 = 0xFFFFFF;
        pub const WHITE_SMOKE: u32 = 0xF5F5F5;
        pub const YELLOW: u32 = 0xFFFF00;
        pub const YELLOW_GREEN: u32 = 0x9ACD32;
    }

    impl From<ColorF> for D2D1_COLOR_F {
        fn from(value: ColorF) -> Self {
            from_rgb(value.0)
        }
    }

    fn from_rgb(value: u32) -> D2D1_COLOR_F {
        let red_shift: u32   = 16;
        let green_shift: u32 = 8;
        let blue_shift: u32  = 0;

        let red_mask: u32   = 0xff << red_shift;
        let green_mask: u32 = 0xff << green_shift;
        let blue_mask: u32  = 0xff << blue_shift;

        // alpha is always 1.0
        let a: f32 = 1.0;
        let r: f32 = ((value & red_mask) >> red_shift) as f32 / 255.;
        let g: f32 = ((value & green_mask) >> green_shift) as f32 / 255.;
        let b: f32 = ((value & blue_mask) >> blue_shift) as f32 / 255.;

        D2D1_COLOR_F { r, g, b, a }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn from_known_color() {
            let expected = D2D1_COLOR_F { r: 0., g: 0., b: 0., a: 1. };
            let actual = D2D1_COLOR_F::from(ColorF(ColorF::BLACK));
            assert_eq!(expected.r, actual.r);
            assert_eq!(expected.g, actual.g);
            assert_eq!(expected.b, actual.b);
            assert_eq!(expected.a, actual.a);

            let expected = D2D1_COLOR_F { r: 1., g: 1., b: 1., a: 1. };
            let actual = D2D1_COLOR_F::from(ColorF(ColorF::WHITE));

            assert_eq!(expected.r, actual.r);
            assert_eq!(expected.g, actual.g);
            assert_eq!(expected.b, actual.b);
            assert_eq!(expected.a, actual.a);
        }

        #[test]
        fn from_packed_rgb_color() {
            let expected = D2D1_COLOR_F { r: 0.502, g: 0.502, b: 0.502, a: 1.0 };
            let actual = D2D1_COLOR_F::from(ColorF(0x808080));

            assert!(actual.r >= 0.5);
            assert!(actual.g >= 0.5);
            assert!(actual.b >= 0.5);
            assert!(actual.a == 1.0);
        }
    }
}