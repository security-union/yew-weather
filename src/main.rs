use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
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
    pub periods: Vec<Period>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Forecast {
    pub properties: Properties,
}

#[derive(PartialEq, Properties)]
struct PeriodComponentProps {
    pub period: Period,
}

#[function_component(PeriodComponent)]
fn period_component(props: &PeriodComponentProps) -> Html {
    let PeriodComponentProps { period } = props;
    html! {
        <div class="period">
            <div class="name">{period.name.to_owned()}</div>
            <div class="temp">{period.temperature.to_owned()}{period.temperature_unit.to_owned()}</div>
            <div class="forecast">{period.short_forecast.to_owned()}</div>
            <img src={period.icon.to_owned()}/>
        </div>
    }
}

#[function_component(App)]
fn app_component() -> Html {
    let forecast = Box::new(use_state(|| None));
    let error = Box::new(use_state(|| None));
    let retry = {
        let forecast = forecast.clone();
        let error = error.clone();
        Callback::from(move |_| {
            let forecast = forecast.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let forecast_endpoint = format!(
                    "https://api.weather.gov/gridpoints/{office}/{x},{y}/forecast",
                    office = "DTX",
                    x = 65,
                    y = 33
                );
                let fetched_forecast = Request::get(&forecast_endpoint).send().await;

                match fetched_forecast {
                    Ok(response) => {
                        let json: Result<Forecast, _> = response.json().await;
                        match json {
                            Ok(f) => {
                                forecast.set(Some(f));
                            }
                            Err(e) => error.set(Some(e.to_string())),
                        }
                    }
                    Err(e) => error.set(Some(e.to_string())),
                }
            });
        })
    };

    match (*forecast).as_ref() {
        Some(f) => f
            .properties
            .periods
            .iter()
            .map(|period| {
                html! {
                    <PeriodComponent period={period.clone()}/>
                }
            })
            .collect(),
        None => match (*error).as_ref() {
            Some(e) => {
                html! {
                    <>
                        {"error"} {e}
                        <button onclick={retry}>{"retry"}</button>
                    </>
                }
            }
            None => {
                html! {
                    <>
                        {"No data yet"}
                        <button onclick={retry}>{"Call API"}</button>
                    </>
                }
            }
        },
    }
}

fn main() {
    yew::start_app::<App>();
}
