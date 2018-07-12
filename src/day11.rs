
#[derive(PartialEq, Eq, Hash)]
struct Node {
    e: usize,
    hg: usize,
    hm: usize,
    lg: usize,
    lm: usize,
}

impl Node {
}

fn print_node(n: &Node) {
    let pif = |a,b| print!("\t{}", if a { b } else { "" });
    for floor in (0..5).rev() {
        print!("F{}", floor); 
        pif(n.e == floor, "E");
        pif(n.hg == floor, "HG");
        pif(n.hm == floor, "HM");
        pif(n.lg == floor, "LG");
        if n.lm == floor {
            println!("\tLM");
        }
        else {
            println!("");
        }
    }
}

pub fn run() {
    let test = Node {
        e: 1,
        hg: 2,
        hm: 1,
        lg: 3,
        lm: 1,
    };
    print_node(&test);
}

#[cfg(test)]
mod tests {
    use super::*;
}
