use zed_extension_api as zed;

struct ZedWhichKey {
}

impl zed::Extension for ZedWhichKey {
    fn new() -> Self {
        Self {
        }
    }
}

zed::register_extension!(ZedWhichKey);
