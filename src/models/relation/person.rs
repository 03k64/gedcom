use crate::{
    models::{
        gedcom::{change_node_to_date_time, GedcomLineTag, GedcomTreeNode},
        relation::{Birth, Name},
    },
    xref_id_to_numeric_id, DATE_CREATED_FORMAT,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::convert::TryFrom;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Person {
    date_created: NaiveDateTime,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    facts: Option<Vec<Birth>>,
    gender: Gender,
    id: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    is_living: Option<bool>,
    names: Vec<Name>,
}

impl TryFrom<&GedcomTreeNode> for Person {
    type Error = &'static str;

    fn try_from(node: &GedcomTreeNode) -> Result<Self, Self::Error> {
        let mut builder = Person::builder();

        if let Some(xref_id) = node.xref_id() {
            let id = xref_id_to_numeric_id(xref_id.as_str()).unwrap_or_default();
            builder.with_id(id);
        }

        for child in node.children().into_iter() {
            match child.tag() {
                &GedcomLineTag::Birth => {
                    let birth = Birth::from(child);
                    builder.with_birth(birth);
                }
                &GedcomLineTag::Change => {
                    let date_created = change_node_to_date_time(child)?;
                    builder.with_date_created(date_created);
                }
                &GedcomLineTag::Sex => {
                    let sex = child.line_value().as_ref().map_or("", |v| v.as_str());
                    let gender = Gender::from(sex);
                    builder.with_gender(gender);
                }
                &GedcomLineTag::Name => {
                    let name = Name::from(child);
                    builder.with_name(name);
                }
                _ => {}
            }
        }
        builder.build()
    }
}

impl Person {
    pub fn builder() -> PersonBuilder {
        PersonBuilder::new()
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

#[derive(Clone, Debug, Default)]
pub struct PersonBuilder {
    date_created: Option<NaiveDateTime>,
    facts: Option<Vec<Birth>>,
    gender: Option<Gender>,
    id: Option<u32>,
    is_living: Option<bool>,
    names: Option<Vec<Name>>,
}

impl PersonBuilder {
    fn new() -> Self {
        Default::default()
    }

    pub fn build(&mut self) -> Result<Person, &'static str> {
        let date_created = self.date_created.ok_or("Person must have a date_created")?;
        let gender = self.gender.ok_or("Person must have a gender")?;
        let id = self.id.ok_or("Person must have an id")?;
        let names = self.names.take().unwrap_or_default();

        let person = Person {
            date_created,
            gender,
            id,
            names,
            facts: self.facts.take(),
            is_living: Some(true),
        };

        Ok(person)
    }

    pub fn is_living(&mut self) -> &mut Self {
        self.is_living = Some(true);
        self
    }

    pub fn with_birth(&mut self, birth: Birth) -> &mut Self {
        match self.facts {
            Some(ref mut facts) => facts.push(birth),
            None => self.facts = Some(vec![birth]),
        };
        self
    }

    pub fn with_date_created_from_str(&mut self, date_created: &str) -> &mut Self {
        match NaiveDateTime::parse_from_str(date_created, DATE_CREATED_FORMAT) {
            Ok(date_created) => {
                self.date_created = Some(date_created);
                self
            }
            Err(_) => self,
        }
    }

    pub fn with_date_created(&mut self, date_created: NaiveDateTime) -> &mut Self {
        self.date_created = Some(date_created);
        self
    }

    pub fn with_gender(&mut self, gender: Gender) -> &mut Self {
        self.gender = Some(gender);
        self
    }

    pub fn with_id(&mut self, id: u32) -> &mut Self {
        self.id = Some(id);
        self
    }

    pub fn with_name(&mut self, name: Name) -> &mut Self {
        match self.names {
            Some(ref mut names) => names.push(name),
            None => self.names = Some(vec![name]),
        };
        self
    }
}

#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, PartialEq, Serialize_repr)]
#[repr(u8)]
pub enum Gender {
    Male = 1,
    Female = 2,
    Other = 3,
}

impl From<&str> for Gender {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "m" => Gender::Male,
            "f" => Gender::Female,
            _ => Gender::Other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Gender, Person};
    use crate::models::relation::{Birth, Name, Place};
    use serde_json;

    #[test]
    fn can_deserialize_person() {
        let name = Name::builder()
            .with_given_names("Gavin")
            .with_surnames("Henderson")
            .build();

        let expected = Person::builder()
            .is_living()
            .with_date_created_from_str("2020-04-15T16:44:00")
            .with_gender(Gender::Male)
            .with_id(1)
            .with_name(name)
            .build()
            .unwrap();

        let input = r#"{ "Id": 1, "Gender": 1, "IsLiving": true, "DateCreated": "2020-04-15T16:44:00", "Names": [ { "FactTypeId": 100, "GivenNames": "Gavin", "Surnames": "Henderson" } ] }"#;
        let actual: Person = serde_json::from_str(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn can_serialize_person() {
        let name = Name::builder()
            .with_given_names("Gavin")
            .with_surnames("Henderson")
            .build();

        let input = Person::builder()
            .with_date_created_from_str("2020-04-15T16:44:00")
            .with_gender(Gender::Male)
            .with_id(1)
            .with_name(name)
            .build()
            .unwrap();

        let actual = serde_json::json!(input).to_string();
        let expected = r#"{"DateCreated":"2020-04-15T16:44:00","Gender":1,"Id":1,"IsLiving":true,"Names":[{"FactTypeId":100,"GivenNames":"Gavin","Surnames":"Henderson"}]}"#;

        assert_eq!(actual, expected);
    }

    #[test]
    fn can_deserialize_person_with_facts() {
        let place = Place::new("Dundee");
        let birth = Birth::builder().is_preferred().with_place(place).build();
        let name = Name::builder()
            .with_given_names("Jane")
            .with_surnames("Reed")
            .build();

        let expected = Person::builder()
            .with_birth(birth)
            .with_date_created_from_str("2020-04-15T16:39:15")
            .with_gender(Gender::Female)
            .with_id(1)
            .with_name(name)
            .build()
            .unwrap();

        let input = r#"{
            "Gender": 2,
            "Id": 1,
            "DateCreated": "2020-04-15T16:39:15",
            "Names": [
              {
                "FactTypeId": 100,
                "GivenNames": "Jane",
                "Surnames": "Reed"
              }
            ],
            "Facts": [
              {
                "FactTypeId": 405,
                "Place": {
                  "PlaceName": "Dundee"
                },
                "Preferred": true
              }
            ],
            "IsLiving": true
        }"#;

        let actual: Person = serde_json::from_str(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn can_serialize_person_with_facts() {
        let place = Place::new("Dundee");
        let birth = Birth::builder().is_preferred().with_place(place).build();
        let name = Name::builder()
            .with_given_names("Jane")
            .with_surnames("Reed")
            .build();

        let input = Person::builder()
            .with_birth(birth)
            .with_date_created_from_str("2020-04-15T16:39:15")
            .with_gender(Gender::Female)
            .with_id(1)
            .with_name(name)
            .build()
            .unwrap();

        let expected = r#"{"DateCreated":"2020-04-15T16:39:15","Facts":[{"FactTypeId":405,"Place":{"PlaceName":"Dundee"},"Preferred":true}],"Gender":2,"Id":1,"IsLiving":true,"Names":[{"FactTypeId":100,"GivenNames":"Jane","Surnames":"Reed"}]}"#;
        let actual = serde_json::json!(input).to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn can_deserialize_person_with_is_living() {
        let name = Name::builder()
            .with_given_names("Gavin")
            .with_surnames("Henderson")
            .build();

        let expected = Person::builder()
            .is_living()
            .with_date_created_from_str("2020-04-15T16:19:21")
            .with_gender(Gender::Male)
            .with_id(1)
            .with_name(name)
            .build()
            .unwrap();

        let input = r#"{
            "Id": 1,
            "IsLiving": true,
            "Gender": 1,
            "DateCreated": "2020-04-15T16:19:21",
            "Names": [
              {
                "FactTypeId": 100,
                "GivenNames": "Gavin",
                "Surnames": "Henderson"
              }
            ]
          }"#;
        let actual: Person = serde_json::from_str(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn can_serialize_person_with_is_living() {
        let name = Name::builder()
            .with_given_names("Gavin")
            .with_surnames("Henderson")
            .build();

        let input = Person::builder()
            .is_living()
            .with_date_created_from_str("2020-04-15T16:19:21")
            .with_gender(Gender::Male)
            .with_id(1)
            .with_name(name)
            .build()
            .unwrap();

        let expected = r#"{"DateCreated":"2020-04-15T16:19:21","Gender":1,"Id":1,"IsLiving":true,"Names":[{"FactTypeId":100,"GivenNames":"Gavin","Surnames":"Henderson"}]}"#;
        let actual = serde_json::json!(input).to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn can_deserialize_person_with_facts_and_is_living() {
        let place = Place::new("Dundee");
        let birth = Birth::builder().is_preferred().with_place(place).build();
        let name = Name::builder()
            .with_given_names("Jane")
            .with_surnames("Reed")
            .build();

        let expected = Person::builder()
            .is_living()
            .with_birth(birth)
            .with_date_created_from_str("2020-04-15T16:39:15")
            .with_gender(Gender::Female)
            .with_id(1)
            .with_name(name)
            .build()
            .unwrap();

        let input = r#"{
            "Id": 1,
            "IsLiving": true,
            "Gender": 2,
            "DateCreated": "2020-04-15T16:39:15",
            "Names": [
              {
                "FactTypeId": 100,
                "GivenNames": "Jane",
                "Surnames": "Reed"
              }
            ],
            "Facts": [
              {
                "FactTypeId": 405,
                "Place": {
                  "PlaceName": "Dundee"
                },
                "Preferred": true
              }
            ]
        }"#;

        let actual: Person = serde_json::from_str(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn can_serialize_person_with_facts_and_is_living() {
        let place = Place::new("Dundee");
        let birth = Birth::builder().is_preferred().with_place(place).build();
        let name = Name::builder()
            .with_given_names("Jane")
            .with_surnames("Reed")
            .build();

        let input = Person::builder()
            .is_living()
            .with_birth(birth)
            .with_date_created_from_str("2020-04-15T16:39:15")
            .with_gender(Gender::Female)
            .with_id(1)
            .with_name(name)
            .build()
            .unwrap();

        let expected = r#"{"DateCreated":"2020-04-15T16:39:15","Facts":[{"FactTypeId":405,"Place":{"PlaceName":"Dundee"},"Preferred":true}],"Gender":2,"Id":1,"IsLiving":true,"Names":[{"FactTypeId":100,"GivenNames":"Jane","Surnames":"Reed"}]}"#;
        let actual = serde_json::json!(input).to_string();

        assert_eq!(actual, expected);
    }
}
