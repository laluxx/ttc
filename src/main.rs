use std::io::{stdout, Write};
use crossterm::{
    execute,
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};

struct Torrent {
    name: String,
    size: String,
    progress: String,
    status: String,
    seeds: String,
}

// TODO take flag for all the color they will overwrite the config
// TODO config for the 2 colors, torrent dir etc..
// TODO invert text color for the entry when selected

// TODO enter on a torrent should expand it bringing it to the top
//  just after the headerline and clearing all the other space to use it
//  to display informations

fn get_torrents() -> Vec<Torrent> {
    vec![
        Torrent {
            name: "example1.torrent".to_string(),
            size: "1.2 GB".to_string(),
            progress: "50%".to_string(),
            status: "Downloading".to_string(),
            seeds: "10".to_string(),
        },
        Torrent {
            name: "example2.torrent".to_string(),
            size: "700 MB".to_string(),
            progress: "75%".to_string(),
            status: "Downloading".to_string(),
            seeds: "5".to_string(),
        },
        Torrent {
            name: "example3.torrent".to_string(),
            size: "1.5 GB".to_string(),
            progress: "20%".to_string(),
            status: "Paused".to_string(),
            seeds: "8".to_string(),
        },
        Torrent {
            name: "example4.torrent".to_string(),
            size: "500 MB".to_string(),
            progress: "90%".to_string(),
            status: "Seeding".to_string(),
            seeds: "12".to_string(),
        },
        Torrent {
            name: "example5.torrent".to_string(),
            size: "2.0 GB".to_string(),
            progress: "60%".to_string(),
            status: "Downloading".to_string(),
            seeds: "15".to_string(),
        },
        Torrent {
            name: "example6.torrent".to_string(),
            size: "1.8 GB".to_string(),
            progress: "30%".to_string(),
            status: "Downloading".to_string(),
            seeds: "20".to_string(),
        },
	Torrent {
            name: "example7.torrent".to_string(),
            size: "1.8 GB".to_string(),
            progress: "30%".to_string(),
            status: "Downloading".to_string(),
            seeds: "20".to_string(),
        },
	Torrent {
            name: "example8.torrent".to_string(),
            size: "1.8 GB".to_string(),
            progress: "30%".to_string(),
            status: "Downloading".to_string(),
            seeds: "20".to_string(),
        },
	Torrent {
            name: "example9.torrent".to_string(),
            size: "1.8 GB".to_string(),
            progress: "30%".to_string(),
            status: "Downloading".to_string(),
            seeds: "20".to_string(),
        },
	Torrent {
            name: "example10.torrent".to_string(),
            size: "1.8 GB".to_string(),
            progress: "30%".to_string(),
            status: "Downloading".to_string(),
            seeds: "20".to_string(),
        },
	Torrent {
            name: "example11.torrent".to_string(),
            size: "1.8 GB".to_string(),
            progress: "30%".to_string(),
            status: "Downloading".to_string(),
            seeds: "20".to_string(),
        },
	Torrent {
            name: "example12.torrent".to_string(),
            size: "1.8 GB".to_string(),
            progress: "30%".to_string(),
            status: "Downloading".to_string(),
            seeds: "20".to_string(),
        },
	Torrent {
            name: "example13.torrent".to_string(),
            size: "1.8 GB".to_string(),
            progress: "30%".to_string(),
            status: "Downloading".to_string(),
            seeds: "20".to_string(),
        },
	Torrent {
            name: "example14.torrent".to_string(),
            size: "1.8 GB".to_string(),
            progress: "30%".to_string(),
            status: "Downloading".to_string(),
            seeds: "20".to_string(),
        },
	Torrent {
            name: "example15.torrent".to_string(),
            size: "1.8 GB".to_string(),
            progress: "30%".to_string(),
            status: "Downloading".to_string(),
            seeds: "20".to_string(),
        },
	Torrent {
            name: "example16.torrent".to_string(),
            size: "1.8 GB".to_string(),
            progress: "30%".to_string(),
            status: "Downloading".to_string(),
            seeds: "20".to_string(),
        },
    ]
}


struct Minibuffer {
    content: String,
    height: u16,
}

