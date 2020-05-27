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

#[cfg(test)]
mod tests {
    use crate::models::gedcom::{
        change_node_to_date_time, GedcomLine, GedcomLineTag, GedcomTreeNodeBuilder,
    };
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

    #[test]
    fn test_change_node_to_date_time() {
        let time = GedcomLine::builder()
            .with_level(2)
            .with_optional_line_value(Some(String::from("00:00:00")))
            .with_tag(GedcomLineTag::Time)
            .build()
            .unwrap();
        let time = GedcomTreeNodeBuilder::from(time).build().unwrap();

        let date = GedcomLine::builder()
            .with_level(1)
            .with_optional_line_value(Some(String::from("1 Jan 1900")))
            .with_tag(GedcomLineTag::Date)
            .build()
            .unwrap();
        let date = GedcomTreeNodeBuilder::from(date)
            .with_children(vec![time])
            .build()
            .unwrap();

        let change = GedcomLine::builder()
            .with_level(0)
            .with_tag(GedcomLineTag::Change)
            .build()
            .unwrap();
        let change = GedcomTreeNodeBuilder::from(change)
            .with_children(vec![date])
            .build()
            .unwrap();

        let actual = change_node_to_date_time(&change);
        assert!(actual.is_ok());

        let expected = NaiveDateTime::new(
            NaiveDate::from_ymd(1900, 1, 1),
            NaiveTime::from_hms(0, 0, 0),
        );

        let actual = actual.unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_change_node_to_date_time_with_invalid_time_value() {
        let time = GedcomLine::builder()
            .with_level(2)
            .with_optional_line_value(Some(String::from("INVALID_TIME")))
            .with_tag(GedcomLineTag::Time)
            .build()
            .unwrap();
        let time = GedcomTreeNodeBuilder::from(time).build().unwrap();

        let date = GedcomLine::builder()
            .with_level(1)
            .with_optional_line_value(Some(String::from("1 Jan 1900")))
            .with_tag(GedcomLineTag::Date)
            .build()
            .unwrap();
        let date = GedcomTreeNodeBuilder::from(date)
            .with_children(vec![time])
            .build()
            .unwrap();

        let change = GedcomLine::builder()
            .with_level(0)
            .with_tag(GedcomLineTag::Change)
            .build()
            .unwrap();
        let change = GedcomTreeNodeBuilder::from(change)
            .with_children(vec![date])
            .build()
            .unwrap();

        let actual = change_node_to_date_time(&change);
        assert!(actual.is_err());

        let expected = "Gedcom Change has invalid date_time";
        let actual = actual.unwrap_err();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_change_node_to_date_time_with_invalid_date_value() {
        let time = GedcomLine::builder()
            .with_level(2)
            .with_optional_line_value(Some(String::from("00:00:00")))
            .with_tag(GedcomLineTag::Time)
            .build()
            .unwrap();
        let time = GedcomTreeNodeBuilder::from(time).build().unwrap();

        let date = GedcomLine::builder()
            .with_level(1)
            .with_optional_line_value(Some(String::from("INVALID_DATE")))
            .with_tag(GedcomLineTag::Date)
            .build()
            .unwrap();
        let date = GedcomTreeNodeBuilder::from(date)
            .with_children(vec![time])
            .build()
            .unwrap();

        let change = GedcomLine::builder()
            .with_level(0)
            .with_tag(GedcomLineTag::Change)
            .build()
            .unwrap();
        let change = GedcomTreeNodeBuilder::from(change)
            .with_children(vec![date])
            .build()
            .unwrap();

        let actual = change_node_to_date_time(&change);
        assert!(actual.is_err());

        let expected = "Gedcom Change has invalid date_time";
        let actual = actual.unwrap_err();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_change_node_to_date_time_with_invalid_date_time_value() {
        let time = GedcomLine::builder()
            .with_level(2)
            .with_optional_line_value(Some(String::from("INVALID_DATE_TIME")))
            .with_tag(GedcomLineTag::Time)
            .build()
            .unwrap();
        let time = GedcomTreeNodeBuilder::from(time).build().unwrap();

        let date = GedcomLine::builder()
            .with_level(1)
            .with_optional_line_value(Some(String::from("INVALID_DATE")))
            .with_tag(GedcomLineTag::Date)
            .build()
            .unwrap();
        let date = GedcomTreeNodeBuilder::from(date)
            .with_children(vec![time])
            .build()
            .unwrap();

        let change = GedcomLine::builder()
            .with_level(0)
            .with_tag(GedcomLineTag::Change)
            .build()
            .unwrap();
        let change = GedcomTreeNodeBuilder::from(change)
            .with_children(vec![date])
            .build()
            .unwrap();

        let actual = change_node_to_date_time(&change);
        assert!(actual.is_err());

        let expected = "Gedcom Change has invalid date_time";
        let actual = actual.unwrap_err();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_change_node_to_date_time_with_no_time_value() {
        let time = GedcomLine::builder()
            .with_level(2)
            .with_tag(GedcomLineTag::Time)
            .build()
            .unwrap();
        let time = GedcomTreeNodeBuilder::from(time).build().unwrap();

        let date = GedcomLine::builder()
            .with_level(1)
            .with_optional_line_value(Some(String::from("1 Jan 1900")))
            .with_tag(GedcomLineTag::Date)
            .build()
            .unwrap();
        let date = GedcomTreeNodeBuilder::from(date)
            .with_children(vec![time])
            .build()
            .unwrap();

        let change = GedcomLine::builder()
            .with_level(0)
            .with_tag(GedcomLineTag::Change)
            .build()
            .unwrap();
        let change = GedcomTreeNodeBuilder::from(change)
            .with_children(vec![date])
            .build()
            .unwrap();

        let actual = change_node_to_date_time(&change);
        assert!(actual.is_err());

        let expected = "Gedcom Time has no value";
        let actual = actual.unwrap_err();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_change_node_to_date_time_with_no_time() {
        let date = GedcomLine::builder()
            .with_level(1)
            .with_optional_line_value(Some(String::from("1 Jan 1900")))
            .with_tag(GedcomLineTag::Date)
            .build()
            .unwrap();
        let date = GedcomTreeNodeBuilder::from(date).build().unwrap();

        let change = GedcomLine::builder()
            .with_level(0)
            .with_tag(GedcomLineTag::Change)
            .build()
            .unwrap();
        let change = GedcomTreeNodeBuilder::from(change)
            .with_children(vec![date])
            .build()
            .unwrap();

        let actual = change_node_to_date_time(&change);
        assert!(actual.is_err());

        let expected = "Gedcom Date has no time";
        let actual = actual.unwrap_err();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_change_node_to_date_time_with_no_date_value() {
        let date = GedcomLine::builder()
            .with_level(1)
            .with_tag(GedcomLineTag::Date)
            .build()
            .unwrap();
        let date = GedcomTreeNodeBuilder::from(date).build().unwrap();

        let change = GedcomLine::builder()
            .with_level(0)
            .with_tag(GedcomLineTag::Change)
            .build()
            .unwrap();
        let change = GedcomTreeNodeBuilder::from(change)
            .with_children(vec![date])
            .build()
            .unwrap();

        let actual = change_node_to_date_time(&change);
        assert!(actual.is_err());

        let expected = "Gedcom Date has no value";
        let actual = actual.unwrap_err();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_change_node_to_date_time_with_no_date() {
        let change = GedcomLine::builder()
            .with_level(0)
            .with_tag(GedcomLineTag::Change)
            .build()
            .unwrap();
        let change = GedcomTreeNodeBuilder::from(change).build().unwrap();

        let actual = change_node_to_date_time(&change);
        assert!(actual.is_err());

        let expected = "Gedcom Change has no date";
        let actual = actual.unwrap_err();
        assert_eq!(actual, expected);
    }
}
