type StoneColour = 'b' | 'w';
type StoneColourOrEmpty = StoneColour | '.' | '#';
declare class IllegalMove extends Error {
    readonly board: Board;
    readonly row: number;
    readonly col: number;
    readonly reason: 'occupied' | 'ko recapture' | 'self-capture';
    constructor(board: Board, row: number, col: number, reason: 'occupied' | 'ko recapture' | 'self-capture');
}
/**
 * A board position.
 */
declare class Board {
    static empty(size: number): Board;
    /**
     * Represents the board position as a multi-line string.
     */
    private readonly board;
    /**
     * The board size. The board is square, so this is the dimension on both
     * sides.
     */
    readonly size: number;
    constructor(board: string);
    toString(): string;
    /**
     * Returns the player whose turn it is next ('b' or 'w').
     */
    nextPlayer(): StoneColour;
    /**
     * Returns what is on the board at the given coordinates: either the stone
     * colour ('b' or 'w'), or '.' for an empty space, or '#' for an empty
     * space which is an illegal ko recapture.
     */
    at(row: number, col: number): StoneColourOrEmpty;
    /**
     * Returns a boolean indicating whether it is legal for the current player
     * to play at the given coordinates.
     */
    isLegal(row: number, col: number): boolean;
    play(row: number, col: number): Board;
    /**
     * Converts coordinates to an index into the board string.
     */
    private index;
    private neighbours;
    /**
     * Removes captured stones of the given `colour` from the `board`, starting
     * at `index`. Returns the number of stones captured.
     */
    private removeCaptures;
    /**
     * Checks whether the stone placed at `index` is a ko capture, and if so,
     * labels the ko recapture as an illegal move in the next board position.
     */
    private markKoBan;
}
declare class BoardView {
    static readonly CANVAS_SIZE = 500;
    static readonly BOARD_COLOUR = "#907040";
    static readonly BLACK_STONE_COLOUR = "#202028";
    static readonly WHITE_STONE_COLOUR = "#fffff0";
    static readonly LINE_COLOUR = "black";
    static readonly LINE_THICKNESS = "2px";
    private board;
    readonly canvas: HTMLCanvasElement;
    private readonly ctx;
    constructor(board: Board);
    draw(): void;
    private static isStarPoint;
}
