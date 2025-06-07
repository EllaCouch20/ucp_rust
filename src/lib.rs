use pelican_ui::{include_assets, Context, Assets, Plugins, Plugin, Service, Services, ServiceList, maverick_start, start, Application, PelicanEngine, MaverickOS, HardwareContext, ColorResources, FontResources, BrandResources, IconResources, Theme};
use pelican_ui::drawable::{Drawable, Color};
use pelican_ui_std::{AvatarIconStyle, AvatarContent, Interface, NavigateEvent};
use pelican_ui::theme::{ButtonColorScheme, TextColor, BrandColor, StatusColor, ButtonColors, ShadesColor, BackgroundColor, OutlineColor};
use std::any::TypeId;
use std::collections::BTreeMap;
use std::pin::Pin;
use std::future::Future;
use std::fs::DirEntry;

pub mod screens;
pub use screens::*;
pub mod plugin;
pub use plugin::*;
pub mod components; 
pub use components::*;

mod service;
use service::APIService;


// fn service<'a>(ctx: &'a mut HardwareContext) -> Pin<Box<dyn Future<Output = Box<dyn Service>> + 'a >> {
//     Box::pin(async move {Box::new(APIService::new(ctx).await) as Box<dyn Service>})
// }


fn service<'a>(ctx: &'a mut HardwareContext) -> Pin<Box<dyn Future<Output = Box<dyn Service>> + 'a >> {
    Box::pin(async move {Box::new(APIService::new(ctx).await) as Box<dyn Service>})
}

pub struct MyApp;
impl Services for MyApp {
    fn services() -> ServiceList {
        BTreeMap::from([(
            TypeId::of::<APIService>(), 
            Box::new(service) as Box<dyn for<'a> FnOnce(&'a mut HardwareContext) -> Pin<Box<dyn Future<Output = Box<dyn Service>> + 'a>>>
        )])
    }
}

impl Plugins for MyApp {
    fn plugins(ctx: &mut Context) -> Vec<Box<dyn Plugin>> {
        vec![]
    }
}

impl Application for MyApp {

    async fn new(ctx: &mut Context) -> Box<dyn Drawable> {
        ctx.assets.include_assets(include_assets!("./assets"));
        let new_theme = ucp_theme(&mut ctx.assets);
        ctx.theme = new_theme;
        let home = SelectInstitution::new(ctx).0;
        Box::new(Interface::new(ctx, home, None))
    }
}

start!(MyApp);

fn ucp_theme(assets: &mut Assets) -> Theme {
    Theme::new(
        ColorResources::new(
            BackgroundColor {
                primary: Color::from_hex("ffffff", 255),
                secondary: Color::from_hex("e9eff5", 255)
            },
            OutlineColor {
                primary: Color::from_hex("c9d6e3", 255),
                secondary: Color::from_hex("e3eaf1", 255)
            },
            StatusColor::default(),
            TextColor {
                heading: Color::from_hex("1a2e45", 255),
                primary: Color::from_hex("2f3e51", 255),
                secondary: Color::from_hex("6b7c93", 255)
            },
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
                    label: Color::from_hex("000000", 255),
                    outline: Color::from_hex("c9d6e3", 255),
                },
                secondary_disabled: ButtonColorScheme {
                    background: Color::from_hex("78716c", 255),
                    label: Color::from_hex("e9eff5", 255),
                    outline:Color::from_hex("c9d6e3", 255),
                },
                secondary_hover: ButtonColorScheme {
                    background: Color::from_hex("262322", 255),
                    label: Color::from_hex("000000", 255),
                    outline: Color::from_hex("c9d6e3", 255),
                },
                secondary_selected: ButtonColorScheme {
                    background: Color::from_hex("262322", 255),
                    label: Color::from_hex("000000", 255),
                    outline: Color::from_hex("c9d6e3", 255),
                },
                secondary_pressed: ButtonColorScheme {
                    background: Color::from_hex("262322", 255),
                    label: Color::from_hex("000000", 255),
                    outline: Color::from_hex("c9d6e3", 255),
                },
    
                ghost_default: ButtonColorScheme {
                    background: Color::from_hex("ffffff", 0),
                    label: Color::from_hex("000000", 255),
                    outline: Color::from_hex("000000", 0),
                },
                ghost_disabled: ButtonColorScheme {
                    background: Color::from_hex("ffffff", 0),
                    label: Color::from_hex("78716c", 255),
                    outline: Color::from_hex("000000", 0),
                },
                ghost_hover: ButtonColorScheme {
                    background: Color::from_hex("e3eaf1", 255),
                    label: Color::from_hex("000000", 255),
                    outline: Color::from_hex("e3eaf1", 0),
                },
                ghost_selected: ButtonColorScheme {
                    background: Color::from_hex("e3eaf1", 255),
                    label: Color::from_hex("000000", 255),
                    outline: Color::from_hex("e3eaf1", 0),
                },
                ghost_pressed: ButtonColorScheme {
                    background: Color::from_hex("e3eaf1", 255),
                    label: Color::from_hex("000000", 255),
                    outline: Color::from_hex("e3eaf1", 0),
                },
            }
        ),
        FontResources::default(assets),
        IconResources::default(assets),
        BrandResources::default(assets)
    )
}
