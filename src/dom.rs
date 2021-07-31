use std::collections::HashMap;
pub type AttrMap = HashMap<String, String>;

/**
    nodeの定義
    node_type： Nodeの種類に応じた情報
    children: 子Node
*/
#[derive(Debug, PartialEq)]
pub struct Node {
    pub node_type: NodeType,
    pub children: Vec<Box<Node>>,
}

/**
  NodeType
  Html要素かテキストのenumで定義
*/
#[derive(Debug, PartialEq)]
pub enum NodeType {
    Element(Element),
    Text(Text),
}

/**
    NodeTypeの一種、Elementの定義
    タグの名前と属性(複数)を保持
*/
#[derive(Debug, PartialEq)]
pub struct Element {
    pub tag_name: String,
    pub attributes: AttrMap,
}

// メソッド
impl Element {
    pub fn new(name: String, attributes: AttrMap, children: Vec<Box<Node>>) -> Box<Node> {
        Box::new(Node {
            node_type: NodeType::Element(Element {
                tag_name: name,
                attributes: attributes,
            }),
            children,
        })
    }
}

/**
    NodeTypeの一種、Textの定義
    タグの中の文字列。
    内容はdataとして保持
*/
#[derive(Debug, PartialEq)]
pub struct Text {
    pub data: String,
}

// メソッド
impl Text {
    pub fn new(text: String) -> Box<Node> {
        Box::new(Node {
            node_type: NodeType::Text(Text { data: text }),
            children: vec![],
        })
    }
}
