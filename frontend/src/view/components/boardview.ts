class BoardView {
    public static readonly CANVAS_SIZE = 500;
    public static readonly BOARD_COLOUR = '#907040';
    public static readonly BLACK_STONE_COLOUR = '#202028';
    public static readonly WHITE_STONE_COLOUR = '#fffff0';
    public static readonly LINE_COLOUR = 'black';
    public static readonly LINE_THICKNESS = 2;
    public static readonly HOVERED_ALPHA = 0.5;
    public static readonly STAR_POINT_SIZE = 1/12;
    public static readonly KO_BAN_SIZE = 1/3;
    
    public readonly canvas: HTMLCanvasElement;
    private readonly ctx: CanvasRenderingContext2D;
    
    // These are initialised by the call to `setBoard`
    private board!: Board;
    private cellSize!: number;
    
    /**
     * Determines whether the board view accepts click events to play stones.
     * Also controls whether hovered stones are drawn.
     */
    public playEnabled: boolean = false;
    
    private hoveredRow: number = -1;
    private hoveredCol: number = -1;
    
    private readonly onPlayCallbacks: ((row: number, col: number) => void)[] = [];
    
    public constructor(board: Board) {
        this.setBoard(board);
        
        const canvas = this.canvas = document.createElement('canvas');
        canvas.width = BoardView.CANVAS_SIZE;
        canvas.height = BoardView.CANVAS_SIZE;
        this.ctx = canvas.getContext('2d')
            ?? fail(`failed to get 2D canvas context`);
        
        this.canvas.addEventListener('click', e => this.handleClick(e));
        canvas.addEventListener('mouseenter', e => this.handleHover(e));
        canvas.addEventListener('mousemove', e => this.handleHover(e));
        canvas.addEventListener('mouseleave', e => this.handleMouseLeave());
    }
    
    /**
     * Registers a callback function to be called when a move is played on this
     * board. Only legal moves can be played, and only when play is enabled.
     */
    public onPlay(callback: (row: number, col: number) => void): void {
        this.onPlayCallbacks.push(callback);
    }
    
    private handleClick(e: MouseEvent): void {
        const [row, col] = this.fromXY(e);
        if(this.playEnabled && this.board.isLegal(row, col)) {
            for(const callback of this.onPlayCallbacks) {
                callback(row, col);
            }
        }
        
        this.handleHover(e);
    }
    
    private handleHover(e: MouseEvent): void {
        const [row, col] = this.fromXY(e);
        if(this.board.isLegal(row, col)) {
            this.hoveredRow = row;
            this.hoveredCol = col;
        } else {
            this.handleMouseLeave();
        }
    }
    
    private handleMouseLeave(): void {
        this.hoveredRow = -1;
        this.hoveredCol = -1;
    }
    
    public setBoard(board: Board): void {
        this.board = board;
        this.cellSize = Math.floor(BoardView.CANVAS_SIZE / (board.size + 1));
    }
    
    public draw(): void {
        const {ctx, board, cellSize} = this;
        
        ctx.clearRect(0, 0, BoardView.CANVAS_SIZE, BoardView.CANVAS_SIZE);
        
        // Board colour
        ctx.fillStyle = BoardView.BOARD_COLOUR;
        ctx.fillRect(0, 0, BoardView.CANVAS_SIZE, BoardView.CANVAS_SIZE);
        
        // Board lines
        ctx.strokeStyle = `${BoardView.LINE_THICKNESS}px solid ${BoardView.LINE_COLOUR}`;
        ctx.beginPath();
        const [start, end] = this.xy(0, board.size - 1);
        for(let i = 0; i < board.size; ++i) {
            const [offset, _] = this.xy(i, i);
            
            // Horizontal line
            ctx.moveTo(start, offset);
            ctx.lineTo(end, offset);
            
            // Vertical line
            ctx.moveTo(offset, start);
            ctx.lineTo(offset, end);
        }
        ctx.stroke();
        
        // Stones and decorations
        const stoneSize = (cellSize - BoardView.LINE_THICKNESS) / 2;
        const starPointSize = cellSize * BoardView.STAR_POINT_SIZE;
        
        for(let row = 0; row < board.size; ++row) {
            for(let col = 0; col < board.size; ++col) {
                const there = board.at(row, col);
                const isStone = there === 'b' || there === 'w';
                const isHovered = this.playEnabled && row === this.hoveredRow && col === this.hoveredCol;
                const [x, y] = this.xy(row, col);
                
                // Draw a star point if there is one
                if(!isStone && BoardView.isStarPoint(board.size, row, col)) {
                    ctx.fillStyle = BoardView.LINE_COLOUR;
                    ctx.beginPath();
                    ctx.ellipse(x, y, starPointSize, starPointSize, 0, 0, Math.PI * 2);
                    ctx.fill();
                }
                
                // Draw a square to indicate ko ban, if there is one
                if(there === '#') {
                    const koBanSize = Math.round(cellSize * BoardView.KO_BAN_SIZE);
                    ctx.strokeRect(x - koBanSize, y - koBanSize, koBanSize * 2, koBanSize * 2);
                }
                
                // Draw a stone if there is one
                if(isStone || isHovered) {
                    const colour = isHovered ? board.nextPlayer() : there;
                    ctx.fillStyle = colour === 'b' ? BoardView.BLACK_STONE_COLOUR : BoardView.WHITE_STONE_COLOUR;
                    
                    if(isHovered) { ctx.globalAlpha = BoardView.HOVERED_ALPHA; }
                    ctx.beginPath();
                    ctx.ellipse(x, y, stoneSize, stoneSize, 0, 0, Math.PI * 2);
                    ctx.fill();
                    ctx.stroke();
                    if(isHovered) { ctx.globalAlpha = 1; }
                }
            }
        }
    }
    
    /**
     * Returns the (x, y) coordinates of the centre of the given point,
     * relative to the canvas origin.
     */
    private xy(row: number, col: number): [x: number, y: number] {
        return [
            (col + 1) * this.cellSize,
            (row + 1) * this.cellSize,
        ];
    }
    
    /**
     * Returns the (row, col) coordinates of the point which contains the mouse
     * cursor. They may be out of bounds of the current board.
     */
    private fromXY(e: MouseEvent): [row: number, col: number] {
        return [
            Math.round(e.offsetY / this.cellSize) - 1,
            Math.round(e.offsetX / this.cellSize) - 1,
        ];
    }
    
    /**
     * Determines if a star point should be drawn at the given coordinates, on
     * a board of the given size.
     */
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
