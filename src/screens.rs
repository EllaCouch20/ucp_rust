use rust_on_rails::prelude::*;
use pelican_ui::prelude::*;
// use pelican_ui::prelude::Text;
use crate::UCPPlugin;

#[derive(Debug, Copy, Clone)]
pub enum UCPFlow {
    SelectInstitution,
    EnterCredentials,
    // ConnectingBank,
    VerifyIdentityCaptcha,
    // VerifyIdentityColor,
    // VerifyIdentityCode,
    // VerifyIdentityToken,
}

impl AppFlow for UCPFlow {
    fn get_page(&self, ctx: &mut Context) -> Box<dyn AppPage> {
        match self {
            UCPFlow::SelectInstitution => Box::new(SelectInstitution::new(ctx)) as Box<dyn AppPage>,
            UCPFlow::EnterCredentials => Box::new(EnterCredentials::new(ctx)) as Box<dyn AppPage>,
            // UCPFlow::ConnectingBank => Box::new(ConnectingBank::new(ctx)) as Box<dyn AppPage>,
            UCPFlow::VerifyIdentityCaptcha => Box::new(VerifyIdentityCaptcha::new(ctx)) as Box<dyn AppPage>,
            // UCPFlow::VerifyIdentityColor => Box::new(VerifyIdentityCode::new(ctx)) as Box<dyn AppPage>,
            // UCPFlow::VerifyIdentityPhoneNumber => Box::new(VerifyIdentityPhoneNumber::new(ctx)) as Box<dyn AppPage>,
            // UCPFlow::VerifyIdentityToken => Box::new(VerifyIdentityToken::new(ctx)) as Box<dyn AppPage>,
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
        let header = Header::home(ctx, "Select institution");

        let banks = ctx.get::<UCPPlugin>().get_banks();

        let banks = ListItemGroup::new(
            banks.into_iter().map(|(name, url, image)| {
                let img = ctx.add_image(image);
                let avatar = AvatarContent::Image(img);
                ListItem::new(
                    ctx, true, name, None, Some(url), None, None, None, None, Some(avatar), None, 
                    move |ctx: &mut Context| UCPFlow::EnterCredentials.navigate(ctx)
                )
            }).collect()
        );
        
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

        let back = IconButton::navigation(ctx, "left", |ctx: &mut Context| UCPFlow::SelectInstitution.navigate(ctx));
        let header = Header::stack(ctx, Some(back), "Enter credentials", None);
        
        let content = Content::new(Offset::Start, vec![Box::new(user_id), Box::new(password)]);
        let bumper = Bumper::single_button(Button::primary(ctx, "Continue", |ctx: &mut Context| UCPFlow::VerifyIdentityCaptcha.navigate(ctx)));
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
        let user_id = TextInput::new(ctx, None, Some("User ID"), "User ID...", None, icon_button);
        let password = TextInput::new(ctx, None, Some("Password"), "Password...", None, icon_button);

        let back = IconButton::navigation(ctx, "left", |ctx: &mut Context| UCPFlow::SelectInstitution.navigate(ctx));
        let header = Header::stack(ctx, Some(back), "Enter credentials", None);
        
        let content = Content::new(Offset::Start, vec![Box::new(user_id), Box::new(password)]);
        let bumper = Bumper::single_button(Button::primary(ctx, "Continue", |ctx: &mut Context|{
            // println!("NEXT!")
            let request = ctx.get::<UCPPlugin>().get_captcha();

            println!("body = {:?}", request);
        }
         /*BitcoinFlow::Success.navigate(ctx)*/));
        VerifyIdentityCaptcha(Stack::center(), Page::new(header, content, Some(bumper), false))
    }
}