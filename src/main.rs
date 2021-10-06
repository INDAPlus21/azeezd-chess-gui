/**
 * Chess GUI template.
 * Author: Viola Söderlund <violaso@kth.se>
 * Last updated: 2021-10-03
 */

use ggez::{conf, event, graphics, ContextBuilder, Context, GameError, GameResult};
use std::{path, env, collections::HashMap};
use eliasfl_chess::{Game, Color as Colour, Piece as PieceType, GameState, Position};

/// A chess board is 8x8 tiles.
const GRID_SIZE: i16 = 8;
/// Sutible size of each tile.
const GRID_CELL_SIZE: (i16, i16) = (90, 90);

/// Size of the application window.
const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE as f32 * GRID_CELL_SIZE.1 as f32 + 150.0,
);

// GUI Color representations
const BLACK: graphics::Color = graphics::Color::new(30.0/255.0, 30.0/255.0, 30.0/255.0, 1.0);
const WHITE: graphics::Color = graphics::Color::new(70.0/255.0, 70.0/255.0, 70.0/255.0, 1.0);

// Moves to be made after a promotion
struct PendingMove {
    _from: String,
    _to: String
}

/// GUI logic and event implementation structure. 
struct AppState {
    sprites: HashMap<PieceType, graphics::Image>,
    game: Game,
    legal: Vec<(u8, u8)>, // When clicking on a piece, it saves the legal moves in this vec to display the indicators on the board
    previous_click: Option<(u8, u8)>, // The previous square clicked by the player
    promoting: bool, // If the player is currently promoting a piece (makes a small window pop up for the player to choose)
    pending_promotion_move: PendingMove,
    deaths: HashMap<Colour, Vec<PieceType>>
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
    fn load_sprites(ctx: &mut Context) -> HashMap<PieceType, graphics::Image> {

        [
            ((PieceType::King(Colour::Black)), "/black_king.png".to_string()),
            ((PieceType::Queen(Colour::Black)), "/black_queen.png".to_string()),
            ((PieceType::Rook(Colour::Black)), "/black_rook.png".to_string()),
            ((PieceType::Pawn(Colour::Black)), "/black_pawn.png".to_string()),
            ((PieceType::Bishop(Colour::Black)), "/black_bishop.png".to_string()),
            ((PieceType::Knight(Colour::Black)), "/black_knight.png".to_string()),
            ((PieceType::King(Colour::White)), "/white_king.png".to_string()),
            ((PieceType::Queen(Colour::White)), "/white_queen.png".to_string()),
            ((PieceType::Rook(Colour::White)), "/white_rook.png".to_string()),
            ((PieceType::Pawn(Colour::White)), "/white_pawn.png".to_string()),
            ((PieceType::Bishop(Colour::White)), "/white_bishop.png".to_string()),
            ((PieceType::Knight(Colour::White)), "/white_knight.png".to_string())
        ]
            .iter()
            .map(|(_piece, _path)| {
                (*_piece, graphics::Image::new(ctx, _path).unwrap())
            })
            .collect::<HashMap<PieceType, graphics::Image>>()
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

        // create text representation
        let state_text = graphics::Text::new(
                graphics::TextFragment::from(
                    match self.game.get_game_state() {
                        GameState::InProgress => format!("{}'s turn!", if current_colour == Colour::Black {"Haskeller"} else {"Rustacean"}),
                        GameState::Check => "It's Check!!!".to_string(),
                        GameState::CheckMate => (if current_colour == Colour::Black {"Farewell Haskell!"} else {"Rust lost? PANIC!"}).to_string()
                    }
                
            )
            .scale(graphics::PxScale { x: 30.0, y: 30.0 }));

        // get size of text
        let text_dimensions = state_text.dimensions(ctx);
        // create background rectangle with white coulouring
        let background_box = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(),
            graphics::Rect::new((SCREEN_SIZE.0 - text_dimensions.w as f32) / 2f32 as f32 - 8.0,
                                (SCREEN_SIZE.0 - text_dimensions.h as f32) / 2f32 as f32,
                                text_dimensions.w as f32 + 16.0, text_dimensions.h as f32),
                                [1.0, 1.0, 1.0, 1.0].into()
        )?;

        // draw background
        graphics::draw(ctx, &background_box, graphics::DrawParam::default()).expect("Failed to draw background.");

