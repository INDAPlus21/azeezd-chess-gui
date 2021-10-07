/**
 * Chess GUI template.
 * Author: Viola Söderlund <violaso@kth.se>
 * Last updated: 2021-10-03
 */

pub mod graphics_funcs;
pub mod consts;
pub mod help_funcs;

use ggez::{conf, event, graphics, ContextBuilder, Context, GameError, GameResult};
use std::{path, env, collections::HashMap};
use eliasfl_chess::{Game, Color as Colour, Piece, GameState};
use graphics_funcs::*;
use consts::*;
use help_funcs::*;


// Moves to be made after a promotion
struct PendingMove {
    _from: String,
    _to: String
}

/// GUI logic and event implementation structure. 
pub struct AppState {
    sprites: HashMap<Piece, graphics::Image>,
    game: Game,
    legal: Vec<(u8, u8)>, // When clicking on a piece, it saves the legal moves in this vec to display the indicators on the board
    previous_click: Option<(u8, u8)>, // The previous square clicked by the player
    promoting: bool, // If the player is currently promoting a piece (makes a small window pop up for the player to choose)
    pending_promotion_move: PendingMove,
    deaths: HashMap<Colour, Vec<Piece>>
}

impl AppState {
    /// Initialise new application, i.e. initialise new game and load resources.
    fn new(ctx: &mut Context) -> GameResult<AppState> {

        let mut state = AppState {
            sprites: AppState::load_sprites(ctx),
            game: Game::new(),
            legal: vec![],
            previous_click: None,
            promoting: false,
            pending_promotion_move: PendingMove{_from: "".to_string(), _to: "".to_string()},
            deaths: HashMap::new()
        };

        state.deaths.insert(Colour::Black, vec![]);
        state.deaths.insert(Colour::White, vec![]);
        Ok(state)
    }

    /// Loads chess piese images into vector.
    fn load_sprites(ctx: &mut Context) -> HashMap<Piece, graphics::Image> {

        [
            ((Piece::King(Colour::Black)), "/black_king.png".to_string()),
            ((Piece::Queen(Colour::Black)), "/black_queen.png".to_string()),
            ((Piece::Rook(Colour::Black)), "/black_rook.png".to_string()),
            ((Piece::Pawn(Colour::Black)), "/black_pawn.png".to_string()),
            ((Piece::Bishop(Colour::Black)), "/black_bishop.png".to_string()),
            ((Piece::Knight(Colour::Black)), "/black_knight.png".to_string()),
            ((Piece::King(Colour::White)), "/white_king.png".to_string()),
            ((Piece::Queen(Colour::White)), "/white_queen.png".to_string()),
            ((Piece::Rook(Colour::White)), "/white_rook.png".to_string()),
            ((Piece::Pawn(Colour::White)), "/white_pawn.png".to_string()),
            ((Piece::Bishop(Colour::White)), "/white_bishop.png".to_string()),
            ((Piece::Knight(Colour::White)), "/white_knight.png".to_string())
        ]
            .iter()
            .map(|(_piece, _path)| {
                (*_piece, graphics::Image::new(ctx, _path).unwrap())
            })
            .collect::<HashMap<Piece, graphics::Image>>()
    }
}

impl event::EventHandler<GameError> for AppState {

    /// For updating game logic, which front-end doesn't handle.
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    /// Draw interface, i.e. draw game board
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let current_colour = self.game.active_color;

        // clear interface with gray background colour
        graphics::clear(ctx, (match current_colour {
            Colour::Black => [0.37, 0.31, 0.53, 1.0],
            _ => [0.97, 0.3, 0.0, 1.0]
        }).into());

        // draw grid
        for _row in 0..8 {
            for _col in 0..8 {

                // draw tile
                draw_funcs::draw_tile(ctx, _row, _col);

                // draw piece
                draw_funcs::draw_piece(ctx, &self, _row, _col);

                // Draw an indicator (white circle) on legal moves for the piece clicked
                draw_funcs::draw_legal_indicator(ctx, &self, _row, _col);
            }
        }

