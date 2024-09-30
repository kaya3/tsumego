"use strict";
class IllegalMove extends Error {
    board;
    row;
    col;
    reason;
    constructor(board, row, col, reason) {
        super(`Illegal move at row = ${row}, col = ${col}: ${reason}\n${board.toString()}`);
        this.board = board;
        this.row = row;
        this.col = col;
        this.reason = reason;
    }
}
/**
 * A board position.
 */
class Board {
    static empty(size) {
        if (size <= 1) {
            throw new Error(`Board size must be at least 2, was ${size}`);
        }
        const row = '.'.repeat(size);
        const board = 'b' + `\n${row}`.repeat(size);
        return new Board(board);
    }
    /**
     * Represents the board position as a multi-line string.
     */
    board;
    /**
     * The board size. The board is square, so this is the dimension on both
     * sides.
     */
    size;
    constructor(board) {
        this.board = board;
        const rows = board.split('\n');
        const size = this.size = rows.length - 1;
        if (rows[0] !== 'b' && rows[0] !== 'w') {
            throw new Error(`Invalid next player; expected 'b' or 'w', was '${rows[0]}'`);
        }
        for (let i = 1; i < rows.length; ++i) {
            const row = rows[i];
            if (row.length !== size) {
                throw new Error(`Board row length should equal board height; expected width ${size}, was ${row.length}\n${board}`);
            }
            else if (!/^[bw\.#]+$/.test(row)) {
                throw new Error(`Board position has invalid character; expected only 'b', 'w', '.', '#'\n${board}`);
            }
        }
    }
    toString() {
        return this.board;
    }
    /**
     * Returns the player whose turn it is next ('b' or 'w').
     */
    nextPlayer() {
        return this.board[0];
    }
    /**
     * Returns what is on the board at the given coordinates: either the stone
     * colour ('b' or 'w'), or '.' for an empty space, or '#' for an empty
     * space which is an illegal ko recapture.
     */
    at(row, col) {
        return this.board[this.index(row, col)];
    }
    /**
     * Returns a boolean indicating whether it is legal for the current player
     * to play at the given coordinates.
     */
    isLegal(row, col) {
        // Move is illegal if it is out of bounds
        if (row < 0 || row >= this.size || col < 0 || col >= this.size) {
            return false;
        }
        // Move is illegal if there is already a stone there, or a ko ban '#'
        if (this.at(row, col) !== '.') {
            return false;
        }
        try {
            this.play(row, col);
        }
        catch (e) {
            // Rethrow other types of exception
            if (!(e instanceof IllegalMove)) {
                throw e;
            }
            // If an `IllegalMove` exception is thrown, the move is illegal
            return false;
        }
        // Otherwise, the move is legal
        return true;
    }
    play(row, col) {
        // Move is illegal if there is already a stone there, or a ko ban '#'
        const there = this.at(row, col);
        if (there !== '.') {
            const reason = there === '#' ? 'ko recapture' : 'occupied';
            throw new IllegalMove(this, row, col, reason);
        }
        const colour = this.nextPlayer();
        const opponent = colour === 'b' ? 'w' : 'b';
        // Build the new board as a mutable array. Replace ko bans '#' with
        // empty spaces, since ko bans only last for one turn.
        const board = [...this.board.replace('#', '.')];
        // The opponent is the next player in the new position
        board[0] = opponent;
        const index = this.index(row, col);
        board[index] = colour;
        let captures = 0;
        for (const neighbour of this.neighbours(index)) {
            if (board[neighbour] === opponent) {
                captures += this.removeCaptures(board, neighbour, opponent);
            }
        }
        // A move which captures no opponent's stones, and leaves a chain of
        // your own stones with no liberties, is an illegal self-capture.
        if (captures === 0 && this.removeCaptures(board, index, colour) > 0) {
            throw new IllegalMove(this, row, col, 'self-capture');
        }
        // Ko rule: a move is a ko capture if it captures one stone, and the
        // new stone placed is not part of a chain, and has exactly one
        // liberty. If this is a ko capture, need to mark the captured stone
        // as an illegal move in the new position.
        if (captures === 1) {
            this.markKoBan(board, index, colour);
        }
        return new Board(board.join(''));
    }
    /**
     * Converts coordinates to an index into the board string.
     */
    index(row, col) {
        const size = this.size;
        if (row < 0 || row >= size || col < 0 || col >= this.size) {
            throw new Error(`Row or column index out of bounds for board size = ${size}; was row = ${row}, col = ${col}`);
        }
        // The board string begins with one character for the next player, then
        // a newline character. Each row is (size + 1) characters long because
        // of the newline.
        return 2 + row * (size + 1) + col;
    }
    neighbours(index) {
        const size = this.size;
        // Invert index calculation
        const row = Math.floor((index - 2) / (size + 1));
        const col = (index - 2) % (size + 1);
        const neighbours = [];
        if (row > 0) {
            neighbours.push(this.index(row - 1, col));
        }
        if (row < size - 1) {
            neighbours.push(this.index(row + 1, col));
        }
        if (col > 0) {
            neighbours.push(this.index(row, col - 1));
        }
        if (col < size - 1) {
            neighbours.push(this.index(row, col + 1));
        }
        return neighbours;
    }
    /**
     * Removes captured stones of the given `colour` from the `board`, starting
     * at `index`. Returns the number of stones captured.
     */
    removeCaptures(board, index, colour) {
        const stack = [index];
        const seen = new Set(stack);
        while (stack.length > 0) {
            index = stack.pop();
            for (const neighbour of this.neighbours(index)) {
                if (seen.has(neighbour)) {
                    continue;
                }
                const there = board[neighbour];
                if (there === colour) {
                    stack.push(neighbour);
                    seen.add(neighbour);
                }
                else if (there === '.' || there === '#') {
                    // The chain has at least one liberty, so it is not captured
                    return 0;
                }
            }
        }
        for (const removedIndex of seen) {
            board[removedIndex] = '.';
        }
        return seen.size;
    }
    /**
     * Checks whether the stone placed at `index` is a ko capture, and if so,
     * labels the ko recapture as an illegal move in the next board position.
     */
    markKoBan(board, index, colour) {
        let libertyIndex = -1;
        for (const neighbour of this.neighbours(index)) {
            const there = board[neighbour];
            if (there === colour) {
                // The placed stone is part of a chain, so this is not a ko capture
                return;
            }
            else if (there === '.' || there === '#') {
                if (libertyIndex >= 0) {
                    // The placed stone has two or more liberties, so this is not a ko capture
                    return;
                }
                libertyIndex = neighbour;
            }
        }
        if (libertyIndex >= 0) {
            board[libertyIndex] = '#';
        }
    }
}
class BoardView {
    static CANVAS_SIZE = 500;
    static BOARD_COLOUR = '#907040';
    static BLACK_STONE_COLOUR = '#202028';
    static WHITE_STONE_COLOUR = '#fffff0';
    static LINE_COLOUR = 'black';
    static LINE_THICKNESS = 2;
    static HOVERED_ALPHA = 0.5;
    static KO_BAN_SIZE = 2 / 3;
    canvas;
    ctx;
    // These are initialised by the call to `setBoard`
    board;
    cellSize;
    halfCellSize;
    starPointSize;
    hoveredRow = -1;
    hoveredCol = -1;
    constructor(board) {
        this.setBoard(board);
        const canvas = this.canvas = document.createElement('canvas');
        canvas.width = BoardView.CANVAS_SIZE;
        canvas.height = BoardView.CANVAS_SIZE;
        this.ctx = canvas.getContext('2d');
        const onHover = (e) => {
            const [row, col] = this.fromXY(e);
            if (this.board.isLegal(row, col)) {
                this.hoveredRow = row;
                this.hoveredCol = col;
            }
            else {
                this.hoveredRow = -1;
                this.hoveredCol = -1;
            }
        };
        const unHover = (e) => {
            this.hoveredRow = -1;
            this.hoveredCol = -1;
        };
        canvas.addEventListener('mouseenter', onHover);
        canvas.addEventListener('mousemove', onHover);
        canvas.addEventListener('mouseleave', unHover);
        canvas.addEventListener('click', (e) => {
            onHover(e);
            if (this.hoveredRow >= 0 && this.hoveredCol >= 0) {
                this.setBoard(this.board.play(this.hoveredRow, this.hoveredCol));
                unHover(e);
            }
        });
    }
    setBoard(board) {
        this.board = board;
        const size = board.size;
        const cellSize = Math.floor(BoardView.CANVAS_SIZE / (size + 1));
        this.cellSize = cellSize;
        this.halfCellSize = Math.floor(cellSize / 2);
        this.starPointSize = Math.floor(cellSize / 12);
    }
    draw() {
        const { ctx, board, halfCellSize, starPointSize } = this;
        ctx.clearRect(0, 0, BoardView.CANVAS_SIZE, BoardView.CANVAS_SIZE);
        // Board colour
        ctx.fillStyle = BoardView.BOARD_COLOUR;
        ctx.fillRect(0, 0, BoardView.CANVAS_SIZE, BoardView.CANVAS_SIZE);
        // Board lines
        ctx.strokeStyle = `${BoardView.LINE_THICKNESS}px solid ${BoardView.LINE_COLOUR}`;
        ctx.beginPath();
        const [start, end] = this.xy(0, board.size - 1);
        for (let i = 0; i < board.size; ++i) {
            const [offset, _] = this.xy(i, i);
            // Horizontal line
            ctx.moveTo(start, offset);
            ctx.lineTo(end, offset);
            // Vertical line
            ctx.moveTo(offset, start);
            ctx.lineTo(offset, end);
        }
        ctx.stroke();
        // Stones
        for (let row = 0; row < board.size; ++row) {
            for (let col = 0; col < board.size; ++col) {
                const isHovered = row === this.hoveredRow && col === this.hoveredCol;
                const there = board.at(row, col);
                const [x, y] = this.xy(row, col);
                const isStone = there === 'b' || there === 'w';
                // Draw a star point if there is one
                if (!isStone && BoardView.isStarPoint(board.size, row, col)) {
                    ctx.fillStyle = BoardView.LINE_COLOUR;
                    ctx.beginPath();
                    ctx.ellipse(x, y, starPointSize, starPointSize, 0, 0, Math.PI * 2);
                    ctx.fill();
                }
                // Draw a square to indicate ko ban, if there is one
                if (there === '#') {
                    const r = Math.floor(halfCellSize * BoardView.KO_BAN_SIZE);
                    ctx.strokeRect(x - r, y - r, r * 2, r * 2);
                }
                // Draw a stone if there is one
                if (isStone || isHovered) {
                    const colour = isHovered ? board.nextPlayer() : there;
                    ctx.fillStyle = colour === 'b' ? BoardView.BLACK_STONE_COLOUR : BoardView.WHITE_STONE_COLOUR;
                    if (isHovered) {
                        ctx.globalAlpha = BoardView.HOVERED_ALPHA;
                    }
                    ctx.beginPath();
                    ctx.ellipse(x, y, halfCellSize - BoardView.LINE_THICKNESS / 2, halfCellSize - BoardView.LINE_THICKNESS / 2, 0, 0, Math.PI * 2);
                    ctx.fill();
                    ctx.stroke();
                    if (isHovered) {
                        ctx.globalAlpha = 1;
                    }
                }
            }
        }
    }
    xy(row, col) {
        return [
            (col + 1) * this.cellSize,
            (row + 1) * this.cellSize,
        ];
    }
    fromXY(e) {
        return [
            Math.round(e.offsetY / this.cellSize) - 1,
            Math.round(e.offsetX / this.cellSize) - 1,
        ];
    }
    static isStarPoint(size, row, col) {
        // No star points on small boards
        if (size < 5) {
            return false;
        }
        // Star points in the middle for odd-sized boards
        const half = size % 2 === 1 ? Math.floor(size / 2) : -1;
        if (size < 9) {
            // only tengen
            return row === half && col === half;
        }
        // Corner and side star points at 3-3 for smaller boards, and 4-4 for larger boards
        const edge = size < 13 ? 2 : 3;
        const isRowEdge = row === edge || row === size - edge - 1;
        const isColEdge = col === edge || col === size - edge - 1;
        // Corners and tengen
        if (isRowEdge && isColEdge || row === half && col === half) {
            return true;
        }
        // Star points on the sides for larger boards
        if (size >= 13 && (isRowEdge && col === half || isColEdge && row === half)) {
            return true;
        }
        return false;
    }
}
