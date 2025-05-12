use rust_on_rails::prelude::*;
use pelican_ui::prelude::*;
use pelican_ui::prelude::Text;

use reqwest::blocking::get;
use image::{load_from_memory, RgbaImage};

#[derive(Debug, Copy, Clone)]
pub enum UCPFlow {
    SelectInstitution,
    // EnterCredentials,
    // ConnectingBank,
    // VerifyIdentityCaptcha,
    // VerifyIdentityColor,
    // VerifyIdentityCode,
    // VerifyIdentityToken,
}

impl AppFlow for UCPFlow {
    fn get_page(&self, ctx: &mut Context) -> Box<dyn AppPage> {
        match self {
            UCPFlow::SelectInstitution => Box::new(SelectInstitution::new(ctx)) as Box<dyn AppPage>,
            // UCPFlow::EnterCredentials => Box::new(EnterCredentials::new(ctx)) as Box<dyn AppPage>,
            // UCPFlow::ConnectingBank => Box::new(ConnectingBank::new(ctx)) as Box<dyn AppPage>,
            // UCPFlow::VerifyIdentityCaptcha => Box::new(VerifyIdentityCaptcha::new(ctx)) as Box<dyn AppPage>,
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
        let header = Header::home(ctx, "Wallet");

        // let avatar = AvatarContent::Image(img);
        // ListItem::new(
        //     ctx, true, name, None, Some(link), None, None, None, None, Some(avatar), None, 
        //     move |ctx: &mut Context| { println!("I want this bank!")}
        // )
        
        // let banks = ctx.get::<UCPPlugin>().get_banks();

        let text_size = ctx.get::<PelicanUI>().theme.fonts.size.h5;
        let text = Text::new(ctx, "Select your institution", TextStyle::Heading, text_size, Align::Left);
        let banks = Text::new(ctx, "Select your institution", TextStyle::Heading, text_size, Align::Left);
        let content = Content::new(Offset::Start, vec![Box::new(text), Box::new(searchbar), Box::new(banks)]);
        SelectInstitution(Stack::center(), Page::new(header, content, None, false))
    }
}

use rust_on_rails::prelude::*;
// use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;
use std::str::FromStr;

pub struct UCPPlugin {
    banks: Arc<Mutex<Option<BankInstitutions>>>,
}

impl UCPPlugin {
    pub async fn _init(&mut self) {//Include theme
        println!("Initialized UCPPlugin");
    }

    pub async fn get_banks(&mut self) -> ListItemGroup {
        println!("banks: {:?}", self.banks.lock().unwrap());
        ListItemGroup::new(vec![])
    }
}

#[derive(Debug)]
pub struct BankInstitutions(Vec<(&'static str, &'static str, RgbaImage)>);

impl Plugin for UCPPlugin {
    async fn background_tasks(ctx: &mut HeadlessContext) -> Tasks {
        vec![]
    }

    // async fn new(_ctx: &mut Context, h_ctx: &mut HeadlessContext) -> (Self, Tasks) {
    //     let persister = Self::create_wallet(&mut h_ctx.cache).await;
    //     let persister = Arc::new(Mutex::new(persister));
    //     let price = Arc::new(Mutex::new(0.0));
    //     (UCPPlugin{
    //         persister: persister.clone(), 
    //         price: price.clone(), 
    //         recipient_address: Arc::new(Mutex::new(None)),
    //         wallet: Arc::new(Mutex::new(None)),
    //     }, tasks![CachePersister(persister), GetPrice(price)])
    // }

    async fn new(_ctx: &mut Context, h_ctx: &mut HeadlessContext) -> (Self, Tasks) {
        let banks = Arc::new(Mutex::new(None));
        (UCPPlugin{ banks: banks.clone() }, tasks![BankSync(banks)])
    }
}

pub struct BankSync(Arc<Mutex<Option<BankInstitutions>>>);
#[async_trait]
impl Task for BankSync {
    fn interval(&self) -> Option<Duration> {Some(Duration::from_secs(10))}

    async fn run(&mut self, _h_ctx: &mut HeadlessContext) {
        let test_banks = vec![
            ("Sophtron Bank", "https://sophtron.com", "https://sophtron.com/../../Images/logo.png"),
            ("Sophtron Bank", "https://sophtron.com", "https://sophtron.com/../../Images/logo.png"),
            ("Sophtron Bank", "https://sophtron.com", "https://sophtron.com/../../Images/logo.png"),
            ("Sophtron Bank", "https://sophtron.com", "https://sophtron.com/../../Images/logo.png"),
        ].into_iter().map(|(name, link, image)| {
            let response = get(link).expect("Could not get link");
            let bytes = response.bytes().expect("Could not get bytes");
            let img: RgbaImage = load_from_memory(&bytes).expect("Could not load from memory.").into();
            // let img = ctx.add_image(img);
            (name, link, img)
        }).collect();
        *self.0.lock().unwrap() = Some(BankInstitutions(test_banks));
    }
}