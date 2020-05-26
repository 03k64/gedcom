mod line;
mod tree;

pub use self::line::{GedcomLine, GedcomLineBuilder, GedcomLineTag};
pub use self::tree::{GedcomTree, GedcomTreeNode, GedcomTreeNodeBuilder};
use chrono::NaiveDateTime;

pub fn change_node_to_date_time(node: &GedcomTreeNode) -> Result<NaiveDateTime, &'static str> {
    let date_node = node.children().get(0).ok_or("Gedcom Change has no date")?;
    let date = date_node
        .line_value()
        .as_ref()
        .ok_or("Gedcom Date has no value")?;

    let time_node = date_node
        .children()
        .get(0)
        .ok_or("Gedcom Date has no time")?;

    let time = time_node
        .line_value()
        .as_ref()
        .ok_or("Gedcom Time has no value")?;

    let date_time = format!("{} {}", date, time);
    let date_time = NaiveDateTime::parse_from_str(&date_time, "%-d %b %Y %H:%M:%S")
        .map_err(|_| "Gedcom Change has invalid date_time")?;

    Ok(date_time)
}