        // If the player is not promoting at the moment. Display the the turn and the state of the game
        if !self.promoting {
            // create text representation
            let state_text = graphics_funcs::draw_funcs::prepare_text(&mut self.game, &current_colour);
            draw_funcs::draw_text(ctx, &state_text, (0.0, 440.0));

            if self.game.get_game_state() == GameState::CheckMate {
                let replay_text = graphics::Text::new(
                    graphics::TextFragment::from("Click in this area to replay!")
                .scale(graphics::PxScale { x: 20.0, y: 20.0 }));
                
                draw_funcs::draw_text(ctx, &replay_text, (0.0, 480.0));
            }

        draw_funcs::draw_rectangle(ctx, (5.0, 725.0, 710.0, 40.0));
        
        for deaths_of_colour in self.deaths.iter() {
            let mut index : f32 = 0.0;
            for _piece in deaths_of_colour.1 {

                let position = match deaths_of_colour.0 {
                    Colour::White => (10.0 + 20.0 * index, 730.0),
                    _ => (670.0 - (20.0 * index), 730.0)
                };

                draw_funcs::draw_icon(ctx, &self, position, &_piece, 0.4);
                index += 1.0;
            }
        }
        }
        else { // Player is promoting

            // Draw a grey rectangle where the choices for promotion will be
            draw_funcs::draw_rectangle(ctx, (20.0, 740.0, 680.0, 110.0));

            // Draw Queen Icon
            draw_funcs::draw_icon(ctx, &self, (50.0, 750.0), &Piece::Queen(current_colour), 1.0);

            // Draw Knight icon
            draw_funcs::draw_icon(ctx, &self, (230.0, 750.0), &Piece::Knight(current_colour), 1.0);

            // Draw Rook Icon
            draw_funcs::draw_icon(ctx, &self, (410.0, 750.0), &Piece::Rook(current_colour), 1.0);

            // Draw Bishop Icon
            draw_funcs::draw_icon(ctx, &self, (590.0, 750.0), &Piece::Bishop(current_colour), 1.0);
                
        }

        // render updated graphics
        graphics::present(ctx).expect("Failed to update graphics.");

