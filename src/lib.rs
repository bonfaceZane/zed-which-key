use zed_extension_api::{self as zed, gpui, Result};

struct ZedWhichKey;

impl zed::Extension for ZedWhichKey {
    fn new() -> Self {
        Self
    }

    fn init(&mut self, cx: &mut zed::ExtensionContext) -> Result<()> {
        cx.register_action("zed-which-key::toggle", |this, cx| {
            this.toggle(cx);
            Ok(())
        })?;

        cx.register_action("zed-which-key::dismiss", |_, cx| {
            cx.dispatch_global_action("zed::Dismiss", ());
            Ok(())
        })?;

        Ok(())
    }
}

impl ZedWhichKey {
    fn toggle<'a>(&mut self, cx: &'a mut zed::ActionContext) {
        cx.add_modal(|cx| WhichKeyModal::new(cx));
    }
}

struct WhichKeyModal;

impl WhichKeyModal {
    fn new(cx: &mut gpui::ViewContext<Self>) -> Self {
        cx.on_keystroke(|_, cx| {
            cx.dispatch_global_action("zed-which-key::dismiss", ());
            true
        })
        .detach();
        Self
    }
}

impl gpui::Entity for WhichKeyModal {
    type Event = ();
}

impl gpui::View for WhichKeyModal {
    fn ui_name() -> &'static str {
        "WhichKeyModal"
    }

    fn render(&mut self, cx: &mut gpui::RenderContext<Self>) -> gpui::ElementBox {
        gpui::elements::Label::new("Which Key".to_string())
            .contained()
            .with_style(cx.style().label)
            .boxed()
    }
}
zed::register_extension!(ZedWhichKey);