        // draw grid
        for _row in 0..8 {
            for _col in 0..8 {

                // draw tile
                let rectangle = graphics::Mesh::new_rectangle(ctx, 
                    graphics::DrawMode::fill(), 
                    graphics::Rect::new_i32(
                        _col * GRID_CELL_SIZE.0 as i32,
                        _row * GRID_CELL_SIZE.1 as i32,
                        GRID_CELL_SIZE.0 as i32,
                        GRID_CELL_SIZE.1 as i32,
                    ), match _col % 2 {
                        0 => 
                            if _row % 2 == 0 { WHITE } 
                            else { BLACK },
                        _ => 
                            if _row % 2 == 0 { BLACK } 
                            else { WHITE },
                    }).expect("Failed to create tile.");
                graphics::draw(ctx, &rectangle, graphics::DrawParam::default()).expect("Failed to draw tiles.");

                // draw piece
                if let Some(_piece) = self.game.board.get(&Position{rank: (8 - _row) as u8, file: (_col + 1) as u8}) {
                    graphics::draw(ctx, self.sprites.get(&_piece).unwrap(), graphics::DrawParam::default()
                        .dest(
                            [_col as f32 * GRID_CELL_SIZE.0 as f32, _row as f32 * GRID_CELL_SIZE.1 as f32],
                        )
                    ).expect("Failed to draw piece.");
                }

                // Draw an indicator (white circle) on legal moves for the piece clicked
                if self.legal.contains(&(_col as u8, _row as u8)) {
                    let circle = graphics::Mesh::new_circle(ctx, 
                        graphics::DrawMode::fill(), ggez::mint::Point2{
                        x: (_col as i16 * GRID_CELL_SIZE.0 + GRID_CELL_SIZE.0 / 2) as f32,
                        y: (_row as i16 * GRID_CELL_SIZE.1 + GRID_CELL_SIZE.1 / 2) as f32
                    }, 25.0, 1.0, graphics::Color::new(0.6, 1.0, 0.6, 0.5)).unwrap();
                    
                    graphics::draw(ctx,
                        &circle,
                        graphics::DrawParam::default()).expect("Failed to draw legal move indictator");
                }
            }
        }

        // If the player is not promoting at the moment. Display the the turn and the state of the game
        if !self.promoting {
            // draw text with dark gray colouring and center position
            graphics::draw(ctx, &state_text, graphics::DrawParam::default().color([0.0, 0.0, 0.0, 1.0].into())
                .dest(ggez::mint::Point2 {
                    x: (SCREEN_SIZE.0 - text_dimensions.w as f32) / 2f32 as f32,
                    y: (SCREEN_SIZE.0 - text_dimensions.h as f32) / 2f32 as f32 + 440.0,
                })).expect("Failed to draw text.");

            if self.game.get_game_state() == GameState::CheckMate {
                let replay_text = graphics::Text::new(
                    graphics::TextFragment::from("Click in this area to replay!")
                .scale(graphics::PxScale { x: 20.0, y: 20.0 }));
                
                let replay_dimensions = replay_text.dimensions(ctx);

                graphics::draw(ctx, &replay_text, graphics::DrawParam::default().color([0.0, 0.0, 0.0, 1.0].into())
                    .dest(ggez::mint::Point2 {
                        x: (SCREEN_SIZE.0 - replay_dimensions.w as f32) / 2f32 as f32,
                        y: (SCREEN_SIZE.0 - replay_dimensions.h as f32) / 2f32 as f32 + 480.0,
                    })).expect("Failed to draw text.");
            }
        }
        else { // Player is promoting

            // Draw a grey rectangle where the choices for promotion will be
            let background = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(),
            graphics::Rect::new(20.0, 740.0, 680.0, 110.0),
                                [0.2, 0.2, 0.2, 1.0].into())?;

            graphics::draw(ctx, &background, graphics::DrawParam::default()).expect("Failed to draw background.");

            // Draw Queen Icon
            graphics::draw(ctx, self.sprites.get(&PieceType::Queen(current_colour)).unwrap(), graphics::DrawParam::default()
                .dest(
                    [50.0, 750.0],
                )).expect("Failed to draw piece.");

            // Draw Knight icon
            graphics::draw(ctx, self.sprites.get(&PieceType::Knight(current_colour)).unwrap(), graphics::DrawParam::default()
                .dest(
                    [230.0, 750.0],
                )).expect("Failed to draw piece.");

            // Draw Rook Icon
            graphics::draw(ctx, self.sprites.get(&PieceType::Rook(current_colour)).unwrap(), graphics::DrawParam::default()
                .dest(
                    [410.0, 750.0],
                )).expect("Failed to draw piece.");

