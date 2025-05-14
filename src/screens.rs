use rust_on_rails::prelude::*;
use pelican_ui::prelude::*;
use pelican_ui::prelude::Text;
use image::{load_from_memory, RgbaImage};
use crate::UCPPlugin;
use crate::components::*;

#[derive(Debug, Copy, Clone)]
pub enum UCPFlow {
    SelectInstitution,
    EnterCredentials,
    VerifyIdentityCaptcha,
    VerifyIdentityColor,
    VerifyIdentityPhoneNumber,
    VerifyIdentityToken,
    VerifyIdentityImages,
}

impl AppFlow for UCPFlow {
    fn get_page(&self, ctx: &mut Context) -> Box<dyn AppPage> {
        match self {
            UCPFlow::SelectInstitution => Box::new(SelectInstitution::new(ctx)) as Box<dyn AppPage>,
            UCPFlow::EnterCredentials => Box::new(EnterCredentials::new(ctx)) as Box<dyn AppPage>,
            UCPFlow::VerifyIdentityCaptcha => Box::new(VerifyIdentityCaptcha::new(ctx)) as Box<dyn AppPage>,
            UCPFlow::VerifyIdentityColor => Box::new(VerifyIdentityColor::new(ctx)) as Box<dyn AppPage>,
            UCPFlow::VerifyIdentityPhoneNumber => Box::new(VerifyIdentityPhoneNumber::new(ctx)) as Box<dyn AppPage>,
            UCPFlow::VerifyIdentityToken => Box::new(VerifyIdentityToken::new(ctx)) as Box<dyn AppPage>,
            UCPFlow::VerifyIdentityImages => Box::new(VerifyIdentityImages::new(ctx)) as Box<dyn AppPage>,
        }
    }
}

