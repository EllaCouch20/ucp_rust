use std::collections::{BTreeSet, BTreeMap};
use std::time::Duration;

use maverick_os::runtime::{Channel, Service, ServiceContext, async_trait, Callback, Error};
use maverick_os::hardware;
use maverick_os::air::AirService;
use maverick_os::State;
use maverick_os::air::air;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Responses(pub BTreeMap<String, Vec<u8>>);

pub struct APIService;

#[async_trait]
impl Service for APIService {
    async fn new(_hardware: &mut hardware::Context) -> Self {
        APIService
    }

    async fn run(&mut self, ctx: &mut ServiceContext, channel: &mut Channel) -> Result<Duration, Error> {
        match async {
            while let Some(request) = channel.receive() {
                let response = reqwest::get(request.clone()).await?.bytes().await?.to_vec();
                channel.send(serde_json::to_string(&(request, response)).unwrap());
            }

            Ok::<(), Error>(())
        }.await {
            Ok(()) => {},
            Err(e) => log::error!("{:?}", e)
        }

        Ok(Duration::from_millis(100))
    }

    fn callback(&self) -> Box<Callback> {Box::new(|state: &mut State, response: String| {
        let mut responses = state.get::<Responses>().0;
        let (request, response) = serde_json::from_str::<(String, Vec<u8>)>(&response).unwrap();
        responses.insert(request, response);
        state.set(&Responses(responses));
    })}
}
