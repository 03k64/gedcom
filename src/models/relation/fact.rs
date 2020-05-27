use crate::models::gedcom::{GedcomLineTag, GedcomTreeNode};
use chrono::NaiveDate;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};

const DATE_DETAIL_FORMAT: &'static str = "%-d %b %Y";

#[derive(Clone, Copy, Deserialize_repr, Serialize_repr)]
#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
#[repr(u16)]
pub enum FactTypeId {
    Birth = 405,
    Name = 100,
}

#[derive(Deserialize, Serialize)]
#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct Birth {
    #[serde(
        default,
        deserialize_with = "Birth::deserialize_opt_date_detail",
        serialize_with = "Birth::serialize_date_detail",
        skip_serializing_if = "Option::is_none"
    )]
    date_detail: Option<NaiveDate>,
    fact_type_id: FactTypeId,
    #[serde(skip_serializing_if = "Option::is_none")]
    place: Option<Place>,
    preferred: bool,
}

impl From<&GedcomTreeNode> for Birth {
    fn from(node: &GedcomTreeNode) -> Self {
        let mut builder = Birth::builder();

        for child in node.children().into_iter() {
            let tag = child.tag().clone();

            match tag {
                GedcomLineTag::Custom(custom) => {
                    if custom.clone().as_str() == "_PRIM"
                        && child
                            .line_value()
                            .as_ref()
                            .map(|v| v.as_str())
                            .unwrap_or("N")
                            == "Y"
                    {
                        builder.is_preferred();
                    }
                }
                GedcomLineTag::Date => {
                    if let Some(date) = child.line_value().as_ref() {
                        builder.with_date_detail_from_str(date.as_str());
                    }
                }
                GedcomLineTag::Place => {
                    if let Some(place) = child.line_value().as_ref() {
                        let place = Place::new(place.as_str());
                        builder.with_place(place);
                    }
                }
                _ => {}
            }
        }

        builder.build()
    }
}

impl Birth {
    pub fn builder() -> BirthBuilder {
        BirthBuilder::new()
    }

    fn deserialize_opt_date_detail<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wrapper(#[serde(deserialize_with = "Birth::deserialize_date_detail")] NaiveDate);

        let d = Option::deserialize(deserializer)?;
        Ok(d.map(|Wrapper(d)| d))
    }

    fn deserialize_date_detail<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, DATE_DETAIL_FORMAT).map_err(de::Error::custom)
    }

    fn serialize_date_detail<S>(
        date_detail: &Option<NaiveDate>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date_detail.unwrap().format(DATE_DETAIL_FORMAT));
        serializer.serialize_str(&s)
    }
}

#[derive(Default)]
pub struct BirthBuilder {
    date_detail: Option<NaiveDate>,
    place: Option<Place>,
    preferred: Option<bool>,
}

impl BirthBuilder {
    fn new() -> Self {
        Default::default()
    }

    pub fn build(&mut self) -> Birth {
        Birth {
            date_detail: self.date_detail.take(),
            fact_type_id: FactTypeId::Birth,
            place: self.place.take(),
            preferred: self.preferred.unwrap(),
        }
    }

    pub fn is_preferred(&mut self) -> &mut Self {
        self.preferred = Some(true);
        self
    }

    pub fn with_date_detail_from_str(&mut self, date_detail: &str) -> &mut Self {
        match NaiveDate::parse_from_str(date_detail, DATE_DETAIL_FORMAT) {
            Ok(date_detail) => {
                self.date_detail = Some(date_detail);
                self
            }
            Err(_) => self,
        }
    }

    pub fn with_place(&mut self, place: Place) -> &mut Self {
        self.place = Some(place);
        self
    }
}

#[derive(Deserialize, Serialize)]
#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct Name {
    fact_type_id: FactTypeId,
    given_names: Option<String>,
    surnames: Option<String>,
}

impl From<&GedcomTreeNode> for Name {
    fn from(node: &GedcomTreeNode) -> Self {
        let mut builder = Self::builder();

        for child in node.children().into_iter() {
            match child.tag() {
                &GedcomLineTag::GivenName => {
                    if let Some(given_names) = child.line_value().as_ref() {
                        builder.with_given_names(given_names.as_str());
                    }
                }
                &GedcomLineTag::Surname => {
                    if let Some(surnames) = child.line_value().as_ref() {
                        builder.with_surnames(surnames.as_str());
                    }
                }
                _ => {}
            }
        }

        builder.build()
    }
}

impl Name {
    pub fn builder() -> NameBuilder {
        NameBuilder::new()
    }
}

#[derive(Default)]
pub struct NameBuilder {
    given_names: Option<String>,
    surnames: Option<String>,
}

impl NameBuilder {
    fn new() -> Self {
        Default::default()
    }

