use pelican_ui::events::{Event, OnEvent, TickEvent, MouseEvent, MouseState};
use pelican_ui::drawable::{Drawable, Component, Align};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};

use pelican_ui_std::{
    AppPage, Stack, Page,
    Header, IconButton,
    Avatar, AvatarContent,
    AvatarIconStyle, Icon, Text,
    TextStyle, Content,
    Offset, ListItem,
    Button, ButtonState,
    Bumper, TextInput,
    SetActiveInput, IS_MOBILE,
    QuickActions, ListItemSelector,
    NavigateEvent, DataItem,
    Timestamp, ListItemGroup,
    ElementID, Outline, Flair,
    Wrap
};

use image::{RgbaImage, load_from_memory};

use crate::{MyBank, Bank, EnterCredentials};

pub struct ListItemUCP;

impl ListItemUCP {
    pub fn bank_item(ctx: &mut Context, name: String, url: String, image_name: String) -> ListItem {
        let image = &ctx.assets.load_file(&image_name).unwrap();
        let image = load_from_memory(&image).expect("Could not load from memory.").into();
        let img = ctx.assets.add_image(image);
        let avatar = AvatarContent::Image(img);
        ListItem::new(
            ctx, true, &name.clone(), None, Some(&url.clone()), None, None, None, None, Some(avatar), None, 
            move |ctx: &mut Context| { 
                ctx.state().set(&MyBank(Bank(name.clone(), url.clone(), image_name.clone())));
                let page = EnterCredentials::new(ctx);
                ctx.trigger_event(NavigateEvent::new(page));
            }
        )
    }

    pub fn my_bank_item(ctx: &mut Context, name: String, url: String, image_name: String) -> ListItem {
        let image = &ctx.assets.load_file(&image_name).unwrap();
        let image = load_from_memory(&image).expect("Could not load from memory.").into();
        let img = ctx.assets.add_image(image);
        let avatar = AvatarContent::Image(img);
        ListItem::new(
            ctx, false, &name.clone(), None, Some(&url.clone()), None, None, None, None, Some(avatar), None, 
            move |_ctx: &mut Context| {}
        )
    }
}

#[derive(Debug, Component)]
pub struct AvatarSelector(Wrap, Vec<SelectableAvatar>);

impl AvatarSelector {
    pub fn new(avatars: Vec<SelectableAvatar>) -> Self {
        AvatarSelector(Wrap::new(8.0, 8.0), avatars)
    }
}

impl OnEvent for AvatarSelector {}

#[derive(Debug, Component)]
pub struct SelectableAvatar(Stack, Avatar, #[skip] ElementID, #[skip] f32);


impl OnEvent for SelectableAvatar {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(event) = event.downcast_ref::<MouseEvent>() {
            if let MouseEvent{state: MouseState::Pressed, position: Some(_)} = event {
                println!("Click!");
                self.select(ctx); 
                ctx.trigger_event(AvatarSelect(self.2));
            }
        } else if let Some(AvatarSelect(id)) = event.downcast_ref::<AvatarSelect>() {
            if *id != self.2 {
                self.deselect();
            }
        }
        false
    }
}

impl SelectableAvatar {
    pub fn new(
        ctx: &mut Context, 
        content: AvatarContent, 
        flair: Option<(&'static str, AvatarIconStyle)>, 
        outline: bool, 
        size: f32,
    ) -> Self {
        let id = ElementID::new();
        SelectableAvatar(Stack::center(), Avatar::new(ctx, content, flair, outline, size, None), id, size)
    }

    pub fn deselect(&mut self) {
        *self.1.outline() = None;
        *self.1.flair() = None;
    }

    pub fn select(&mut self, ctx: &mut Context) {
        let color = ctx.theme.colors;
        let (a, b) = (color.brand.primary, color.shades.white);
        let size = self.3;
        *self.1.outline() = Some(Outline::circle(size, a));
        *self.1.flair() = Some(Flair::new(ctx, "checkmark", AvatarIconStyle::Brand, size / 3.0, b))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AvatarSelect(pub ElementID);

impl Event for AvatarSelect {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        children.into_iter().map(|_| Some(Box::new(*self) as Box<dyn Event>)).collect()
    }
}
