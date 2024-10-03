class TsumegoView extends BoardView {
    public static EMPTY_TSUMEGO = Tsumego.fromJSONObject({
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
    
    public clear(): void {
        this.tsumego = TsumegoView.EMPTY_TSUMEGO;
        this.playEnabled = false;
    }
    
    public onComplete(callback: (win: boolean) => void): void {
        this.onCompleteCallbacks.push(callback);
    }
}