    pub fn build(&mut self) -> Name {
        Name {
            fact_type_id: FactTypeId::Name,
            given_names: self.given_names.take(),
            surnames: self.surnames.take(),
        }
    }

    pub fn with_given_names(&mut self, given_names: &str) -> &mut Self {
        self.given_names = Some(given_names.to_owned());
        self
    }

    pub fn with_surnames(&mut self, surnames: &str) -> &mut Self {
        self.surnames = Some(surnames.to_owned());
        self
    }
}

#[derive(Deserialize, Serialize)]
#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct Place {
    place_name: String,
}

impl Place {
    pub fn new(place_name: &str) -> Self {
        Self {
            place_name: String::from(place_name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Birth, Name, Place};
    use serde_json;

    #[test]
    fn can_deserialize_birth() {
        let expected = Birth::builder().is_preferred().build();

        let input = r#"{ "FactTypeId": 405, "Preferred": true }"#;
        let actual: Birth = serde_json::from_str(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn can_serialize_birth() {
        let input = Birth::builder().is_preferred().build();

        let actual = serde_json::json!(input).to_string();
        let expected = r#"{"FactTypeId":405,"Preferred":true}"#;

        assert_eq!(actual, expected);
    }

    #[test]
    fn can_deserialize_birth_with_date_detail() {
        let expected = Birth::builder()
            .is_preferred()
            .with_date_detail_from_str("1 Jan 1990")
            .build();

        let input = r#"{ "DateDetail": "1 Jan 1990", "FactTypeId": 405, "Preferred": true }"#;
        let actual: Birth = serde_json::from_str(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn can_serialize_birth_with_date_detail() {
        let input = Birth::builder()
            .is_preferred()
            .with_date_detail_from_str("1 Jan 1990")
            .build();

        let actual = serde_json::json!(input).to_string();
        let expected = r#"{"DateDetail":"1 Jan 1990","FactTypeId":405,"Preferred":true}"#;

        assert_eq!(actual, expected);
    }

    #[test]
    fn can_deserialize_birth_with_place() {
        let place = Place::new("Dundee");
        let expected = Birth::builder().is_preferred().with_place(place).build();

        let input =
            r#"{ "FactTypeId": 405, "Place": { "PlaceName": "Dundee" }, "Preferred": true }"#;
        let actual: Birth = serde_json::from_str(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn can_serialize_birth_with_place() {
        let place = Place::new("Dundee");
        let input = Birth::builder().is_preferred().with_place(place).build();

        let actual = serde_json::json!(input).to_string();
        let expected = r#"{"FactTypeId":405,"Place":{"PlaceName":"Dundee"},"Preferred":true}"#;

        assert_eq!(actual, expected);
    }

    #[test]
    fn can_deserialize_birth_with_date_detail_and_place() {
        let place = Place::new("Dundee");
        let expected = Birth::builder()
            .is_preferred()
            .with_date_detail_from_str("1 Jan 1990")
            .with_place(place)
            .build();

        let input = r#"{ "DateDetail": "1 Jan 1990", "FactTypeId": 405, "Place": { "PlaceName": "Dundee" }, "Preferred": true }"#;
        let actual: Birth = serde_json::from_str(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn can_serialize_birth_with_date_detail_and_place() {
        let place = Place::new("Dundee");
        let input = Birth::builder()
            .is_preferred()
            .with_date_detail_from_str("1 Jan 1990")
            .with_place(place)
            .build();

        let actual = serde_json::json!(input).to_string();
        let expected = r#"{"DateDetail":"1 Jan 1990","FactTypeId":405,"Place":{"PlaceName":"Dundee"},"Preferred":true}"#;

        assert_eq!(actual, expected);
    }

    #[test]
    fn can_deserialize_name() {
        let expected = Name::builder()
            .with_given_names("Gavin")
            .with_surnames("Henderson")
            .build();

        let input = r#"{ "FactTypeId": 100, "GivenNames": "Gavin", "Surnames": "Henderson" }"#;
        let actual: Name = serde_json::from_str(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn can_serialize_name() {
        let input = Name::builder()
            .with_given_names("Gavin")
            .with_surnames("Henderson")
            .build();

        let actual = serde_json::json!(input).to_string();
        let expected = r#"{"FactTypeId":100,"GivenNames":"Gavin","Surnames":"Henderson"}"#;

        assert_eq!(actual, expected);
    }

    #[test]
    fn can_deserialize_place() {
        let expected = Place::new("Dundee");

        let input = r#"{ "PlaceName": "Dundee" }"#;
        let actual: Place = serde_json::from_str(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn can_serialize_place() {
        let input = Place::new("Dundee");

        let actual = serde_json::json!(input).to_string();
        let expected = r#"{"PlaceName":"Dundee"}"#;

        assert_eq!(actual, expected);
    }
}
