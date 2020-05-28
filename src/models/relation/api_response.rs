use crate::models::{
    gedcom::{change_node_to_date_time, GedcomLineTag, GedcomTree},
    relation::{Child, Family, Person, PersonBuilder},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryFrom;

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
        let mut person_id: u32 = 1;
        let mut family_id: u32 = 10_001;
        let mut child_id: u32 = 1_000_001;

        let mut childs = vec![];
        let mut familys = vec![];
        let mut persons = vec![];
        let mut persons_id_map: HashMap<&str, u32> = HashMap::new();

        for node in tree.nodes().into_iter() {
            let tag = node.tag().clone();

            match tag {
                GedcomLineTag::Individual => {
                    if let Some(xref_id) = node.xref_id() {
                        if let Ok(mut builder) = PersonBuilder::try_from(node) {
                            builder.with_id(person_id);
                            if let Ok(person) = builder.build() {
                                persons.push(person);
                                persons_id_map.insert(xref_id, person_id);
                                person_id += 1;
                            }
                        }
                    }
                }
                GedcomLineTag::Family => {
                    let mut builder = Family::builder();
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
                                    if let Some(person_id) = persons_id_map.get(xref_id.as_str()) {
                                        let child = Child::new(*person_id, child_id, family_id);
                                        childs.push(child);
                                        child_id += 1;
                                    }
                                }
                            }
                            GedcomLineTag::Husband => {
                                if let Some(xref_id) = child.line_value() {
                                    if let Some(person_id) = persons_id_map.get(xref_id.as_str()) {
                                        builder.with_father_id(*person_id);
                                    }
                                }
                            }
                            GedcomLineTag::Wife => {
                                if let Some(xref_id) = child.line_value() {
                                    if let Some(person_id) = persons_id_map.get(xref_id.as_str()) {
                                        builder.with_mother_id(*person_id);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }

                    if let Ok(family) = builder.build() {
                        familys.push(family);
                        family_id += 1;
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
