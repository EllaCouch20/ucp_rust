use rust_on_rails::prelude::*;
use std::sync::{Arc, Mutex};
use image::{load_from_memory, RgbaImage};
use std::collections::HashMap;

pub struct UCPPlugin {
    banks: Arc<Mutex<BankInstitutions>>,
    captcha: Arc<Mutex<HashMap<&'static str, resources::Image>>>,
    my_bank: Arc<Mutex<Option<(&'static str, &'static str, RgbaImage)>>>,
    return_page: Arc<Mutex<Option<Box<dyn FnMut(&mut Context)>>>>,
    back_page: Arc<Mutex<Option<Box<dyn FnMut(&mut Context)>>>>,
}

impl UCPPlugin {
    pub async fn _init(&mut self) {
        println!("Initialized UCPPlugin");
    }

    pub fn get_banks(&self) -> Vec<(&'static str, &'static str, RgbaImage)> {
        self.banks.lock().unwrap().0.clone()
    }

    pub fn captcha_images(&self) -> HashMap<&'static str, resources::Image> {
        self.captcha.lock().unwrap().clone()
    }

    pub fn set_bank(&mut self, bank: (&'static str, &'static str, RgbaImage)) {
        *self.my_bank.lock().unwrap() = Some(bank)
    }

    pub fn get_bank(&self) -> (&'static str, &'static str, RgbaImage) {
        self.my_bank.lock().unwrap().as_mut().unwrap().clone()
    }

    pub fn on_return(&self) -> Option<Box<dyn FnMut(&mut Context)>> {
        self.return_page.lock().unwrap().take()
    }

    pub fn set_on_return(&mut self, action: Box<dyn FnMut(&mut Context)>) {
        *self.return_page.lock().unwrap() = Some(action);
    }

    pub fn back(&self) -> Option<Box<dyn FnMut(&mut Context)>> {
        self.back_page.lock().unwrap().take()
    }

    pub fn set_back(&mut self, action: Box<dyn FnMut(&mut Context)>) {
        *self.back_page.lock().unwrap() = Some(action);
    }
}

#[derive(Debug)]
pub struct BankInstitutions(Vec<(&'static str, &'static str, RgbaImage)>);

impl Plugin for UCPPlugin {
    async fn background_tasks(_ctx: &mut HeadlessContext) -> Tasks {vec![]}

    async fn new(ctx: &mut Context, _h_ctx: &mut HeadlessContext) -> (Self, Tasks) {
        ctx.include_assets(include_assets!("./assets"));

        // let test_banks = vec![
        //     ("Sophtron Bank", "https://sophtron.com", "https://docs.sophtron.com/favicon.ico"),
        //     ("MX Bank", "https://mx.com", "https://content.moneydesktop.com/storage/MD_Assets/Ipad%20Logos/100x100/INS-68e96dd6-eabd-42d3-9f05-416897f0746c_100x100.png"),
        //     ("Sophtron Bank", "https://sophtron.com", "https://docs.sophtron.com/favicon.ico"),
        //     ("MX Bank", "https://mx.com", "https://content.moneydesktop.com/storage/MD_Assets/Ipad%20Logos/100x100/INS-68e96dd6-eabd-42d3-9f05-416897f0746c_100x100.png"),
        // ];

        let test_banks = vec![
            ("Sophtron Bank", "https://sophtron.com", "sophtron.png"),
            ("MX Bank", "https://mx.com", "mx.png"),
            ("Wells Fargo", "https://wellsfargo.com", "wellsfargo.png"),
            ("Huntington Bank", "https://huntington.com", "huntington.png"),
            ("Ally", "https://ally.com", "ally.png"),
            ("American Express", "https://amex.com", "amex.png"),
            ("USAA", "https://usaa.com", "usaa.png"),
            ("Fidelity", "https://fidelity.com", "fidelity.png"),
            ("Discover", "https://discover.com", "discover.png"),
            ("Captial One", "https://captialone.com", "capital.png"),
        ];

        let mut banks = Vec::new();
        // only do this when first asking for it
        for (name, link, image) in test_banks.into_iter() {
            // let response = get(image).await.expect("Could not get link");
            // let bytes = response.bytes().await.expect("Could not get bytes");
            println!("image: {:?}", image);
            let img = &ctx.load_file(image).unwrap();
            let img: RgbaImage = load_from_memory(&img).expect("Could not load from memory.").into();
            banks.push((name, link, img))
        }

        let banks = Arc::new(Mutex::new(BankInstitutions(banks)));

        let mut captcha = HashMap::new();
        let paths = vec!["image1.jpeg", "image2.jpeg", "image3.jpeg", "image4.jpeg", "image5.jpeg", "image6.jpeg"];
        paths.into_iter().for_each(|path| {
            let img = &ctx.load_file(path).unwrap();
            let img = image::load_from_memory(img).unwrap();
            captcha.insert(path, ctx.add_image(img.into()));
        });

        let captcha = Arc::new(Mutex::new(captcha));
        let my_bank = Arc::new(Mutex::new(None));
        let return_page = Arc::new(Mutex::new(None));
        let back_page = Arc::new(Mutex::new(None));
        (UCPPlugin{ banks: banks.clone(), captcha, my_bank, return_page, back_page }, vec![])
    }
}