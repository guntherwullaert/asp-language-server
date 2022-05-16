

/**
 * Convert a token value into a human readable string
 */
pub fn humanize_token(token: &String) -> String {
    if token == "RPAREN" { 
        return ")".to_string();
    }
    else if token == "LPAREN" { 
        return "(".to_string(); 
    } else {
        return format!("UNKNOWN TOKEN CONVERSION: {}", token);
    }
}

/*pub fn traverse_tree(tree : &Tree, nodes: &Vec<Node>){
    let mut cursor = tree.walk();

    let mut reached_root = false;
    while !reached_root {
        nodes.push(cursor.node());

        if cursor.goto_first_child(){
            continue;
        }

        if cursor.goto_next_sibling(){
            continue;
        }

        let mut retracing = true;
        while retracing{
            if !cursor.goto_parent() {
                retracing = false;
                reached_root = true;
            }

            if cursor.goto_next_sibling() {
                retracing = false;
            }
        }
    }
}*/