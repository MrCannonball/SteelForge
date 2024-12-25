#[link(name = "game.dll", kind="dylib")]
extern {
    fn main_game();
}
#[link(name = "sfcore.dll", kind="dylib")]
extern {
    fn sfcore_load();
}

fn main() {
    unsafe {
        sfcore_load();
        main_game();
    }
    println!("Hello, world!");
}
