class TsumegoView extends BoardView {
    public static EMPTY_TSUMEGO = Tsumego.fromData({
        board: Board.empty(9).toString(),
        tree: 'lose',
    });
    
    public tsumego: Tsumego;
    
    /**
     * Wait this long, in milliseconds, between the player's move and the
     * opponent's response.
     */
    public moveDelay: number = 500;
    
    private readonly onCompleteCallbacks: ((win: boolean) => void)[] = [];
    
    public constructor(tsumego?: Tsumego) {
        tsumego ??= TsumegoView.EMPTY_TSUMEGO;
        super(tsumego.board);
        
        this.tsumego = tsumego;
        this.playEnabled = !tsumego.isComplete();
        
        this.onPlay((row, col) => {
            this.setTsumego(this.tsumego.play(row, col));
            
            if(!this.tsumego.isComplete()) {
                setTimeout(() => {
                    this.setTsumego(this.tsumego.playRandom());
                }, this.moveDelay);
            }
        });
    }
    
    /**
     * Updates the currently visible tsumego, allowing play to continue if it
     * is the black player's turn and the tsumego is not complete. If the
     * tsumego is complete, any registered callbacks are invoked.
     */
    public setTsumego(tsumego: Tsumego): void {
        this.tsumego = tsumego;
        this.playEnabled = !tsumego.isComplete() && tsumego.board.nextPlayer() === 'b';
        this.setBoard(tsumego.board);
        
        // If the tsumego is now completed, trigger callbacks
        if(tsumego.isComplete()) {
            const win = tsumego.isWon();
            for(const callback of this.onCompleteCallbacks) {
                callback(win);
            }
        }
    }
    
    /**
     * Clears this view, showing an empty board on which play is disabled.
     */
    public clear(): void {
        this.tsumego = TsumegoView.EMPTY_TSUMEGO;
        this.playEnabled = false;
    }
    
    /**
     * Registers a callback function to be called when this tsumego is
     * completed, either won or lost.
     */
    public onComplete(callback: (win: boolean) => void): void {
        this.onCompleteCallbacks.push(callback);
    }
}
