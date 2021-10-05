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
    arrows_enabled: bool,
    paths_enabled: bool,
    squares_enabled: bool,
    circles_enabled: bool,
    link: ComponentLink<Self>,
}

enum Msg {
    ToggleArrows,
    TogglePaths,
    ToggleCircles,
    ToggleSquares,
}

struct Circle {
    p: Point,
    r: usize,
}

struct Arrow {
    p: Point,
    angle: f32,
}

impl Arrow {
    fn draw(&self) -> Html {
        html! {
            <g transform={format!("rotate({},{},{})", Arrow::rad_to_deg(self.angle), self.p.x, self.p.y)}>
                <use x=self.p.x y=self.p.y href="#arrow" fill="black" />
            </g>
        }
    }

    fn rad_to_deg(rad: f32) -> f32 {
        -((rad + std::f32::consts::PI * 0.5) * 180.0 / std::f32::consts::PI)
    }
}
struct Square {
    p: Point,
    link_right: bool,
    link_down: bool,
}

impl Square {
    fn draw(&self) -> Html {
        let square = html! { <use x=self.p.x y=self.p.y href="#square" fill="black" /> };
        let link_right = if self.link_right {
            html! { <use x=self.p.x y=self.p.y href="#link_right" fill="black" />}
        } else {
            html! {}
        };
        let link_down = if self.link_down {
            html! {<use x=self.p.x y=self.p.y href="#link_down" fill="black" />}
        } else {
            html! {}
        };
        vec![square, link_right, link_down]
            .into_iter()
            .collect::<Html>()
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn distance_to(&self, p: &Point) -> f32 {
        ((self.x - p.x).powi(2) + (self.y - p.y).powi(2)).sqrt()
    }

    fn from_usize(x: usize, y: usize) -> Point {
        Point {
            x: x as f32,
            y: y as f32,
        }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
struct UsizePoint {
    x: usize,
    y: usize,
}

struct Path {
    items: Vec<Point>,
}

impl Path {
    fn draw(&self, color: &str) -> Html {
        let path = self
            .items
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
            });

        html! {
            <path d={path} stroke={color} stroke-width="1" fill="transparent"/>
        }
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
            arrows_enabled: false,
            paths_enabled: false,
            squares_enabled: true,
            circles_enabled: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleArrows => self.arrows_enabled = !self.arrows_enabled,
            Msg::TogglePaths => self.paths_enabled = !self.paths_enabled,
            Msg::ToggleSquares => self.squares_enabled = !self.squares_enabled,
            Msg::ToggleCircles => self.circles_enabled = !self.circles_enabled,
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
                <h1> { "ahoy!" } </h1>
                <svg
                    width={self.width}
                    height={self.height}
                    viewBox={format!("0 0 {} {}", self.width, self.height)}
                    fill="none"
                    xmlns="http://www.w3.org/2000/svg">
                    <defs>
                        <circle id="myCircle" cx="0" cy="0" r="10" />
                        <path d="M 0 0 L 0 10 L -1 9 L 1 9 L 0 10" id="arrow" stroke="black" fill="transparent"/>
                        <path d="M 0 0 L 0 10 L 10 10 L 10 0 L 0 0" id="square" stroke="black" fill="black"/>
                        <path d="M 10 3 L 10 7 L 14 7 L 14 3 L 10 3" id="link_right" stroke="black" fill="black"/>
                        <path d="M 3 10 L 3 14 L 7 14 L 7 10 L 3 10" id="link_down" stroke="black" fill="black"/>

                        <linearGradient id="myGradient" gradientTransform="rotate(90)">
                            <stop offset="10%" stop-color="white" />
                            <stop offset="90%" stop-color="gold" />
                        </linearGradient>
                    </defs>
                    {
                        if self.squares_enabled {
                            self.render_squares()
                        } else{
                            html!{}
                        }
                    }
                    {
                        if self.arrows_enabled {
                            self.render_arrows()
                        } else{
                            html!{}
                        }
                    }
                    {
                        if self.paths_enabled {
                            self.render_paths()
                        } else{
                            vec![html!{}]
                        }
                    }
                    {
                        if self.circles_enabled {
                            self.render_circles()
                        } else{
                            vec![html!{}]
                        }
                    }
                </svg>
                <br/>
                <input
                    type="checkbox"
                    id="toggle_arrows"
                    checked=self.arrows_enabled
                    onclick=self.link.callback(|_| Msg::ToggleArrows)
                />
                {" render arrows" }
                <br/>
                <input
                    type="checkbox"
                    id="toggle_circles"
                    checked=self.circles_enabled
                    onclick=self.link.callback(|_| Msg::ToggleCircles)
                />
                {" render circles" }
                <br/>
                <input
                    type="checkbox"
                    id="toggle_squares"
                    checked=self.squares_enabled
                    onclick=self.link.callback(|_| Msg::ToggleSquares)
                />
                {" render squares" }
                <br/>
                <input
                    type="checkbox"
                    id="toggle_paths"
                    checked=self.paths_enabled
                    onclick=self.link.callback(|_| Msg::TogglePaths)
                />
                {" render paths" }
                <br/>
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

    fn border_point(&self, i: usize) -> UsizePoint {
        UsizePoint {
            x: if i < 2 * self.width {
                i % self.width
            } else if i < 2 * self.width + self.height {
                0
            } else {
                self.width - 1
            },
            y: if i < self.width {
                0
            } else if i < 2 * self.width {
                self.height - 1
            } else {
                (i - 2 * self.width) % self.height
            },
        }
    }

    fn render_arrows(&self) -> Html {
        (0..self.height - self.step)
            .step_by(self.step)
            .skip(1)
            .map(|y| self.render_arrow_line(y))
            .collect::<Html>()
    }

    fn render_arrow_line(&self, y: usize) -> Html {
        (0..self.width - self.step)
            .step_by(self.step)
            .skip(1)
            .map(|x| {
                Arrow {
                    p: Point::from_usize(x, y),
                    angle: self.angle_at(Point::from_usize(x, y)),
                }
                .draw()
            })
            .collect::<Html>()
    }

    fn render_squares(&self) -> Html {
        (0..self.height - self.step)
            .step_by(self.step)
            .skip(1)
            .map(|y| self.render_square_line(y))
            .collect::<Html>()
    }

    fn render_square_line(&self, y: usize) -> Html {
        (0..self.width - self.step)
            .step_by(self.step)
            .skip(1)
            .map(|x| {
                let link_down = rand::thread_rng().gen_range(0, 2) > 0;
                let link_right = rand::thread_rng().gen_range(0, 2) > 0;
                Square {
                    p: Point::from_usize(x, y),
                    link_down,
                    link_right,
                }
                .draw()
            })
            .collect::<Html>()
    }

    fn gen_random_point(&self, diameter: usize, circles: &Vec<(Circle, &'static str)>) -> Point {
        let mut i = 0;
        loop {
            let x = rand::thread_rng().gen_range(0, self.width) as f32;
            let y = rand::thread_rng().gen_range(0, self.height) as f32;
            let p = Point { x, y };
            let mut matching_circles = circles
                .iter()
                .filter(|(c, _)| self.in_circle(&p, c, diameter));

            if matching_circles.next().is_none() {
                log!("worked at {}", i);
                return p;
            }
            i += 1;
            if i == 10000 {
                panic!("should never be reached");
            }
        }
    }

    fn circles(&self) -> Vec<(Circle, &'static str)> {
        let num_circles = 100;
        (0..num_circles).fold(Vec::new(), |mut acc, i| {
            acc.push(if i < 10 {
                (
                    Circle {
                        p: self.gen_random_point(60, &acc),
                        r: 50,
                    },
                    "#E4572E",
                )
            } else if i < 40 {
                (
                    Circle {
                        p: self.gen_random_point(20, &acc),
                        r: 20,
                    },
                    "#F3A712",
                )
            } else {
                (
                    Circle {
                        p: self.gen_random_point(10, &acc),
                        r: 10,
                    },
                    "#A8C686",
                )
            });
            acc
        })
    }

    fn render_circles(&self) -> Vec<Html> {
        let circles = self.circles();
        circles
            .iter()
            .map(|(circle, color)| {
                html! {
                    <circle cx={circle.p.x} cy={circle.p.y} r={circle.r} fill={color} />
                }
            })
            .collect()
    }

    fn render_paths(&self) -> Vec<Html> {
        let num_paths = (0.05 * ((self.width * self.height) as f32)) as usize;
        let mut all_points = Vec::new();
        let circles = self.circles();
        let borders = (self.width + self.height) * 2;
        (0..(num_paths + borders)).fold(Vec::new(), |mut acc, i| {
            let point = if i < borders {
                self.border_point(i)
            } else {
                self.random_point(&all_points)
            };
            let item = self.render_path(point);
            let color = self.select_path_color(&item, &circles);
            acc.push(item.draw(color));
            for p in item.items {
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

    fn select_path_color(
        &self,
        item: &Path,
        circles: &Vec<(Circle, &'static str)>,
    ) -> &'static str {
        let first_item = item.items.first().unwrap();
        let mut candidates = circles
            .iter()
            .filter(|(circle, _color)| self.in_circle(first_item, circle, 0));
        match candidates.next() {
            Some((_, color)) => color,
            None => "#669BBC",
        }
    }

    fn render_path(&self, p: UsizePoint) -> Path {
        let start_point = Point {
            x: p.x as f32,
            y: p.y as f32,
        };
        let length = (self.width + self.height) / 200;
        let path = Path {
            items: vec![start_point],
        };
        let val = (0..length).fold((path, start_point), |(mut acc, last_point), _i| {
            let angle = self.angle_at(last_point);
            let next_point = Point {
                x: last_point.x + angle.cos() * self.step as f32,
                y: last_point.y + angle.sin() * self.step as f32,
            };
            acc.items.push(next_point);
            (acc, next_point)
        });
        val.0
    }

    fn in_circle(&self, p: &Point, c: &Circle, other_radius: usize) -> bool {
        p.distance_to(&c.p) <= c.r as f32 + other_radius as f32
    }

    fn modify_angle_at(&self, p: Point, angle: f32) -> f32 {
        let max_effect_point = Point { x: 250.0, y: 250.0 };
        let distance = p.distance_to(&max_effect_point) * 4.0;
        let factor = 1.0 / ((distance / self.width as f32).powf(2.0) + 1.0);
        //log!(
        //    "point is {:?}, distance is {}, factor is {}",
        //    p,
        //    distance,
        //    factor
        //);

        let cos = angle.cos();
        let sin = angle.sin();
        let bias_x = 0.0;
        let bias_y = 1.0;
        let new_x = (1.0 - factor) * cos + factor * bias_x;
        let new_y = (1.0 - factor) * sin + factor * bias_y;

        new_y.atan2(new_x)
    }

    fn zero_to_one_flow_field(&self, p: Point) -> f32 {
        let height = self.height as f32;
        let width = self.width as f32;
        let x = width / ((p.x - 0.5 * width) * 0.2 - width)
            - ((p.x - 0.5 * width) * 2.0 - width * 0.5) / width;
        let y = p.y * p.y - height * height * 0.7;
        x * ((y / (height * height)) * 0.5)
    }

    fn angle_at(&self, p: Point) -> f32 {
        self.modify_angle_at(
            p,
            self.zero_to_one_flow_field(p) * std::f32::consts::PI * 2.0,
        )
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    App::<Model>::new().mount_to_body();
}
