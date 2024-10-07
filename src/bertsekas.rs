//! Bertekas Auction Algorithm for the Assignment Problem
use crate::matrix::Matrix;
use num_traits::{Float, FloatConst};
use std::marker::PhantomData;
use std::sync::mpsc::{channel, Receiver, Sender};

/// This is a mapping of every single possible task and the corresponding set of bids that each
/// agent made for that task. Thus, each task, j, has a list of `(agent, bid)` tuples that were put
/// in by each agent, i.
struct BidsForTasks<T> {
    agents: Vec<Vec<usize>>,
    bids: Vec<Vec<T>>,
}

impl<T> BidsForTasks<T>
where
    T: Float,
{
    pub fn new(num_tasks: usize) -> Self {
        Self {
            agents: vec![Vec::new(); num_tasks],
            bids: vec![Vec::new(); num_tasks],
        }
    }

    fn clear(&mut self) {
        for agents in &mut self.agents {
            agents.clear();
        }
        for bids in &mut self.bids {
            bids.clear();
        }
    }

    fn add_bid(&mut self, task: usize, agent: usize, bid: T) {
        self.agents[task].push(agent);
        self.bids[task].push(bid);
    }
}

/// A simple data structure that keeps track of all data required to
/// assign agents to tasks.
pub struct Auction<'a, T> {
    cost_matrix: &'a Matrix<T>,
    assignments: Vec<Option<usize>>,
    prices: Vec<T>,
    epsilon: T,
    epsilon_scaling_factor: T,
    task_bids: BidsForTasks<T>,
    _phantom: PhantomData<T>,
    tx: Sender<Option<Bid<T>>>,
    rx: Receiver<Option<Bid<T>>>,
}

