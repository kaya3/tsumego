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
    static LINE_THICKNESS = '2px';
    board;
    canvas;
    ctx;
    constructor(board) {
        this.board = board;
        const canvas = this.canvas = document.createElement('canvas');
        canvas.width = BoardView.CANVAS_SIZE;
        canvas.height = BoardView.CANVAS_SIZE;
        this.ctx = canvas.getContext('2d');
    }
    draw() {
        const board = this.board;
        const size = board.size;
        const cellSize = Math.floor(BoardView.CANVAS_SIZE / size);
        const halfCellSize = Math.floor(cellSize / 2);
        const starPointSize = Math.floor(cellSize / 12);
        function xy(row, col) {
            return [
                col * cellSize + halfCellSize,
                row * cellSize + halfCellSize,
            ];
        }
        const ctx = this.ctx;
        ctx.clearRect(0, 0, BoardView.CANVAS_SIZE, BoardView.CANVAS_SIZE);
        ctx.fillStyle = BoardView.BOARD_COLOUR;
        ctx.fillRect(0, 0, BoardView.CANVAS_SIZE, BoardView.CANVAS_SIZE);
        ctx.strokeStyle = `${BoardView.LINE_THICKNESS} solid ${BoardView.LINE_COLOUR}`;
        ctx.beginPath();
        for (let i = 0; i < size; ++i) {
            // Horizontal line
            ctx.moveTo(...xy(i, 0));
            ctx.lineTo(...xy(i, size - 1));
            // Vertical line
            ctx.moveTo(...xy(0, i));
            ctx.lineTo(...xy(size - 1, i));
        }
        ctx.stroke();
        for (let row = 0; row < size; ++row) {
            for (let col = 0; col < size; ++col) {
                const there = board.at(row, col);
                const [x, y] = xy(row, col);
                if (there === 'b' || there === 'w') {
                    ctx.fillStyle = there === 'b' ? BoardView.BLACK_STONE_COLOUR : BoardView.WHITE_STONE_COLOUR;
                    ctx.beginPath();
                    ctx.ellipse(x, y, halfCellSize, halfCellSize, 0, 0, Math.PI * 2);
                    ctx.fill();
                    ctx.stroke();
                }
                else {
                    if (BoardView.isStarPoint(size, row, col)) {
                        ctx.fillStyle = BoardView.LINE_COLOUR;
                        ctx.beginPath();
                        ctx.ellipse(x, y, starPointSize, starPointSize, 0, 0, Math.PI * 2);
                        ctx.fill();
                    }
                }
            }
        }
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
