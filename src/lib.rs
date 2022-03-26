#![recursion_limit = "1024"]
#![allow(clippy::unused_unit)]

use gloo::storage::{LocalStorage, Storage};
use rand::Rng;
use std::{
    collections::{HashMap, HashSet},
    convert::TryFrom,
    str::FromStr,
};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

extern crate console_error_panic_hook;
use std::panic;
extern crate web_sys;
use serde::{Deserialize, Serialize};

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    }
}

const STORAGE_KEY: &str = "yew.genny.database";

struct Model {
    p: ModelProperties,
}

#[derive(Serialize, Deserialize)]
struct ModelProperties {
    step: usize,
    arrows_enabled: bool,
    paths_enabled: bool,
    squares_enabled: bool,
    circles_enabled: bool,
    color_scheme: String,
    variant: Variant,
    size: Size,
}

impl Default for ModelProperties {
    fn default() -> Self {
        Self {
            step: 15,
            arrows_enabled: false,
            paths_enabled: false,
            squares_enabled: true,
            circles_enabled: false,
            variant: Variant::Filled,
            size: Size::Small,
            color_scheme: "accented".to_owned(),
        }
    }
}

impl Model {
    fn colors() -> HashMap<String, Vec<String>> {
        vec![
            (
                "bluish",
                vec!["#CDC392", "#E8E5DA", "#9EB7E5", "#648DE5", "#304C89"],
            ),
            (
                "tropical",
                vec!["#BF3100", "#8EA604", "#D76A03", "#EC9F05", "#F5BB00"],
            ),
            (
                "accented",
                vec!["#011627", "#D5CAD6", "#2EC4B6", "#E71D36", "#FF9F1C"],
            ),
            (
                "pastel",
                vec!["#1A535C", "#4ECDC4", "#721817", "#FF6B6B", "#F49D37"],
            ),
            (
                "reddish",
                vec!["#370617", "#DC2F02", "#F48C06", "#FFBA08", "#9D0208"],
            ),
        ]
        .iter()
        .map(|(k, v)| {
            (
                (*k).to_owned(),
                v.iter().map(|x| (*x).to_owned()).collect::<Vec<String>>(),
            )
        })
        .collect()
    }
}

#[allow(dead_code)]
enum Msg {
    ToggleArrows,
    TogglePaths,
    ToggleCircles,
    ToggleSquares,
    UpdateColor(String),
    UpdateVariant(String),
    UpdateSize(String),
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
                <use x={self.p.x.to_string()} y={self.p.y.to_string()} href="#arrow" fill="black" />
            </g>
        }
    }

    fn rad_to_deg(rad: f32) -> f32 {
        -((rad + std::f32::consts::PI * 0.5) * 180.0 / std::f32::consts::PI)
    }
}

struct InitialSquare {
    p: Point,
    link_right: bool,
    link_down: bool,
}