impl<'a, T> Auction<'a, T>
where
    T: Float,
{
    /// Returns a new [`Auction`] based on the cost matrix used to determine optimal assignments.
    ///
    /// # Panics
    ///
    /// Panics if not able to covert `1 / (n + 1)` into the cost matrix's underlying type.
    #[must_use]
    pub fn new(cost_matrix: &'a Matrix<T>) -> Self {
        // The # of rows in the matrix corresponds to the # of agents
        let m = cost_matrix.rows;

        // The # of columns in the matrix corresponds to the # of tasks
        let n = cost_matrix.columns;

        assert!(m <= n);

        let prices = vec![T::zero(); n];
        let assignments = vec![None; m];

        let epsilon = T::from(n + 1)
            .expect("couldn't convert n + 1 = {n} + 1 to the required type!")
            .recip();

        let (tx, rx): (Sender<Option<Bid<T>>>, Receiver<Option<Bid<T>>>) = channel();

        Self {
            cost_matrix,
            assignments,
            prices,
            epsilon,
            epsilon_scaling_factor: T::from(2.0).unwrap(),
            task_bids: BidsForTasks::new(n),
            _phantom: PhantomData,
            tx,
            rx,
        }
    }

    /// Same a [`Self::new`], except the user is given the option of setting a custom
    /// `epsilon_scaling_factor` via the [`es`] parameter.
    ///
    /// # Notes
    /// In general, the value of epsilon determines how quickly the algorithm
    /// will converge. The higher the value, the more *aggressive* the bidding.
    /// Epsilon scaling is required to keep this algorithm from exhibiting
    /// psuedopolynomial run-time. The scaling factor determines how fast the value
    /// of `epsilon` grows after each round of bidding/assigning.
    #[must_use]
    pub fn with_epsilon_scaling_factor(es: T, cost_matrix: &'a Matrix<T>) -> Self {
        let mut auc = Self::new(cost_matrix);
        auc.epsilon_scaling_factor = es;
        auc
    }

    /// Compute the score after assigning all agents to tasks
    ///
    /// # Panics
    ///
    /// Panics if getting an assignment `(i, j)`, where `i` is the agent assigned to task `j`,
    /// is not found in the cost matrix. This shouldn't really happen.
    #[must_use]
    pub fn score(&self) -> Option<T>
    where
        T: Float + std::ops::AddAssign,
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
        T: Float + FloatConst + std::ops::MulAssign,
    {
        self.epsilon *= self.epsilon_scaling_factor;
    }

    pub(crate) fn update_price(&mut self, task: usize, price: T) {
        self.prices[task] = price;
    }

    /// The number of agents (i.e., the # of rows in the cost matrix.)
    #[must_use]
    pub fn num_agents(&self) -> usize {
        self.cost_matrix.rows
    }

    /// The number of tasks (i.e., the # of cols in the cost matrix.)
    #[must_use]
    pub fn num_tasks(&self) -> usize {
        self.cost_matrix.columns
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

    #[must_use]
    fn is_unassigned(&self, agent: usize) -> bool {
        self.assignments[agent].is_none()
    }

    fn add_task_bid(&mut self, agent: usize, task: usize, bid: T)
    where
        T: Float,
    {
        self.task_bids.add_bid(task, agent, bid);
    }

    // /// We need to clear out all the bids that each agent made for each task
    // fn clear_task_bids(&mut self) {
    //     for bids in &mut self.task_bids {
    //         bids.clear();
    //     }
    // }
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
        T: Float + FloatConst,
    {
        Self {
            agent,
            task,
            amount,
        }
    }
}

fn bid<T>(
    agent_row: &[T],
    prices: &[T],
    epsilon: T,
    unassigned_agent: usize,
    tx: &Sender<Option<Bid<T>>>,
) where
    T: Float + FloatConst,
{
    let mut best_task = None;
    let mut best_profit = T::neg_infinity();
    let mut next_best_profit = T::neg_infinity();

    for ((j, &val), &price_j) in agent_row.iter().enumerate().zip(prices.iter()) {
        let profit = val - price_j;

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
        tx.send(Some(bid_for_agent)).unwrap();
    }
}

/// This is known as the "Jacobi bidding" version. Essentially, all agents
/// bid for tasks, and only then do we make an assignment.
fn bid_phase<T>(auction_data: &mut Auction<T>)
where
    T: Float + FloatConst,
{
    for p in 0..auction_data.num_agents() {
        if auction_data.is_unassigned(p) {
            let agent_row = auction_data.cost_matrix.get_row(p).unwrap();
            let prices = auction_data.prices.as_slice();
            let eps = auction_data.epsilon;
            let tx = auction_data.tx.clone();
            bid(agent_row, prices, eps, p, &tx);
        }
    }

    // Send a sentinel value, `None`, indicating we are finished sending all bids.
    auction_data.tx.send(None).unwrap();

    while let Some(bid) = auction_data.rx.recv().unwrap() {
        auction_data.add_task_bid(bid.agent, bid.task, bid.amount);
    }
}

fn assignment_phase<T>(auction_data: &mut Auction<T>)
where
    T: Float + FloatConst,
{
    for (task, (agents, bids)) in auction_data
        .task_bids
        .agents
        .iter()
        .zip(&auction_data.task_bids.bids)
        .enumerate()
    {
        let mut max_bid = T::neg_infinity();
        let mut bid_winner = None;

        for (&agent, &bid) in agents.iter().zip(bids.iter()) {
            if bid > max_bid {
                max_bid = bid;
                bid_winner = Some(agent);
            }
        }

        if let Some(bw) = bid_winner {
            auction_data
                .tx
                .send(Some(Bid::new(bw, task, max_bid)))
                .unwrap();
        }
    }

    // Send a sentinel value, `None`, indicating we are finished finding the *new* best assignments.
    auction_data.tx.send(None).unwrap();

    while let Some(Bid {
        agent: bid_winner,
        task,
        amount: max_bid,
    }) = auction_data.rx.recv().unwrap()
    {
        auction_data.update_price(task, max_bid);
        auction_data.assign(bid_winner, task);
    }

    // Clear bids after each assignment phase
    auction_data.task_bids.clear();
}

/// Run the Bertsekas algorithm to create an assignment. The way to think about this is that agents
/// are going to bid for tasks. Agents will be assigned to a task after each bidding phase.
pub fn bertsekas_aaap<T>(auction_data: &mut Auction<T>)
where
    T: Float + FloatConst + std::ops::MulAssign,
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
    use crate::kuhn_munkres::kuhn_munkres;
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

        let mut auction_data = Auction::new(&matrix);
        bertsekas_aaap(&mut auction_data);

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
        let mut auction_data = Auction::new(&matrix);
        bertsekas_aaap(&mut auction_data);

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

        let expected_score: f64 = 4000.0;
        assert_eq!(auction_data.score().unwrap(), expected_score);
    }

    #[test]
    fn single_agent_single_task() {
        let matrix = vec![vec![42.0]];
        let matrix = Matrix::from_rows(matrix).unwrap();

        let mut auction_data = Auction::new(&matrix);
        bertsekas_aaap(&mut auction_data);

        // The only assignment possible should be agent 0
        let expected_assignments = vec![Some(0)];
        assert_eq!(auction_data.assignments, expected_assignments);
    }

    #[test]
    fn empty() {
        let matrix: Matrix<f64> = Matrix::new_empty(0);

        let mut auction_data = Auction::new(&matrix);
        bertsekas_aaap(&mut auction_data);
    }

    #[test]
    fn large_matrix() {
        let m = 700;
        let matrix = Matrix::from_fn(m, m, |(i, j)| (i + j) as f64);

        let mut auction_data = Auction::new(&matrix);
        bertsekas_aaap(&mut auction_data);

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
        let mut auction_data = Auction::new(&float_matrix);
        bertsekas_aaap(&mut auction_data);
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
