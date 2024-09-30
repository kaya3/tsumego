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
 * A board position, including the stones on the board, the next colour to
 * play, and the state of any ko.
 */
declare class Board {
    /**
     * Constructs an empty board position of the given size. Black will be the
     * first player.
     */
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
    private isInBounds;
    /**
     * Returns the indices of all points which are adjacent to the point given
     * by `index`.
     */
    private neighbours;
    /**
     * Removes captured stones of the given `colour` from the `board`, starting
     * at `index`. Returns the number of stones captured, which may be zero if
     * the chain is not captured.
     */
    private removeCaptures;
    /**
     * Checks whether the stone placed at `index` is a ko capture, and if so,
     * labels the ko recapture as an illegal move in the next board position.
     */
    private markKoBan;
}
/**
 * Converts row and column indices to a string like `'A4'`. The letter
 * represents the column and the number represents the row. By convention, the
 * top-left of the board is `'A1'`, and there is no 'I' column.
 */
declare function toCoordinates(row: number, col: number): string;
/**
 * Parses a coordinate string like `'A4'` to row and column indices.
 */
declare function fromCoordinates(coordinates: string): [row: number, col: number];
type VariationTree = 'win' | 'lose' | {
    readonly [moveCoordinates: string]: VariationTree;
};
declare class Tsumego {
    readonly board: Board;
    private readonly tree;
    static fromJSON(json: string): Tsumego;
    private static validateTree;
    private constructor();
    isComplete(): boolean;
    isWon(): boolean;
    play(row: number, col: number): Tsumego;
    playRandom(): Tsumego;
}
declare class BoardView {
    static readonly CANVAS_SIZE = 500;
    static readonly BOARD_COLOUR = "#907040";
    static readonly BLACK_STONE_COLOUR = "#202028";
    static readonly WHITE_STONE_COLOUR = "#fffff0";
    static readonly LINE_COLOUR = "black";
    static readonly LINE_THICKNESS = 2;
    static readonly HOVERED_ALPHA = 0.5;
    static readonly STAR_POINT_SIZE: number;
    static readonly KO_BAN_SIZE: number;
    readonly canvas: HTMLCanvasElement;
    private readonly ctx;
    private board;
    private cellSize;
    /**
     * Determines whether the board view accepts click events to play stones.
     * Also controls whether hovered stones are drawn.
     */
    playEnabled: boolean;
    private hoveredRow;
    private hoveredCol;
    constructor(board: Board);
    onPlay(callback: (row: number, col: number) => void): void;
    private onHover;
    private onMouseLeave;
    setBoard(board: Board): void;
    draw(): void;
    /**
     * Returns the (x, y) coordinates of the centre of the given point,
     * relative to the canvas origin.
     */
    private xy;
    /**
     * Returns the (row, col) coordinates of the point which contains the mouse
     * cursor. They may be out of bounds of the current board.
     */
    private fromXY;
    private static isStarPoint;
}
declare class TsumegoView extends BoardView {
    tsumego: Tsumego;
    /**
     * Wait this long, in milliseconds, between the player's move and the
     * opponent's response.
     */
    moveDelay: number;
    private readonly onCompleteCallbacks;
    constructor(tsumego: Tsumego);
    setTsumego(tsumego: Tsumego): void;
    onComplete(callback: (win: boolean) => void): void;
}
