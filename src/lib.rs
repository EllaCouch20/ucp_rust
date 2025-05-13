use rust_on_rails::prelude::*;
use pelican_ui::prelude::*;

mod screens;
use screens::*;
mod plugin;
use plugin::*;

pub struct MyApp;

impl App for MyApp {
    // async fn background_tasks(ctx: &mut HeadlessContext) -> Tasks {
    //    // BDKPlugin::background_tasks(ctx).await
    // }
    async fn plugins(ctx: &mut Context, h_ctx: &mut HeadlessContext) -> (Plugins, Tasks) {
        let (mut pelican, _ptasks) = PelicanUI::new(ctx, h_ctx).await;

        let (ucp, tasks) = UCPPlugin::new(ctx, h_ctx).await;

        (std::collections::HashMap::from([
            (std::any::TypeId::of::<PelicanUI>(), Box::new(pelican) as Box<dyn std::any::Any>),
            (std::any::TypeId::of::<UCPPlugin>(), Box::new(ucp) as Box<dyn std::any::Any>)
        ]), tasks)
    }

    async fn new(ctx: &mut Context) -> Box<dyn Drawable> {
        ctx.get::<PelicanUI>().theme = Theme::new(
            ColorResources::new(
                BackgroundColor::default(),
                OutlineColor::default(),
                StatusColor::default(),
                TextColor::default(),
                BrandColor {
                    primary: Color::from_hex("0c95df", 255),
                    secondary: Color::from_hex("ffffff", 255)
                },
                ShadesColor::default(),
                ButtonColors {
                    primary_default: ButtonColorScheme {
                        background: Color::from_hex("0c95df", 255),
                        label: Color::from_hex("ffffff", 255),
                        outline: Color::from_hex("000000", 0),
                    },
                    primary_disabled: ButtonColorScheme {
                        background: Color::from_hex("443f3f", 255),
                        label: Color::from_hex("000000", 255),
                        outline: Color::from_hex("000000", 0),
                    },
                    primary_hover: ButtonColorScheme {
                        background: Color::from_hex("0078df", 255),
                        label: Color::from_hex("ffffff", 255),
                        outline: Color::from_hex("000000", 0),
                    },
                    primary_selected: ButtonColorScheme {
                        background: Color::from_hex("0078df", 255),
                        label: Color::from_hex("ffffff", 255),
                        outline: Color::from_hex("000000", 0),
                    },
                    primary_pressed: ButtonColorScheme {
                        background: Color::from_hex("0078df", 255),
                        label: Color::from_hex("ffffff", 255),
                        outline: Color::from_hex("000000", 0),
                    },
        
                    secondary_default: ButtonColorScheme {
                        background: Color::from_hex("000000", 0),
                        label: Color::from_hex("ffffff", 255),
                        outline: Color::from_hex("585250", 255),
                    },
                    secondary_disabled: ButtonColorScheme {
                        background: Color::from_hex("78716c", 255),
                        label: Color::from_hex("000000", 255),
                        outline:Color::from_hex("585250", 255),
                    },
                    secondary_hover: ButtonColorScheme {
                        background: Color::from_hex("262322", 255),
                        label: Color::from_hex("ffffff", 255),
                        outline: Color::from_hex("585250", 255),
                    },
                    secondary_selected: ButtonColorScheme {
                        background: Color::from_hex("262322", 255),
                        label: Color::from_hex("ffffff", 255),
                        outline: Color::from_hex("585250", 255),
                    },
                    secondary_pressed: ButtonColorScheme {
                        background: Color::from_hex("262322", 255),
                        label: Color::from_hex("ffffff", 255),
                        outline: Color::from_hex("585250", 255),
                    },
        
                    ghost_default: ButtonColorScheme {
                        background: Color::from_hex("000000", 0),
                        label: Color::from_hex("ffffff", 255),
                        outline: Color::from_hex("000000", 0),
                    },
                    ghost_disabled: ButtonColorScheme {
                        background: Color::from_hex("000000", 0),
                        label: Color::from_hex("78716c", 255),
                        outline: Color::from_hex("000000", 0),
                    },
                    ghost_hover: ButtonColorScheme {
                        background: Color::from_hex("262322", 255),
                        label: Color::from_hex("ffffff", 255),
                        outline: Color::from_hex("000000", 0),
                    },
                    ghost_selected: ButtonColorScheme {
                        background: Color::from_hex("262322", 255),
                        label: Color::from_hex("ffffff", 255),
                        outline: Color::from_hex("000000", 0),
                    },
                    ghost_pressed: ButtonColorScheme {
                        background: Color::from_hex("262322", 255),
                        label: Color::from_hex("ffffff", 255),
                        outline: Color::from_hex("000000", 0),
                    },
                }
            ),
            FontResources::default(ctx),
            IconResources::default(ctx),
            BrandResources::default(ctx)
        );
        let home = SelectInstitution::new(ctx);
        Box::new(Interface::new(ctx, home, None, None))
    }
}

create_entry_points!(MyApp);