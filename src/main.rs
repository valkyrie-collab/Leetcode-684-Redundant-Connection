use std::io::{self, Write};

fn find(graph: &mut Vec<i32>, val: usize) -> i32 {

    if graph[val] != val as i32 {
        graph[val] = find(graph, graph[val] as usize);
    }
    
    graph[val]
}

fn union(graph: &mut Vec<i32>, rank: &mut Vec<i32>, num_one: i32, num_two: i32) {
    let root_xi: usize = find(graph, num_one as usize) as usize;
    let root_xj: usize = find(graph, num_two as usize) as usize;

    if rank[root_xi] < rank[root_xj] {
        graph[root_xi] = root_xj as i32; 
    } else if rank[root_xi] > rank[root_xj] {
        graph[root_xj] = root_xi as i32;
    } else {
        graph[root_xi] = root_xj as i32;
        rank[root_xj] += 1;
    }

}

fn main() {
    let mut buf: String = String::new();

    print!("Enter the number of vertices ");

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buf).unwrap();

    let size: usize = buf.trim().parse().unwrap_or(0);
    let mut graph: Vec<Vec<i32>> = vec![vec![0; 2]; size];
    
    buf.clear();

    for xi in 0..size {

        print!("Enter the start edge ");

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buf).unwrap();

        let ed_one: i32 = match buf.trim().parse() {
            Ok(xeo) => {

                if xeo <= size as i32 && xeo > 0 {
                    xeo
                } else {
                    0
                }

            }
            Err(xer) => {
                eprintln!("Error! Exited with {}", xer);
                return;
            }
        };

        buf.clear();

        print!("Enter the end edge ");

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buf).unwrap();

        let ed_two: i32 = match buf.trim().parse() {
            Ok(xeo) => {

                if xeo <= size as i32 && xeo > 0 {
                    xeo
                } else {
                    0
                }

            }
            Err(xer) => {
                eprintln!("Error! Exited with {}", xer);
                return;
            }
        };

        buf.clear();

        graph[xi][0] = ed_one;
        graph[xi][1] = ed_two;
    }

    println!("The graph is {:?}", graph);

    let mut parents: Vec<i32> = (0..=graph.len() as i32).collect();
    let mut common: Vec<i32> = vec![0, 0];
    let mut rank: Vec<i32> = vec![0; parents.len()];

    println!("Default parents {:?}", parents);

    for xvs in graph {
        let boss_num_one: i32 = find(&mut parents, xvs[0] as usize);
        let boss_num_two: i32 = find(&mut parents, xvs[1] as usize);

        println!("boss_num_one {} | boss_num_two {}", boss_num_one, boss_num_two);

        if boss_num_one != boss_num_two {
            union(&mut parents, &mut rank, xvs[0], xvs[1]);
            println!("value of parents after union {:?} | value of rank after union {:?}", parents, rank);
        } else {
            println!("Common {:?}", xvs);
            common = xvs;
        }

    }

    println!("final value of parents after union {:?} | common {:?}", parents, common);
}
