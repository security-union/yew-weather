use js_sys::JsString;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use web_sys::console;
use yew::prelude::*;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
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

#[derive(PartialEq, Properties)]
struct PeriodComponentProps {
    pub period: Period
}

#[function_component(PeriodComponent)]
fn period_component(props: &PeriodComponentProps) -> Html {
    html! {
        <> 
            { props.period.start_time.to_owned()}
        </>
    }
}

#[function_component(App)]
fn app_component() -> Html {
    let forecast = use_state( || None);
    let forecast_clone = forecast.clone();
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
        forecast.set(Some(fetched_forecast));
    });

    match forecast_clone.as_ref() {
        Some(f) => {
            f.properties.periods.iter().map(|period| {
                html! {
                    <PeriodComponent period={period.clone()}/>
                }
            }).collect()
        },
        None => html!(
            {
                "No data yet".to_string()
            }
        )
    }
}

fn main() {
    yew::start_app::<App>();
}