#[derive(Debug, Component)]
pub struct SelectInstitution(Stack, Page);
impl AppPage for SelectInstitution {}
impl OnEvent for SelectInstitution {}
impl SelectInstitution {
    pub fn new(ctx: &mut Context) -> Self {
        let icon_button = None::<(&'static str, fn(&mut Context, &mut String))>;
        let searchbar = TextInput::new(ctx, None, None, "Search", None, icon_button);

        let back = {
            let on_return = ctx.get::<UCPPlugin>().back();
            on_return.map(|mut action| {
                IconButton::navigation(ctx, "left", move |ctx: &mut Context| {
                    action(ctx)
                })
            })
        };

        let header = Header::stack(ctx, back, "Select your institution", None);

        let banks = ctx.get::<UCPPlugin>().get_banks();
        let banks = ListItemGroup::new(banks.into_iter().map(|(name, url, image)| bank_item(ctx, name, url, image.clone())).collect());
        
        let content = Content::new(Offset::Start, vec![Box::new(searchbar), Box::new(banks)]);
        SelectInstitution(Stack::center(), Page::new(header, content, None, false))
    }
}

#[derive(Debug, Component)]
pub struct EnterCredentials(Stack, Page);
impl AppPage for EnterCredentials {}
impl OnEvent for EnterCredentials {}
impl EnterCredentials {
    pub fn new(ctx: &mut Context) -> Self {
        let icon_button = None::<(&'static str, fn(&mut Context, &mut String))>;
        let user_id = TextInput::new(ctx, None, Some("User ID"), "User ID...", None, icon_button);
        let password = TextInput::new(ctx, None, Some("Password"), "Password...", None, icon_button);

        let (name, url, image) = ctx.get::<UCPPlugin>().get_bank();
        let my_bank = my_bank_item(ctx, name, url, image);

        let back = IconButton::navigation(ctx, "left", |ctx: &mut Context| UCPFlow::SelectInstitution.navigate(ctx));
        let header = Header::stack(ctx, Some(back), "Enter credentials", None);
        
        let content = Content::new(Offset::Start, vec![Box::new(my_bank), Box::new(user_id), Box::new(password)]);
        let button = Button::primary(ctx, "Continue", |ctx: &mut Context| UCPFlow::VerifyIdentityCaptcha.navigate(ctx));
        let bumper = Bumper::single_button(ctx, button);
        EnterCredentials(Stack::center(), Page::new(header, content, Some(bumper), false))
    }
}

#[derive(Debug, Component)]
pub struct VerifyIdentityCaptcha(Stack, Page);
impl AppPage for VerifyIdentityCaptcha {}
impl OnEvent for VerifyIdentityCaptcha {}
impl VerifyIdentityCaptcha {
    pub fn new(ctx: &mut Context) -> Self {
        let icon_button = None::<(&'static str, fn(&mut Context, &mut String))>;
        let captcha = TextInput::new(ctx, None, Some("Captcha code"), "Please enter the Captcha code...", None, icon_button);

        let bytes = &ctx.load_file("captcha.png").unwrap();
        let img = image::load_from_memory(&bytes).unwrap();
        let image = Image{shape: ShapeType::Rectangle(0.0, (140.0, 50.0)), image: ctx.add_image(img.into()), color: None};

        let (name, url, i) = ctx.get::<UCPPlugin>().get_bank();
        let my_bank = my_bank_item(ctx, name, url, i);

        let back = IconButton::navigation(ctx, "left", |ctx: &mut Context| UCPFlow::EnterCredentials.navigate(ctx));
        let header = Header::stack(ctx, Some(back), "Verify identity", None);
        
        let content = Content::new(Offset::Start, vec![Box::new(my_bank), Box::new(image), Box::new(captcha)]);
        let button = Button::primary(ctx, "Continue", |ctx: &mut Context| UCPFlow::VerifyIdentityColor.navigate(ctx));
        let bumper = Bumper::single_button(ctx, button);
        VerifyIdentityCaptcha(Stack::center(), Page::new(header, content, Some(bumper), false))
    }
}

#[derive(Debug, Component)]
pub struct VerifyIdentityColor(Stack, Page);
impl AppPage for VerifyIdentityColor {}
impl OnEvent for VerifyIdentityColor {}
impl VerifyIdentityColor {
    pub fn new(ctx: &mut Context) -> Self {
        let icon_button = None::<(&'static str, fn(&mut Context, &mut String))>;
        let color = TextInput::new(ctx, None, Some("Your favorite color"), "Please enter your favorite color...", None, icon_button);

        let (name, url, i) = ctx.get::<UCPPlugin>().get_bank();
        let my_bank = my_bank_item(ctx, name, url, i);

        let back = IconButton::navigation(ctx, "left", |ctx: &mut Context| UCPFlow::VerifyIdentityCaptcha.navigate(ctx));
        let header = Header::stack(ctx, Some(back), "Verify identity", None);
        
        let content = Content::new(Offset::Start, vec![Box::new(my_bank), Box::new(color)]);
        let button = Button::primary(ctx, "Continue", |ctx: &mut Context| UCPFlow::VerifyIdentityPhoneNumber.navigate(ctx));
        let bumper = Bumper::single_button(ctx, button);
        VerifyIdentityColor(Stack::center(), Page::new(header, content, Some(bumper), false))
    }
}

#[derive(Debug, Component)]
pub struct VerifyIdentityPhoneNumber(Stack, Page);
impl AppPage for VerifyIdentityPhoneNumber {}
impl OnEvent for VerifyIdentityPhoneNumber {}
impl VerifyIdentityPhoneNumber {
    pub fn new(ctx: &mut Context) -> Self {
        let selector = ListItemSelector::new(ctx,
            ("xxx-xxx-1234", "Verification code will be sent here", None),
            ("xxx-xxx-8632", "Verification code will be sent here", None),
            None, None
        );       
        
        let (name, url, i) = ctx.get::<UCPPlugin>().get_bank();
        let my_bank = my_bank_item(ctx, name, url, i);

        let back = IconButton::navigation(ctx, "left", |ctx: &mut Context| UCPFlow::VerifyIdentityColor.navigate(ctx));
        let header = Header::stack(ctx, Some(back), "Verify identity", None);
        
        let content = Content::new(Offset::Start, vec![Box::new(my_bank), Box::new(selector)]);
        let button = Button::primary(ctx, "Continue", |ctx: &mut Context| UCPFlow::VerifyIdentityToken.navigate(ctx));
        let bumper = Bumper::single_button(ctx, button);
        VerifyIdentityPhoneNumber(Stack::center(), Page::new(header, content, Some(bumper), false))
    }
}

#[derive(Debug, Component)]
pub struct VerifyIdentityToken(Stack, Page);
impl AppPage for VerifyIdentityToken {}
impl OnEvent for VerifyIdentityToken {}
impl VerifyIdentityToken {
    pub fn new(ctx: &mut Context) -> Self {
        let icon_button = None::<(&'static str, fn(&mut Context, &mut String))>;
        let color = TextInput::new(ctx, None, Some("Enter the token"), "Please enter the token...", None, icon_button);

        let (name, url, i) = ctx.get::<UCPPlugin>().get_bank();
        let my_bank = my_bank_item(ctx, name, url, i);

        let back = IconButton::navigation(ctx, "left", |ctx: &mut Context| UCPFlow::VerifyIdentityPhoneNumber.navigate(ctx));
        let header = Header::stack(ctx, Some(back), "Verify identity", None);
        
        let content = Content::new(Offset::Start, vec![Box::new(my_bank), Box::new(color)]);
        let button = Button::primary(ctx, "Continue", |ctx: &mut Context| UCPFlow::VerifyIdentityImages.navigate(ctx));
        let bumper = Bumper::single_button(ctx, button);
        VerifyIdentityToken(Stack::center(), Page::new(header, content, Some(bumper), false))
    }
}

#[derive(Debug, Component)]
pub struct VerifyIdentityImages(Stack, Page);
impl AppPage for VerifyIdentityImages {}
impl OnEvent for VerifyIdentityImages {}
impl VerifyIdentityImages {
    pub fn new(ctx: &mut Context) -> Self {
        // let cache = &ctx.get::<UCPPlugin>();
        let (name, url, i) = ctx.get::<UCPPlugin>().get_bank();
        let my_bank = my_bank_item(ctx, name, url, i);

        let back = IconButton::navigation(ctx, "left", |ctx: &mut Context| UCPFlow::VerifyIdentityToken.navigate(ctx));
        let header = Header::stack(ctx, Some(back), "Verify identity", None);

        let text_size = ctx.get::<PelicanUI>().theme.fonts.size.h5;
        let instructions = Text::new(ctx, "Choose the image containing a cat.", TextStyle::Heading, text_size, Align::Left);

        let verify_images = ctx.get::<UCPPlugin>().captcha_images();
        println!("Images: {:?}", verify_images);

        let avatars: Vec<_> = verify_images.into_iter().map(|(_, i)| SelectableAvatar::new(ctx, AvatarContent::Image(i), None, false, 112.0)).collect();

        let images = AvatarSelector::new(avatars);
        
        let content = Content::new(Offset::Start, vec![Box::new(my_bank), Box::new(instructions), Box::new(images)]);
        let button = Button::primary(ctx, "Continue", move |ctx: &mut Context| {
            let mut on_return = ctx.get::<UCPPlugin>().on_return();
            match &mut on_return {
                Some(action) => (action)(ctx),
                None => UCPFlow::SelectInstitution.navigate(ctx)
            }
        });
        let bumper = Bumper::single_button(ctx, button);
        VerifyIdentityImages(Stack::center(), Page::new(header, content, Some(bumper), false))
    }
}

pub fn bank_item(ctx: &mut Context, name: &'static str, url: &'static str, image: RgbaImage) -> ListItem {
    let img = ctx.add_image(image.clone());
    let avatar = AvatarContent::Image(img);
    ListItem::new(
        ctx, true, name, None, Some(url), None, None, None, None, Some(avatar), None, 
        move |ctx: &mut Context| { 
            ctx.get::<UCPPlugin>().set_bank((name, url, image.clone()));
            UCPFlow::EnterCredentials.navigate(ctx)
        }
    )
}

pub fn my_bank_item(ctx: &mut Context, name: &'static str, url: &'static str, image: RgbaImage) -> ListItem {
    let img = ctx.add_image(image.clone());
    let avatar = AvatarContent::Image(img);
    ListItem::new(
        ctx, false, name, None, Some(url), None, None, None, None, Some(avatar), None, 
        move |ctx: &mut Context| {}
    )
}