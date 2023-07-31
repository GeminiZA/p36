
struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn is_valid_arr(arr: Vec<char>) -> bool {
        let mut set: HashSet<char> = HashSet::new();
        for s in arr {
            if s != '.' {
                if set.contains(&s) {
                    return false;
                } else {
                    set.insert(s.clone());
                }
            }
        }
        return true;
    }

    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let hor = board.clone();
        let mut ver: Vec<Vec<char>> = vec![vec!['.'; 9]; 9];
        for i in 0..9 {
            if !(Solution::is_valid_arr(hor[i].clone())) {
                return false;
            }
        }
        for i in 0..9 {
            for j in 0..9 {
                ver[j][8 - i] = hor[i][j].clone();
            }
        }
        for i in 0..9 {
            if !(Solution::is_valid_arr(ver[i].clone())) {
                return false;
            }
        }
        for i in 0..3 {
            for j in 0..3 {
                let mut block: Vec<char> = Vec::new();
                for row in i*3..(i + 1)*3 {
                    for col in j*3..(j+1)*3 {
                        block.push(board[row][col]);
                    }
                }
                if !(Solution::is_valid_arr(block)) {
                    return false;
                }
            }
        }
        return true;
    }

}
fn main() {
    let board = 
    vec![vec!['.','.','.','9','.','.','.','.','.'],
    vec!['.','.','.','.','.','.','.','.','.'],
    vec!['.','.','3','.','.','.','.','.','1'],
    vec!['.','.','.','.','.','.','.','.','.'],
    vec!['1','.','.','.','.','.','3','.','.'],
    vec!['.','.','.','.','2','.','6','.','.'],
    vec!['.','9','.','.','.','.','.','7','.'],
    vec!['.','.','.','.','.','.','.','.','.'],
    vec!['8','.','.','8','.','.','.','.','.']];
    let result = Solution::is_valid_sudoku(board);
    println!("{}", result);
}

