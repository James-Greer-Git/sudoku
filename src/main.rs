fn main (){
    let board_size = 4;
    let mut board = Board::new(board_size);

    board.set_cell_value(0, 2, 3);
    board.set_cell_value(1, 2, 1);
    board.set_cell_value(2, 3, 1);
    board.set_cell_value(3, 0, 3);
    board.set_cell_value(3, 2, 2);

    // Print board
    println!("Initial Board");
    board.print_board();

    println!("Notes Board");
    board.print_notes();
}

pub struct Board {
    cells: Vec<Vec<Cell>>, 
}

impl Board {
    // A constructor for Board
    pub fn new(n: usize) -> Self{
        //Create nxn board with default values
        let cells = vec![vec![Cell::new(0, Note::new(n as i32)); n];n];
        Board { cells }
    }

    // Print values of the cells
    pub fn print_board(&self) {
        for row in &self.cells {
            for cell in row {
                print!("{}", cell.value);
            }
            println!();
        }
        println!();
    }

    pub fn set_cell_value(&mut self, row: usize, col: usize, new_value: i32) {
        if let Some(cell) = self.cells.get_mut(row).and_then(|r| r.get_mut(col)) {
            // This will execute if the cell was successfully retrieved
            cell.set_value(new_value);
        }
        else {
            println!("Invalid row or column index.");
        }
    }

    pub fn print_notes(&self) {
        let board_size = self.cells.len();
        let sqrt = (board_size as f64).sqrt() as usize;

        // for row in 0..board_size {
        //     for col in 0..board_size {
        //         for note_row in &self.cells[col][row].note.matrix {
        //             for &value in note_row {
        //                 print!("{:2}", value);
        //             }
        //             println!();
        //         }
        //     }
        // }

        for row in 0..board_size {
            for i in 0..sqrt {
                for col in 0..board_size {
                    for j in 0..sqrt {
                        let note = &self.cells[row][col].note;
                        print!("{:2}", note.matrix[i][j]);
                    }
                }
                println!();
            }
        }
    }
 }

pub struct Cell {
    value: i32,
    note: Note,
}

impl Cell {
    // A constructor for Cell
    pub fn new(value: i32, note: Note) -> Self{
        Cell { value, note }
    }

    pub fn set_value(&mut self, new_value: i32) {
        self.value = new_value;
    }
}

impl Clone for Cell {
    fn clone(&self) -> Self{
        Cell {
            value: self.value,
            note: self.note.clone(),
        }
    }
}

#[derive(Clone)]
pub struct Note {
    matrix: Vec<Vec<i32>>,
}

impl Note {
    // Constructor for Note with default values from 1 to n
    pub fn new(n: i32) -> Self {
        let size = (n as f64).sqrt() as usize;
        let matrix: Vec<Vec<i32>> = (1..=n)
            .collect::<Vec<_>>()
            .chunks(size)
            .map(|chunk| chunk.to_vec())
            .collect();

        Note { matrix }
    }
}