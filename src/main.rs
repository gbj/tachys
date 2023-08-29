#![feature(impl_trait_in_assoc_type)]

use std::fmt::Debug;
mod component;
mod dom;
mod view;

use component::{Component, ComponentLink};
use dom::{Attr, Dom, El};
use rand::prelude::*;
use std::cmp::min;
use view::{
    html::{
        attribute::{on, On},
        Html,
    },
    keyed::Keyed,
    Static, View,
};

#[derive(Debug)]
struct App {
    rows: Vec<RowData>,
    next_id: usize,
    selected_id: Option<usize>,
    rng: SmallRng,
}

#[derive(Debug, PartialEq)]
struct RowData {
    id: usize,
    label: String,
}

impl RowData {
    fn new(id: usize, rng: &mut SmallRng) -> Self {
        let adjective = *ADJECTIVES.choose(rng).unwrap();
        let colour = *COLOURS.choose(rng).unwrap();
        let noun = *NOUNS.choose(rng).unwrap();

        let label = [adjective, colour, noun].join(" ");

        Self { id, label }
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            next_id: 1,
            selected_id: None,
            rng: SmallRng::from_entropy(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Msg {
    Run(usize),
    Add(usize),
    Update(usize),
    Clear,
    Swap,
    Remove(usize),
    Select(usize),
}

impl Component for App {
    type Msg = Msg;
    type View = impl View;

    fn update(&mut self, msg: Self::Msg) {
        match msg {
            Msg::Run(amount) => {
                let rng = &mut self.rng;
                let next_id = self.next_id;
                let update_amount = min(amount, self.rows.len());
                for index in 0..update_amount {
                    self.rows[index] = RowData::new(next_id + index, rng);
                }
                self.rows.extend(
                    (update_amount..amount).map(|index| RowData::new(next_id + index, rng)),
                );
                self.next_id += amount;
            }
            Msg::Add(amount) => {
                let rng = &mut self.rng;
                let next_id = self.next_id;
                self.rows
                    .extend((0..amount).map(|index| RowData::new(next_id + index, rng)));
                self.next_id += amount;
            }
            Msg::Update(step) => {
                for index in (0..self.rows.len()).step_by(step) {
                    self.rows[index].label += " !!!";
                }
            }
            Msg::Clear => {
                self.rows.clear();
            }
            Msg::Swap => {
                if self.rows.len() > 998 {
                    self.rows.swap(1, 998);
                }
            }
            Msg::Remove(id) => {
                if let Some(index) = self.rows.iter().position(|row| row.id == id) {
                    self.rows.remove(index);
                }
            }
            Msg::Select(id) => {
                self.selected_id = Some(id);
            }
        }
    }

    fn view(&self, link: &ComponentLink<Self>) -> Self::View {
        Html {
            tag: El::div,
            attributes: Static((Attr::class, "container")),
            children: (
                Static(Html {
                    tag: El::div,
                    attributes: (Attr::class, "row"),
                    children: (
                        Html {
                            tag: El::div,
                            attributes: (Attr::class, "col-md-6"),
                            children: Html {
                                tag: El::h1,
                                attributes: (),
                                children: "Tachys",
                            },
                        },
                        Html {
                            tag: El::div,
                            attributes: (Attr::class, "col-md-6"),
                            children: (
                                Html {
                                    tag: El::div,
                                    attributes: (Attr::class, "col-sm-6 smallpad"),
                                    children: Html {
                                        tag: El::button,
                                        attributes: (
                                            (Attr::r#type, "button"),
                                            (Attr::id, "run"),
                                            (Attr::class, "btn btn-primary btn-block"),
                                            on("click", {
                                                let link = link.clone();
                                                move |_| link.send(Msg::Run(1000))
                                            }),
                                        ),
                                        children: "Create 1,000 rows",
                                    },
                                },
                                Html {
                                    tag: El::div,
                                    attributes: (Attr::class, "col-sm-6 smallpad"),
                                    children: Html {
                                        tag: El::button,
                                        attributes: (
                                            (Attr::r#type, "button"),
                                            (Attr::id, "runlots"),
                                            (Attr::class, "btn btn-primary btn-block"),
                                            on("click", {
                                                let link = link.clone();
                                                move |_| link.send(Msg::Run(10_000))
                                            }),
                                        ),
                                        children: "Create 10,000 rows",
                                    },
                                },
                                Html {
                                    tag: El::div,
                                    attributes: (Attr::class, "col-sm-6 smallpad"),
                                    children: Html {
                                        tag: El::button,
                                        attributes: (
                                            (Attr::r#type, "button"),
                                            (Attr::id, "add"),
                                            (Attr::class, "btn btn-primary btn-block"),
                                            on("click", {
                                                let link = link.clone();
                                                move |_| link.send(Msg::Add(1000))
                                            }),
                                        ),
                                        children: "Create 1,000 rows",
                                    },
                                },
                                Html {
                                    tag: El::div,
                                    attributes: (Attr::class, "col-sm-6 smallpad"),
                                    children: Html {
                                        tag: El::button,
                                        attributes: (
                                            (Attr::r#type, "button"),
                                            (Attr::id, "update"),
                                            (Attr::class, "btn btn-primary btn-block"),
                                            on("click", {
                                                let link = link.clone();
                                                move |_| link.send(Msg::Update(10))
                                            }),
                                        ),
                                        children: "Update every 10th row",
                                    },
                                },
                                Html {
                                    tag: El::div,
                                    attributes: (Attr::class, "col-sm-6 smallpad"),
                                    children: Html {
                                        tag: El::button,
                                        attributes: (
                                            (Attr::r#type, "button"),
                                            (Attr::id, "clear"),
                                            (Attr::class, "btn btn-primary btn-block"),
                                            on("click", {
                                                let link = link.clone();
                                                move |_| link.send(Msg::Clear)
                                            }),
                                        ),
                                        children: "Clear",
                                    },
                                },
                                Html {
                                    tag: El::div,
                                    attributes: (Attr::class, "col-sm-6 smallpad"),
                                    children: Html {
                                        tag: El::button,
                                        attributes: (
                                            (Attr::r#type, "button"),
                                            (Attr::id, "swap"),
                                            (Attr::class, "btn btn-primary btn-block"),
                                            on("click", {
                                                let link = link.clone();
                                                move |_| link.send(Msg::Swap)
                                            }),
                                        ),
                                        children: "Swap Rows",
                                    },
                                },
                            ),
                        },
                    ),
                }),
                Html {
                    tag: El::table,
                    attributes: (Attr::class, "table table-hover table-striped test-data"),
                    children: Html {
                        tag: El::tbody,
                        attributes: (Attr::id, "tbody"),
                        children: self
                            .rows
                            .iter()
                            .map(|row| {
                                Keyed(
                                    row.id,
                                    Html {
                                        tag: El::tr,
                                        attributes: (
                                            Attr::class,
                                            if self.selected_id == Some(row.id) {
                                                "danger"
                                            } else {
                                                ""
                                            },
                                        ),
                                        children: (
                                            Html {
                                                tag: El::td,
                                                attributes: (Attr::class, "col-md-1"),
                                                children: row.id.to_string(),
                                            },
                                            Html {
                                                tag: El::td,
                                                attributes: (
                                                    (Attr::class, "col-md-4"),
                                                    on("click", {
                                                        let link = link.clone();
                                                        let id = row.id;
                                                        move |_| link.send(Msg::Select(id))
                                                    }),
                                                ),
                                                children: Html {
                                                    tag: El::a,
                                                    attributes: (Attr::class, "lbl"),
                                                    children: row.label.clone(),
                                                },
                                            },
                                            Html {
                                                tag: El::td,
                                                attributes: (Attr::class, "col-md-1"),
                                                children: Html {
                                                    tag: El::a,
                                                    attributes: (
                                                        (Attr::class, "remove"),
                                                        on("click", {
                                                            let link = link.clone();
                                                            let id = row.id;
                                                            move |_| link.send(Msg::Remove(id))
                                                        }),
                                                    ),
                                                    children: Html {
                                                        tag: El::span,
                                                        attributes: (
                                                            (
                                                                Attr::class,
                                                                "glyphicon glyphicon-remove remove",
                                                            ),
                                                            (Attr::aria_hidden, "true"),
                                                        ),
                                                        children: (),
                                                    },
                                                },
                                            },
                                            Html {
                                                tag: El::td,
                                                attributes: (Attr::class, "col-md-6"),
                                                children: (),
                                            },
                                        ),
                                    },
                                )
                            })
                            .collect::<Vec<_>>(),
                    },
                },
                Static(Html {
                    tag: El::span,
                    attributes: (
                        (Attr::class, "preloadicon glyphicon glyphicon-remove"),
                        (Attr::aria_hidden, "true"),
                    ),
                    children: (),
                }),
            ),
        }
    }
}

fn main() {
    let counter = App::new();
    let mut view = counter.build();
    App::mount(&mut view, Dom::body());
    Dom::flush();
}

static ADJECTIVES: &[&str] = &[
    "pretty",
    "large",
    "big",
    "small",
    "tall",
    "short",
    "long",
    "handsome",
    "plain",
    "quaint",
    "clean",
    "elegant",
    "easy",
    "angry",
    "crazy",
    "helpful",
    "mushy",
    "odd",
    "unsightly",
    "adorable",
    "important",
    "inexpensive",
    "cheap",
    "expensive",
    "fancy",
];

static COLOURS: &[&str] = &[
    "red", "yellow", "blue", "green", "pink", "brown", "purple", "brown", "white", "black",
    "orange",
];

static NOUNS: &[&str] = &[
    "table", "chair", "house", "bbq", "desk", "car", "pony", "cookie", "sandwich", "burger",
    "pizza", "mouse", "keyboard",
];
