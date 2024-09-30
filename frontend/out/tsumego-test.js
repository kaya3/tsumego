"use strict";
Punyt.test(class BoardTest {
    board = `b
wb...
.b...
.b...
bb...
.....`;
    testSize() {
        const board = new Board(this.board);
        Assert.equal(5, board.size, 'Board size should be 5');
    }
    testNextPlayer() {
        const board = new Board(this.board);
        Assert.equal('b', board.nextPlayer(), 'Next player is black');
    }
    testStoneAt() {
        const board = new Board(this.board);
        Assert.equal('w', board.at(0, 0), 'White stone at A1');
        Assert.equal('b', board.at(0, 1), 'Black stone at B1');
        Assert.equal('.', board.at(1, 0), 'Empty space at A2');
    }
    testIllegalOccupied() {
        const board = new Board(this.board);
        Assert.isFalse(board.isLegal(0, 0), 'Point occupied by white stone is an illegal move');
        Assert.isFalse(board.isLegal(0, 1), 'Point occupied by black stone is an illegal move');
        Assert.isTrue(board.isLegal(1, 0), 'Empty space is a legal move');
        Assert.isTrue(board.isLegal(4, 2), 'Empty space is a legal move');
        Assert.throwsLike(() => board.play(0, 0), e => e instanceof IllegalMove && e.reason === 'occupied', 'Playing at an occupied position should throw');
    }
    testAtOutOfBounds() {
        const board = new Board(this.board);
        Assert.throws(() => {
            board.at(-1, 0);
        }, 'Negative row number is out of bounds');
        Assert.throws(() => {
            board.at(5, 0);
        }, 'Large row number is out of bounds');
        Assert.throws(() => {
            board.at(0, -1);
        }, 'Negative column number is out of bounds');
        Assert.throws(() => {
            board.at(0, 5);
        }, 'Large column number is out of bounds');
    }
    testRectangularBoard() {
        Assert.throws(() => {
            new Board(`b
.....
.....
.....`);
        }, 'Non-square board is invalid');
    }
    testUnevenRows() {
        Assert.throws(() => {
            new Board(`b
.....
....
.....
.....
.....`);
        }, 'Uneven board is invalid');
    }
    testInvalidCharacter() {
        Assert.throws(() => {
            new Board(`b
.....
..q..
.....
.....
.....`);
        }, 'Uneven board is invalid');
    }
    testInvalidNextPlayer() {
        Assert.throws(() => {
            new Board(`q
.....
.....
.....
.....
.....`);
        }, 'Next player must be b or w');
    }
    testPlay() {
        const board = new Board(this.board);
        Assert.isTrue(board.isLegal(0, 2), 'Playing at an unoccupied point is legal');
        const newBoard = board.play(0, 2);
        Assert.equal('.', board.at(0, 2), 'Point at C1 was empty in original board');
        Assert.equal('b', newBoard.at(0, 2), 'Point at C1 is black stone in new board');
        Assert.equal('w', newBoard.nextPlayer(), 'White is next after black');
    }
    testCaptureOne() {
        const board = new Board(this.board);
        Assert.isTrue(board.isLegal(1, 0), 'Capturing one stone is legal');
        const newBoard = board.play(1, 0);
        Assert.equal('.', board.at(1, 0), 'Point at A2 was empty in original board');
        Assert.equal('b', newBoard.at(1, 0), 'Point at A2 is black stone in new board');
        Assert.equal('w', board.at(0, 0), 'Stone at A1 was white in original board');
        Assert.equal('.', newBoard.at(0, 0), 'Stone at A3 is removed in new board');
    }
    testCaptureMultiple() {
        const board = new Board(`b
.....
bbb..
www.w
bbbwb
...b.`);
        Assert.isTrue(board.isLegal(2, 3), 'Capturing multiple stones is legal');
        const newBoard = board.play(2, 3);
        const expectedPosition = `w
.....
bbb..
...bw
bbb.b
...b.`;
        Assert.equal(expectedPosition, newBoard.toString(), 'New position has white stones removed');
    }
    testKo() {
        const board = new Board(`b
.....
.bw..
bw.w.
.bw..
.....`);
        Assert.isTrue(board.isLegal(2, 2), 'Capturing a ko for the first time is legal');
        const newBoard = board.play(2, 2);
        Assert.equal('b', newBoard.at(2, 2), 'New position has a black stone where it was played');
        Assert.equal('#', newBoard.at(2, 1), 'New position has a ko ban where the white stone was captured');
        const expectedPosition = `w
.....
.bw..
b#bw.
.bw..
.....`;
        Assert.equal(expectedPosition, newBoard.toString(), 'New position has the new stone, and the captured stone is replaced with a ko ban');
    }
    testKoBan() {
        const board = new Board(`b
.....
.bw..
bw#w.
.bw..
.....`);
        Assert.isFalse(board.isLegal(2, 2), 'Recapturing a ko is illegal');
        Assert.throwsLike(() => board.play(2, 2), e => e instanceof IllegalMove && e.reason === 'ko recapture', 'Recapturing a ko throws an exception');
        const newBoard = board.play(0, 4);
        Assert.equal('.', newBoard.at(2, 2), 'Playing elsewhere clears ko ban');
        Assert.isTrue(newBoard.isLegal(2, 2), 'Filling in the ko is legal');
    }
    testSelfCapture() {
        const board = new Board(`b
bw...
bw...
.w...
w.w..
.w...`);
        Assert.isFalse(board.isLegal(3, 1), 'Single-stone self-capture is illegal');
        Assert.isFalse(board.isLegal(2, 0), 'Multi-stone self-capture is illegal');
        Assert.throwsLike(() => board.play(2, 0), e => e instanceof IllegalMove && e.reason === 'self-capture', 'Self-capture throws an exception');
    }
});
Punyt.test(class CoordinatesTest {
    testFromRowCol() {
        Assert.equal('A1', toCoordinates(0, 0), 'A1 is row 0, column 0');
        Assert.equal('C5', toCoordinates(4, 2), 'C5 is row 4, column 2');
        Assert.equal('K10', toCoordinates(9, 9), 'K10 is row 9, column 9');
    }
    testToRowCol() {
        Assert.shallowEqual([0, 0], fromCoordinates('A1'), 'A1 is row 0, column 0');
        Assert.shallowEqual([4, 2], fromCoordinates('C5'), 'C5 is row 4, column 2');
        Assert.shallowEqual([9, 9], fromCoordinates('K10'), 'K10 is row 9, column 9');
    }
    testRoundTrip() {
        for (let row = 0; row < 25; ++row) {
            for (let col = 0; col < 25; ++col) {
                const coords = toCoordinates(row, col);
                Assert.shallowEqual([row, col], fromCoordinates(coords), `Round trip for row = ${row}, col = ${col}`);
            }
        }
    }
});
Punyt.test(class TsumegoTest {
    board = `b
.bw..
.bw..
.bw..
bbw..
www..`;
    json = `{
        "board": ${JSON.stringify(this.board)},
        "tree": {
            "A1": {"A2": "lose"},
            "A2": "win",
            "A3": {"A2": "lose"}
        }
    }`;
    testFromJSON() {
        const tsumego = Tsumego.fromJSON(this.json);
        Assert.equal(this.board, tsumego.board.toString(), 'Parsed tsumego should have the same board');
        Assert.isFalse(tsumego.isComplete(), 'Parsed tsumego is not won or lost yet');
        Assert.isFalse(tsumego.isWon(), 'Parsed tsumego is not won yet');
    }
    testPlay() {
        const tsumego = Tsumego.fromJSON(this.json);
        const newTsumego = tsumego.play(0, 0);
        const expectedBoard = `w
bbw..
.bw..
.bw..
bbw..
www..`;
        Assert.equal(expectedBoard, newTsumego.board.toString(), 'Playing a move in a tsumego should update the board state');
        Assert.isFalse(newTsumego.isComplete(), 'Tsumego is not won or lost yet after playing at A1');
        Assert.isFalse(newTsumego.isWon(), 'Tsumego is not won yet after playing at A1');
        const expectedFinalBoard = `b
bbw..
wbw..
.bw..
bbw..
www..`;
        // This tsumego has only one possible "random" move from this position
        const finalTsumego = newTsumego.playRandom();
        Assert.equal(expectedFinalBoard, finalTsumego.board.toString(), 'Playing a random move in a tsumego should update the board state');
        Assert.isTrue(finalTsumego.isComplete(), 'Tsumego is complete after playing A1 then A2');
        Assert.isFalse(finalTsumego.isWon(), 'Tsumego is lost after playing A1 then A2');
    }
    testWin() {
        const tsumego = Tsumego.fromJSON(this.json);
        const newTsumego = tsumego.play(1, 0);
        const expectedBoard = `w
.bw..
bbw..
.bw..
bbw..
www..`;
        Assert.equal(expectedBoard, newTsumego.board.toString(), 'Playing a move in a tsumego should update the board state');
        Assert.isTrue(newTsumego.isComplete(), 'Tsumego is complete after playing A2');
        Assert.isTrue(newTsumego.isWon(), 'Tsumego is won after playing A2');
    }
    testInvalidBoard() {
        Assert.throws(() => {
            Tsumego.fromJSON(`{"tree": "win"}`);
        }, 'Missing board position should throw');
        Assert.throws(() => {
            Tsumego.fromJSON(`{"board", "foo bar", "tree": "win"}`);
        }, 'Invalid board position should throw');
    }
    testInvalidTree() {
        Assert.throws(() => {
            Tsumego.fromJSON(`{"board": ${JSON.stringify(this.board)}}`);
        }, 'Missing tree should throw');
        Assert.throws(() => {
            Tsumego.fromJSON(`{
                "board": ${JSON.stringify(this.board)},
                "tree": "foo bar",
            }`);
        }, 'Invalid tree type should throw');
        Assert.throws(() => {
            Tsumego.fromJSON(`{
                "board": ${JSON.stringify(this.board)},
                "tree": {"A0": "win"},
            }`);
        }, 'Invalid coordinates in tree should throw');
        Assert.throws(() => {
            Tsumego.fromJSON(`{
                "board": ${JSON.stringify(this.board)},
                "tree": {"B1": "lose"},
            }`);
        }, 'Illegal move in tree should throw');
        Assert.throws(() => {
            Tsumego.fromJSON(`{
                "board": ${JSON.stringify(this.board)},
                "tree": {"A1": "foo bar"},
            }`);
        }, 'Invalid child tree should throw');
        Assert.throws(() => {
            Tsumego.fromJSON(`{
                "board": ${JSON.stringify(this.board)},
                "tree": {"A1": {}},
            }`);
        }, 'Empty child tree should throw');
    }
});
