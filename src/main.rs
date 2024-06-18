use crossterm::{
    cursor::MoveTo,
    event::{self, poll, read, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    style::{
        Color, Print, PrintStyledContent, ResetColor, SetBackgroundColor, SetForegroundColor,
        Stylize,
    },
    terminal::{enable_raw_mode, size, Clear, EnterAlternateScreen},
    ExecutableCommand,
};
use csv::Writer;
use std::{
    f64::consts::PI,
    io::{stdout, Stdout, Write},
    ops::Div,
    time::Duration,
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
        if (poll(Duration::from_millis(100)))? {
            let event = read()?;
            match event {
                Event::Key(e) => {
                    if let KeyCode::Esc = e.code {
                        break;
                    }
                }
                Event::Resize(width, height) => {
                    todo!()
                }
                _ => continue,
            }
        }
        single_pendulum(&mut pendulum, SingleParams::default());
        let (x, y) = calc_coords(vec![1.0], vec![pendulum.theta]);
        let (new_x, new_y) = rescaled_coords(x, y, 2.0, get_dimensions(5)?);
        execute!(
            stdout(),
            Clear(crossterm::terminal::ClearType::All),
            MoveTo(new_x as u16, new_y as u16),
            PrintStyledContent("█".magenta())
        )?;
        draw_line((center.0, center.1), (new_x, new_y));
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

//TODO, this needs to pass in an array, and do some funky junky maths
fn calc_coords(l: Vec<f64>, theta: Vec<f64>) -> (f64, f64) {
    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    for i in 0..l.len() {
        sum_x += l[i] * f64::sin(theta[i]);
        sum_y -= l[i] * f64::cos(theta[i])
    }
    (sum_x, sum_y)
}

// struct DoublePendulum {
//     theta1: f64,
//     vel1: f64,
//     theta2: f64,
//     vel2: f64,
// }

// fn double_pendulum(pendulum: &mut DoublePendulum, params: DoubleParams) {}

fn draw_line((x1, y1): (u16, u16), (x2, y2): (u16, u16)) {
    if u16::abs_diff(y1, y2) < u16::abs_diff(x1, x2) {
        if x1 > x2 {
            draw_line_low((x2, y2), (x1, y1));
        } else {
            draw_line_low((x1, y1), (x2, y2));
        }
    } else {
        if y1 > y2 {
            draw_line_high((x2, y2), (x1, y1));
        } else {
            draw_line_high((x1, y1), (x2, y2));
        }
    }
}

/// line drawing algorithm taken from wikipedia
fn draw_line_high((x1, y1): (u16, u16), (x2, y2): (u16, u16)) {
    let mut dx = x2 as i16 - x1 as i16;
    let dy = y2 as i16 - y1 as i16;
    let mut xi = 1;
    if dx < 0 {
        xi = -1;
        dx = -dx;
    }
    let mut d = 2 * dx - dy;
    let mut x = x1 as i16;
    for y in y1..y2 {
        execute!(
            stdout(),
            MoveTo(x as u16, y),
            PrintStyledContent("█".magenta())
        )
        .unwrap();
        if d > 0 {
            x += xi;
            d += 2 * (dx - dy);
        } else {
            d += 2 * dx;
        }
    }
}

fn draw_line_low((x1, y1): (u16, u16), (x2, y2): (u16, u16)) {
    let dx = x2 as i16 - x1 as i16;
    let mut dy = y2 as i16 - y2 as i16;
    let mut yi = 1;
    if dy < 0 {
        yi = -1;
        dy = -dy;
    }
    let mut d = (2 * dy) - dx;
    let mut y = y1 as i16;
    for x in x1..x2 {
        execute!(
            stdout(),
            MoveTo(x, y as u16),
            PrintStyledContent("█".magenta())
        )
        .unwrap();
        if d > 0 {
            y += yi;
            d += (2 * (dy - dx));
        } else {
            d += 2 * dy;
        }
    }
}

fn get_dimensions(margin: i16) -> Result<i16> {
    let (x, y) = size()?;
    let dimensions = if x < y {
        x as i16 - margin.div(2)
    } else {
        y as i16 - margin.div(2)
    };
    Ok(dimensions)
}

// rescales the coordinates to the console dimensions, & returns the sanitized coordinates to be drawn in the console
fn rescaled_coords(x: f64, y: f64, starting_max: f64, output_max: i16) -> (u16, u16) {
    let (m, e) = size().unwrap();
    let x_scaled =
        (((x / starting_max) * output_max as f64).round() as i16 + (m / 2) as i16) as u16;
    let y_scaled =
        (((y / starting_max) * output_max as f64).round() as i16 + (e / 2) as i16) as u16;
    (x_scaled, y_scaled)
}
