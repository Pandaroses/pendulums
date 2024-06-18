use crossterm::{
    cursor::MoveTo,
    event::{poll, read, Event, KeyCode},
    execute,
    style::{PrintStyledContent, Stylize},
    terminal::{enable_raw_mode, size, Clear, EnterAlternateScreen},
};
use std::{io::stdout, ops::Div, time::Duration};

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

struct Params {
    lengths: Vec<f64>,
    masses: Vec<f64>,
    gravity: f64,
    dt: f64,
    margin: i16,
    n: usize,
    s: usize,
    paused: bool,
}

struct Pendulum {
    thetas: Vec<f64>,
    vels: Vec<f64>,
}
fn main() -> Result {
    let mut size: (u16, u16) = size().unwrap();
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    let mut _i = 0;
    let mut center = (size.0 / 2, size.1 / 2);
    let mut pendulum = Pendulum {
        thetas: vec![1.0; 1],
        vels: vec![1000.0; 1],
    };
    let params = Params {
        lengths: vec![1.0; 10],
        masses: vec![1.0; 10],
        gravity: -9.81,
        dt: 0.001,
        margin: 30,
        n: 1,
        s: 0,
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
        let (x, y) = calc_coords(vec![params.lengths[0]], vec![pendulum.thetas[0]], params.n);
        let (new_x, new_y) = rescaled_coords(x, y, 2.0, get_dimensions(5)?);
        execute!(
            stdout(),
            Clear(crossterm::terminal::ClearType::All),
            MoveTo(new_x, new_y),
            PrintStyledContent("█".magenta())
        )?;
        draw_line(
            (center.0 as i16, center.1 as i16),
            (new_x as i16, new_y as i16),
        );
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

fn calc_coords(l: Vec<f64>, theta: Vec<f64>, n: usize) -> (f64, f64) {
    let mut x = 0.0;
    let mut y = 0.0;
    for i in 0..n {
        x += l[i] * f64::sin(theta[i]);
        y -= l[i] * f64::cos(theta[i])
    }
    (x, y)
}

//TODO draw_circle

//TODO draw_pendulum
// passes in a pendulum, with an optional selected N, and a length, draws lines between origin and pendulums until it meows

fn draw_line((mut x1, mut y1): (i16, i16), (x2, y2): (i16, i16)) {
    let dx = (x2 - x1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let dy = -(y2 - y1).abs();
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut error = dx + dy;
    // mwa ha ha ha while true loop being used in an algorithm!!!!
    loop {
        execute!(
            stdout(),
            MoveTo(x1 as u16, y1 as u16),
            PrintStyledContent("█".magenta())
        )
        .unwrap();
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

fn get_dimensions(margin: i16) -> Result<i16> {
    let (x, y) = size()?;
    let dimensions = if x < y {
        x as i16 - margin.div(2)
    } else {
        y as i16 - margin.div(2)
    };
    Ok(dimensions)
}

fn rescaled_coords(x: f64, y: f64, starting_max: f64, output_max: i16) -> (u16, u16) {
    let (m, e) = size().unwrap();
    let o = (((x / starting_max) * output_max as f64).round() as i16 + (m / 2) as i16) as u16;
    let w = (((y / starting_max) * output_max as f64).round() as i16 + (e / 2) as i16) as u16;
    (o, w)
}
