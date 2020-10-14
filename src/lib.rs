#![recursion_limit = "1024"]

use rand::Rng;
use std::convert::TryFrom;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
extern crate console_error_panic_hook;
use std::panic;
extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

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

struct Line {
    items: Vec<Point>,
}

struct Circle {
    p: Point,
    r: usize,
}

#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
struct UsizePoint {
    x: usize,
    y: usize,
}

impl Line {
    fn draw(&self) -> std::string::String {
        self.items
            .iter()
            .enumerate()
            .fold("".to_string(), |acc, (i, item)| {
                format!(
                    "{} {} {} {}",
                    acc,
                    if i == 0 { "M" } else { "L" },
                    item.x,
                    item.y
                )
            })
    }
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
        /*let mut all_points = (0..self.width)
        .flat_map(|x| {
            (0..self.height)
                .map(|y| UsizePoint { x, y })
                .collect::<Vec<UsizePoint>>()
        })
        .collect::<Vec<UsizePoint>>();*/
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
                        <path d="M 0 0 L 0 10 L -1 9 L 1 9 L 0 10" id="arrow" stroke="black" fill="transparent"/>

                        <linearGradient id="myGradient" gradientTransform="rotate(90)">
                            <stop offset="10%" stop-color="white" />
                            <stop offset="90%" stop-color="gold" />
                        </linearGradient>
                    </defs>
                    //{self.render_arrows()}
                    {self.render_paths()}
                </svg>
            </div>
        }
    }
}

impl Model {
    fn random_point(&self, points: &Vec<UsizePoint>) -> UsizePoint {
        let mut i = 0;
        loop {
            let x = rand::thread_rng().gen_range(0, self.width);
            let y = rand::thread_rng().gen_range(0, self.height);
            let p = UsizePoint { x, y };
            if !points.contains(&p) {
                log!("worked at {}", i);
                return p;
            }
            i += 1;
            if i == 10000 {
                panic!("should never be reached");
            }
        }
    }

    fn render_arrows(&self) -> Html {
        html! {{
            (0..self.height-self.step).step_by(self.step).skip(1).map(|y| self.render_arrow_line(y)).collect::<Html>()
        }}
    }

    fn render_arrow_line(&self, y: usize) -> Html {
        html! {{
            (0..self.width-self.step).step_by(self.step).skip(1).map(|x| self.render_arrow(UsizePoint{x, y})).collect::<Html>()
        }}
    }

    fn render_arrow(&self, p: UsizePoint) -> Html {
        let angle = self.angle_at_deg(p.x, p.y);
        html! {
            <g transform={format!("rotate({},{},{})", angle, p.x, p.y)}>
                <use x=p.x y=p.y href="#arrow" fill="url('#myGradient')" />
            </g>
        }
    }

    fn render_paths(&self) -> Vec<Html> {
        let num_lines = 10000;
        let mut all_points = Vec::new();
        let circles = vec![
            (
                Circle {
                    p: Point { x: 150.0, y: 100.0 },
                    r: 100,
                },
                "#E4572E",
            ),
            (
                Circle {
                    p: Point { x: 200.0, y: 300.0 },
                    r: 100,
                },
                "#F3A712",
            ),
            (
                Circle {
                    p: Point { x: 300.0, y: 250.0 },
                    r: 100,
                },
                "#A8C686",
            ),
        ];
        (0..num_lines).fold(Vec::new(), |mut acc, _i| {
            let item = self.render_path(self.random_point(&all_points));
            acc.push(html! {
                <path d={item.draw()} stroke={
                    let first_item = item.items.first().unwrap();
                    let mut candidates = circles
                        .iter().filter(|(circle, _color)|
                                self.in_circle(first_item, circle));
                    match candidates.next() {
                        Some((_, color)) => color,
                        None => "#669BBC"
                    }
                } stroke-width="1" fill="transparent"/>
            });
            for i in item.items {
                let p = i;
                let x = p.x as u32;
                let y = p.y as u32;
                if x < self.width as u32 && y < self.height as u32 {
                    let x = usize::try_from(x).unwrap();
                    let y = usize::try_from(y).unwrap();
                    all_points.push(UsizePoint { x, y });
                }
            }
            acc
        })
    }

    fn render_path(&self, p: UsizePoint) -> Line {
        let start_point = Point {
            x: p.x as f32,
            y: p.y as f32,
        };
        let length = 15;
        let line = Line {
            items: vec![start_point],
        };
        let val = (0..length).fold((line, start_point), |(mut acc, last_point), _i| {
            let angle = self.angle_at_rad(last_point);
            let next_point = Point {
                x: last_point.x + angle.cos() * self.step as f32,
                y: last_point.y + angle.sin() * self.step as f32,
            };
            acc.items.push(next_point);
            return (acc, next_point);
        });
        val.0
    }

    fn in_circle(&self, p: &Point, c: &Circle) -> bool {
        ((p.x - c.p.x).powi(2) + (p.y - c.p.y).powi(2)).sqrt() < c.r as f32
    }

    fn angle_calculation(&self, p: Point) -> f32 {
        let height = self.height as f32;
        let width = self.width as f32;
        let x = p.x * 2.0 - width * 0.5;
        let y = p.y * p.y - height * height * 0.7;
        (x / width) * ((y / (height * height)) * 0.5)
    }

    fn angle_at_rad(&self, p: Point) -> f32 {
        self.angle_calculation(p) * 2.0 * std::f32::consts::PI + 0.5 * std::f32::consts::PI
    }

    fn angle_at_deg(&self, x: usize, y: usize) -> f32 {
        let x = x as f32;
        let y = y as f32;
        self.angle_calculation(Point { x, y }) * 360.0
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    App::<Model>::new().mount_to_body();
}
