extern crate rustty;

use rustty::{
  Terminal,
  HasSize,
  Color,
  Event,
  Attr,
  Cell
};

use rustty::ui::{
  Painter,
  Dialog,
  DialogResult,
  Alignable,
  HorizontalAlign,
  VerticalAlign,
};

struct Cursor {
  pos: Position,
  lpos: Position,
  color: Color,
}

#[derive(Copy, Clone)]
struct Position {
  x: usize,
  y: usize,
}

fn main() {
  let mut cursor = Cursor {
    pos: Position { x: 1, y: 1 },
    lpos: Position { x: 1, y: 1 },
    color: Color::Blue,
  };
  let welcome_msg = "Welcome to rmpc! Navigate with j/k to pick a song and play it with ↵ and q for quit";
  let mut songs: Vec<String> = Vec::new();
  let songs = ["01 - takk....mp3", "03 - hoppípolla.mp3", "05 - sé lest.mp3", "07 - mílanó.mp3", "09 - andvari.mp3", "11 - heysátan.mp3", "02 - glósóli.mp3", "04 - með blóðnasir.mp3", "06 - sæglópur.mp3", "08 - gong.mp3", "10 - svo hljótt.mp3"];
  let mut term = Terminal::new().unwrap();
  let current_song = -1;
  term[(cursor.pos.x, cursor.pos.y)].set_bg(cursor.color);
  term.printline(0,0, welcome_msg);
  let line_start = "[ ] ";
  for (i, song) in songs.iter().enumerate() {
    term.printline(0, i+1, line_start);
    term.printline(line_start.len(), i+1, song);
  }
  term[(cursor.lpos.x, cursor.lpos.y)].set_bg(Color::Default);
  term[(cursor.pos.x, cursor.pos.y)].set_bg(cursor.color);
  term.swap_buffers().unwrap();
  loop {
    let evt = term.get_event(100).unwrap();
    if let Some(Event::Key(ch)) = evt {
      match ch {
        'q' => {
          break;
        },
        '\r' => {
          for (i, song) in songs.iter().enumerate() {
            term[(1, i+1)].set_ch(' ');
          }
          term[(cursor.pos.x, cursor.pos.y)].set_ch('*');
          let current_song = cursor.pos.y - 1;
          term.printline(0, songs.len()+3, "currently playing: ");
          term.printline(20, songs.len()+3, songs[current_song]);
        },
        'k' => {
          cursor.lpos = cursor.pos;
          if cursor.pos.y > 1 {
            cursor.pos.y -= 1;
          }
        },
        'j' => {
          cursor.lpos = cursor.pos;
          if cursor.pos.y < songs.len() {
            cursor.pos.y += 1;
          }
        },
        _ => {
        },
      }
      term[(cursor.lpos.x, cursor.lpos.y)].set_bg(Color::Default);
      term[(cursor.pos.x, cursor.pos.y)].set_bg(cursor.color);
      term.swap_buffers().unwrap();
    }
  }
}
