"use strict";
Punyt.test(class BoardTest {
    board = `b
wb...
.b...
.b...
bb...
.....`;
    testBasic() {
        const board = new Board(this.board);
        Assert.equal(5, board.size, 'Board size should be 5');
        Assert.equal('b', board.nextPlayer(), 'Next player is black');
    }
    testStoneAt() {
        const board = new Board(this.board);
        Assert.equal('w', board.at(0, 0), 'White stone at A1');
        Assert.equal('b', board.at(0, 1), 'Black stone at B1');
        Assert.equal('.', board.at(1, 0), 'Empty space at A2');
    }
});
