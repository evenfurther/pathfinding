//! Bertekas Auction Algorithm for the Assignment Problem
use num_traits::FloatConst;

use crate::{matrix::Matrix, prelude::Weights};
use std::sync::mpsc::{channel, Receiver, Sender};

/// A simple data structure that keeps track of all data required to
/// assign agents to tasks.
pub struct Auction<T> {
    cost_matrix: Matrix<T>,
    assignments: Vec<Option<usize>>,
    prices: Vec<T>,
    epsilon: T,
    /// This is a mapping of every single possible task and the corresponding set of bids that
    /// each agent made for that task. Thus, each task, j,  has a list of (agent, bid) tuples
    /// that were put in by each agent, i.
    task_bids: Vec<Vec<(usize, T)>>,
}

impl<T> Auction<T>
where
    T: num_traits::Float,
{
    /// Returns a new [`Auction`] based on the cost matrix used to determine optimal assignments.
    #[must_use]
    pub fn new(cost_matrix: Matrix<T>) -> Self {
        let m = cost_matrix.rows;
        let n = cost_matrix.columns;

        let prices = vec![T::zero(); n];
        let assignments = vec![None; m];
        let task_bids = vec![Vec::with_capacity(m); n];

        let epsilon = T::from(n + 1).unwrap().recip();

        Self {
            cost_matrix,
            assignments,
            prices,
            epsilon,
            task_bids,
        }
    }

    /// Compute the score after assigning all agents to tasks
    #[must_use]
    pub fn score(&self) -> Option<T>
    where
        T: num_traits::Float + std::ops::AddAssign,
    {
        let mut res: T = T::zero();

        if self.all_assigned() {
            for (i, assigned_task) in self.assignments.iter().enumerate() {
                if let Some(j) = *assigned_task {
                    res += *self.cost_matrix.get((i, j)).unwrap();
                }
            }
            Some(res)
        } else {
            None
        }
    }

    pub(crate) fn scale_epsilon(&mut self)
    where
        T: num_traits::Float + num_traits::FloatConst + std::ops::MulAssign,
    {
        self.epsilon *= T::from(1.01).unwrap();
    }

    pub(crate) fn update_price(&mut self, task: usize, price: T) {
        self.prices[task] = price;
    }

    /// The number of agents (i.e., the # of rows in the cost matrix.)
    #[must_use]
    pub fn num_agents(&self) -> usize {
        self.cost_matrix.rows()
    }

    /// The number of tasks (i.e., the # of cols in the cost matrix.)
    #[must_use]
    pub fn num_tasks(&self) -> usize {
        self.cost_matrix.columns()
    }

    fn num_assigned(&self) -> usize {
        self.assignments.iter().filter(|&&x| x.is_some()).count()
    }

    fn assign(&mut self, agent: usize, task: usize) {
        for (i, a) in self.assignments.iter_mut().enumerate() {
            if i != agent && *a == Some(task) {
                *a = None;
            }
        }
        self.assignments[agent] = Some(task);
    }

    /// Check if all agents were assigned. There are 3 possible cases to consider:
    ///
    /// 1. `# agents < # tasks`
    ///
    ///     In this case, we know once all agents are assigned to a subset of ojects,
    ///     all agents are assigned.
    ///
    /// 2. `# agents == # tasks`
    ///
    ///     This is the simplest case and *should* be optimal. That is,
    ///     we have a bijection between agents and tasks
    ///
    /// 3. `# agents > # tasks`
    ///     
    ///     This is the case where the number of possible agents assigned
    ///     is always going to be less than the number of possible ojbects.
    ///     Let m be the number of agents, and let n be the number of tasks,
    ///     then we will always have `k = m - n` agents that can be assigned.
    #[must_use]
    pub fn all_assigned(&self) -> bool {
        if self.num_agents() > self.num_tasks() {
            // Case 3: More agents than tasks. We should have exactly `n` agents assigned.
            self.num_assigned() == self.num_tasks()
        } else {
            // Case 1 & 2: Less or equal agents than tasks. We should have all agents assigned.
            self.num_assigned() == self.num_agents()
        }
    }

    /// Should this be public?
    #[must_use]
    pub fn is_unassigned(&self, agent: usize) -> bool {
        self.assignments[agent].is_none()
    }

    fn add_task_bid(&mut self, agent: usize, task: usize, bid: T)
    where
        T: num_traits::Float,
    {
        self.task_bids[task].push((agent, bid));
    }

    /// We need to clear out all the bids that each agent made for each task
    fn clear_task_bids(&mut self) {
        for bids in &mut self.task_bids {
            bids.clear();
        }
    }
}

