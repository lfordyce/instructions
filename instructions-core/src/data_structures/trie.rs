use std::collections::HashMap;
use tokio::sync::TryAcquireError;

pub struct TrieNode {
    value: Option<char>,
    is_final: bool,
    child_nodes: HashMap<char, TrieNode>,
}

impl TrieNode {
    pub fn new(c: char, is_final: bool) -> TrieNode {
        TrieNode {
            value: Some(c),
            is_final,
            child_nodes: HashMap::new(),
        }
    }
    pub fn new_root() -> TrieNode {
        TrieNode {
            value: None,
            is_final: false,
            child_nodes: HashMap::new(),
        }
    }

    pub fn check_value(self, c: char) -> bool {
        self.value == Some(c)
    }

    pub fn insert_value(&mut self, c: char, is_final: bool) {
        self.child_nodes.insert(c, TrieNode::new(c, is_final));
    }
}

struct Trie {
    root_node: TrieNode,
}

impl Trie {
    pub fn create() -> Trie {
        Trie {
            root_node: TrieNode::new_root(),
        }
    }

    pub fn insert(&mut self, string_val: String) {
        let mut current_node = &mut self.root_node;
        let char_list: Vec<char> = string_val.chars().collect();
        let mut last_match = 0;

        for letter_counter in 0..char_list.len() {
            if current_node
                .child_nodes
                .contains_key(&char_list[letter_counter])
            {
                current_node = current_node
                    .child_nodes
                    .get_mut(&char_list[letter_counter])
                    .unwrap();
            } else {
                last_match = letter_counter;
                break;
            }
            last_match = letter_counter + 1;
        }

        if last_match == char_list.len() {
            current_node.is_final = true;
        } else {
            for new_counter in last_match..char_list.len() {
                println!(
                    "Inserting {} into {}",
                    char_list[new_counter],
                    current_node.value.unwrap_or_default()
                );
                current_node.insert_value(char_list[new_counter], false);
                current_node = current_node
                    .child_nodes
                    .get_mut(&char_list[new_counter])
                    .unwrap();
            }
            current_node.is_final = true;
        }
    }

    // Find a string
    pub fn find(&mut self, string_val: String) -> bool {
        let mut current_node = &mut self.root_node;
        let char_list: Vec<char> = string_val.chars().collect();

        for counter in 0..char_list.len() {
            if !current_node.child_nodes.contains_key(&char_list[counter]) {
                return false;
            } else {
                current_node = current_node
                    .child_nodes
                    .get_mut(&char_list[counter])
                    .unwrap();
            }
        }
        return true;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a_trie() {
        // Create Trie
        let mut trie_test = Trie::create();

        // Insert Stuff
        trie_test.insert("Test".to_string());
        trie_test.insert("Tea".to_string());
        trie_test.insert("Background".to_string());
        trie_test.insert("Back".to_string());
        trie_test.insert("Brown".to_string());

        // Find Stuff
        println!(
            "Is Testing in the trie? {}",
            trie_test.find("Testing".to_string())
        );
        println!(
            "Is Brown in the trie? {}",
            trie_test.find("Brown".to_string())
        );
    }
}
