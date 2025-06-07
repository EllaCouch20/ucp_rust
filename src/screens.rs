use pelican_ui::events::{Event, OnEvent, TickEvent};
use pelican_ui::drawable::{Drawable, Component, Align, Image, ShapeType};
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
};

use crate::{AvatarSelector, SelectableAvatar, BankInstitutions, ListItemUCP, MyBank};

#[derive(Debug, Component, AppPage)]
pub struct SelectInstitution(Stack, Page);
impl OnEvent for SelectInstitution {}
impl SelectInstitution {
    pub fn new(ctx: &mut Context) -> (Self, bool) {
        ctx.state().set(&BankInstitutions::new());
        let icon_button = None::<(&'static str, fn(&mut Context, &mut String))>;
        let searchbar = TextInput::new(ctx, None, None, "Search", None, icon_button);

        // let back = {
        //     let on_return = ctx.get::<UCPPlugin>().back();
        //     on_return.map(|mut action| {
        //         IconButton::navigation(ctx, "left", move |ctx: &mut Context| {
        //             action(ctx)
        //         })
        //     })
        // };

        let header = Header::stack(ctx, None, "Select your institution", None);

        let banks = ctx.state().get::<BankInstitutions>().0;
        let banks = ListItemGroup::new(banks.into_iter().map(|bank| ListItemUCP::bank_item(ctx, bank.0, bank.1, bank.2)).collect());
        
        let content = Content::new(Offset::Start, vec![Box::new(searchbar), Box::new(banks)]);
        (SelectInstitution(Stack::center(), Page::new(header, content, None)), false)
    }
}

#[derive(Debug, Component, AppPage)]
pub struct EnterCredentials(Stack, Page);
impl OnEvent for EnterCredentials {}
impl EnterCredentials {
    pub fn new(ctx: &mut Context) -> (Self, bool) {
        let icon_button = None::<(&'static str, fn(&mut Context, &mut String))>;
        let user_id = TextInput::new(ctx, None, Some("User ID"), "User ID...", None, icon_button);
        let password = TextInput::new(ctx, None, Some("Password"), "Password...", None, icon_button);

        let my_bank = ctx.state().get::<MyBank>().0;
        let my_bank = ListItemUCP::my_bank_item(ctx, my_bank.0, my_bank.1, my_bank.2);

        let back = IconButton::navigation(ctx, "left", |ctx: &mut Context| {
            let page = SelectInstitution::new(ctx);
            ctx.trigger_event(NavigateEvent::new(page))
        });

        let button = Button::primary(ctx, "Continue", |ctx: &mut Context| {
            let page = VerifyIdentityCaptcha::new(ctx);
            ctx.trigger_event(NavigateEvent::new(page))
        });

        let header = Header::stack(ctx, Some(back), "Enter credentials", None);
        let content = Content::new(Offset::Start, vec![Box::new(my_bank), Box::new(user_id), Box::new(password)]);
        let bumper = Bumper::single_button(ctx, button);
        (EnterCredentials(Stack::center(), Page::new(header, content, Some(bumper))), false)
    }
}

#[derive(Debug, Component, AppPage)]
pub struct VerifyIdentityCaptcha(Stack, Page);
impl OnEvent for VerifyIdentityCaptcha {}
impl VerifyIdentityCaptcha {
    pub fn new(ctx: &mut Context) -> (Self, bool) {
        let icon_button = None::<(&'static str, fn(&mut Context, &mut String))>;
        let captcha = TextInput::new(ctx, None, Some("Captcha code"), "Please enter the Captcha code...", None, icon_button);

        let bytes = &ctx.assets.load_file("captcha.png").unwrap();
        let img = image::load_from_memory(&bytes).unwrap();
        let image = Image{shape: ShapeType::Rectangle(0.0, (140.0, 50.0)), image: ctx.assets.add_image(img.into()), color: None};

        let my_bank = ctx.state().get::<MyBank>().0;
        let my_bank = ListItemUCP::my_bank_item(ctx, my_bank.0, my_bank.1, my_bank.2);

        let back = IconButton::navigation(ctx, "left", |ctx: &mut Context| {
            let page = EnterCredentials::new(ctx);
            ctx.trigger_event(NavigateEvent::new(page))
        });

        let button = Button::primary(ctx, "Continue", |ctx: &mut Context| {
            let page = VerifyIdentityColor::new(ctx);
            ctx.trigger_event(NavigateEvent::new(page))
        });

        let header = Header::stack(ctx, Some(back), "Verify identity", None);
        let content = Content::new(Offset::Start, vec![Box::new(my_bank), Box::new(image), Box::new(captcha)]);
        let bumper = Bumper::single_button(ctx, button);
        (VerifyIdentityCaptcha(Stack::center(), Page::new(header, content, Some(bumper))), false)
    }
}

#[derive(Debug, Component, AppPage)]
pub struct VerifyIdentityColor(Stack, Page);
impl OnEvent for VerifyIdentityColor {}
impl VerifyIdentityColor {
    pub fn new(ctx: &mut Context) -> (Self, bool) {
        let icon_button = None::<(&'static str, fn(&mut Context, &mut String))>;
        let color = TextInput::new(ctx, None, Some("Your favorite color"), "Please enter your favorite color...", None, icon_button);
       
        let my_bank = ctx.state().get::<MyBank>().0;
        let my_bank = ListItemUCP::my_bank_item(ctx, my_bank.0, my_bank.1, my_bank.2);

        let back = IconButton::navigation(ctx, "left", |ctx: &mut Context| {
            let page = VerifyIdentityCaptcha::new(ctx);
            ctx.trigger_event(NavigateEvent::new(page))
        });

        let header = Header::stack(ctx, Some(back), "Verify identity", None);
        
        let content = Content::new(Offset::Start, vec![Box::new(my_bank), Box::new(color)]);
        let button = Button::primary(ctx, "Continue", |ctx: &mut Context| {
            let page = VerifyIdentityPhoneNumber::new(ctx);
            ctx.trigger_event(NavigateEvent::new(page))
        });

        let bumper = Bumper::single_button(ctx, button);
        (VerifyIdentityColor(Stack::center(), Page::new(header, content, Some(bumper))), false)
    }
}

#[derive(Debug, Component, AppPage)]
pub struct VerifyIdentityPhoneNumber(Stack, Page);
impl OnEvent for VerifyIdentityPhoneNumber {}
impl VerifyIdentityPhoneNumber {
    pub fn new(ctx: &mut Context) -> (Self, bool) {
        let selector = ListItemSelector::new(ctx,
            ("xxx-xxx-1234", "Verification code will be sent here", None),
            ("xxx-xxx-8632", "Verification code will be sent here", None),
            None, None
        );       
        
        let my_bank = ctx.state().get::<MyBank>().0;
        let my_bank = ListItemUCP::my_bank_item(ctx, my_bank.0, my_bank.1, my_bank.2);

        let back = IconButton::navigation(ctx, "left", |ctx: &mut Context| {
            let page = VerifyIdentityColor::new(ctx);
            ctx.trigger_event(NavigateEvent::new(page))
        });

        let header = Header::stack(ctx, Some(back), "Verify identity", None);
        
        let content = Content::new(Offset::Start, vec![Box::new(my_bank), Box::new(selector)]);
        let button = Button::primary(ctx, "Continue", |ctx: &mut Context| {
            let page = VerifyIdentityToken::new(ctx);
            ctx.trigger_event(NavigateEvent::new(page))
        });

        let bumper = Bumper::single_button(ctx, button);
        (VerifyIdentityPhoneNumber(Stack::center(), Page::new(header, content, Some(bumper))), false)
    }
}

#[derive(Debug, Component, AppPage)]
pub struct VerifyIdentityToken(Stack, Page);
impl OnEvent for VerifyIdentityToken {}
impl VerifyIdentityToken {
    pub fn new(ctx: &mut Context) -> (Self, bool) {
        let icon_button = None::<(&'static str, fn(&mut Context, &mut String))>;
        let color = TextInput::new(ctx, None, Some("Enter the token"), "Please enter the token...", None, icon_button);

        let my_bank = ctx.state().get::<MyBank>().0;
        let my_bank = ListItemUCP::my_bank_item(ctx, my_bank.0, my_bank.1, my_bank.2);

        let back = IconButton::navigation(ctx, "left", |ctx: &mut Context| {
            let page = VerifyIdentityPhoneNumber::new(ctx);
            ctx.trigger_event(NavigateEvent::new(page))
        });

        let header = Header::stack(ctx, Some(back), "Verify identity", None);
        
        let content = Content::new(Offset::Start, vec![Box::new(my_bank), Box::new(color)]);
        let button = Button::primary(ctx, "Continue", |ctx: &mut Context| {
            let page = VerifyIdentityImages::new(ctx);
            ctx.trigger_event(NavigateEvent::new(page))
        });

        let bumper = Bumper::single_button(ctx, button);
        (VerifyIdentityToken(Stack::center(), Page::new(header, content, Some(bumper))), false)
    }
}

#[derive(Debug, Component, AppPage)]
pub struct VerifyIdentityImages(Stack, Page);
impl OnEvent for VerifyIdentityImages {}
impl VerifyIdentityImages {
    pub fn new(ctx: &mut Context) -> (Self, bool) {
        // let cache = &ctx.get::<UCPPlugin>();
        let my_bank = ctx.state().get::<MyBank>().0;
        let my_bank = ListItemUCP::my_bank_item(ctx, my_bank.0, my_bank.1, my_bank.2);

        let back = IconButton::navigation(ctx, "left", |ctx: &mut Context| {
            let page = VerifyIdentityToken::new(ctx);
            ctx.trigger_event(NavigateEvent::new(page))
        });

        let header = Header::stack(ctx, Some(back), "Verify identity", None);

        let text_size = ctx.theme.fonts.size.h5;
        let instructions = Text::new(ctx, "Choose the image containing a cat.", TextStyle::Heading, text_size, Align::Left);

        let paths = vec!["image1.jpeg", "image2.jpeg", "image3.jpeg", "image4.jpeg", "image5.jpeg", "image6.jpeg"];
        let avatars = paths.into_iter().map(|path| {
            let img = image::load_from_memory(&ctx.assets.load_file(path).unwrap()).unwrap();
            let new_img = ctx.assets.add_image(img.into());
            SelectableAvatar::new(ctx, AvatarContent::Image(new_img), None, false, 112.0)
        }).collect::<Vec<SelectableAvatar>>();
        // println!("Images: {:?}", verify_images);

        let images = AvatarSelector::new(avatars);
        
        let content = Content::new(Offset::Start, vec![Box::new(my_bank), Box::new(instructions), Box::new(images)]);
        let button = Button::primary(ctx, "Continue", move |ctx: &mut Context| {
            // let mut on_return = ctx.get::<UCPPlugin>().on_return();
            // match &mut on_return {
            //     Some(action) => (action)(ctx),
            //     None => UCPFlow::SelectInstitution.navigate(ctx)
            // }

            let page = SelectInstitution::new(ctx);
            ctx.trigger_event(NavigateEvent::new(page))
        });

        let bumper = Bumper::single_button(ctx, button);
        (VerifyIdentityImages(Stack::center(), Page::new(header, content, Some(bumper))), false)
    }
}

