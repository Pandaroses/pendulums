use crossterm::{
    cursor::MoveTo,
    event::{poll, read, Event, KeyCode},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{enable_raw_mode, size, Clear, EnterAlternateScreen},
};
use std::{io::stdout, ops::Mul, time::Duration};

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
struct Params {
    lengths: Vec<f64>,
    // mass will be from 0-10 when adjusting, in factors of 1, fuck knows why its a f64
    masses: Vec<f64>,
    gravity: f64,
    dt: f64,
    margin: i16,
    n: usize,
    sel: usize,
    paused: bool,
}

#[derive(Debug)]
struct Pendulum {
    thetas: Vec<f64>,
    vels: Vec<f64>,
}
fn main() -> Result {
    let size: (u16, u16) = size().unwrap();
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    let mut _i = 0;
    let mut center = (size.0 / 2, size.1 / 2);
    // TODO meow
    let mut pendulum = Pendulum {
        thetas: vec![1.0; 1],
        vels: vec![0.0; 1],
    };
    let params = Params {
        lengths: vec![5.0; 1],
        masses: vec![1.0; 1],
        gravity: -9.81,
        dt: 0.01,
        margin: 15,
        n: 1,
        sel: 0,
        paused: false,
    };
    loop {
        if (poll(Duration::from_millis((1000.0 * params.dt).round() as u64)))? {
            let event = read()?;
            match event {
                //TODO keybinds for:
                // add pendulum
                // remove pendulum
                // select mass, selected mass should be red, after selected can change mass, angle
                // play/pause: can be implemented with simple if statement
                Event::Key(e) => {
                    if let KeyCode::Esc = e.code {
                        break;
                    }
                }
                Event::Resize(width, height) => {
                    center = (width / 2, height / 2);
                    continue;
                }
                _ => continue,
            }
        }

        //todo instead of single pedulum calculations inside of main, instead make generalized n-ulum function & draw-pendulum function
        // they need to be INDEPENDENT, because if its paused, calculation shouldn't run, but pendulum should still be modifiable
        single_pendulum(&mut pendulum, &params);
        draw_pendulum(&pendulum, &params, center);
        execute!(stdout(), Clear(crossterm::terminal::ClearType::All))?;
        _i += 1
    }
    Ok(())
}

//replace with n_pendulum function
fn single_pendulum(pendulum: &mut Pendulum, params: &Params) {
    pendulum.thetas[0] += pendulum.vels[0] * params.dt;
    pendulum.vels[0] -=
        (params.gravity / params.lengths[0]) * f64::sin(pendulum.thetas[0]) * params.dt;
}

fn calc_coords(l: &Vec<f64>, theta: &Vec<f64>, n: usize) -> (f64, f64) {
    let mut x = 0.0;
    let mut y = 0.0;
    for i in 0..n {
        x += l[i] * f64::sin(theta[i]);
        y -= l[i] * f64::cos(theta[i])
    }
    (x, y)
}

//TODO draw_pendulum
// passes in a pendulum, with an optional selected N, and a length, draws lines between origin and pendulums until it meows
fn draw_pendulum(pendulum: &Pendulum, params: &Params, (mut px, mut py): (u16, u16)) {
    for i in 0..params.n {
        let (ix, iy) = calc_coords(&params.lengths, &pendulum.thetas, i + 1);
        let (x, y) = rescaled_coords(
            ix,
            iy,
            params.lengths.iter().sum(),
            get_dimensions(params.margin).unwrap(),
        );
        let color = if i == params.sel {
            Color::Red
        } else {
            Color::Blue
        };
        // add back mass size
        draw_circle((x, y), 2 as u16, color);
        draw_line((px as i16, py as i16), (x as i16, y as i16), Color::White);
        (px, py) = (x, y);
    }
}

///  this function draws a circle, based on the midpoint circle algorithm, not optimal, and has a limitation of the circle having a radius lower than 10 for unforeseen reason
fn draw_circle((x, y): (u16, u16), r: u16, color: Color) {
    let mut sx: i16 = 0;
    let mut sy = r as i16;
    let mut p = 3 - 2 * r as i16;
    while sx <= sy {
        plot_point(sx + x as i16, sy + y as i16, color);
        plot_point(-sx + x as i16, sy + y as i16, color);
        plot_point(sx + x as i16, -sy + y as i16, color);
        plot_point(-sx + x as i16, -sy + y as i16, color);
        plot_point(sy + x as i16, sx + y as i16, color);
        plot_point(-sy + x as i16, sx + y as i16, color);
        plot_point(sy + x as i16, -sx + y as i16, color);
        plot_point(-sy + x as i16, -sx + y as i16, color);
        sx += 1;
        if p > 0 {
            sy -= 1;
            p += 4 * (sx - sy) + 10;
        } else {
            p += 4 * sx + 6;
        }
    }
}

/// wrapper function for crossterm execute, as execute is not very pretty
fn plot_point(x: i16, y: i16, color: Color) {
    execute!(
        stdout(),
        MoveTo(x as u16, y as u16),
        SetForegroundColor(color),
        Print("█")
    )
    .unwrap();
}

/// based on bresenhams line algorithm,draws a line between two points
fn draw_line((mut x1, mut y1): (i16, i16), (x2, y2): (i16, i16), color: Color) {
    let dx = (x2 - x1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let dy = -(y2 - y1).abs();
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut error = dx + dy;
    // mwa ha ha ha while true loop being used in an algorithm!!!!
    loop {
        plot_point(x1, y1, color);
        if x1 == x2 && y1 == y2 {
            break;
        }
        let e2 = 2 * error;
        if e2 >= dy {
            if x1 == x2 {
                break;
            };
            error += dy;
            x1 += sx;
        }
        if e2 <= dx {
            if y1 == y2 {
                break;
            };
            error += dx;
            y1 += sy;
        }
    }
}

/// returns the dimension
fn get_dimensions(margin: i16) -> Result<i16> {
    let (x, y) = size()?;
    let dimensions = if x * 2 < y {
        (x as i16) * 2 - margin.mul(2)
    } else {
        y as i16 - margin.mul(2)
    };
    Ok(dimensions)
}

fn rescaled_coords(x: f64, y: f64, starting_max: f64, output_max: i16) -> (u16, u16) {
    let (m, e) = size().unwrap();
    let o = (((x / starting_max) * output_max as f64).round() as i16 + (m / 2) as i16) as u16;
    let w = (((y / starting_max) * output_max as f64).round() as i16 + (e / 2) as i16) as u16;
    (o, w)
}
