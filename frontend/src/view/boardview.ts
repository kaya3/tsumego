class BoardView {
    public static readonly CANVAS_SIZE = 500;
    public static readonly BOARD_COLOUR = '#907040';
    public static readonly BLACK_STONE_COLOUR = '#202028';
    public static readonly WHITE_STONE_COLOUR = '#fffff0';
    public static readonly LINE_COLOUR = 'black';
    public static readonly LINE_THICKNESS = '2px';
    
    private board: Board;
    
    public readonly canvas: HTMLCanvasElement;
    private readonly ctx: CanvasRenderingContext2D;
    
    public constructor(board: Board) {
        this.board = board;
        
        const canvas = this.canvas = document.createElement('canvas');
        canvas.width = BoardView.CANVAS_SIZE;
        canvas.height = BoardView.CANVAS_SIZE;
        this.ctx = canvas.getContext('2d')!;
    }
    
    public draw(): void {
        const board = this.board;
        const size = board.size;
        const cellSize = Math.floor(BoardView.CANVAS_SIZE / size);
        const halfCellSize = Math.floor(cellSize / 2);
        const starPointSize = Math.floor(cellSize / 12);
        
        function xy(row: number, col: number): [x: number, y: number] {
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
        for(let i = 0; i < size; ++i) {
            // Horizontal line
            ctx.moveTo(...xy(i, 0));
            ctx.lineTo(...xy(i, size - 1));
            
            // Vertical line
            ctx.moveTo(...xy(0, i));
            ctx.lineTo(...xy(size - 1, i));
        }
        ctx.stroke();
        
        for(let row = 0; row < size; ++row) {
            for(let col = 0; col < size; ++col) {
                const there = board.at(row, col);
                const [x, y] = xy(row, col);
                if(there === 'b' || there === 'w') {
                    ctx.fillStyle = there === 'b' ? BoardView.BLACK_STONE_COLOUR : BoardView.WHITE_STONE_COLOUR;
                    ctx.beginPath();
                    ctx.ellipse(x, y, halfCellSize, halfCellSize, 0, 0, Math.PI * 2);
                    ctx.fill();
                    ctx.stroke();
                } else {
                    if(BoardView.isStarPoint(size, row, col)) {
                        ctx.fillStyle = BoardView.LINE_COLOUR;
                        ctx.beginPath();
                        ctx.ellipse(x, y, starPointSize, starPointSize, 0, 0, Math.PI * 2);
                        ctx.fill();
                    }
                }
            }
        }
    }
    
    private static isStarPoint(size: number, row: number, col: number): boolean {
        // No star points on small boards
        if(size < 5) {
            return false;
        }
        
        // Star points in the middle for odd-sized boards
        const half = size % 2 === 1 ? Math.floor(size / 2) : -1;
        if(size < 9) {
            // only tengen
            return row === half && col === half;
        }
        
        // Corner and side star points at 3-3 for smaller boards, and 4-4 for larger boards
        const edge = size < 13 ? 2 : 3;
        
        const isRowEdge = row === edge || row === size - edge - 1;
        const isColEdge = col === edge || col === size - edge - 1;
        
        // Corners and tengen
        if(isRowEdge && isColEdge || row === half && col === half) {
            return true;
        }
        
        // Star points on the sides for larger boards
        if(size >= 13 && (isRowEdge && col === half || isColEdge && row === half)) {
            return true;
        }
        
        return false;
    }
}
