use js_sys::JsString;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use web_sys::console;
use yew::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Period {
    pub name: String,
    pub start_time: String,
    pub end_time: String,
    pub is_daytime: bool,
    pub temperature: f32,
    pub temperature_unit: String,
    pub wind_speed: String,
    pub wind_direction: String,
    pub icon: String,
    pub short_forecast: String,
    pub detailed_forecast: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Properties {
    pub periods: Vec<Period>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Forecast {
    pub properties: Properties
}

#[function_component(App)]
fn app_component() -> Html {
    wasm_bindgen_futures::spawn_local(async move {
        let forecast_endpoint = format!(
            "https://api.weather.gov/gridpoints/{office}/{x},{y}/forecast",
            office="DTX",
            x = 65,
            y = 33
        );
        let fetched_forecast: Forecast = Request::get(&forecast_endpoint)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        console::log_1(&JsString::from(
            serde_json::to_string(&fetched_forecast).unwrap())
        ) 
    });

    html!(
        {"Hi"}
    )
}

fn main() {
    yew::start_app::<App>();
}
