type StoneColour = 'b' | 'w';
type StoneColourOrEmpty = StoneColour | '.';
/**
 * A board position.
 */
declare class Board {
    /**
     * Represents the board position as a multi-line string.
     */
    readonly board: string;
    /**
     * The board size. The board is square, so this is the dimension on both
     * sides.
     */
    readonly size: number;
    constructor(board: string);
    /**
     * Returns the player whose turn it is next ('b' or 'w').
     */
    nextPlayer(): StoneColour;
    /**
     * Returns the stone colour ('b' or 'w') at the given coordinates, or '.'
     * if there is no stone there.
     */
    at(row: number, col: number): StoneColourOrEmpty;
}