fn draw_minibuffer(stdout: &mut std::io::Stdout, minibuffer: &Minibuffer, theme: &Theme) -> std::io::Result<()> {
    let (_, term_height) = crossterm::terminal::size()?;
    let minibuffer_y = term_height - minibuffer.height;

    for line in 0..minibuffer.height {
        execute!(
            stdout,
            crossterm::cursor::MoveTo(0, minibuffer_y + line),
            SetBackgroundColor(theme.minibuffer),
            SetForegroundColor(theme.text),
            Clear(ClearType::CurrentLine)
        )?;
    }

    execute!(
        stdout,
        crossterm::cursor::MoveTo(0, minibuffer_y),
        Print(&minibuffer.content),
        ResetColor,
    )?;

    stdout.flush()
}

fn main() -> std::io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    // TODO Look for torrent files in ~/torrents
    let torrents = get_torrents();
    let themes = get_themes();    
    let mut selected = 0;
    let mut theme_index = 0;
    let max_entries = 13;
    let mut scroll_offset = 0;
    let mut minibuffer = Minibuffer {
        content: "".to_string(),
        height: 2,
    };
    let entry_height: usize = 4;

    loop {
	let current_theme = &themes[theme_index];
	
	draw_header(& mut stdout, current_theme)?;
        draw_ui(& mut stdout, &torrents, selected, &mut scroll_offset, max_entries, current_theme, entry_height)?;
        draw_minibuffer(&mut stdout, &minibuffer, current_theme)?;
	
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    break;
                }
		KeyCode::Char('g') => {
		    selected = 0;
                }
		KeyCode::Char('j') | KeyCode::Char('n') => {
                    if selected < torrents.len() - 1 {
			selected += 1;
                    } else {
			selected = 0;
                    }
		}
		KeyCode::Char('k') | KeyCode::Char('p') => {
                    if selected > 0 {
			selected -= 1;
                    } else {
			selected = torrents.len() - 1;
                    }
		}
		KeyCode::Char('l') => {
                    theme_index = (theme_index + 1) % themes.len();
                }

                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}


fn draw_header(stdout: &mut std::io::Stdout, theme: &Theme) -> std::io::Result<()> {
    execute!(
        stdout,
        crossterm::cursor::MoveTo(0, 0),
        SetBackgroundColor(theme.header),
        SetForegroundColor(theme.header_text),
        Clear(ClearType::CurrentLine), // Only clear the current line to avoid clearing the whole screen
        Print(format!("{:<30} {:<10} {:<10} {:<15} {:<6}", "Name", "Size", "Progress", "Status", "Seeds")),
        ResetColor,
    )?;

    stdout.flush()
}

fn draw_ui(stdout: &mut std::io::Stdout,
           torrents: &[Torrent],
           selected: usize,
           scroll_offset: &mut usize,
           max_entries: usize,
           theme: &Theme,
	   entry_height: usize
) -> std::io::Result<()> {
    // Determine the range of torrents to display
    let start = *scroll_offset;
    let end = (*scroll_offset + max_entries).min(torrents.len());

    // keep selection in view
    if selected < start {
        *scroll_offset = selected;
    } else if selected >= end {
        *scroll_offset = selected.saturating_sub(max_entries - 1);
    }

    // Print rows
    for (i, torrent) in torrents[*scroll_offset..(*scroll_offset + max_entries).min(torrents.len())].iter().enumerate() {
        let index = i + *scroll_offset;
        let background_color = if index == selected {
            theme.selection
        } else if index % 2 == 0 {
            theme.bg2
        } else {
            theme.bg1
        };

        // Print the entire block
        for row in 0..entry_height {
            // Move cursor to the appropriate position for the row
            execute!(
                stdout,
                crossterm::cursor::MoveTo(0, (i * entry_height + row) as u16 + 1),
                SetBackgroundColor(background_color)
            )?;

            // Print the torrent text
            if row == 1 {
                let text = format!(
                    "{:<30} {:<10} {:<10} {:<15} {:<6}",
                    torrent.name,
                    torrent.size,
                    torrent.progress,
                    torrent.status,
                    torrent.seeds
                );

                let (term_width, _) = crossterm::terminal::size()?;
                let text_length = text.len();
                let line_length = term_width as usize;

                execute!(
                    stdout,
                    Print(text)
                )?;

                // Fill the remaining space on the line
                if text_length < line_length {
                    let spaces = " ".repeat(line_length - text_length);
                    execute!(
                        stdout,
                        Print(spaces)
                    )?;
                }

                execute!(stdout, ResetColor)?;
            } else {
                let (term_width, _) = crossterm::terminal::size()?;
                execute!(
                    stdout,
                    Print(" ".repeat(term_width as usize))
                )?;
            }
        }
    }

    stdout.flush()
}

