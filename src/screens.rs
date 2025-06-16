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

use serde::{Serialize, Deserialize};

use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Responses(pub BTreeMap<String, Vec<u8>>);

use crate::{AvatarSelector, SelectableAvatar, BankInstitutions, ListItemUCP, MyBank};

use maverick_os::{State, runtime::{thread, Channel}};

use reqwest::blocking::get;

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

impl VerifyIdentityCaptcha {
    pub fn new(ctx: &mut Context) -> (Self, bool) {
        let icon_button = None::<(&'static str, fn(&mut Context, &mut String))>;
        let instructions = TextInput::new(ctx, None, Some("Captcha code"), "Please enter the Captcha code...", None, icon_button);

        //Url for our captcha image
        let url = "https://www.pandasecurity.com/en/mediacenter/src/uploads/2014/09/avoid-captcha.jpg".to_string();
        //Spawn an async task 
        ctx.runtime.spawn((
            //This async task runs once and calls the callback function at the bottom with the
            //results
            async move || {
                //Make our api call and return the bytes
                reqwest::get(url).await.unwrap().bytes().await.unwrap().to_vec()
            },
            //This is the callback that takes the bytes and sets it to the state so it can be
            //pulled in the tick function below
            |state: &mut State, r: Vec<u8>| state.set_raw("captcha_image".to_string(), r)
        ));

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

        // Display the image as the second item in the content
        let content = Content::new(Offset::Start, vec![Box::new(my_bank), Box::new(instructions)]);
        let bumper = Bumper::single_button(ctx, button);

        // Returning the page containing our content (and header, and bumper)
        (VerifyIdentityCaptcha(Stack::center(), Page::new(header, content, Some(bumper))), false)
    }
}

impl OnEvent for VerifyIdentityCaptcha {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(TickEvent) = event.downcast_ref::<TickEvent>() {
            //If the images is not inside the content of the page yet
            if !(self.1.content().items().len() > 2) {
                //Get the image from the state and if its found decode it(load_from_memory)
                let image = ctx.state().get_raw("captcha_image").map(|b| image::load_from_memory(&b).expect("Error: Could not load image from memory"));
                if let Some(image) = image {
                    //If the image was found and decoded insert it into the content of the page
                    let image = Image{shape: ShapeType::Rectangle(0.0, (140.0, 50.0)), image: ctx.assets.add_image(image.into()), color: None};
                    self.1.content().items().insert(1, Box::new(image));
                }
            }
        }
        true
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

