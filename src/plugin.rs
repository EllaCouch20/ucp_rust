use rust_on_rails::prelude::*;
// use pelican_ui::prelude::Text;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use reqwest::get;
use image::{load_from_memory, RgbaImage};

pub struct UCPPlugin {
    banks: Arc<Mutex<BankInstitutions>>,
    captcha: Arc<Mutex<String>>,
    my_bank: Arc<Mutex<Option<(&'static str, &'static str, RgbaImage)>>>,
}

impl UCPPlugin {
    pub async fn _init(&mut self) {
        println!("Initialized UCPPlugin");
    }

    pub fn get_banks(&mut self) -> Vec<(&'static str, &'static str, RgbaImage)> {
        self.banks.lock().unwrap().0.clone()
    }

    pub fn get_captcha(&mut self) -> String {
        self.captcha.lock().unwrap().clone()
    }

    pub fn set_bank(&mut self, bank: (&'static str, &'static str, RgbaImage)) {
        *self.my_bank.lock().unwrap() = Some(bank)
    }

    pub fn my_bank(&mut self) -> (&'static str, &'static str, RgbaImage) {
        self.my_bank.lock().unwrap().as_mut().unwrap().clone()
    }
}

#[derive(Debug)]
pub struct BankInstitutions(Vec<(&'static str, &'static str, RgbaImage)>);

impl Plugin for UCPPlugin {
    async fn background_tasks(_ctx: &mut HeadlessContext) -> Tasks {
        vec![]
    }

    async fn new(_ctx: &mut Context, _h_ctx: &mut HeadlessContext) -> (Self, Tasks) {
        let test_banks = vec![
            ("Sophtron Bank", "https://sophtron.com", "https://sophtron.com/Images/logo.png"),
            ("MX Bank", "https://mx.com", "https://content.moneydesktop.com/storage/MD_Assets/Ipad%20Logos/100x100/INS-68e96dd6-eabd-42d3-9f05-416897f0746c_100x100.png"),
            ("Sophtron Bank", "https://sophtron.com", "https://sophtron.com/Images/logo.png"),
            ("MX Bank", "https://mx.com", "https://content.moneydesktop.com/storage/MD_Assets/Ipad%20Logos/100x100/INS-68e96dd6-eabd-42d3-9f05-416897f0746c_100x100.png"),
        ];

        let mut banks = Vec::new();
        
        for (name, link, image) in test_banks.into_iter() {
            let response = get(image).await.expect("Could not get link");
            println!("RESPONSE {:?}", response);
            let bytes = response.bytes().await.expect("Could not get bytes");
            let img: RgbaImage = load_from_memory(&bytes).expect("Could not load from memory.").into();
            banks.push((name, link, img))
        }
        let banks = Arc::new(Mutex::new(BankInstitutions(banks)));

        let captcha = reqwest::get("https://sophtron.com/../serviceClients/sophtronClient/v2")
                .await.expect("Couldn't get request")
                .text()
                .await.expect("Couldn't get text");

        let captcha = Arc::new(Mutex::new(captcha));
        let my_bank = Arc::new(Mutex::new(None));
        (UCPPlugin{ banks: banks.clone(), captcha, my_bank }, tasks![BankSync(banks)])
    }
}

pub struct BankSync(Arc<Mutex<BankInstitutions>>);
#[async_trait]
impl Task for BankSync {
    fn interval(&self) -> Option<Duration> {Some(Duration::from_secs(10))}

    async fn run(&mut self, _h_ctx: &mut HeadlessContext) {
        
        let test_banks = vec![
            ("Sophtron Bank", "https://sophtron.com", "https://sophtron.com/Images/logo.png"),
            ("MX Bank", "https://mx.com", "https://content.moneydesktop.com/storage/MD_Assets/Ipad%20Logos/100x100/INS-68e96dd6-eabd-42d3-9f05-416897f0746c_100x100.png"),
            ("Sophtron Bank", "https://sophtron.com", "https://sophtron.com/Images/logo.png"),
            ("MX Bank", "https://mx.com", "https://content.moneydesktop.com/storage/MD_Assets/Ipad%20Logos/100x100/INS-68e96dd6-eabd-42d3-9f05-416897f0746c_100x100.png"),
        ];

        let mut banks = Vec::new();
        
        for (name, link, image) in test_banks.into_iter() {
            let response = get(image).await.expect("Could not get link");
            let bytes = response.bytes().await.expect("Could not get bytes");
            let img: RgbaImage = load_from_memory(&bytes).expect("Could not load from memory.").into();
            banks.push((name, link, img))
        }

        *self.0.lock().unwrap() = BankInstitutions(banks);
    }
}

// let ApiEndpoints.INSTITUTIONS = './institutions'

// loadPopularInstitutions(query) {
//     const url =
//       typeof query === 'undefined'
//         ? `${ApiEndpoints.INSTITUTIONS}/favorite`
//         : `${ApiEndpoints.INSTITUTIONS}/favorite${FireflyAPI.buildQueryString(query)}`

//     return axiosInstance.get(url).then(response => {
//       return response.data
//     })
//   },

//   buildQueryString(queryObj) {
//     return _reduce(
//       queryObj,
//       (queryStr, value, queryName) => {
//         const queryParam = FireflyAPI.buildQueryParameter(queryName, value)

//         return queryStr === '' ? `?${queryParam}` : `${queryStr}&${queryParam}`
//       },
//       '',
//     )
//   },

//   buildQueryParameter(key, value) {
//     return _isArray(value)
//       ? value.map(val => `${key}[]=${encodeURIComponent(val)}`).join('&')
//       : `${key}=${encodeURIComponent(value)}`
//   },