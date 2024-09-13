//! Bertekas Auction Algorithm for the Assignment Problem
//!
use std::sync::mpsc::{channel, Receiver, Sender};

/// A simple data structure that keeps track of all data required to
/// assign agents to tasks.
pub struct Auction {
    cost_matrix: Vec<Vec<f64>>,
    assignments: Vec<Option<usize>>,
    prices: Vec<f64>,
    epsilon: f64,
    /// This is a mapping of every single possible task and the corresponding set of bids that
    /// each agent made for that task. Thus, each task, j,  has a list of (agent, bid) tuples
    /// that were put in by each agent, i.
    task_bids: Vec<Vec<(usize, f64)>>,
}

impl Auction {
    /// Returns a new [`Auction`] based on the cost matrix used to determine optimal assignments.
    pub fn new(cost_matrix: Vec<Vec<f64>>) -> Self {
        let m = cost_matrix.len();
        let n = cost_matrix[0].len();

        let prices = vec![0.0; n];
        let assignments = vec![None; m];
        let task_bids = vec![Vec::with_capacity(m); n];

        let epsilon = 1.0 / (n + 1) as f64;

        Self {
            cost_matrix,
            assignments,
            prices,
            epsilon,
            task_bids,
        }
    }

    /// Compute the score after assigning all agents to tasks
    pub fn score(&self) -> Option<f64> {
        let mut res = 0.0;

        if self.all_assigned() {
            for (i, assigned_task) in self.assignments.iter().enumerate() {
                if let Some(j) = *assigned_task {
                    res += self.cost_matrix[i][j];
                }
            }
            Some(res)
        } else {
            None
        }
    }

    pub(crate) fn scale_epsilon(&mut self) {
        self.epsilon *= 2.0;
    }

    pub(crate) fn update_price(&mut self, task: usize, price: f64) {
        self.prices[task] = price;
    }

    /// The number of agents (i.e., the # of rows in the cost matrix.)
    pub fn num_agents(&self) -> usize {
        self.cost_matrix.len()
    }

    /// The number of tasks (i.e., the # of cols in the cost matrix.)

    pub fn num_tasks(&self) -> usize {
        self.cost_matrix[0].len()
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
    pub fn is_unassigned(&self, agent: usize) -> bool {
        self.assignments[agent].is_none()
    }

    fn add_task_bid(&mut self, agent: usize, task: usize, bid: f64) {
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
struct Bid(usize, usize, f64);

impl Bid {
    pub fn new(agent: usize, task: usize, bid: f64) -> Self {
        Self(agent, task, bid)
    }
}

fn bid(agent_row: &[f64], prices: &[f64], epsilon: f64, unassigned_agent: usize, tx: &Sender<Bid>) {
    let mut best_task = None;
    let mut best_profit = f64::NEG_INFINITY;
    let mut next_best_profit = f64::NEG_INFINITY;

    for ((j, val), price_j) in agent_row.iter().enumerate().zip(prices.iter()) {
        // deferencing these first makes the flamegraph take
        // less time on Sub<&f64>
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
fn bid_phase(auction_data: &mut Auction) {
    let (tx, rx): (Sender<Bid>, Receiver<Bid>) = channel();

    let mut num_bids = 0;
    for p in 0..auction_data.num_agents() {
        if auction_data.is_unassigned(p) {
            num_bids += 1;
        }
    }

    for p in 0..auction_data.num_agents() {
        if auction_data.is_unassigned(p) {
            let agent_row = &auction_data.cost_matrix[p];
            let prices = &auction_data.prices;
            let eps = auction_data.epsilon;
            bid(agent_row, prices, eps, p, &tx);
        }
    }

    // println!("waiting to assign bids for tasks...");
    for _ in 0..num_bids {
        let bid = rx.recv().unwrap();

        // auction_data.add_task_bid(unassigned_agent, best_obj, bid_value);
        auction_data.add_task_bid(bid.0, bid.1, bid.2);
    }
    // println!("bidding phase complete");
}

fn assignment_phase(auction_data: &mut Auction) {
    let (tx, rx): (Sender<Option<Bid>>, Receiver<Option<Bid>>) = channel();

    let mut num_tasks = 0;
    for _ in auction_data.task_bids.iter() {
        num_tasks += 1;
    }

    for (task, bids) in auction_data.task_bids.iter().enumerate() {
        let mut max_bid = f64::NEG_INFINITY;
        let mut bid_winner = None;

        for b in bids.iter() {
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
        if let Some(Bid(bid_winner, task, max_bid)) = rx.recv().unwrap() {
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
pub fn forward(auction_data: &mut Auction) {
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
    fn rectangular_matrix_more_agents_maximization() {
        let matrix = vec![
            vec![10.0, 15.0, 20.0],
            vec![5.0, 30.0, 25.0],
            vec![35.0, 10.0, 15.0],
            vec![10.0, 20.0, 25.0],
        ];

        let mut auction_data = Auction::new(matrix);
        forward(&mut auction_data);

        assert!(auction_data.all_assigned());
        assert_eq!(
            auction_data
                .assignments
                .iter()
                .filter(|x| x.is_some())
                .count(),
            3
        );
    }

    #[test]
    fn single_agent_single_task() {
        let matrix = vec![vec![42.0]];

        let mut auction_data = Auction::new(matrix);
        forward(&mut auction_data);

        // The only assignment possible should be agent 0
        let expected_assignments = vec![Some(0)];
        assert_eq!(auction_data.assignments, expected_assignments);
    }

    #[test]
    fn large_matrix() {
        let m = 700;
        let matrix: Vec<Vec<f64>> = (0..m)
            .map(|i| (0..m).map(|j| (i + j) as f64).collect())
            .collect();

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
        let matrix: Vec<i64> = (0..rows * cols).map(|_| rng.gen_range(0..100)).collect();
        let matrix = matrix::Matrix::from_vec(rows, cols, matrix).unwrap();

        let mut cost_matrix = Vec::new();
        for r in matrix.iter() {
            cost_matrix.push(r.iter().map(|a| *a as f64).collect());
        }

        let now = std::time::Instant::now();
        let mut auction_data = Auction::new(cost_matrix);
        forward(&mut auction_data);
        let elapsed = now.elapsed().as_micros();
        println!("bertekas auction complete in {elapsed}");
        println!("score: {}", auction_data.score().unwrap());
        println!("assignments: {:?}\n", auction_data.assignments);

        // Run Munkres algorithm using pathfinding crate
        let now = std::time::Instant::now();
        let (score, assignments) = kuhn_munkres(&matrix);
        let elapsed = now.elapsed().as_micros();
        println!("hungarian algo complete in {elapsed}");
        println!("hungarian algo score: {score}");
        println!("hungarian algo assignments: {assignments:?}");
    }
}
