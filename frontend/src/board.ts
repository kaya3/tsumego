type StoneColour = 'b' | 'w'

type StoneColourOrEmpty = StoneColour | '.'

/**
 * A board position.
 */
class Board {
    /**
     * Represents the board position as a multi-line string.
     */
    public readonly board: string;
    
    /**
     * The board size. The board is square, so this is the dimension on both
     * sides.
     */
    public readonly size: number;
    
    public constructor(board: string) {
        this.board = board;
        
        const rows = board.split('\n');
        const size = this.size = rows.length - 1;
        
        if(rows[0] !== 'b' && rows[0] !== 'w') {
            throw new Error(`Invalid next player; expected 'b' or 'w', was '${rows[0]}'`);
        }
        
        for(let i = 1; i < rows.length; ++i) {
            const row = rows[i];
            if(row.length !== size) {
                throw new Error(`Board row length should equal board height; expected width ${size}, was ${row.length}\n${board}`);
            } else if(!/^[bw\.#]+$/.test(row)) {
                throw new Error(`Board position has invalid character; expected only 'b', 'w', '.', '#'\n${board}`);
            }
        }
    }
    
    /**
     * Returns the player whose turn it is next ('b' or 'w').
     */
    public nextPlayer(): StoneColour {
        return this.board[0] as StoneColour;
    }
    
    /**
     * Returns the stone colour ('b' or 'w') at the given coordinates, or '.'
     * if there is no stone there.
     */
    public at(row: number, col: number): StoneColourOrEmpty {
        const size = this.size;
        if(row < 0 || row >= size || col < 0 || col >= this.size) {
            throw new Error(`Row or column index out of bounds for board size = ${size}; was row = ${row}, col = ${col}`);
        }
        
        // The board string begins with one character for the next player, then
        // a newline character. Each row is (size + 1) characters long because
        // of the newline.
        const index = 2 + row * (size + 1) + col;
        return this.board[index] as StoneColourOrEmpty;
    }
}
