use crate::components::{
    control::{Cities, Control},
    map_component::{City, MapComponent, Point},
};
use yew::prelude::*;
mod components;

enum Msg {
    SelectCity(City),
}

struct Model {
    city: City,
    cities: Cities,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let aachen = City {
            name: "Fuji".to_string(),
            lat: Point(35.3622222, 138.7313889),
        };
        let stuttgart = City {
            name: "Stuttgart".to_string(),
            lat: Point(48.7784f64, 9.1742f64),
        };
        let cities: Cities = Cities {
            list: vec![aachen, stuttgart],
        };
        let city = cities.list[0].clone();
        Self { city, cities }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SelectCity(city) => {
                self.city = self
                    .cities
                    .list
                    .iter()
                    .find(|c| c.name == city.name)
                    .unwrap()
                    .clone();
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cb = ctx.link().callback(Msg::SelectCity);
        html! {
            <>
                <MapComponent city={&self.city}  />
                <Control select_city={cb} cities={&self.cities}/>
            </>
        }
    }
}
#[function_component(App)]
fn app() -> Html {
    let aachen = City {
        name: "Fuji".to_string(),
        lat: Point(35.3622222, 138.7313889),
    };
    let stuttgart = City {
        name: "Stuttgart".to_string(),
        lat: Point(48.7784f64, 9.1742f64),
    };
    let cities: Cities = Cities {
        list: vec![aachen, stuttgart],
    };
    let city = use_state(|| cities.list[0].clone());
    let cb = Callback::from({
        let city = city.clone();
        move |city_: City| {
            city.set(city_);
        }
    });
    html! {
        <>
            <MapComponent city={&*city}  />
            <Control select_city={cb} cities={cities}/>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
