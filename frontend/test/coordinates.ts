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
        for(let row = 0; row < 25; ++row) {
            for(let col = 0; col < 25; ++col) {
                const coords = toCoordinates(row, col);
                Assert.shallowEqual([row, col], fromCoordinates(coords), `Round trip for row = ${row}, col = ${col}`);
            }
        }
    }
});