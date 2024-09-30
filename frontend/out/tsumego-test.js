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
    testLegalUnoccupied() {
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
