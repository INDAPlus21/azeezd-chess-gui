use eliasfl_chess::{Color as Colour, GameState, Game, Piece as Piece};
use ggez::{graphics, Context};
use super::super::{consts, AppState, help_funcs};

/// ## `pepare_text`
/// Returns an instance of the `ggez::graphics::Text` holding a text that informs the state of the game.
/// ### Parameters
/// - `_game: Game`: The `Game` instance from The Elias Engine.
/// - `_current_colour: &Colour`: The current colour (turn colour) in the game
/// ### Return
/// Returns an `ggez::graphics::Text` with the game state info (comedically humorous)
/// 
/// ### The different ouputs:
/// - `InProgress`: (USES RUST `format!()`)
/// - - Haskellers: `"Haskellers's Turn"`
///   - Rustaceans: `"Rustaceans's Turn"`
/// - `Check`: `"It's Check!!!"`
/// - `CheckMate`: 
/// - - Haskellers: `"Farewell Haskell!"`
/// - - Rustaceans: `"Rust lost? PANIC!"`
pub fn prepare_text(_game: &mut Game, _current_colour: &Colour) -> graphics::Text {
        let state_text = graphics::Text::new(
            graphics::TextFragment::from(
                match _game.get_game_state() {
                    GameState::InProgress => format!("{}'s turn!", if *_current_colour == Colour::Black {"Haskeller"} else {"Rustacean"}),
                    GameState::Check => "It's Check!!!".to_string(),
                    GameState::CheckMate => (if *_current_colour == Colour::Black {"Farewell Haskell!"} else {"Rust lost? PANIC!"}).to_string()
                }
            
        )
        .scale(graphics::PxScale { x: 30.0, y: 30.0 }));

        state_text
}

/// ## `draw_text`
/// Takes a `ggez:graphics::Text` and draws it on the screen using an offset from the center
/// ### Parameters
/// - `_ctx: &mut Context`: Instace of GGEZ context
/// - `_text: &ggez::graphics::Text`: GGEZ Text instance
/// - `_offset: (f32, f32)`: Offset from the center, i.e (50.0, 40.0), moves the text 50 units to right and 40 units down
pub fn draw_text(_ctx: &mut Context, _text: &graphics::Text, _offset: (f32, f32)) {
    let text_dimensions = _text.dimensions(_ctx);

    // draw text with dark gray colouring and center position
    graphics::draw(_ctx, _text, graphics::DrawParam::default().color([0.0, 0.0, 0.0, 1.0].into())
        .dest(ggez::mint::Point2 {
            x: (consts::SCREEN_SIZE.0 - text_dimensions.w as f32) / 2f32 as f32 + _offset.0,
            y: (consts::SCREEN_SIZE.0 - text_dimensions.h as f32) / 2f32 as f32 + _offset.1,
        })).expect("Failed to draw text.");
}

/// ## `draw_tile`
/// Takes a row and a column and draws a coloured tile (grey or greyer).
/// ### Parameters
/// - `_ctx: &mut Context`: Instace of GGEZ context
/// - `_row: i32` The number of the row, 0 indexed
/// - `_col: i32` The number of the column, 0 indexed
pub fn draw_tile(_ctx: &mut Context, _row: i32, _col: i32) {
    let rectangle = graphics::Mesh::new_rectangle(_ctx, 
        graphics::DrawMode::fill(), 
        graphics::Rect::new_i32(
            _col * consts::GRID_CELL_SIZE.0 as i32,
            _row * consts::GRID_CELL_SIZE.1 as i32,
            consts::GRID_CELL_SIZE.0 as i32,
            consts::GRID_CELL_SIZE.1 as i32,
        ), match _col % 2 {
            0 => 
                if _row % 2 == 0 { consts::WHITE } 
                else { consts::BLACK },
            _ => 
                if _row % 2 == 0 { consts::BLACK } 
                else { consts::WHITE },
        }).expect("Failed to create tile.");

    graphics::draw(_ctx, &rectangle, graphics::DrawParam::default()).expect("Failed to draw tiles.");
}

