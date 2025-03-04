pub mod message_caster;
pub mod messages;

use message_caster::test_node::QTestNode;


fn main() {
    let mut testNode = QTestNode::new("127.0.0.1:8080".to_string());

    testNode.listen();
}

