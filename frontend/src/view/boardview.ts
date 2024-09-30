class BoardView {
    public static readonly CANVAS_SIZE = 500;
    public static readonly BOARD_COLOUR = '#907040';
    public static readonly BLACK_STONE_COLOUR = '#202028';
    public static readonly WHITE_STONE_COLOUR = '#fffff0';
    public static readonly LINE_COLOUR = 'black';
    public static readonly LINE_THICKNESS = 2;
    public static readonly HOVERED_ALPHA = 0.5;
    public static readonly KO_BAN_SIZE = 2/3;
    
    public readonly canvas: HTMLCanvasElement;
    private readonly ctx: CanvasRenderingContext2D;
    
    // These are initialised by the call to `setBoard`
    private board!: Board;
    private cellSize!: number;
    private halfCellSize!: number;
    private starPointSize!: number;
    
    private hoveredRow: number = -1;
    private hoveredCol: number = -1;
    
    public constructor(board: Board) {
        this.setBoard(board);
        
        const canvas = this.canvas = document.createElement('canvas');
        canvas.width = BoardView.CANVAS_SIZE;
        canvas.height = BoardView.CANVAS_SIZE;
        this.ctx = canvas.getContext('2d')!;
        
        const onHover = (e: MouseEvent): void => {
            const [row, col]  = this.fromXY(e);
            if(this.board.isLegal(row, col)) {
                this.hoveredRow = row;
                this.hoveredCol = col;
            } else {
                this.hoveredRow = -1;
                this.hoveredCol = -1;
            }
        };
        const unHover = (e: MouseEvent): void => {
            this.hoveredRow = -1;
            this.hoveredCol = -1;
        };
        canvas.addEventListener('mouseenter', onHover);
        canvas.addEventListener('mousemove', onHover);
        canvas.addEventListener('mouseleave', unHover);
        
        canvas.addEventListener('click', (e: MouseEvent) => {
            onHover(e);
            if(this.hoveredRow >= 0 && this.hoveredCol >= 0) {
                this.setBoard(this.board.play(this.hoveredRow, this.hoveredCol));
                unHover(e);
            }
        });
    }
    
    public setBoard(board: Board): void {
        this.board = board;
        
        const size = board.size;
        const cellSize = Math.floor(BoardView.CANVAS_SIZE / (size + 1));
        this.cellSize = cellSize;
        this.halfCellSize = Math.floor(cellSize / 2);
        this.starPointSize = Math.floor(cellSize / 12);
    }
    
    public draw(): void {
        const {ctx, board, halfCellSize, starPointSize} = this;
        
        ctx.clearRect(0, 0, BoardView.CANVAS_SIZE, BoardView.CANVAS_SIZE);
        
        // Board colour
        ctx.fillStyle = BoardView.BOARD_COLOUR;
        ctx.fillRect(0, 0, BoardView.CANVAS_SIZE, BoardView.CANVAS_SIZE);
        
        // Board lines
        ctx.strokeStyle = `${BoardView.LINE_THICKNESS}px solid ${BoardView.LINE_COLOUR}`;
        ctx.beginPath();
        const [start, end] = this.xy(0, board.size - 1);
        for(let i = 0; i < board.size; ++i) {
            const [offset, _] = this.xy(i, i)
            
            // Horizontal line
            ctx.moveTo(start, offset);
            ctx.lineTo(end, offset);
            
            // Vertical line
            ctx.moveTo(offset, start);
            ctx.lineTo(offset, end);
        }
        ctx.stroke();
        
        // Stones
        for(let row = 0; row < board.size; ++row) {
            for(let col = 0; col < board.size; ++col) {
                const isHovered = row === this.hoveredRow && col === this.hoveredCol;
                
                const there = board.at(row, col);
                const [x, y] = this.xy(row, col);
                
                const isStone = there === 'b' || there === 'w';
                
                // Draw a star point if there is one
                if(!isStone && BoardView.isStarPoint(board.size, row, col)) {
                    ctx.fillStyle = BoardView.LINE_COLOUR;
                    ctx.beginPath();
                    ctx.ellipse(x, y, starPointSize, starPointSize, 0, 0, Math.PI * 2);
                    ctx.fill();
                }
                
                // Draw a square to indicate ko ban, if there is one
                if(there === '#') {
                    const r = Math.floor(halfCellSize * BoardView.KO_BAN_SIZE);
                    ctx.strokeRect(x - r, y - r, r * 2, r * 2);
                }
                
                // Draw a stone if there is one
                if(isStone || isHovered) {
                    const colour = isHovered ? board.nextPlayer() : there;
                    ctx.fillStyle = colour === 'b' ? BoardView.BLACK_STONE_COLOUR : BoardView.WHITE_STONE_COLOUR;
                    if(isHovered) { ctx.globalAlpha = BoardView.HOVERED_ALPHA; }
                    ctx.beginPath();
                    ctx.ellipse(x, y, halfCellSize - BoardView.LINE_THICKNESS / 2, halfCellSize - BoardView.LINE_THICKNESS / 2, 0, 0, Math.PI * 2);
                    ctx.fill();
                    ctx.stroke();
                    if(isHovered) { ctx.globalAlpha = 1; }
                }
            }
        }
    }
    
    private xy(row: number, col: number): [x: number, y: number] {
        return [
            (col + 1) * this.cellSize,
            (row + 1) * this.cellSize,
        ];
    }
    
    private fromXY(e: MouseEvent): [row: number, col: number] {
        return [
            Math.round(e.offsetY / this.cellSize) - 1,
            Math.round(e.offsetX / this.cellSize) - 1,
        ];
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