struct WithLinksSquare {
    p: Point,
    link_up: bool,
    link_left: bool,
    link_right: bool,
    link_down: bool,
}
#[allow(dead_code)]
struct WithClustersSquare {
    p: Point,
    link_up: bool,
    link_left: bool,
    link_right: bool,
    link_down: bool,
    cluster_id: usize,
    cluster_size: usize,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
enum Variant {
    Outline,
    Filled,
}

impl ToString for Variant {
    fn to_string(&self) -> String {
        match self {
            Variant::Outline => "Outline".to_owned(),
            Variant::Filled => "Filled".to_owned(),
        }
    }
}

impl FromStr for Variant {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "Outline" {
            Ok(Variant::Outline)
        } else if s == "Filled" {
            Ok(Variant::Filled)
        } else {
            Err(format!("Could not parse Variant from str: {}", s))
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
enum Size {
    Small,
    Medium,
    Large,
}

impl ToString for Size {
    fn to_string(&self) -> String {
        match self {
            Size::Small => "Small".to_owned(),
            Size::Medium => "Medium".to_owned(),
            Size::Large => "Large".to_owned(),
        }
    }
}

impl FromStr for Size {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "Small" {
            Ok(Size::Small)
        } else if s == "Medium" {
            Ok(Size::Medium)
        } else if s == "Large" {
            Ok(Size::Large)
        } else {
            Err(format!("Could not parse size from str: {}", s))
        }
    }
}

impl WithClustersSquare {
    fn draw(
        &self,
        squares: &[Vec<WithClustersSquare>],
        colors: &[String],
        variant: Variant,
    ) -> Html {
        let max_cluster_size = squares
            .iter()
            .flatten()
            .map(|square| square.cluster_size)
            .max()
            .expect("there's always at least the current cluster");
        let color = if self.cluster_size == 1 {
            colors[0].to_owned()
        } else if (self.cluster_size as f32) < ((2.0 / 5.0) * max_cluster_size as f32) {
            colors[1].to_owned()
        } else if (self.cluster_size as f32) < ((3.0 / 5.0) * max_cluster_size as f32) {
            colors[2].to_owned()
        } else if self.cluster_size == max_cluster_size {
            colors[4].to_owned()
        } else {
            colors[3].to_owned()
        };
        match variant {
            Variant::Filled => {
                let square =
                    html! { <use x={self.p.x.to_string()} y={self.p.y.to_string()} href="#square" stroke={color.clone()} fill={color.clone()}/>};
                let link_right = if self.link_right {
                    html! { <use x={self.p.x.to_string()} y={self.p.y.to_string()} href="#link_right" stroke={color.clone()} fill={color.clone()}/>}
                } else {
                    html! {}
                };
                let link_down = if self.link_down {
                    html! {<use x={self.p.x.to_string()} y={self.p.y.to_string()} href="#link_down" stroke={color.clone()} fill={color.clone()}/>}
                } else {
                    html! {}
                };
                vec![square, link_right, link_down]
            }
            Variant::Outline => {
                let top = if self.link_up {
                    html! {<use x={self.p.x.to_string()} y={self.p.y.to_string()} href="#connection_up" stroke ={color.clone()}/>}
                } else {
                    html! {<use x={self.p.x.to_string()} y={self.p.y.to_string()} href="#closed_top"  stroke ={color.clone()}/>}
                };
                let right = if self.link_right {
                    html! {<use x={self.p.x.to_string()} y={self.p.y.to_string()} href="#connection_right" stroke ={color.clone()}/>}
                } else {
                    html! {<use x={self.p.x.to_string()} y={self.p.y.to_string()} href="#closed_right" stroke ={color.clone()}/>}
                };
                let left = if self.link_left {
                    html! {<use x={self.p.x.to_string()} y={self.p.y.to_string()} href="#connection_left" stroke ={color.clone()}/>}
                } else {
                    html! {<use x={self.p.x.to_string()} y={self.p.y.to_string()} href="#closed_left" stroke ={color.clone()}/>}
                };
                let bottom = if self.link_down {
                    html! {<use x={self.p.x.to_string()} y={self.p.y.to_string()} href="#connection_down" stroke ={color.clone()}/>}
                } else {
                    html! {<use x={self.p.x.to_string()} y={self.p.y.to_string()} href="#closed_bottom" stroke ={color.clone()}/>}
                };
                vec![top, left, right, bottom]
            }
        }
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
            <path d={path} stroke={color.to_string()} stroke-width="1" fill="transparent"/>
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_ctx: &yew::Context<Self>) -> Self {
        let p = LocalStorage::get(STORAGE_KEY).unwrap_or_else(|_| ModelProperties::default());

        let p = if Model::colors().get(&p.color_scheme).is_some() {
            p
        } else {
            ModelProperties::default()
        };

        Self { p }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleArrows => self.p.arrows_enabled = !self.p.arrows_enabled,
            Msg::TogglePaths => self.p.paths_enabled = !self.p.paths_enabled,
            Msg::ToggleSquares => self.p.squares_enabled = !self.p.squares_enabled,
            Msg::ToggleCircles => self.p.circles_enabled = !self.p.circles_enabled,
            Msg::UpdateColor(color_scheme) => {
                if Model::colors().get(&color_scheme).is_some() {
                    self.p.color_scheme = color_scheme;
                } else {
                    log!("color scheme invalid: {}", color_scheme);
                }
            }
            Msg::UpdateVariant(variant) => {
                let variant = if variant == "Outline" {
                    Variant::Outline
                } else if variant == "Filled" {
                    Variant::Filled
                } else {
                    unreachable!()
                };
                self.p.variant = variant;
            }
            Msg::UpdateSize(variant) => {
                let size = if variant == "Small" {
                    Size::Small
                } else if variant == "Medium" {
                    Size::Medium
                } else if variant == "Large" {
                    Size::Large
                } else {
                    unreachable!()
                };
                self.p.size = size;
            }
        }
        LocalStorage::set(STORAGE_KEY, &self.p).expect("failed to set");
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
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
                <div class="row align-items-center">
                    <div class="col-sm-9">
                        <svg
                            viewBox={format!("0 0 {} {}", self.get_width(), self.get_height())}
                            fill="none"
                            xmlns="http://www.w3.org/2000/svg">
                            <defs>
                                <circle id="myCircle" cx="0" cy="0" r="10" />
                                <path d="M 0 0 L 0 10 L -1 9 L 1 9 L 0 10 Z" id="arrow" stroke="black" fill="transparent"/>
                                <rect x="0" y="0" width="10" height="10" rx="3" ry ="3" id="square"/>
                                <path d="M 7 0 L 7 10 L 17 10 L 17 0 L 10 0 Z" id="link_right" />
                                <path d="M 0 7 L 0 17 L 10 17 L 10 7 L 0 7 Z" id="link_down" />

