use crate::models::gedcom::{GedcomLine, GedcomLineTag};

#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub struct GedcomTree {
    nodes: Vec<GedcomTreeNode>,
}

impl From<Vec<GedcomLine>> for GedcomTree {
    fn from(mut lines: Vec<GedcomLine>) -> Self {
        let mut nodes = vec![];
        let mut children = vec![];

        let mut previous_level = 100;

        while let Some(line) = lines.pop() {
            let current_level = line.level();

            if current_level == 0 {
                let node_children = children.drain(..).collect();
                let node = GedcomTreeNodeBuilder::from(line)
                    .with_children(node_children)
                    .build();

                nodes.push(node);
            } else if current_level < previous_level {
                let node_children = children
                    .drain_filter(|c| c.level() == current_level + 1)
                    .collect();
                let node = GedcomTreeNodeBuilder::from(line)
                    .with_children(node_children)
                    .build();

                children.push(node);
            } else {
                let node = GedcomTreeNodeBuilder::from(line).build();
                children.push(node);
            }

            previous_level = current_level;
        }

        nodes.reverse();

        Self { nodes }
    }
}

impl GedcomTree {
    pub fn nodes(&self) -> &Vec<GedcomTreeNode> {
        &self.nodes
    }
}

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub struct GedcomTreeNode {
    children: Vec<Self>,
    level: u8,
    line_value: Option<String>,
    tag: GedcomLineTag,
    xref_id: Option<String>,
}

impl GedcomTreeNode {
    pub fn children(&self) -> &Vec<GedcomTreeNode> {
        &self.children
    }

    pub fn level(&self) -> u8 {
        self.level
    }

    pub fn line_value(&self) -> &Option<String> {
        &self.line_value
    }

    pub fn tag(&self) -> &GedcomLineTag {
        &self.tag
    }

    pub fn xref_id(&self) -> &Option<String> {
        &self.xref_id
    }
}

pub struct GedcomTreeNodeBuilder {
    children: Vec<GedcomTreeNode>,
    level: u8,
    line_value: Option<String>,
    tag: GedcomLineTag,
    xref_id: Option<String>,
}

impl From<GedcomLine> for GedcomTreeNodeBuilder {
    fn from(line: GedcomLine) -> Self {
        Self {
            children: vec![],
            level: line.level(),
            line_value: line.line_value().to_owned(),
            tag: line.tag().to_owned(),
            xref_id: line.xref_id().to_owned(),
        }
    }
}

impl GedcomTreeNodeBuilder {
    pub fn build(&mut self) -> GedcomTreeNode {
        GedcomTreeNode {
            children: self.children.drain(..).rev().collect(),
            level: self.level,
            line_value: self.line_value.to_owned(),
            tag: self.tag.to_owned(),
            xref_id: self.xref_id.to_owned(),
        }
    }

    pub fn with_children(&mut self, children: Vec<GedcomTreeNode>) -> &mut Self {
        self.children = children;
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::models::gedcom::{
        GedcomLine, GedcomLineTag, GedcomTree, GedcomTreeNode, GedcomTreeNodeBuilder,
    };

    #[test]
    fn gedcom_tree_node_builder_works_correctly_with_no_children() {
        let input = GedcomLine::builder()
            .with_level(0)
            .with_tag(GedcomLineTag::Individual)
            .build()
            .unwrap();

        let actual = GedcomTreeNodeBuilder::from(input).build();
        let expected = GedcomTreeNode {
            children: vec![],
            level: 0,
            line_value: None,
            tag: GedcomLineTag::Individual,
            xref_id: None,
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn gedcom_tree_node_builder_works_correctly_with_children() {
        let line = GedcomLine::builder()
            .with_level(0)
            .with_tag(GedcomLineTag::Individual)
            .build()
            .unwrap();

        let child = GedcomTreeNode {
            children: vec![],
            level: 1,
            line_value: Some(String::from("Name")),
            tag: GedcomLineTag::Name,
            xref_id: None,
        };

        let expected_child = child.clone();

        let actual = GedcomTreeNodeBuilder::from(line)
            .with_children(vec![child])
            .build();

        let expected = GedcomTreeNode {
            children: vec![expected_child],
            level: 0,
            line_value: None,
            tag: GedcomLineTag::Individual,
            xref_id: None,
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn gedcom_tree_from_works_with_simple_example() {
        let indi_line = GedcomLine::builder()
            .with_level(0)
            .with_tag(GedcomLineTag::Individual)
            .build()
            .unwrap();

        let name_line = GedcomLine::builder()
            .with_level(1)
            .with_optional_line_value(Some(String::from("Name")))
            .with_tag(GedcomLineTag::Name)
            .build()
            .unwrap();

        let given_name_line = GedcomLine::builder()
            .with_level(2)
            .with_optional_line_value(Some(String::from("Given Name")))
            .with_tag(GedcomLineTag::GivenName)
            .build()
            .unwrap();

        let input = vec![indi_line, name_line, given_name_line];

        let given_name_node = GedcomTreeNode {
            children: vec![],
            level: 2,
            line_value: Some(String::from("Given Name")),
            tag: GedcomLineTag::GivenName,
            xref_id: None,
        };

        let name_node = GedcomTreeNode {
            children: vec![given_name_node],
            level: 1,
            line_value: Some(String::from("Name")),
            tag: GedcomLineTag::Name,
            xref_id: None,
        };

        let indi_node = GedcomTreeNode {
            children: vec![name_node],
            level: 0,
            line_value: None,
            tag: GedcomLineTag::Individual,
            xref_id: None,
        };

        let actual = GedcomTree::from(input);
        let expected = GedcomTree {
            nodes: vec![indi_node],
        };

        assert_eq!(actual, expected);
    }
}
