mod entity;
use entity::Entity;

fn main() {
  let id = entity::node::Node::alloc().unwrap();
  match entity::node::Node::load(id) {
    Err(e) => println!("Error: {:?}", e),
    Ok(mut node) => {
      node.in_use = true;
      node.next_property_id = 1165656;
      node.next_relation_id = 4156165;
      node.save().unwrap();
    }
  }
  
}
