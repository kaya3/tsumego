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
