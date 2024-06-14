use crossterm::{
    event::{self, read, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{enable_raw_mode, size, EnterAlternateScreen},
    ExecutableCommand,
};
use csv::Writer;
use std::{
    f64::consts::PI,
    io::{stdout, Write},
};

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

struct SingleParams {
    length: f64,
    gravity: f64,
    dt: f64,
    margin: i32,
}

impl Default for SingleParams {
    fn default() -> Self {
        SingleParams {
            length: 1.0,
            gravity: -9.81,
            dt: 0.01,
            margin: 5,
        }
    }
}
struct DoubleParams {
    length: (f64, f64),
    gravity: f64,
    dt: f64,
    mass: (f64, f64),
}

fn main() -> Result {
    let size = size().unwrap();
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    let mut i = 0;
    let mut center = (size.0 / 2, size.1 / 2);
    let mut pendulum = SinglePendulum {
        theta: 1.0,
        vel: 0.0,
    };
    loop {
        let event = read()?;
        match event {
            Event::Key(e) => match e.code {
                KeyCode::Esc => break,
                _ => println!("meow"),
            },
            Event::Resize(width, height) => {
                todo!()
            }
            _ => todo!(),
        }
        single_pendulum(&mut pendulum, SingleParams::default());
        i += 1
    }
    Ok(())
}

#[derive(Clone, Copy, Debug)]
struct SinglePendulum {
    theta: f64,
    vel: f64,
}
fn single_pendulum(pendulum: &mut SinglePendulum, params: SingleParams) {
    pendulum.theta += pendulum.vel * params.dt;
    pendulum.vel -= (params.gravity / params.length) * f64::sin(pendulum.theta) * params.dt;
}

struct DoublePendulum {
    theta1: f64,
    vel1: f64,
    theta2: f64,
    vel2: f64,
}

fn double_pendulum(pendulum: &mut DoublePendulum, params: DoubleParams) {}

fn draw_line((x1, y1): (i16, i16), (x2, y2): (i16, i16)) {
    todo!()
}
