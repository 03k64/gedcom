use crate::{
    models::{
        gedcom::{change_node_to_date_time, GedcomLineTag, GedcomTree},
        relation::{Child, Family, Person},
    },
    xref_id_to_numeric_id,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryFrom;

const FAMILY_ID_SEED: u32 = 100;
const CHILD_ID_SEED: u32 = 1000;

#[derive(Default, Deserialize, Serialize)]
#[cfg_attr(test, derive(Debug))]
#[serde(rename_all = "PascalCase")]
pub struct ApiResponse {
    childs: Vec<Child>,
    fact_types: Vec<()>,
    familys: Vec<Family>,
    master_sources: Vec<()>,
    medias: Vec<()>,
    persons: Vec<Person>,
    source_repos: Vec<()>,
}

impl From<GedcomTree> for ApiResponse {
    fn from(tree: GedcomTree) -> Self {
        let mut childs = vec![];
        let mut familys = vec![];
        let mut persons = vec![];
        let mut persons_id_map: HashMap<&str, u32> = HashMap::new();

        for node in tree.nodes().into_iter() {
            let tag = node.tag().clone();

            match tag {
                GedcomLineTag::Individual => {
                    if let Some(xref_id) = node.xref_id() {
                        if let Ok(person) = Person::try_from(node) {
                            persons_id_map.insert(xref_id, person.id());
                            persons.push(person);
                        }
                    }
                }
                GedcomLineTag::Family => {
                    if let Some(xref_id) = node.xref_id() {
                        let mut builder = Family::builder();
                        let family_id =
                            FAMILY_ID_SEED + xref_id_to_numeric_id(xref_id).unwrap_or_default();
                        builder.with_id(family_id);

                        for child in node.children().into_iter() {
                            let tag = child.tag().clone();

                            match tag {
                                GedcomLineTag::Change => {
                                    if let Ok(date_created) = change_node_to_date_time(child) {
                                        builder.with_date_created(date_created);
                                    }
                                }
                                GedcomLineTag::Child => {
                                    if let Some(xref_id) = child.line_value() {
                                        if let Some(person_id) =
                                            persons_id_map.get(xref_id.as_str())
                                        {
                                            let child_id = CHILD_ID_SEED + *person_id;
                                            let child = Child::new(*person_id, child_id, family_id);
                                            childs.push(child);
                                        }
                                    }
                                }
                                GedcomLineTag::Husband => {
                                    if let Some(xref_id) = child.line_value() {
                                        if let Some(person_id) =
                                            persons_id_map.get(xref_id.as_str())
                                        {
                                            builder.with_father_id(*person_id);
                                        }
                                    }
                                }
                                GedcomLineTag::Wife => {
                                    if let Some(xref_id) = child.line_value() {
                                        if let Some(person_id) =
                                            persons_id_map.get(xref_id.as_str())
                                        {
                                            builder.with_mother_id(*person_id);
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }

                        if let Ok(family) = builder.build() {
                            familys.push(family);
                        }
                    }
                }
                _ => {}
            }
        }

        Self {
            childs,
            familys,
            persons,
            ..Default::default()
        }
    }
}
