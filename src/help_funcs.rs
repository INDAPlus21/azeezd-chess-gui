use eliasfl_chess::{Position, Piece, Color as Colour};
use super::{AppState};

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

/// Gets the colour of a given piece
pub fn get_piece_colour(_piece: &Piece) -> &Colour {
    // I am sorry if this look very ugly, it's the only solution I found on the Internet
    // to get a value out from an Enum with the same type as a value

    match _piece {
        Piece::Pawn(colour) => colour,
        Piece::Knight(colour) => colour,
        Piece::Bishop(colour) => colour,
        Piece::Rook(colour) => colour,
        Piece::Queen(colour) => colour,
        Piece::King(colour) => colour
    }
}

impl AppState {
    /// Calls the move functions from the engine, clears the board and updates the dear bar
    pub fn make_move_full(&mut self, _clicked: (u8, u8), _from: String, _to: String) {
        if let Some(_piece) = self.game.board.get(&to_engine_coords(&_clicked)) {

            // Get colour of the piece

            if self.game.active_color != *get_piece_colour(_piece) {
                self.deaths.get_mut(&!self.game.active_color).unwrap().push(*_piece);
            }
        }
        self.game.make_move(_from, _to).ok();
        self.legal.clear();
    }
}