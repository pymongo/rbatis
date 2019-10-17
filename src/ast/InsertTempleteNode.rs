use crate::ast::NodeType::NodeType;
use crate::ast::Node::SqlNode;
use serde_json::Value;

#[derive(Clone)]
pub struct InsertTempleteNode {
    pub id:String,
    pub childs: Vec<NodeType>,
}

impl SqlNode for InsertTempleteNode{
    fn eval(&mut self, env: &mut Value) -> Result<String, String> {
        unimplemented!()
    }

    fn print(&self) -> String {
        let mut result="<insert ".to_string();
        for x in &self.childs {
            result=result+x.print().as_str();
        }
        return result;
    }
}
