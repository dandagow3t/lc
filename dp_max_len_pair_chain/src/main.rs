use dp_max_len_pair_chain::solution::Solution;
fn main() {
    // let chain = vec![vec![1,2],vec![7,8],vec![4,5]];
    // let chain = vec![vec![9,10], vec![-4,9], vec![-5,6], vec![-5,9], vec![8,9]];
    let chain = vec![vec![-6,9], vec![1,6], vec![8,10], vec![-1,4], vec![-6,-2], vec![-9,8], vec![-5,3], vec![0,3]];
    println!("Solution {:?}", Solution::solve_v4(chain));
}
