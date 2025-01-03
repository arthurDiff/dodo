use clap::Args;
use std::{
    io::Write,
    time::{Duration, Instant},
};

use crate::text::Font;

// 17 fps
const FRAME_DELAY: u64 = 60;
const DODO_PORTRAIT: &str = r#"
        .--"""""""-.
       /        +++++++++
     .'        |  __  __ \
    /         /  /  \/  \ \_
   |         |   \_0/\_0/   \_
   |:        |                "-------__________
   |:.       \             -----              oo`\
   |:.        \                                   \
   |' ;         \_   _______                       |
   |:..   .       \         "-------_________      |
   |::.|'     ,    "-----------______________\_   /
   |:::.; ' | .    '   /                       )_/
   |::; | | ; ; |      |
  /::::.|-| |_|-|, |   \
 /'-=-'`  '-'   '--'-   \
/                        \
"#;

const DODO_FRAMES: [&str; 2] = [
    r#"
        .--"""""""-.
       /        +++++++++
     .'        |  __  __ \
    /         /  /  \/  \ \_
   |         |   \_0/\_0/   \_
   |:        |                "-------__________
   |:.       \             -----              oo`\
   |:.        \                                   \
   |' ;         \_   _______                       |
   |:..   .       \         "-------_________      |
   |::.|'     ,    "-----------______________\_   /
   |:::.; ' | .    '   /                       )_/
   |::; | | ; ; |      |
  /::::.|-| |_|-|, |   \
 /'-=-'`  '-'   '--'-   \
/                        \
Just press 'b' to bark (No Audio File Yet), or 'q' to quit
"#,
    r#"
        .--"""""""-.
       /        +++++++++
     .'        |         \
    /         /  ________ \_
   |         |   \_0/\_0/   \_
   |:        |                "-------__________
   |:.       \             -----              oo`\
   |:.        \                                   \
   |' ;         \_   _______                       |
   |:..   .       \         "-------_________      |
   |::.|'     ,    "----___      "---.__     \_   /
   |:::.; ' | .    '   /   "----.___    "---__ )_/
   |::; | | ; ; |      |            "----.___/
  /::::.|-| |_|-|, |   \                   
 /'-=-'`  '-'   '--'-   \
/                        \
Just press 'b' to bark (No Audio File Yet), or 'q' to quit
"#,
];

#[derive(Debug, Args)]
pub(crate) struct BarkArgs;

impl super::DoDoArgs for BarkArgs {
    fn execute(&self) -> crate::Result<()> {
        if !cfg!(windows) {
            println!(
                "{}\n{}",
                DODO_PORTRAIT,
                "Yikes i can only bark on windows.".underline()
            );
            return Ok(());
        }

        let mut start_inst = Instant::now();
        // Frames to go trough (0 == no animation)
        let mut bark_frame: usize = 0;
        loop {
            if bark_frame > 0 {
                bark_frame = bark_frame.saturating_sub(1);
            }
            print!("\x1B[2J\x1B[1;1H{}", DODO_FRAMES[bark_frame % 2],);
            std::io::stdout().flush()?;

            if q_pressed() {
                break;
            }
            if bark_frame == 0 && b_pressed() {
                bark_frame = 2;
            }

            let dur_since = Instant::now().duration_since(start_inst);
            if Duration::from_millis(FRAME_DELAY) > dur_since {
                std::thread::sleep(Duration::from_millis(FRAME_DELAY) - dur_since);
            }
            start_inst = Instant::now();
        }

        Ok(())
    }
}

const KEY_Q: i32 = 0x51;
fn q_pressed() -> bool {
    let state = unsafe { winapi::um::winuser::GetAsyncKeyState(KEY_Q) };
    i32::from(state) & 0x8001 != 0
}

const KEY_B: i32 = 0x42;
fn b_pressed() -> bool {
    let state = unsafe { winapi::um::winuser::GetAsyncKeyState(KEY_B) };
    i32::from(state) & 0x8001 != 0
}
