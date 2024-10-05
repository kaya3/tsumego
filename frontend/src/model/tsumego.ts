interface TsumegoData {
    readonly id: number;
    readonly name: string;
    readonly board: string,
    readonly tree: VariationTree;
}

type VariationTree =
    | 'win'
    | 'lose'
    | {readonly [moveCoordinates: string]: VariationTree}

class Tsumego {
    public static fromJSON(json: string): Tsumego {
        return Tsumego.fromData(JSON.parse(json));
    }
    
    public static fromData(obj: Pick<TsumegoData, 'board' | 'tree'>): Tsumego {
        if(!('board' in obj) || typeof obj.board !== 'string') {
            throw new Error(`Tsumego JSON object must have 'board' property of type 'string'`);
        }
        
        const board = new Board(obj.board);
        
        if(!('tree' in obj)) {
            throw new Error(`Tsumego JSON object must have 'tree' property which is a valid variation tree`);
        }
        Tsumego.validateTree(board, obj.tree);
        
        return new Tsumego(board, obj.tree);
    }
    
    private static validateTree(board: Board, tree: unknown): asserts tree is VariationTree {
        if(tree === 'win' || tree === 'lose') {
            return;
        } else if(!tree || typeof tree !== 'object') {
            throw new Error("Tsumego tree must be 'win', 'lose' or an object");
        }
        
        const pairs: [string, unknown][] = Object.entries(tree);
        if(pairs.length === 0) {
            throw new Error('Tsumego tree object must have at least one entry');
        }
        
        for(const [coords, childTree] of pairs) {
            const [row, col] = fromCoordinates(coords);
            if(!board.isLegal(row, col)) {
                throw new Error(`Tsumego tree contains illegal move '${coords}'`);
            }
            
            const childBoard = board.play(row, col);
            Tsumego.validateTree(childBoard, childTree);
        }
    }
    
    private constructor(
        public readonly board: Board,
        private readonly tree: VariationTree,
    ) {}
    
    public isComplete(): boolean {
        return this.tree === 'win' || this.tree === 'lose';
    }
    
    public isWon(): boolean {
        return this.tree === 'win';
    }
    
    public play(row: number, col: number): Tsumego {
        if(typeof this.tree !== 'object') {
            throw new Error('Tried to play a move in a completed tsumego');
        }
        
        const newBoard = this.board.play(row, col);
        // Playing a move out of the tree is automatically a loss
        const newTree = this.tree[toCoordinates(row, col)] ?? 'lose';
        return new Tsumego(newBoard, newTree);
    }
    
    public playRandom(): Tsumego {
        if(typeof this.tree !== 'object') {
            throw new Error('Tried to play a move in a completed tsumego');
        }
        
        const options = Object.keys(this.tree);
        const coords = options[Math.floor(options.length * Math.random())];
        const [row, col] = fromCoordinates(coords);
        
        return this.play(row, col);
    }
}
