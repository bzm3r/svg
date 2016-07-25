extern crate svg;

use svg::Tag;
use svg::parser::Event;
use svg::tag::path::{Command, Data};

fn main() {
    let path = "tests/fixtures/benton.svg";
    for event in svg::open(path).unwrap() {
        match event {
            Event::Tag(Tag::Path(_, attributes)) => {
                let data = attributes.get("d").unwrap();
                let data = Data::parse(data).unwrap();
                draw(data);
            },
            _ => {
                println!("Not sure how to react.");
            },
        }
    }
}

fn draw(data: Data) {
    for command in data.iter() {
        match *command {
            Command::MoveTo(_, ref parameters) => {
                println!("Move to {:?}.", parameters);
            },
            Command::LineTo(_, ref parameters) => {
                println!("Line to {:?}.", parameters);
            },
            Command::CurveTo(_, ref parameters) => {
                println!("Curve to {:?}.", parameters);
            },
            Command::SmoothCurveTo(_, ref parameters) => {
                println!("Smooth curve to {:?}.", parameters);
            },
            Command::ClosePath => {
                println!("Close the path.");
            },
            _ => {
                println!("Not sure what to do.");
            }
        }
    }
}
