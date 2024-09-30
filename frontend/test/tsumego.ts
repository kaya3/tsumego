Punyt.test(class TsumegoTest {
    private readonly board = `b
.bw..
.bw..
.bw..
bbw..
www..`;
    
    private readonly json = `{
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