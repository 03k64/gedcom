use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct Child {
    child_id: u32,
    family_id: u32,
    id: u32,
    relationship_to_father: u32,
    relationship_to_mother: u32,
}

impl Default for Child {
    fn default() -> Self {
        Self {
            child_id: 0,
            family_id: 100,
            id: 1000,
            relationship_to_father: 1,
            relationship_to_mother: 1,
        }
    }
}

impl Child {
    pub fn new(child_id: u32, id: u32, family_id: u32) -> Self {
        Self {
            child_id,
            id,
            family_id,
            ..Default::default()
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct Family {
    date_created: NaiveDateTime,
    father_id: u32,
    id: u32,
    mother_id: u32,
}

impl Family {
    pub fn builder() -> FamilyBuilder {
        FamilyBuilder::new()
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

#[derive(Default)]
pub struct FamilyBuilder {
    date_created: Option<NaiveDateTime>,
    father_id: Option<u32>,
    id: Option<u32>,
    mother_id: Option<u32>,
}

impl FamilyBuilder {
    fn new() -> Self {
        Default::default()
    }

    pub fn build(&mut self) -> Result<Family, &str> {
        let date_created = self
            .date_created
            .take()
            .ok_or("Family must have a date_created specified")?;

        let father_id = self
            .father_id
            .take()
            .ok_or("Family must have a father_id specified")?;

        let id = self.id.take().ok_or("Family must have an id specified")?;

        let mother_id = self
            .mother_id
            .take()
            .ok_or("Family must have a mother_id specified")?;

        let family = Family {
            date_created,
            father_id,
            id,
            mother_id,
        };

        Ok(family)
    }

    pub fn with_date_created(&mut self, date_created: NaiveDateTime) -> &mut Self {
        self.date_created = Some(date_created);
        self
    }

    pub fn with_father_id(&mut self, father_id: u32) -> &mut Self {
        self.father_id = Some(father_id);
        self
    }

    pub fn with_id(&mut self, id: u32) -> &mut Self {
        self.id = Some(id);
        self
    }

    pub fn with_mother_id(&mut self, mother_id: u32) -> &mut Self {
        self.mother_id = Some(mother_id);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::{Child, Family};
    use crate::DATE_CREATED_FORMAT;
    use chrono::NaiveDateTime;
    use serde_json;

    #[test]
    fn can_deserialize_child() {
        let expected = Child::new(1, 1, 1);
        let input = r#"{ "ChildId": 1, "Id": 1, "FamilyId": 1, "RelationshipToFather": 1, "RelationshipToMother": 1 }"#;
        let actual: Child = serde_json::from_str(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn can_serialize_child() {
        let input = Child::new(1, 1, 1);

        let actual = serde_json::json!(input).to_string();
        let expected = r#"{"ChildId":1,"FamilyId":1,"Id":1,"RelationshipToFather":1,"RelationshipToMother":1}"#;

        assert_eq!(actual, expected);
    }

    #[test]
    fn can_deserialize_family() {
        let date_created =
            NaiveDateTime::parse_from_str("2020-04-15T16:40:57", DATE_CREATED_FORMAT).unwrap();
        let expected = Family::builder()
            .with_date_created(date_created)
            .with_father_id(1)
            .with_id(1)
            .with_mother_id(1)
            .build()
            .unwrap();

        let input =
            r#"{ "DateCreated": "2020-04-15T16:40:57", "FatherId": 1, "Id": 1, "MotherId": 1 }"#;
        let actual: Family = serde_json::from_str(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn can_serialize_family() {
        let date_created =
            NaiveDateTime::parse_from_str("2020-04-15T16:40:57", DATE_CREATED_FORMAT).unwrap();
        let input = Family::builder()
            .with_date_created(date_created)
            .with_father_id(1)
            .with_id(1)
            .with_mother_id(1)
            .build()
            .unwrap();

        let actual = serde_json::json!(input).to_string();
        let expected = r#"{"DateCreated":"2020-04-15T16:40:57","FatherId":1,"Id":1,"MotherId":1}"#;

        assert_eq!(actual, expected);
    }
}