                                <path d="M 0 0 L 10 0" id="closed_top" stroke-linecap="round"/>
                                <path d="M 10 0 L 10 10" id="closed_right" stroke-linecap="round"/>
                                <path d="M 0 0 L 0 10" id="closed_left" stroke-linecap="round"/>
                                <path d="M 0 10 L 10 10" id="closed_bottom" stroke-linecap="round"/>

                                <path d="M 0 0 L 0 -5" id="connection_up" stroke-linecap="round"/>
                                <path d="M 10 0 L 15 0" id="connection_right" stroke-linecap="round"/>
                                <path d="M 0 10 L -5 10" id="connection_left" stroke-linecap="round"/>
                                <path d="M 10 10 L 10 15" id="connection_down" stroke-linecap="round"/>

                                <linearGradient id="myGradient" gradientTransform="rotate(90)">
                                    <stop offset="10%" stop-color="white" />
                                    <stop offset="90%" stop-color="gold" />
                                </linearGradient>
                            </defs>
                            {
                                if self.p.squares_enabled {
                                    self.render_squares()
                                } else{
                                    html!{}
                                }
                            }
                            {
                                if self.p.arrows_enabled {
                                    self.render_arrows()
                                } else{
                                    html!{}
                                }
                            }
                            {
                                if self.p.paths_enabled {
                                    self.render_paths()
                                } else{
                                    vec![html!{}]
                                }
                            }
                            {
                                if self.p.circles_enabled {
                                    self.render_circles()
                                } else{
                                    vec![html!{}]
                                }
                            }
                        </svg>
                    </div>
                    <div class="col-sm-3">
                        <div class="row text-center">
                            <div class="col">
                                {"Choose theme: " }
                                <br/>
                                {
                                    self.render_color_options(ctx)
                                }
                            </div>
                        </div>
                        <div class="row text-center">
                            <div class="col">
                                {"Choose variant: " }
                                <br/>
                                {
                                    self.render_variant_options(ctx)
                                }
                            </div>
                        </div>
                        <div class="row text-center">
                            <div class="col">
                                {"Choose size: " }
                                <br/>
                                {
                                    self.render_size_options(ctx)
                                }
                            </div>
                        </div>
                    </div>
                </div>
                /*
                <input
                    type="checkbox"
                    id="toggle_arrows"
                    checked=self.p.arrows_enabled
                    onclick=self.link.callback(|_| Msg::ToggleArrows)
                />
                {" render arrows" }
                <br/>
                <input
                    type="checkbox"
                    id="toggle_circles"
                    checked=self.p.circles_enabled
                    onclick=self.link.callback(|_| Msg::ToggleCircles)
                />
                {" render circles" }
                <br/>
                <input
                    type="checkbox"
                    id="toggle_squares"
                    checked=self.p.squares_enabled
                    onclick=self.link.callback(|_| Msg::ToggleSquares)
                />
                {" render squares" }
                <br/>
                <input
                    type="checkbox"
                    id="toggle_paths"
                    checked=self.p.paths_enabled
                    onclick=self.link.callback(|_| Msg::TogglePaths)
                />
                {" render paths" }
                <br/>*/
            </div>
        }
    }
}

impl Model {
    fn get_width(&self) -> usize {
        let base_size = 170;
        match self.p.size {
            Size::Small => base_size,
            Size::Medium => base_size * 2,
            Size::Large => base_size * 4,
        }
    }

    fn get_height(&self) -> usize {
        self.get_width()
    }

    fn random_point(&self, points: &[UsizePoint]) -> UsizePoint {
        let mut i = 0;

        loop {
            let x = rand::thread_rng().gen_range(0..self.get_width());
            let y = rand::thread_rng().gen_range(0..self.get_height());
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
            x: if i < 2 * self.get_width() {
                i % self.get_width()
            } else if i < 2 * self.get_width() + self.get_height() {
                0
            } else {
                self.get_width() - 1
            },
            y: if i < self.get_width() {
                0
            } else if i < 2 * self.get_width() {
                self.get_height() - 1
            } else {
                (i - 2 * self.get_width()) % self.get_height()
            },
        }
    }

