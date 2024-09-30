class TsumegoView extends BoardView {
    /**
     * Wait this long, in milliseconds, between the player's move and the
     * opponent's response.
     */
    public moveDelay: number = 500;
    
    private readonly onCompleteCallbacks: ((win: boolean) => void)[] = [];
    
    public constructor(
        public tsumego: Tsumego,
    ) {
        if(tsumego.board.nextPlayer() !== 'b') {
            throw new Error('TsumegoView must be initialised with black to play');
        }
        
        super(tsumego.board);
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
    
    public onComplete(callback: (win: boolean) => void): void {
        this.onCompleteCallbacks.push(callback);
    }
}