/// ## `draw_piece`
/// Takes the AppState instance (that includes the Chess Engine board) and a row and a column and draws the piece on the row and column (if there are any)
/// ### Parameters
/// - `_ctx: &mut Context`: Instace of GGEZ context
/// - `_appstate: &AppState`: Reference to the AppState instance
/// - `_row: i32` The number of the row, 0 indexed
/// - `_col: i32` The number of the column, 0 indexed
pub fn draw_piece(_ctx: &mut Context, _appstate: &AppState, _row: i32, _col: i32) {
    if let Some(_piece) = _appstate.game.board.get(&help_funcs::to_engine_coords(&(_col as u8, _row as u8))) {
        graphics::draw(_ctx, _appstate.sprites.get(&_piece).unwrap(), graphics::DrawParam::default()
            .dest(
                [_col as f32 * consts::GRID_CELL_SIZE.0 as f32, _row as f32 * consts::GRID_CELL_SIZE.1 as f32],
            )
        ).expect("Failed to draw piece.");
    }
}

/// ## `draw_piece`
/// Takes the AppState intance (that includes the Chess Engine board) and a row and a column
/// and draws a small blue circle on that square if the piece click on has that square as its legal moves
/// ### Parameters
/// - `_ctx: &mut Context`: Instace of GGEZ context
/// - `_appstate: &AppState`: Reference to the AppState instance
/// - `_row: i32` The number of the row, 0 indexed
/// - `_col: i32` The number of the column, 0 indexed
pub fn draw_legal_indicator(_ctx: &mut Context, _appstate: &AppState, _row: i32, _col: i32) {
    if _appstate.legal.contains(&(_col as u8, _row as u8)) {
        let circle = graphics::Mesh::new_circle(_ctx, 
            graphics::DrawMode::fill(), ggez::mint::Point2{
            x: (_col as i16 * consts::GRID_CELL_SIZE.0 + consts::GRID_CELL_SIZE.0 / 2) as f32,
            y: (_row as i16 * consts::GRID_CELL_SIZE.1 + consts::GRID_CELL_SIZE.1 / 2) as f32
        }, 25.0, 1.0, graphics::Color::new(0.6, 1.0, 0.6, 0.5)).unwrap();
        
        graphics::draw(_ctx,
            &circle,
            graphics::DrawParam::default()).expect("Failed to draw legal move indictator");
    }
}

/// ## `draw_rectangle`
/// Takes a geometry representation of a rectangle and draws a grey rectangle using that geometry given
/// ### Parameters
/// - `_ctx: &mut Context`: Instace of GGEZ context
/// - `_geometry: (f32, f32, f32, f32)`: Rectangle in this form (top_left_corner_x, top_left_corner_y, width, height)
pub fn draw_rectangle(_ctx: &mut Context, _geometry: (f32, f32, f32, f32))
{
    let background = graphics::Mesh::new_rectangle(_ctx, graphics::DrawMode::fill(),
                        graphics::Rect::new(_geometry.0, _geometry.1, _geometry.2, _geometry.3),
                                [0.2, 0.2, 0.2, 1.0].into());

    graphics::draw(_ctx, &background.unwrap(), graphics::DrawParam::default()).expect("Failed to draw background.");
}

/// ## `draw_icon`
/// Takes AppState (to get sprites) and a position and piece and draws the piece at the given postion.
/// Also takes scale of the art
/// ### Parameters
/// - `_ctx: &mut Context`: Instace of GGEZ context
/// - `_appstate: &AppState`: Reference to the AppState instance
/// - `_at: (f32, f32)`: The destination of the icon (top left anchored)
/// - `_piece: &Piece`: Reference to a piece from the Elias Engine
/// - `_scale: f32`: Scales the sprite
pub fn draw_icon(_ctx: &mut Context, _appstate: &AppState, _at: (f32, f32), _piece: &Piece, _scale: f32) {
    graphics::draw(_ctx, _appstate.sprites.get(&_piece).unwrap(), graphics::DrawParam::default()
                .scale([_scale, _scale])
                .dest(
                    [_at.0, _at.1],
                )).expect("Failed to draw piece.");
}