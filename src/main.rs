#![allow(warnings, unused)]
use std::io::Write;

use easy_io::InputReader;
// Find Query: Check if two objects are in the same component
// Union Command: Replace components containing the two objects with their union
fn main() {
    let mut input = InputReader::new();
    print!("Enter size of UF : ");
    std::io::stdout().flush();
    let n = input.next::<usize>();
    let mut uf = UF::init(n);
    loop {
        println!("Entire component: {:?}", uf.c);
        let p: usize = input.next();
        let q: usize = input.next();
        // println!("You entered p:{} q:{}", p, q);
        if !uf.is_connected(p, q) {
            println!("{} and {} are not connected, connecting them now.", p, q);
            uf.union(p, q);
        } else {
            println!("{} and {} are already connected", p, q);
        }
    }
}

// Contains N objects enumerated from 0 to N-1
struct UF {
    n: usize,
    // Interpretation :
    // Two objects in index i and j are said to be connected if they contain the same
    // value. For eg: to say 3, and 4 are connected, inner[3] and inner[4] contain
    // the same value for instance inner[3] == inner[4] == 1;
    //
    // 0 stands for uninitialized indices.
    c: Vec<usize>,
    // Not required in the lazy approach. Uncomment for standard approach
    // curr: usize,

    // Contains the size of the tree for each component
    // Instead of using distance to root as a decision for joining trees, since we find that if 4 has
    // some children, but itself is a root, and then the user enters union(9,4) where 9 is self-contained tree
    // then depending upon the specific order in which the user writes the query, in this case (9,4), we get
    // 4 as a child of 9. But that's not optimal since 4 is already a root having 1 level deep children. So 
    // to codify this extra information, just the time it takes to reach a root isn't sufficient. We also need
    // to keep track of how large the tree is, and in case of a union, increment that root's size with the sz 
    // of the newly added element
    sz : Vec<usize>
}
// Lazy Approach
// Change in the interpretation of the array
// Instead of storing an integer to use as a grouping for different indices
// We store at index the root of component.
// c[index] = root
impl UF {
    fn init(n: usize) -> Self {
        // first difference, every index is in it's own component so it is it's own root
        // We also do not require a `curr` field here to denote components so we get rid of it
        let c = (0..n).map(|x| x).collect();
        let sz = vec![0_usize; n];
        Self { n, c , sz}
    }
    fn is_connected(&mut self, p: usize, q: usize) -> bool {
        // p and q are connected if the have the same root
        if self.find_root(p) == self.find_root(q) {
            true
        } else {
            false
        }
    }
    // Find operation takes more time, O(N) in the worst case
    fn find_root(&self, mut p: usize) -> usize {
        let mut p_val = self.c[p];
        while p_val != p {
            p = self.c[p_val];
            p_val = self.c[p];
        }
        p_val
    }
    fn union(&mut self, p: usize, q: usize) {
        let (p_root, q_root) = (self.find_root(p), self.find_root(q));
        if self.sz[p] == self.sz[q] {
            // Order doesn't matter here
            self.c[q_root] = p_root;
            self.sz[p] += 1;
        } else if self.sz[p] < self.sz[q] {
            self.c[p_root] = q_root;
            self.sz[q] += self.sz[p];
        } else if self.sz[p] > self.sz[q] {
            self.c[q_root] = p_root;
            self.sz[p] += self.sz[q];
        } 
    }
    // Quick Union approach O(N*), depends on the find_root which is O(N)
    fn crude_union(&mut self, p: usize, q: usize) {
        // Connecting p and q means setting their roots the same
        // Now our union is O(1)
        let (p_root, q_root) = (self.find_root(p), self.find_root(q));
        // Ignored: Check if they are self-contained components or not
        // Also check if your trees aren't getting too long
        self.c[q] = self.c[p];
    }
    // Weighted Quick Union approach: Avoiding large trees
    // The indices themselves are the components in question 0..N-1
    fn root_level_union(&mut self, p: usize, q: usize) {
        // ************** Old code that doesn't calculate or store tree size ****************
        let ((p_root, p_lvl), (q_root, q_lvl)) =
            (self.find_root_level(p), self.find_root_level(q));
        println!(
            "{q}_root = {q_root}\n{p}_root = {p_root}",
            q = q,
            q_root = q_root,
            p = p,
            p_root = p_root
        );
        println!(
            "{p}_lvl = {p_lvl}\n{q}_lvl = {q_lvl}",
            p = p,
            p_lvl = p_lvl,
            q = q,
            q_lvl = q_lvl
        );
        if p_lvl == q_lvl {
            self.c[q_root] = p_root;
        } else if p_lvl < q_lvl {
            self.c[p_root] = q_root;
        } else if p_lvl > q_lvl {
            self.c[q_root] = p_root;
        }
    }

    fn find_root_level(&self, mut p: usize) -> (usize, usize) {
        let mut counter = 0_usize;
        while self.c[p] != p {
            p = self.c[p];
            counter += 1;
        }
        (p, counter)
    }
}

/*
impl UF {
    // Union : O(n)
    // is-connected: O(1)
    // init : O(n)
    // Limitations: N union commands take O(N²) time
    fn init(n: usize) -> Self {
        Self {
            n,
            c: vec![0; n],
            // current component number, 0 for uninitialized
            curr: 1,
        }
    }
    Quick find approach O(1)
    fn is_connected(&mut self, p: usize, q: usize) -> bool {
        if self.c[p] == self.c[q] && self.c[p] != 0 && self.c[q] != 0 {
            true
        } else {
            false
        }
    }
    fn union(&mut self, p: usize, q: usize) {
        let (p_val, q_val) = (self.c[p], self.c[q]);
        if p_val == 0 && q_val == 0 {
            // P and Q are both uninitialized
            self.c[p] = self.curr;
            self.c[q] = self.curr;
            self.curr += 1;
        } else {
            if p_val == q_val {
                return;
            } else if p_val != 0 && q_val == 0 {
                // P is init but not Q
                self.c[q] = self.c[p];
            } else if q_val != 0 && p_val == 0 {
                // P is init but not Q
                self.c[p] = self.c[q];
            } else if p_val != 0 && q_val != 0 {
                // This operation is O(n)
                // Both P and Q are parts of separate components

                // Color the components on the basis of first argument's component
                let marker = p_val;
                self.c = self
                    .c
                    .iter_mut()
                    .map(|x| if *x == q_val { marker } else { *x })
                    .collect();
            }
        }
    }
}
*/
