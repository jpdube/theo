mod winmgr;

use winmgr::WindowManager;

pub fn main() {
    let mut wm = WindowManager::new();
    wm.init();
}
