use ggez::{Context};
use ggez::timer;

use crate::mygame;
use crate::player;
use crate::bullet;
use crate::enemy;
use crate::room;

use mygame::{MyGame, Game_state};

pub fn update(ctx: &mut Context, mygame: &mut MyGame) {
    // Handle cooldown when shooting bullets.
    const DESIRED_FPS: u32 = 60;
    while timer::check_update_time(ctx, DESIRED_FPS) {
        let seconds = 1.0 / (DESIRED_FPS as f32);
        mygame.player1.reload_time -= seconds;
    }

    // Handle movement for player1.
    mygame.player1.movement(ctx);
    mygame.player1.no_wall_hax();
    if mygame.player1.player_still_alive() {
        println!("SCORE: {}", mygame.score);
        mygame.game_state = Game_state::GAMEOVER;
    }
    player::enter_new_room(mygame);

    bullet::move_bullet(&mut mygame.bullets);
    bullet::remove_old_bullets(&mut mygame.bullets);
    bullet::fire_new_bullet_on_keypress(ctx, &mut mygame.bullets, &mut mygame.player1);

    // Bullet hit enemy.
    for bull in &mut mygame.bullets {
        for ene in &mut mygame.enemies {
            // Copied from https://developer.mozilla.org/en-US/docs/Games/Techniques/2D_collision_detection
            if bull.x_pos < ene.x_pos + ene.width &&
                bull.x_pos + bull.width > ene.x_pos &&
                bull.y_pos < ene.y_pos + ene.height &&
                bull.y_pos + bull.height > ene.y_pos {
                ene.is_alive = false;
                bull.did_hit = true;
                mygame.score += 1;
            }
        }
    }

    // Check if player killed all enemies and if so, mark the room.
    if mygame.enemies.is_empty() {
        mygame.rooms[mygame.player1.current_room.0][mygame.player1.current_room.1].is_finished = true;
    }

    enemy::enemy_hit_player(&mut mygame.enemies, &mut mygame.player1);

    // Updates enemies positions.
    for ene in &mut mygame.enemies {
        ene.update_pos(mygame.player1.x_pos, mygame.player1.y_pos);
    }
    enemy::remove_the_dead(&mut mygame.enemies);
}



pub fn draw(ctx: &mut Context, mygame: &mut MyGame) {
    // Draws background and doors.
    room::draw_background(ctx, mygame);
    room::draw_doors(ctx, mygame);

    bullet::draw(ctx, mygame);
    enemy::draw(ctx, mygame);
    player::draw(ctx, mygame);
}