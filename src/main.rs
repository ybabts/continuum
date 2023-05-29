use crate::entity::Entity;

mod entity;
mod node;
mod relation;

// struct EdgeIterator<'a> {
//     data: &'a Vec<Edge>,
//     index: usize
// }

// impl<'a> Iterator for EdgeIterator<'a> {
//     type Item = &'a Edge;
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.index >= self.data.len() {
//             return None;
//         }
//         let result = &self.data[self.index];
//         self.index += 1;
//         return Some(result);
//     }
// }

fn main() {
    // let node = node::Node {
    //     id: 1,
    //     in_use: true,
    //     next_relation_id: 350,
    //     next_property_id: 120
    // };
    // let result = node.size();
    // let mut result = node::Node::open(1);
    // result.in_use = true;
    // result.next_property_id = 120;
    // result.next_relation_id = 350;
    // result.write();

    let mut node = node::Node::open(2).unwrap();
    
    let result = node.all_relations();

    // let mut relation = relation::Relation::open(1);
    // relation.first_node_id = 1;
    // relation.second_node_id = 2;
    // relation.write();

    println!("{:?}", result);
}