            // Draw Bishop Icon
            graphics::draw(ctx, self.sprites.get(&PieceType::Bishop(current_colour)).unwrap(), graphics::DrawParam::default()
                .dest(
                    [590.0, 750.0],
                )).expect("Failed to draw piece.");
        }

        let background = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(),
            graphics::Rect::new(5.0, 725.0, 710.0, 40.0),
                                [0.2, 0.2, 0.2, 1.0].into())?;

        graphics::draw(ctx, &background, graphics::DrawParam::default()).expect("Failed to draw background.");
        for deaths_of_colour in self.deaths.iter() {
            let mut index : f32 = 0.0;
            for _piece in deaths_of_colour.1 {
                graphics::draw(ctx, self.sprites.get(&_piece).unwrap(), graphics::DrawParam::default()
                    .scale([0.4, 0.4])
                    .dest(
                        match deaths_of_colour.0 {
                            Colour::White => [10.0 + 20.0 * index, 730.0],
                            _ => [670.0 - (20.0 * index), 730.0]
                        }
                    ,
                    )).expect("Failed to draw piece.");

                index += 1.0;
            }
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
                        if let PieceType::Pawn(_colour) = piece.unwrap() {
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
                                // Dead pieces are added to the death vector for display
                                if let Some(_piece) = self.game.board.get(&to_engine_coords(&square_clicked)) {
                                        self.deaths.get_mut(&!self.game.active_color).unwrap().push(*_piece);
                                }
                                self.game.make_move(from, to).ok();
                                self.legal.clear();
                            }
                        }
                        else { // If it was no pawn that is the piece just make the move and clear the legal moves stored

                            // Dead pieces are added to the death vector for display
                            if let Some(_piece) = self.game.board.get(&to_engine_coords(&square_clicked)) {
                                self.deaths.get_mut(&!self.game.active_color).unwrap().push(*_piece);
                            }

                            self.game.make_move(from, to).ok();
                            self.legal.clear();
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

                    // No longer promoting
                    self.promoting = false;

                    // Clear the struct of legal moves that hold the legal moves of the piece
                    self.legal.clear();

                    // Reset Pending Move struct
                    self.pending_promotion_move = PendingMove{_from: "".to_string(), _to: "".to_string()};
                }

                // Checkmate makes the area under the board clickable
                // Upon clicking the game resets
                if self.game.get_game_state() == GameState::CheckMate { 
                    self.game = Game::new(); // New board
                    self.deaths.get_mut(&Colour::Black).unwrap().clear();
                    self.deaths.get_mut(&Colour::White).unwrap().clear();

                    // Reset game storages
                    self.legal.clear();
                    self.previous_click = None;
                    self.promoting = false;
                    self.pending_promotion_move = PendingMove{_from: "".to_string(), _to: "".to_string()};
                }
            }
        }
    }
}

// The two functions below are the same as the ones used in my Engine assignment. They serve their purpose there so I copied them over here to do likewise.

/// Converts a (u8, u8) to String in the form "\<file\>\<rank\>"
pub fn num_to_filerank(_coords: &(u8, u8)) -> String {
    let mut string_coords = String::with_capacity(2);

    string_coords.push((_coords.0 as u8 + 97) as char);
    string_coords.push((56 - _coords.1 as u8) as char);

    string_coords
}

/// Converts a String from "\<file\>\<rank\>" to a coord in the form of (u8, u8)
pub fn filerank_to_num(_filerank: &String) -> (u8, u8) {
    let mut coords = (0,0);
    
    let _filerank = _filerank.as_bytes();

    // The rank represnts the y-axis thus it is the 1st coordinate
    // And the file the x-axes hence the 0th coordinate
    coords.0 = (_filerank[0] - 97) as u8; // Lowercase alphabet to u8 using ascii value different between letter and numerical value that is 1-indexed

    /* Convert number as ascii char to actual numerical value by doing minus 49 (ascii difference) but the the board's origin is at bottom left
        So the x coordinate must shift by 7 - (top left origin coord) thus
        7 - ([ascii val] - 49) gives 56 - [ascii val]
    */
    coords.1 = (56 - _filerank[1]) as u8; 

    coords
}

/// Takes a (u8,u8) coords used in the gui and converts them to coords used by the Chess Engine as the Position struct from the Engine
pub fn to_engine_coords(_coords: &(u8, u8)) -> Position {
    Position{file: _coords.0 + 1, rank: 8 - _coords.1}
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