/// Tuple struct of (agent, task, bid)
struct Bid<T> {
    agent: usize,
    task: usize,
    amount: T,
}

impl<T> Bid<T> {
    pub fn new(agent: usize, task: usize, amount: T) -> Self
    where
        T: num_traits::Float + num_traits::FloatConst,
    {
        Self {
            agent,
            task,
            amount,
        }
    }
}

fn bid<T>(agent_row: &[T], prices: &[T], epsilon: T, unassigned_agent: usize, tx: &Sender<Bid<T>>)
where
    T: num_traits::Float + num_traits::FloatConst,
{
    let mut best_task = None;
    let mut best_profit = T::neg_infinity();
    let mut next_best_profit = T::neg_infinity();

    for ((j, val), price_j) in agent_row.iter().enumerate().zip(prices.iter()) {
        let profit = (*val) - (*price_j);

        if profit > best_profit {
            next_best_profit = best_profit;
            best_profit = profit;
            best_task = Some(j);
        } else if profit > next_best_profit {
            next_best_profit = profit;
        }
    }

    if let Some(best_obj) = best_task {
        let bid_value = prices[best_obj] + best_profit - next_best_profit + epsilon;
        let bid_for_agent = Bid::new(unassigned_agent, best_obj, bid_value);
        tx.send(bid_for_agent).unwrap();
    }
}

/// This is known as the "Jacobi bidding" version.
/// Essentially, all agents bid for tasks, and only then
/// do we make an assignment.
fn bid_phase<T>(auction_data: &mut Auction<T>)
where
    T: num_traits::Float + num_traits::FloatConst,
{
    let (tx, rx): (Sender<Bid<T>>, Receiver<Bid<T>>) = channel();

    let mut num_bids = 0;
    for p in 0..auction_data.num_agents() {
        if auction_data.is_unassigned(p) {
            num_bids += 1;
        }
    }

    for p in 0..auction_data.num_agents() {
        if auction_data.is_unassigned(p) {
            let agent_row = &auction_data.cost_matrix.get_row(p).unwrap();
            let prices = &auction_data.prices;
            let eps = auction_data.epsilon;
            bid(agent_row, prices, eps, p, &tx);
        }
    }

    // println!("waiting to assign bids for tasks...");
    for _ in 0..num_bids {
        let bid = rx.recv().unwrap();

        // auction_data.add_task_bid(unassigned_agent, best_obj, bid_value);
        auction_data.add_task_bid(bid.agent, bid.task, bid.amount);
    }
    // println!("bidding phase complete");
}

fn assignment_phase<T>(auction_data: &mut Auction<T>)
where
    T: num_traits::Float + num_traits::FloatConst,
{
    let (tx, rx): (Sender<Option<Bid<T>>>, Receiver<Option<Bid<T>>>) = channel();

    let mut num_tasks = 0;
    for _ in &auction_data.task_bids {
        num_tasks += 1;
    }

    for (task, bids) in auction_data.task_bids.iter().enumerate() {
        let mut max_bid = T::neg_infinity();
        let mut bid_winner = None;

        for b in bids {
            let (agent, agents_bid) = *b;
            if agents_bid > max_bid {
                max_bid = agents_bid;
                bid_winner = Some(agent);
            }
        }

        if let Some(bw) = bid_winner {
            tx.send(Some(Bid::new(bw, task, max_bid))).unwrap();
        } else {
            tx.send(None).unwrap();
        }
    }
    // println!("sent all bids via tx in assignment phase");

    for _i in 0..num_tasks {
        if let Some(Bid {
            agent: bid_winner,
            task,
            amount: max_bid,
        }) = rx.recv().unwrap()
        {
            auction_data.update_price(task, max_bid);
            auction_data.assign(bid_winner, task);
        }
    }

    auction_data.clear_task_bids(); // Clear bids after each assignment phase
                                    // println!("assignment phase complete");
}