        Ok(())
    }

    /// Update game on mouse click
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: event::MouseButton, x: f32, y: f32) {
        if button == event::MouseButton::Left {
            if y < 720.0 { // Clicks within the board grid

                // The square clicked on by the player represented as a (u8, u8) coordinate
                let square_clicked = ((x as i16 / GRID_CELL_SIZE.0) as u8, (y as i16 / GRID_CELL_SIZE.1) as u8);

                // If the square is something new then do it's either check new legal moves, attack or just pure none-sense
                if self.previous_click != Some(square_clicked) {

                    // If the square is a legal move then it must be a move/attack
                    if self.legal.contains(&square_clicked) && !self.previous_click.is_none() { // Previous click can be none at times when reseting previous clicks

                        // The piece currently moving or attacking
                        let piece = self.game.board.get(&to_engine_coords(&self.previous_click.unwrap()));
                        
                        // Get from and to coords as "<file><rank>" format
                        let from = num_to_filerank(&self.previous_click.unwrap());
                        let to = num_to_filerank(&square_clicked);

                        // If the piece is a pawn then check if it reached the edges for promotion
                        if let Piece::Pawn(_colour) = piece.unwrap() {
                            if (*_colour == Colour::Black && square_clicked.1 == 7) // Black reached bottom of board
                            || (*_colour == Colour::White && square_clicked.1 == 0) { // White reached top of board
                                self.promoting = true; // It's promoting time

                                // Save the moves in the pending move struct to deploy them after the player's choice of piece type to promote to
                                // NOTE: the move occurs in the TOP level if-else clause. It is marked by the (👌)
                                self.pending_promotion_move = PendingMove {
                                    _from: from,
                                    _to: to
                                }
                            }
                            else { // If piece was no at the edge then just do a normal move and move on
                                self.make_move_full(square_clicked, from, to);
                            }
                        }
                        else { // If it was no pawn that is the piece just make the move and clear the legal moves stored

                            // Dead pieces are added to the death vector for display
                            self.make_move_full(square_clicked, from, to);
                        }
                    }
                    else { // If move is not legal
                        // Clear the legal moves because a new piece might calculate it's legal pieces and store it here
                        self.legal.clear(); 

                        // Get the legal moves of the piece
                        let square_as_filerank = num_to_filerank(&square_clicked); // Numerical coord to "<File><Rank>" conversion
                        let moves = self.game.get_possible_moves(square_as_filerank); // Get the legal moves
                        let moves = if moves.is_none() {vec![]} else {moves.unwrap()}; // As per Elias' Engine; no legal moves return None

                        // The previous clicked is now the one the player just clicked
                        self.previous_click = Some(square_clicked);

                        // Newly calculated legal moves are stored in the legal move vec for board indication and legal move checking
                        for _move in moves {
                            self.legal.push(filerank_to_num(&_move));
                        }
                    }
                }             
            }
            else {
                // The player is promoting their pieces. Occurs in the rectangle at
                // x=20, y=740, width=720, height=130
                if self.promoting && y >= 740.0 && y <= 850.0{

                    // Pieces are ordered based on their choice frequency in all chess games

                    if x >= 20.0 && x <= 200.0 { // First icon, upon click choose queen
                        self.game.set_promotion("queen".to_string()).ok();
                    }
                    else if x > 200.0 && x <= 380.0 { // Second icon, upon click choose knight
                        self.game.set_promotion("knight".to_string()).ok();
                    } 
                    else if x > 380.0 && x <= 560.0 { // Third icon, upon click choose rook
                        self.game.set_promotion("rook".to_string()).ok();
                    } 
                    else if x > 560.0 && x <= 740.0 { // Fourth icon, upon click choose bishop
                        self.game.set_promotion("bishop".to_string()).ok();
                    }

                    if let Some(_piece) = self.game.board.get(&to_engine_coords(&filerank_to_num(&self.pending_promotion_move._to.to_string()))) {
                        self.deaths.get_mut(&!self.game.active_color).unwrap().push(*_piece);
                    }

                    // (👌) Pending move occurs here
                    self.game.make_move(self.pending_promotion_move._from.to_string(), self.pending_promotion_move._to.to_string()).ok();

                    // Clear the struct of legal moves that hold the legal moves of the piece
                    self.legal.clear();

                    // Reset Pending Move struct
                    self.pending_promotion_move = PendingMove{_from: "".to_string(), _to: "".to_string()};

                    // No longer promoting
                    self.promoting = false;
                }

                // Checkmate makes the area under the board clickable
                // Upon clicking the game resets
                if self.game.get_game_state() == GameState::CheckMate { 
                    self.game = Game::new(); // New board

                    // Reset game storages
                    self.legal.clear();
                    self.previous_click = None;
                    self.promoting = false;
                    self.pending_promotion_move = PendingMove{_from: "".to_string(), _to: "".to_string()};
                    self.deaths.get_mut(&Colour::Black).unwrap().clear();
                    self.deaths.get_mut(&Colour::White).unwrap().clear();
                }
            }
        }
    }
}

pub fn main() -> GameResult {

    let resource_dir = path::PathBuf::from("./resources");

    let context_builder = ContextBuilder::new("schack", "viola")
        .add_resource_path(resource_dir)        // Import image files to GGEZ
        .window_setup(
            conf::WindowSetup::default()  
                .title("Schack")                // Set window title "Schack"
                .icon("/icon.png")              // Set application icon
        )
        .window_mode(
            conf::WindowMode::default()
                .dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1) // Set window dimensions
                .resizable(false)               // Fixate window size
        );
    let (mut contex, event_loop) = context_builder.build().expect("Failed to build context.");

    let state = AppState::new(&mut contex).expect("Failed to create state.");
    event::run(contex, event_loop, state)       // Run window event loop
}