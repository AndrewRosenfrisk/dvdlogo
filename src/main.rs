use crossterm::{
    cursor::{Hide, MoveTo},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{
        size, Clear,
        ClearType::{All, Purge},
        DisableLineWrap,
    },
};
use rand::{
    distributions::{Distribution, Standard},
    random,
    seq::SliceRandom,
    thread_rng, Rng,
};
use std::{
    io::{self, stdout, Write},
    thread::sleep,
    time::Duration,
};

#[derive(PartialEq, Debug)]
struct Position(u16, u16);

impl Position {
    fn new(x: u16, y: u16) -> Position {
        Position(x, y)
    }
    fn is_at_x_bound(&self, width: u16) -> bool {
        if self.0 == 1 || self.0 == width {
            true
        } else {
            false
        }
    }
    fn is_at_y_bound(&self, height: u16) -> bool {
        if self.1 == 1 || self.1 == height {
            true
        } else {
            false
        }
    }
}

struct Logo {
    pos: Position,
    value: String,
    direction: Direction,
    color: Color,
}

impl Logo {
    fn new(x: u16, y: u16, value: String, direction: Direction, color: Color) -> Logo {
        Logo {
            pos: Position::new(x, y),
            value,
            direction,
            color,
        }
    }
}
#[derive(PartialEq, Debug, Clone, Copy)]
enum Direction {
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

impl Direction {
    fn flip_x(&mut self) -> &mut Self {
        match self {
            Direction::UpRight => {
                *self = Direction::UpLeft;
            }
            Direction::DownRight => {
                *self = Direction::DownLeft;
            }
            Direction::UpLeft => {
                *self = Direction::UpRight;
            }
            Direction::DownLeft => {
                *self = Direction::DownRight;
            }
        }
        self
    }
    fn flip_y(&mut self) -> &mut Self {
        match self {
            Direction::UpRight => {
                *self = Direction::DownRight;
            }
            Direction::DownRight => {
                *self = Direction::UpRight;
            }
            Direction::UpLeft => {
                *self = Direction::DownLeft;
            }
            Direction::DownLeft => {
                *self = Direction::UpLeft;
            }
        }
        self
    }
    fn is_right(&self) -> bool {
        match self {
            Direction::UpRight | Direction::DownRight => true,
            _ => false,
        }
    }
    fn is_left(&self) -> bool {
        match self {
            Direction::UpLeft | Direction::DownLeft => true,
            _ => false,
        }
    }
    fn is_down(&self) -> bool {
        match self {
            Direction::DownRight | Direction::DownLeft => true,
            _ => false,
        }
    }
    fn is_up(&self) -> bool {
        match self {
            Direction::UpLeft | Direction::UpRight => true,
            _ => false,
        }
    }
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..=4) {
            0 => Direction::DownLeft,
            1 => Direction::DownRight,
            2 => Direction::UpRight,
            _ => Direction::UpLeft,
        }
    }
}

fn main() -> std::io::Result<()> {
    execute!(stdout(), Hide, Clear(Purge), Clear(All), DisableLineWrap)?;

    let window_size = size()?;
    let _width: u16 = window_size.0 - 3;
    let _height: u16 = window_size.1 - 1;
    let colors = vec![
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::White,
    ];
    let mut rng = thread_rng();

    let mut logos: Vec<Logo> = vec![];
    for i in 0..=4 {
        logos.push(Logo::new(
            rng.gen_range(1..=_width),
            rng.gen_range(1..=_height),
            "DVD".to_string(),
            random::<Direction>(),
            *colors.choose(&mut rng).unwrap(),
        ));

        if logos[i].pos.0 % 2 == 1 && logos[i].pos.0 > 1 {
            logos[i].pos.0 -= 1;
        }
    }

    let mut corner_count: u32 = 0;

    loop {
        for logo in &mut logos {
            execute!(
                stdout(),
                MoveTo(logo.pos.0, logo.pos.1),
                Print(String::from("   ")),
                SetForegroundColor(logo.color)
            )?;

            if logo.pos.is_at_x_bound(_width) && logo.pos.is_at_y_bound(_height) {
                logo.direction.flip_x().flip_y();
                logo.color = colors[rng.gen_range(0..=6)];
                corner_count += 1;
            } else if (logo.pos.0 == _width && logo.direction.is_right())
                || (logo.pos.0 == 1 && logo.direction.is_left())
            {
                logo.direction.flip_x();
                logo.color = colors[rng.gen_range(0..=6)];
            } else if (logo.pos.1 == 1 && logo.direction.is_up())
                || (logo.pos.1 == _height && logo.direction.is_down())
            {
                logo.direction.flip_y();
                logo.color = colors[rng.gen_range(0..=6)];
            }

            match logo.direction {
                Direction::UpRight => {
                    logo.pos.0 += 1;
                    logo.pos.1 -= 1
                }
                Direction::UpLeft => {
                    logo.pos.0 -= 1;
                    logo.pos.1 -= 1
                }
                Direction::DownRight => {
                    logo.pos.0 += 1;
                    logo.pos.1 += 1
                }
                Direction::DownLeft => {
                    logo.pos.0 -= 1;
                    logo.pos.1 += 1
                }
            }
            //print this loop
            execute!(stdout(), MoveTo(logo.pos.0, logo.pos.1))?;
            print!("{}", logo.value);
            io::stdout().flush()?;

            execute!(stdout(), MoveTo(5, 0), SetForegroundColor(Color::White))?;
            print!("Corner count: {}", corner_count);
            io::stdout().flush()?;

            sleep(Duration::from_secs_f32(0.01));
        }
    }
}
