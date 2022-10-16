#![feature(mixed_integer_ops)]

use std::{
    io::{self, Write},
    time::Duration,
};

use dvd_logo::random_color;

use crossterm::{
    cursor,
    event::{self, poll, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, ClearType, EnterAlternateScreen},
    Result,
};
use rand::Rng;

const ASCII: &str = r#"
   .%%%%%%%%%%%%%%%:   =%%%%%%%%%%%#*=. 
   .------=#%%%%%%%+  +%%#-------=+%%%%:
   *%%%:   .%%%%+%%%.*%%#.:%%%*    +%%%=
   %%%%. .-#%%%- %%%%%%*  =%%%= .:+%%%# 
  :%%%%%%%%%#+.  -%%%%*   *%%%%%%%%%*-  
  .:::::::.       #%%+    :::::::..     
                  :%=                   
                   :                    
   .:--==++****#########****++==-:.     
:*%%%%%%%%%%%%#=:::.::-+%%%%%%%%%%%%#+  
 :=+*#%%%%%%%%%#*++++**%%%%%%%%%%#*+=.  
"#;

const LINE_LENGTH: u16 = 40;
const LINES_COUNT: u16 = 11;
const MIN_PADDING: u16 = 5;

const TICK_TIME: Duration = Duration::from_millis(32);

fn main() -> Result<()> {
    let mut color = random_color();
    let mut column_speed: i16 = 1;
    let mut row_speed: i16 = 1;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    let mut rng = rand::thread_rng();

    let mut size = terminal::size()?;
    let mut position = (
        rng.gen_range(1..size.0 - LINE_LENGTH),
        rng.gen_range(1..size.1 - LINES_COUNT),
    );

    loop {
        queue!(
            stdout,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(position.0, position.1),
        )?;

        if let Some('q') = read_char()? {
            break;
        };

        size = terminal::size()?;

        if size.0 <= LINE_LENGTH + MIN_PADDING || size.1 <= LINES_COUNT + MIN_PADDING {
            queue!(
                stdout,
                cursor::MoveTo(1, 1),
                style::SetForegroundColor(style::Color::Red),
                style::Print("Terminal window is too small!")
            )?;
            stdout.flush()?;
            continue;
        }

        for line in ASCII.split('\n') {
            queue!(
                stdout,
                style::SetForegroundColor(color),
                style::Print(line),
                cursor::MoveDown(1),
                cursor::MoveLeft(line.len() as u16),
            )?;
        }

        if position.0 + LINES_COUNT > size.0 {
            position.0 = size.0 - LINE_LENGTH;
        }

        if position.1 + LINES_COUNT > size.1 {
            position.1 = size.1 - LINES_COUNT;
        }

        if position.0 + LINE_LENGTH == size.0 || position.0 == 1 {
            column_speed = -column_speed;
            color = random_color();
        }
        if position.1 + LINES_COUNT == size.1 || position.1 == 1 {
            row_speed = -row_speed;
            color = random_color();
        }

        position = (
            position.0.saturating_add_signed(column_speed),
            position.1.saturating_add_signed(row_speed),
        );

        queue!(
            stdout,
            cursor::MoveTo(1, 1),
            style::Print(format!("pos: {:?}", position)),
            cursor::MoveToNextLine(1),
            style::Print(format!("speed: {:?}", (column_speed, row_speed))),
            cursor::MoveToNextLine(1),
            style::Print(format!("size: {:?}", size)),
        )?;

        stdout.flush()?;
    }

    execute!(
        stdout,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;

    terminal::disable_raw_mode()
}

pub fn read_char() -> Result<Option<char>> {
    if poll(TICK_TIME)? {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            ..
        })) = event::read()
        {
            return Ok(Some(c));
        }
    }
    Ok(None)
}
