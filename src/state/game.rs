use ggez::{Context};
use ggez::timer;

use crate::{mygame, player, bullet, enemy, room};
use mygame::{MyGame, GameState, new_game};

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

    // GAME OVER!
    if !mygame.player1.player_still_alive() {
        println!("SCORE: {}", mygame.score);
        *mygame = new_game(ctx, GameState::GAMEOVER);
        mygame.game_state = GameState::GAMEOVER;
    }
    player::enter_new_room(mygame);

    bullet::move_bullet(&mut mygame.bullets);
    bullet::remove_old_bullets(&mut mygame.bullets);
    bullet::fire_new_bullet_on_keypress(ctx, &mut mygame.bullets, &mut mygame.player1);

    // Bullet hit enemy.
    for bullet in &mut mygame.bullets {
        for enemy in &mut mygame.enemies {
            // Copied from https://developer.mozilla.org/en-US/docs/Games/Techniques/2D_collision_detection
            if bullet.x_pos < enemy.x_pos + enemy.width &&
                bullet.x_pos + bullet.width > enemy.x_pos &&
                bullet.y_pos < enemy.y_pos + enemy.height &&
                bullet.y_pos + bullet.height > enemy.y_pos {
                enemy.is_alive = false;
                bullet.did_hit = true;
                mygame.score += 1;
            }
        }
    }

    // Check if player killed all enemies and if so, mark the room.
    if mygame.enemies.is_empty() {
        let (row, col) = mygame.player1.current_room;
        mygame.rooms[row][col].is_finished = true;
    }

    enemy::enemy_hit_player(&mut mygame.enemies, &mut mygame.player1);
    enemy::update_pos(&mut mygame.enemies, &mygame.player1);
    enemy::remove_the_dead(&mut mygame.enemies);
}



pub fn draw(ctx: &mut Context, mygame: &mut MyGame) {
    room::draw_background(ctx, mygame);
    room::draw_doors(ctx, mygame);
    bullet::draw(ctx, mygame);
    enemy::draw(ctx, mygame);
    player::draw(ctx, mygame);
}