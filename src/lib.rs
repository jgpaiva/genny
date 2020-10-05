#![recursion_limit = "256"]

use rand::Rng;
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
        let num_lines = 100;
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
                        <path d="M 0 0 L 0 10 L -1 9 L 1 9 L 0 10" id="myTest" stroke="black" fill="transparent"/>

                        <linearGradient id="myGradient" gradientTransform="rotate(90)">
                            <stop offset="10%" stop-color="white" />
                            <stop offset="90%" stop-color="gold" />
                        </linearGradient>
                    </defs>
                    {self.render_board()}
                    {
                        (0..num_lines).map({|_i|
                                            self.render_path(rand::thread_rng().gen_range(0, self.width),
                                                            rand::thread_rng().gen_range(0, self.height))
                        }).collect::<Html>()
                    }
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
        let angle = self.angle_at_deg(x, y);
        html! {
            <g transform={format!("rotate({},{},{})", angle, x, y)}>
                <use x=x y=y href="#myTest" fill="url('#myGradient')" />
            </g>
        }
    }

    fn render_path(&self, x: usize, y: usize) -> Html {
        let start_point = (x as f32, y as f32);
        let length = 35;
        let val = (0..length).fold(
            (
                (format!("M {} {}", start_point.0, start_point.1)),
                start_point,
            ),
            |(acc, last_point), _i| {
                let angle = self.angle_at_rad(last_point.0, last_point.1);
                let next_point = (
                    last_point.0 + angle.cos() * self.step as f32,
                    last_point.1 + angle.sin() * self.step as f32,
                );
                return (
                    format!("{} L {} {}", acc, next_point.0, next_point.1),
                    next_point,
                );
            },
        );
        html! {
            <path d={val.0} stroke="red" stroke-width="3" fill="transparent"/>
        }
    }

    fn angle_at_rad(&self, x: f32, y: f32) -> f32 {
        2.0 * ((if x < 200.0 { x - 100.0 } else { -x }) / (self.width as f32) * 2.5)
            * ((y - 100.0) / (self.height as f32))
            * std::f32::consts::PI
            + 0.5 * std::f32::consts::PI
    }

    fn angle_at_deg(&self, x: usize, y: usize) -> f32 {
        let x = x as f32;
        let y = y as f32;
        2.0 * ((if x < 200.0 { x - 100.0 } else { -x }) / (self.width as f32) * 2.5)
            * ((y - 100.0) / (self.height as f32))
            * 180.0
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
