use rust_on_rails::prelude::*;
use pelican_ui::prelude::*;

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
        let color = ctx.get::<PelicanUI>().theme.colors;
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
