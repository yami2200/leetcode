/*
      T E X T 2
    T 0 0 0 0 0
    E 0
    X 0
    T 0
    1 0
 */
// ###################### Dynamic Programming algorithm O(N^2)
// Note : i can also get what is the longest_common_subsequence with this algorithm (not only the length)
pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let mut grid: Vec<Vec<(i32, (i32, i32))>> = vec![vec![(0, (0, 0)); text1.len()+1]; text2.len()+1];
    for i in 1..grid.len(){
        for j in 1..grid[i].len(){
            if text1.chars().nth(j-1).unwrap() == text2.chars().nth(i-1).unwrap() {
                grid[i][j] = (grid[i-1][j-1].0 + 1, ((i - 1) as i32, (j - 1) as i32));
            } else {
                let left = grid[i-1][j].0;
                let top = grid[i][j-1].0;
                if top > left {
                    grid[i][j] = (top, ((i) as i32, (j - 1) as i32));
                } else {
                    grid[i][j] = (left, ((i-1) as i32, (j) as i32));
                }
            }
        }
    }
    grid[grid.len()-1][grid[grid.len()-1].len()-1].0
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<(i32, (i32, i32))>>){
    let mut text_vec = vec!["".to_owned(); grid[0].len()];
    for i in 0..grid.len(){
        for j in 0..grid[i].len(){
            text_vec[j].push_str(&*grid[i][j].0.to_string().to_owned());
        }
    }
    for i in 0..text_vec.len(){
        println!("{:?}", text_vec[i]);
    }
}