// TODO: Eventually we would want these fonts to be specified externally

use crate::rich_text::FontWeight;

pub struct FontData {
    pub bytes: &'static [u8],
    weight: FontWeight,
    italic: bool,
}

pub const FONTS: &[FontData] = &[
    FontData {
        bytes: include_bytes!("../assets/fonts/inter-static/Inter-Black.ttf"),
        weight: FontWeight::Black,
        italic: false,
    },
    FontData {
        bytes: include_bytes!("../assets/fonts/inter-static/Inter-Bold.ttf"),
        weight: FontWeight::Bold,
        italic: false,
    },
    FontData {
        bytes: include_bytes!("../assets/fonts/inter-static/Inter-BoldItalic.ttf"),
        weight: FontWeight::Bold,
        italic: true,
    },
    FontData {
        bytes: include_bytes!("../assets/fonts/inter-static/Inter-ExtraBold.ttf"),
        weight: FontWeight::ExtraBold,
        italic: false,
    },
    FontData {
        bytes: include_bytes!("../assets/fonts/inter-static/Inter-ExtraBoldItalic.ttf"),
        weight: FontWeight::ExtraBold,
        italic: true,
    },
    FontData {
        bytes: include_bytes!("../assets/fonts/inter-static/Inter-ExtraLight-BETA.ttf"),
        weight: FontWeight::ExtraLight,
        italic: false,
    },
    FontData {
        bytes: include_bytes!("../assets/fonts/inter-static/Inter-ExtraLightItalic-BETA.ttf"),
        weight: FontWeight::ExtraLight,
        italic: true,
    },
    FontData {
        bytes: include_bytes!("../assets/fonts/inter-static/Inter-Regular.ttf"),
        weight: FontWeight::Regular,
        italic: false,
    },
    FontData {
        bytes: include_bytes!("../assets/fonts/inter-static/Inter-Italic.ttf"),
        weight: FontWeight::Regular,
        italic: true,
    },
    FontData {
        bytes: include_bytes!("../assets/fonts/inter-static/Inter-Light-BETA.ttf"),
        weight: FontWeight::Light,
        italic: false,
    },
    FontData {
        bytes: include_bytes!("../assets/fonts/inter-static/Inter-LightItalic-BETA.ttf"),
        weight: FontWeight::Light,
        italic: true,
    },
    FontData {
        bytes: include_bytes!("../assets/fonts/inter-static/Inter-Medium.ttf"),
        weight: FontWeight::Medium,
        italic: false,
    },
    FontData {
        bytes: include_bytes!("../assets/fonts/inter-static/Inter-MediumItalic.ttf"),
        weight: FontWeight::Medium,
        italic: true,
    },
    FontData {
        bytes: include_bytes!("../assets/fonts/inter-static/Inter-SemiBold.ttf"),
        weight: FontWeight::SemiBold,
        italic: false,
    },
    FontData {
        bytes: include_bytes!("../assets/fonts/inter-static/Inter-SemiBoldItalic.ttf"),
        weight: FontWeight::SemiBold,
        italic: true,
    },
    FontData {
        bytes: include_bytes!("../assets/fonts/inter-static/Inter-Thin-BETA.ttf"),
        weight: FontWeight::Thin,
        italic: false,
    },
    FontData {
        bytes: include_bytes!("../assets/fonts/inter-static/Inter-ThinItalic-BETA.ttf"),
        weight: FontWeight::Thin,
        italic: true,
    },
];

// TODO: Replace with a resource manager that loads fonts from URLs and caches them

pub fn find_font_index_by_style(weight: FontWeight, italic: bool) -> usize {
    FONTS
        .iter()
        .position(|f| f.weight == weight && f.italic == italic)
        .unwrap()
}