    fn render_color_options(&self, ctx: &Context<Self>) -> Html {
        html! {
            <select name="colors" id="colors" onchange={ctx.link().callback(|e: Event|{
                let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                Msg::UpdateColor(select.value())
            })}>
            {{
                let self_colors = Self::colors();
                let mut colors:Vec<_> = self_colors.keys().collect();
                colors.sort();
                colors.into_iter().map(|color_name|{
                    if self.p.color_scheme == *color_name {
                        html!{
                            <option value={color_name.clone()} selected=true>{color_name}</option>
                        }
                    } else {
                        html!{
                            <option value={color_name.clone()}>{color_name}</option>
                        }
                    }
                }).collect::<Html>()
            }}
            </select>
        }
    }

    fn render_variant_options(&self, ctx: &Context<Self>) -> Html {
        html! {
            <select name="variants" id="variants" onchange={ctx.link().callback(|e: Event| {
                let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                Msg::UpdateVariant(select.value())
            })}>
            {{
                let variants:Vec<String> = vec![Variant::Filled.to_string(), Variant::Outline.to_string()];
                variants.iter().map(|variant|{
                    if self.p.variant.to_string() == *variant {
                        html!{<option value={variant.clone()} selected= true>{variant}</option>}
                    } else {
                        html!{<option value={variant.clone()}>{variant}</option>}
                    }
                }).collect::<Html>()
            }}
            </select>
        }
    }

    fn render_size_options(&self, ctx: &Context<Self>) -> Html {
        html! {
            <select name="sizes" id="sizes" onchange={ctx.link().callback(|e: Event| {
                let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                Msg::UpdateSize(select.value())
            })}>
            {{
                let sizes:Vec<String> = vec![Size::Small.to_string(), Size::Medium.to_string(), Size::Large.to_string()];
                sizes.iter().map(|size|{
                    if self.p.size.to_string() == *size {
                        html!{<option value={size.clone()} selected=true>{size}</option>}
                    }else {
                        html!{<option value={size.clone()}>{size}</option>}

                    }
                }).collect::<Html>()
            }}
            </select>
        }
    }

    fn render_arrows(&self) -> Html {
        (0..self.get_height() - self.p.step)
            .step_by(self.p.step)
            .skip(1)
            .map(|y| self.render_arrow_line(y))
            .collect::<Html>()
    }

    fn render_arrow_line(&self, y: usize) -> Html {
        (0..self.get_width() - self.p.step)
            .step_by(self.p.step)
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
        let squares = self.create_squares();
        squares
            .iter()
            .map(|line| {
                line.iter()
                    .map(|square| {
                        square.draw(
                            &squares,
                            Self::colors().get(&self.p.color_scheme).unwrap(),
                            self.p.variant,
                        )
                    })
                    .collect::<Html>()
            })
            .collect::<Html>()
    }

    fn create_squares(&self) -> Vec<Vec<WithClustersSquare>> {
        let first_pass: Vec<Vec<_>> = (0..self.get_height() - self.p.step)
            .step_by(self.p.step)
            .skip(1)
            .map(|y| {
                (0..self.get_width() - self.p.step)
                    .step_by(self.p.step)
                    .skip(1)
                    .map(|x| {
                        let link_right = (rand::thread_rng().gen_range(0..3) < 1)
                            && self.not_last(x, self.get_width());
                        let link_down = rand::thread_rng().gen_range(0..3) < 1
                            && self.not_last(y, self.get_height());
                        InitialSquare {
                            p: Point::from_usize(x, y),
                            link_right,
                            link_down,
                        }
                    })
                    .collect()
            })
            .collect();
        let second_pass: Vec<Vec<_>> = first_pass
            .iter()
            .enumerate()
            .map(|(i, line)| {
                line.iter()
                    .enumerate()
                    .map(|(j, square)| WithLinksSquare {
                        p: square.p,
                        link_up: i > 0 && first_pass[i - 1][j].link_down,
                        link_left: j > 0 && first_pass[i][j - 1].link_right,
                        link_right: square.link_right,
                        link_down: square.link_down,
                    })
                    .collect()
            })
            .collect();
        let mut clusters: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        second_pass
            .iter()
            .enumerate()
            .map(|(i, line)| {
                line.iter()
                    .enumerate()
                    .map(|(j, square)| {
                        let (cluster_id, cluster_size) =
                            Self::calculate_cluster(&mut clusters, i, j, &second_pass);
                        WithClustersSquare {
                            p: square.p,
                            link_up: i > 0 && first_pass[i - 1][j].link_down,
                            link_left: j > 0 && first_pass[i][j - 1].link_right,
                            link_right: square.link_right,
                            link_down: square.link_down,
                            cluster_id,
                            cluster_size,
                        }
                    })
                    .collect()
            })
            .collect()
    }

