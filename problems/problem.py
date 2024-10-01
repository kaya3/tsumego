class Problem:
    @staticmethod
    def from_input_format(name: str, obj):
        """
        Constructs a `Problem` instance from the given object. The object must
        be in the format of the JSON input files in the library.
        """
        size = int(obj['SZ'])
        
        if size < 2 or size > 25:
            raise ValueError(f'Board size must be between 2 and 25; was {size}')
        
        solution = obj['SOL']
        next_player = solution[0][0].lower()
        
        if next_player not in ('b', 'w'):
            raise ValueError(f"Next player must be 'b' or 'w', was '{next_player}'")
        
        # Construct initial board position given stone positions
        board = [['.'] * size for _ in range(size)]
        _add_stones(board, obj['AB'], 'b')
        _add_stones(board, obj['AW'], 'w')
        
        # Convert solution sequence to a tree
        tree = 'win'
        for _, coords, _, _ in reversed(solution):
            row, col = from_sgf_coords(coords)
            move = to_letter_number_coords(row, col)
            tree = {move: tree}
        
        return Problem(name, size, board, next_player, tree)
    
    def __init__(self, name: str, size: int, board, next_player: str, tree):
        self.name = name
        self.size = size
        self.board = board
        self.next_player = next_player
        self.tree = tree
    
    def swap_colours(self):
        """
        Returns a new `Problem` instance equivalent to this one, with the
        colours swapped.
        """
        return Problem(
            self.name,
            self.size,
            [[swap_colour(x) for x in row] for row in self.board],
            swap_colour(self.next_player),
            self.tree,
        )
    
    def to_board_string(self):
        """
        Converts this problem's initial board position to a string, in the
        format required by the TypeScript frontend.
        """
        board = '\n'.join(''.join(row) for row in self.board)
        return f'{self.next_player}\n{board}'
    
    def to_output_format(self):
        """
        Converts this problem to an object which can be serialised as JSON,
        as required by the TypeScript frontend.
        """
        return {
            'name': self.name,
            'board': self.to_board_string(),
            'tree': self.tree,
        }

def _add_stones(board, positions, colour: str):
    """
    Helper function to add stones of the given colour to the board, at the
    given list of positions.
    """
    for coords in positions:
        row, col = from_sgf_coords(coords)
        board[row][col] = colour

def from_sgf_coords(coords: str):
    """
    Converts the coordinates from a string in SGF format to the row and column
    indices. In SGF format, both coordinates are represented as letters.
    """
    if len(coords) != 2 or not coords.isalpha() or not coords.isascii():
        raise ValueError(f'SGF coordinates must be two characters [a-z]; was "{coords}"')
    
    # Gracefully handle lower or uppercase letters
    coords = coords.upper()
    
    # 65 is 'A'
    col = ord(coords[0]) - 65
    row = ord(coords[1]) - 65
    return row, col

# No 'I' column, by convention
ALPHABET = 'ABCDEFGHJKLMNOPQRSTUVWXYZ'
def to_letter_number_coords(row: int, col: int):
    """
    Converts the row and column indices to a coordinate string like 'A4'. The
    letter represents the column, and the row represents the number. By
    convention, there is no 'I' column.
    """
    if row < 0 or row >= 25 or col < 0 or col >= 25:
        raise ValueError(f'row or column index out of bounds; was row = {row}, col = {col}')
    return f'{ALPHABET[col]}{row + 1}'

def swap_colour(colour: str):
    """
    Swaps the colours 'b' and 'w', leaving other strings unchanged.
    """
    return 'b' if colour == 'w' \
        else 'w' if colour == 'b' \
        else colour
