use crate::problem::solution::Solution;

pub type Neighbourhood = Box<dyn Iterator<Item = Solution> + Send>;

pub type NeighbourhoodGenerator = fn(&Solution) -> Neighbourhood;

pub fn swap_neighbourhood(solution: &Solution) -> Neighbourhood {
    let sol_list = solution.sol_list.clone();
    let len = sol_list.len();
    let iterator = (0..len).flat_map(move |i| {
        let sol_list = sol_list.clone();
        (i + 1..len).map(move |j| {
            let mut new_sol_list = sol_list.clone();
            new_sol_list.swap(i, j);
            Solution {
                sol_list: new_sol_list,
                sol_val: None,
            }
        })
    });

    Box::new(iterator)
}