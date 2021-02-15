use std::io;
use std::cmp;
use std::io::prelude::*;
use std::default::Default;

const DY: [i32; 4] = [-1, 0, 1, 0];
const DX: [i32; 4] = [0, 1, 0, -1];

struct Config {
    m: usize,
    q: usize,
    arr: Vec<Vec<i32>>,
    l: Vec<i32>,
    is_dec: Vec<Vec<bool>>,
    is_visited: Vec<Vec<bool>>,
    
}

impl Config {
    fn new(input: &str) -> Config {
        let mut input = input.split_whitespace().map(|x| x.parse::<i32>().unwrap());
        
        let n = input.next().unwrap() as usize;
        let q = input.next().unwrap() as usize;
        let m = 1 << n;
        let mut arr = vec![vec![0; m]; m];
        for i in 0..m {
            for j in 0..m {
                arr[i][j] = input.next().unwrap();
            }
        }
        let mut l = vec![0; q];
        for i in 0..q {
            l[i] = input.next().unwrap();
        }
        Config {
            m, q, arr, l, is_dec: Default::default(), is_visited: Default::default()
        }
    }

    fn firestorm_run(&mut self, lv: i32) {
        self.is_dec = vec![vec![false; self.m]; self.m];
        for i in (0..self.m).step_by(1 << lv) {
            for j in (0..self.m).step_by(1 << lv) {
                self.firestorm(i, j, lv);
            }
        }

        for i in 0..self.m {
            for j in 0..self.m {
                let mut counter = 0;
                for d in 0..4 {
                    let (ii, jj) = ((i as i32 + DY[d]) as usize, (j as i32 + DX[d]) as usize);
                    if ii >= self.m || jj >= self.m {
                        continue;
                    }
                    if self.arr[ii][jj] > 0 {
                        counter += 1;
                    }
                }
                if counter < 3 {
                    self.is_dec[i][j] = true;
                }
            }
        }
        for i in 0..self.m {
            for j in 0..self.m {
                if self.is_dec[i][j] && self.arr[i][j] > 0 {
                    self.arr[i][j] -= 1;
                }
            }
        }
    }
    
    fn firestorm(&mut self, y: usize, x: usize, lv: i32) {
        let lv = 1 << lv;
        let mut tmp = vec![vec![0; lv]; lv];
        for i in 0..lv {
            for j in 0..lv {
                tmp[i][j] = self.arr[y + lv - j - 1][x + i];
            }
        }
        for i in 0..lv {
            for j in 0..lv {
                self.arr[y + i][x + j] = tmp[i][j];
            }
        }
    }

    fn sum(&self) -> i32 {
        let mut ret = 0;
        for i in 0..self.m {
            for j in 0..self.m {
                ret += self.arr[i][j];
            }
        }
        ret
    }

    fn dfs_all(&mut self) -> i32 {
        let m = self.m;
        self.is_visited = vec![vec![false; m]; m];
        let mut mx = 0;
        for i in 0..m {
            for j in 0..m {
                if self.arr[i][j] > 0 && !self.is_visited[i][j] {
                    mx = cmp::max(mx, self.dfs(i, j));

                }
            }
        }
        mx
    }


    fn dfs(&mut self, i: usize, j: usize) -> i32 {
        let mut ret = 1;
        self.is_visited[i][j] = true;
        for d in 0..4 {
            let (ii, jj) = ((i as i32 + DY[d]) as usize, (j as i32 + DX[d]) as usize);
            if ii >= self.m || jj >= self.m {
                continue;
            }
            if self.arr[ii][jj] > 0 && !self.is_visited[ii][jj] {
                ret += self.dfs(ii, jj);
                
            }
        }
        ret
    }
}

fn solve(mut cfg: Config) {
    for i in 0..cfg.q {
        let lv = cfg.l[i]; 
        cfg.firestorm_run(lv);
    }
    println!("{}", cfg.sum());
    println!("{}", cfg.dfs_all());
}


fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let cfg = Config::new(&input);
    
    solve(cfg);

    Ok(())
}