struct Theme {
    bg1: Color,
    bg2: Color,
    text: Color,
    header: Color,
    header_text: Color,
    selection: Color,
    modeline: Color,
    minibuffer: Color,
}

fn hex_to_rgb(hex: &str) -> std::result::Result<Color, &'static str> {
    if hex.starts_with('#') && hex.len() == 7 {
        let r = u8::from_str_radix(&hex[1..3], 16).map_err(|_| "Invalid hex format")?;
        let g = u8::from_str_radix(&hex[3..5], 16).map_err(|_| "Invalid hex format")?;
        let b = u8::from_str_radix(&hex[5..7], 16).map_err(|_| "Invalid hex format")?;
        Ok(Color::Rgb { r, g, b })
    } else {
        Err("Invalid hex format")
    }
}

fn get_themes() -> Vec<Theme> {
    vec![
        Theme {
            bg1: hex_to_rgb("#1A1A25").unwrap(),
            bg2: hex_to_rgb("#171721").unwrap(),
	    text: hex_to_rgb("#E6E6E8").unwrap(),
            header_text: hex_to_rgb("#E6E6E8").unwrap(),
            header: hex_to_rgb("#1A1A25").unwrap(),
            selection: hex_to_rgb("#738FD7").unwrap(),
	    modeline: hex_to_rgb("#252534").unwrap(),
	    minibuffer: hex_to_rgb("#1A1A25").unwrap(),
        },
        Theme {
            bg1: hex_to_rgb("#0f172a").unwrap(),
            bg2: hex_to_rgb("#020617").unwrap(),
	    text: hex_to_rgb("#ffffff").unwrap(),
            header_text: hex_to_rgb("#ffffff").unwrap(),
            header: hex_to_rgb("#064e3b").unwrap(),
            selection: hex_to_rgb("#34d399").unwrap(),
	    modeline: hex_to_rgb("#020617").unwrap(),
	    minibuffer: hex_to_rgb("#0f172a").unwrap(),
        },
        Theme {
            bg1: hex_to_rgb("#0f172a").unwrap(),
            bg2: hex_to_rgb("#020617").unwrap(),
            text: hex_to_rgb("#ffffff").unwrap(),
            header_text: hex_to_rgb("#ffffff").unwrap(),
            header: hex_to_rgb("#312E81").unwrap(),
            selection: hex_to_rgb("#818CF8").unwrap(),
            modeline: hex_to_rgb("#020617").unwrap(),
	    minibuffer: hex_to_rgb("#0f172a").unwrap(),
        },
        Theme {
            bg1: hex_to_rgb("#0f172a").unwrap(),
            bg2: hex_to_rgb("#020617").unwrap(),
            text: hex_to_rgb("#ffffff").unwrap(),
            header_text: hex_to_rgb("#ffffff").unwrap(),
            header: hex_to_rgb("#7F1D1D").unwrap(),
            selection: hex_to_rgb("#F87171").unwrap(),
            modeline: hex_to_rgb("#020617").unwrap(),
	    minibuffer: hex_to_rgb("#0f172a").unwrap(),
        },
	Theme {
            bg1: hex_to_rgb("#0f172a").unwrap(),
            bg2: hex_to_rgb("#020617").unwrap(),
            text: hex_to_rgb("#ffffff").unwrap(),
            header_text: hex_to_rgb("#ffffff").unwrap(),
            header: hex_to_rgb("#1E3A8A").unwrap(),
            selection: hex_to_rgb("#60A5FA").unwrap(),
            modeline: hex_to_rgb("#020617").unwrap(),
	    minibuffer: hex_to_rgb("#0f172a").unwrap(),
        },
    ]
}
