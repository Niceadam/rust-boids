use boids::Flock;
use macroquad::prelude::*;
use std::{thread, time};

#[macroquad::main("Boids")]
async fn main() {
    let mut flock = Flock::new(100);

    loop {
        if is_mouse_button_down(MouseButton::Left) {
            flock.add_boid(mouse_position().0, mouse_position().1);
        }

        clear_background(WHITE);
        flock.draw();
        flock.update();

        thread::sleep(time::Duration::from_millis(15));
        next_frame().await
    }
}
