/**
 * Converts row and column indices to a string like `'A4'`. The letter
 * represents the column and the number represents the row. By convention, the
 * top-left of the board is `'A1'`, and there is no 'I' column.
 */
function toCoordinates(row: number, col: number): string {
    if(row < 0 || row >= 25 || col < 0 || col >= 25) {
        throw new Error(`Row or column indices must be between 0 and 24; was row = ${row}, col = ${col}`);
    }
    
    // No 'I' column, by convention
    const alphabet = 'ABCDEFGHJKLMNOPQRSTUVWXYZ';
    return `${alphabet[col]}${row + 1}`;
}

/**
 * Parses a coordinate string like `'A4'` to row and column indices.
 */
function fromCoordinates(coordinates: string): [row: number, col: number] {
    // 65 is 'A'
    let col = coordinates.charCodeAt(0) - 65;
    // Correct for missing 'I' column
    if(col >= 9) { col--; }
    
    const row = parseInt(coordinates.substring(1)) - 1;
    
    return [row, col];
}