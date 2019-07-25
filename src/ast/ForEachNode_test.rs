use crate::ast::ForEachNode::ForEachNode;
use crate::ast::Node::SqlNode;
use serde_json::json;
use crate::ast::NodeType::NodeType;
use crate::ast::StringNode::StringNode;
use crate::engines::ExpressionEngineProxy::ExpressionEngineProxy;
use std::rc::Rc;
use crate::engines::ExpressionEngineCache::ExpressionEngineCache;
use crate::ast::SqlArgTypeConvertDefault::SqlArgTypeConvertDefault;
use crate::engines::ExpressionEngineDefault::ExpressionEngineDefault;
use crate::ast::NodeConfigHolder::NodeConfigHolder;


#[test]
pub fn TestForEachNode(){

    let mut n=ForEachNode{
        childs: vec![NodeType::NString(StringNode::new("index:#{index},item:#{item}",Box::new(NodeConfigHolder::new())))],
        collection: "arg".to_string(),
        index: "index".to_string(),
        item: "item".to_string(),
        open: "(".to_string(),
        close: ")".to_string(),
        separator: ",".to_string()
    };
    let mut john = json!({
        "arg": [1,2,3,4],
    });

    let r=n.eval(&mut john);
    println!("{}", r.unwrap());
}