    fn calculate_cluster(
        clusters: &mut HashMap<(usize, usize), (usize, usize)>,
        i: usize,
        j: usize,
        first_pass: &[Vec<WithLinksSquare>],
    ) -> (usize, usize) {
        let v = clusters.get(&(i, j));
        match v {
            Some((cluster_id, cluster_size)) => (*cluster_id, *cluster_size),
            None => {
                let mut cluster: HashSet<(usize, usize)> = HashSet::new();
                Self::dfs_cluster(i, j, first_pass, &mut cluster);
                let cluster_size = cluster.len();
                let cluster_id = clusters.keys().map(|(id, _size)| id).max().unwrap_or(&0) + 1;
                for item in cluster {
                    clusters.insert(item, (cluster_id, cluster_size));
                }
                (cluster_id, cluster_size)
            }
        }
    }

    fn dfs_cluster(
        i: usize,
        j: usize,
        first_pass: &[Vec<WithLinksSquare>],
        result: &mut HashSet<(usize, usize)>,
    ) {
        if !result.insert((i, j)) {
            return;
        }
        if first_pass[i][j].link_right {
            Self::dfs_cluster(i, j + 1, first_pass, result);
        };
        if first_pass[i][j].link_down {
            Self::dfs_cluster(i + 1, j, first_pass, result);
        }
        if first_pass[i][j].link_up {
            Self::dfs_cluster(i - 1, j, first_pass, result);
        };
        if first_pass[i][j].link_left {
            Self::dfs_cluster(i, j - 1, first_pass, result);
        }
    }

    fn not_last(&self, dimension: usize, max_dimension: usize) -> bool {
        let last = (0..max_dimension - self.p.step)
            .step_by(self.p.step)
            .skip(1)
            .last()
            .unwrap();
        dimension != last
    }

    fn gen_random_point(&self, diameter: usize, circles: &[(Circle, &'static str)]) -> Point {
        let mut i = 0;
        loop {
            let x = rand::thread_rng().gen_range(0..self.get_width()) as f32;
            let y = rand::thread_rng().gen_range(0..self.get_height()) as f32;
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
            .into_iter()
            .map(|(circle, color)| {
                html! {
                    <circle
                        cx={circle.p.x.to_string()}
                        cy={circle.p.y.to_string()}
                        r={circle.r.to_string()}
                        fill={color.to_string()} />
                }
            })
            .collect()
    }

    fn render_paths(&self) -> Vec<Html> {
        let num_paths = (0.05 * ((self.get_width() * self.get_height()) as f32)) as usize;
        let mut all_points = Vec::new();
        let circles = self.circles();
        let borders = (self.get_width() + self.get_height()) * 2;
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
                if x < self.get_width() as u32 && y < self.get_height() as u32 {
                    let x = usize::try_from(x).unwrap();
                    let y = usize::try_from(y).unwrap();
                    all_points.push(UsizePoint { x, y });
                }
            }
            acc
        })
    }

    fn select_path_color(&self, item: &Path, circles: &[(Circle, &'static str)]) -> &'static str {
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
        let length = (self.get_width() + self.get_height()) / 200;
        let path = Path {
            items: vec![start_point],
        };
        let val = (0..length).fold((path, start_point), |(mut acc, last_point), _i| {
            let angle = self.angle_at(last_point);
            let next_point = Point {
                x: last_point.x + angle.cos() * self.p.step as f32,
                y: last_point.y + angle.sin() * self.p.step as f32,
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
        let factor = 1.0 / ((distance / self.get_width() as f32).powf(2.0) + 1.0);

        let cos = angle.cos();
        let sin = angle.sin();
        let bias_x = 0.0;
        let bias_y = 1.0;
        let new_x = (1.0 - factor) * cos + factor * bias_x;
        let new_y = (1.0 - factor) * sin + factor * bias_y;

        new_y.atan2(new_x)
    }

    fn zero_to_one_flow_field(&self, p: Point) -> f32 {
        let height = self.get_height() as f32;
        let width = self.get_width() as f32;
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
    yew::start_app::<Model>();
}
