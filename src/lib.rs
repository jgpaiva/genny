#![recursion_limit = "256"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    step: usize,
    width: usize,
    height: usize,
    link: ComponentLink<Self>,
    value: i64,
}

enum Msg {
    AddOne,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            width: 500,
            height: 500,
            step: 15,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        /*
        <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
        <p>{ self.value }</p>
        */
        /*
        <path d={format!("M 10 250 T 10 250 T {} {} T 490 250", point[0], point[1])} stroke="blue" fill="transparent"/>
        <path d={format!("M 10 250 Q {} {} 490 250", point[0], point[1])} stroke="black" fill="transparent"/>
        <path d={format!("M 10 270 Q {} {} 490 270", point[0], point[1])} stroke="black" fill="transparent"/>
        <path d={format!("M 10 290 Q {} {} 490 290", point[0], point[1])} stroke="black" fill="transparent"/>
        <path d={format!("M 10 150 Q {} {} 490 150", point[0], point[1])} stroke="black" fill="transparent"/>
        <path d={format!("M 10 130 Q {} {} 490 130", point[0], point[1])} stroke="black" fill="transparent"/>
        <path d={format!("M 10 110 Q {} {} 490 110", point[0], point[1])} stroke="black" fill="transparent"/>
        */
        let rotation = 90;
        html! {
            <div class="container">
                <svg
                    width={self.width}
                    height={self.height}
                    viewBox={format!("0 0 {} {}", self.width, self.height)}
                    fill="none"
                    xmlns="http://www.w3.org/2000/svg">
                    <defs>
                        <circle id="myCircle" cx="0" cy="0" r="10" />

                        <linearGradient id="myGradient" gradientTransform="rotate(90)">
                            <stop offset="10%" stop-color="cyan" />
                            <stop offset="90%" stop-color="red" />
                        </linearGradient>
                    </defs>
                    {self.render_board()}
                </svg>
            </div>
        }
    }
}

impl Model {
    fn render_board(&self) -> Html {
        html! {{
            (0..self.height-self.step).step_by(self.step).skip(1).map(|y| self.render_line(y)).collect::<Html>()
        }}
    }

    fn render_line(&self, y: usize) -> Html {
        html! {{
            (0..self.width-self.step).step_by(self.step).skip(1).map(|x| self.render_item(x, y)).collect::<Html>()
        }}
    }

    fn render_item(&self, x: usize, y: usize) -> Html {
        let angle = ((y as f32) / self.height as f32) * 180.0;
        html! {
            <g transform={format!("rotate({} {} {})", angle, x, y)}>
                <use x=x y=y href="#myCircle" fill="url('#myGradient')" />
            </g>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