/// Run the forward auction only. The way to consider this
/// is that agents are going to bid for tasks. Agents will
/// be assigned to a task after each bidding phase.
pub fn forward<T>(auction_data: &mut Auction<T>)
where
    T: num_traits::Float + num_traits::FloatConst + std::ops::MulAssign,
{
    while !auction_data.all_assigned() {
        bid_phase(auction_data);
        assignment_phase(auction_data);
        auction_data.scale_epsilon();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{kuhn_munkres::kuhn_munkres, matrix};
    use rand::Rng;

    #[test]
    fn basic_functionality_maximization() {
        let matrix = vec![
            vec![1.0, 2.0, 20.0, 2.5],
            vec![7.0, 5.0, 11.0, 3.0],
            vec![6.0, 1.0, 1.5, 12.0],
            vec![0.0, 14.0, 3.7, 14.0],
        ];
        let matrix = Matrix::from_rows(matrix).unwrap();

        let mut auction_data = Auction::new(matrix);
        forward(&mut auction_data);

        let expected_assignments = vec![Some(2), Some(0), Some(3), Some(1)];
        assert_eq!(auction_data.assignments, expected_assignments);

        let score = auction_data.score();
        assert!(score.is_some());
        println!("{}", score.unwrap());
    }

    #[test]
    fn all_high_values() {
        let matrix = vec![
            vec![1000.0, 1000.0, 1000.0, 1000.0],
            vec![1000.0, 1000.0, 1000.0, 1000.0],
            vec![1000.0, 1000.0, 1000.0, 1000.0],
            vec![1000.0, 1000.0, 1000.0, 1000.0],
        ];

        let matrix = Matrix::from_rows(matrix).unwrap();
        let mut auction_data = Auction::new(matrix);
        forward(&mut auction_data);

        // Any assignment is optimal since all profits are equal.
        assert!(auction_data.all_assigned());
        assert_eq!(
            auction_data
                .assignments
                .iter()
                .filter(|x| x.is_some())
                .count(),
            4
        );
    }

    #[test]
    fn single_agent_single_task() {
        let matrix = vec![vec![42.0]];
        let matrix = Matrix::from_rows(matrix).unwrap();

        let mut auction_data = Auction::new(matrix);
        forward(&mut auction_data);

        // The only assignment possible should be agent 0
        let expected_assignments = vec![Some(0)];
        assert_eq!(auction_data.assignments, expected_assignments);
    }

    #[test]
    fn empty() {
        let matrix: Matrix<f64> = Matrix::new_empty(0);

        let mut auction_data = Auction::new(matrix);
        forward(&mut auction_data);
    }

    #[test]
    fn large_matrix() {
        let m = 700;
        let matrix = Matrix::from_fn(m, m, |(i, j)| (i + j) as f64);

        let mut auction_data = Auction::new(matrix);
        forward(&mut auction_data);

        assert!(auction_data.all_assigned());
        assert_eq!(auction_data.num_assigned(), m);
    }

    #[test]
    fn compare_with_pathfinding_basic_functionality() {
        let rows = 200;
        let cols = 200;
        let mut rng = rand::thread_rng();

        // Create the integer matrix
        let int_matrix: Matrix<i64> = Matrix::from_fn(rows, cols, |_| rng.gen_range(0..100));

        // Create the float matrix as a clone of the integer matrix
        let float_matrix = int_matrix.clone().map(|value| value as f64);

        let now = std::time::Instant::now();
        let mut auction_data = Auction::new(float_matrix);
        forward(&mut auction_data);
        let elapsed = now.elapsed().as_micros();
        println!("bertekas auction complete in {elapsed}");
        println!("score: {}", auction_data.score().unwrap());
        println!("assignments: {:?}\n", auction_data.assignments);

        // Run Munkres algorithm using pathfinding crate
        let now = std::time::Instant::now();
        let (score, assignments) = kuhn_munkres(&int_matrix);
        let elapsed = now.elapsed().as_micros();
        println!("hungarian algo complete in {elapsed}");
        println!("hungarian algo score: {score}");
        println!("hungarian algo assignments: {assignments:?}");
    }
}
