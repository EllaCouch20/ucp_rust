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

use crate::{AvatarSelector, SelectableAvatar, BankInstitutions, ListItemUCP, MyBank, APIService, service::Responses};

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
pub struct VerifyIdentityCaptcha(Stack, Page, #[skip] bool, #[skip] String);

impl VerifyIdentityCaptcha {
    pub fn new(ctx: &mut Context) -> (Self, bool) {
        let icon_button = None::<(&'static str, fn(&mut Context, &mut String))>;
        let instructions = TextInput::new(ctx, None, Some("Captcha code"), "Please enter the Captcha code...", None, icon_button);

        let url = "https://storage.googleapis.com/kagglesdsdata/datasets/38019/306654/samples/243mm.png?X-Goog-Algorithm=GOOG4-RSA-SHA256&X-Goog-Credential=databundle-worker-v2%40kaggle-161607.iam.gserviceaccount.com%2F20250605%2Fauto%2Fstorage%2Fgoog4_request&X-Goog-Date=20250605T223125Z&X-Goog-Expires=345600&X-Goog-SignedHeaders=host&X-Goog-Signature=73bf6dc44c188c6ab146f3c5c7f2505a446f6411451492adc1e7dd633a6adc41fc65d295643bcf8b6828d3865dbc0a9eda500817e0d7fed63b9dd8a68e1e4057bb6fce0e56054851860edcd3cf027383181379eb902f3885bcf61bd7bfdc333dcb04dcfaa6a7d9378f5caccd53ff161bce40347fe60a2ab664a8b3bd482f94865f13464b138bbc2c3d76742a5fc9609373967926d9dbaa5d5d955cb855bcccf1b9f9161abd4011c2c94868be175b9e5d429a52d744fea24bcfcd7ad2b34c41b80e2d89bc7857fcb345b9800ca4771505e0ce0ecf043fb28f3b83438261fb881e382e3c23b9ce2b4db96462189276767333211853639ab6dfa8b8246fc4cf79ee".to_string();
        ctx.runtime.send::<APIService>(url.clone());

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
        (VerifyIdentityCaptcha(Stack::center(), Page::new(header, content, Some(bumper)), false, url), false)
    }
}

impl OnEvent for VerifyIdentityCaptcha {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(TickEvent) = event.downcast_ref::<TickEvent>() {
            if !self.2 {
                let responses = ctx.state().get::<Responses>().0;
                if let Some(bytes) = responses.get(&self.3) {
                    self.2 = true;
                    let loaded_image = image::load_from_memory(&bytes).expect("Error: Could not load image from memory");
                    let image = Image{shape: ShapeType::Rectangle(0.0, (140.0, 50.0)), image: ctx.assets.add_image(loaded_image.into()), color: None};
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

