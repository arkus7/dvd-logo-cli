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

const TICK_TIME: Duration = Duration::from_millis(50);

fn main() -> Result<()> {
    let color = random_color();
    let mut column_speed: i16 = 1;
    let mut row_speed: i16 = 1;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;


    let mut rng = rand::thread_rng();

    let mut size = terminal::size()?;
    let mut position = (rng.gen_range(1..size.0 - LINE_LENGTH), rng.gen_range(1..size.1 - LINES_COUNT));

    loop {
        size = terminal::size()?;
        queue!(
            stdout,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(position.0, position.1),
        )?;
        for line in ASCII.split('\n') {
            queue!(
                stdout,
                style::SetForegroundColor(color),
                style::Print(line),
                cursor::MoveDown(1),
                cursor::MoveLeft(line.len() as u16),
            )?;
        }

        if position.0 + LINE_LENGTH >= size.0 || position.0 == 0 {
            column_speed = -column_speed;
        }
        if position.1 + LINES_COUNT >= size.1 || position.1 == 0 {
            row_speed = -row_speed;
        }

        position = (
            position.0.saturating_add_signed(column_speed),
            position.1.saturating_add_signed(row_speed),
        );

        stdout.flush()?;

        if let Some('q') = read_char()? {
            break;
        };
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
