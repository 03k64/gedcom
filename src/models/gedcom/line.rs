use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

lazy_static! {
    static ref VALID_CUSTOM_TAG: Regex = Regex::new(r#"^(_[A-Za-z0-9_]+)$"#).unwrap();
}

#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub struct GedcomLine {
    level: u8,
    line_value: Option<String>,
    tag: GedcomLineTag,
    xref_id: Option<String>,
}

impl GedcomLine {
    pub fn builder() -> GedcomLineBuilder {
        GedcomLineBuilder::new()
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

#[derive(Default)]
pub struct GedcomLineBuilder {
    level: Option<u8>,
    optional_line_value: Option<String>,
    tag: Option<GedcomLineTag>,
    optional_xref_id: Option<String>,
}

impl GedcomLineBuilder {
    fn new() -> Self {
        Default::default()
    }

    pub fn build(&mut self) -> Result<GedcomLine, &str> {
        let level = self.level.take().ok_or("GedcomLine must have a level")?;
        let tag = self.tag.take().ok_or("GedcomLine must have a tag")?;

        let gedcom_line = GedcomLine {
            level,
            tag,
            line_value: self.optional_line_value.take(),
            xref_id: self.optional_xref_id.take(),
        };

        Ok(gedcom_line)
    }

    pub fn with_level(&mut self, level: u8) -> &mut Self {
        self.level = Some(level);
        self
    }

    pub fn with_optional_line_value(&mut self, optional_line_value: Option<String>) -> &mut Self {
        self.optional_line_value = optional_line_value;
        self
    }

    pub fn with_tag(&mut self, tag: GedcomLineTag) -> &mut Self {
        self.tag = Some(tag);
        self
    }

    pub fn with_optional_xref_id(&mut self, optional_xref_id: Option<String>) -> &mut Self {
        self.optional_xref_id = optional_xref_id;
        self
    }
}

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub enum GedcomLineTag {
    Abbreviation,
    Address,
    Address1,
    Address2,
    Adoption,
    AncestralFileNumber,
    Age,
    Agency,
    Alias,
    Ancestors,
    AncestorInterest,
    Annulment,
    Associates,
    Author,
    BaptismLds,
    Baptism,
    BarMitzvah,
    BasMitzvah,
    Birth,
    Blessing,
    Burial,
    CallNumber,
    Caste,
    Cause,
    Census,
    Change,
    Character,
    Child,
    Christening,
    AdultChristening,
    City,
    Concatenation,
    Confirmation,
    ConfirmationLds,
    Continued,
    Copyright,
    Corporate,
    Cremation,
    Country,
    Custom(String),
    Data,
    Date,
    Death,
    Descendants,
    DescendantInterest,
    Destination,
    Divorce,
    DivorceFiled,
    PhysicalDescription,
    Education,
    Email,
    Emigration,
    Endowment,
    Engagement,
    Event,
    Fact,
    Family,
    FamilyChild,
    FamilyFile,
    FamilySpouse,
    Facsimile,
    FirstCommunion,
    File,
    Format,
    Phonetic,
    Gedcom,
    GivenName,
    Graduation,
    Header,
    Husband,
    IdentityNumber,
    Immigration,
    Individual,
    Language,
    Latitude,
    Longitude,
    Map,
    MarriageBanns,
    MarriageContract,
    MarriageLicense,
    Marriage,
    MarriageSettlement,
    Media,
    Name,
    Nationality,
    Naturalisation,
    ChildrenCount,
    Nickname,
    MarriageCount,
    Note,
    NamePrefix,
    NameSuffix,
    Object,
    Occupation,
    Ordinance,
    Ordination,
    Page,
    Pedigree,
    Phone,
    Place,
    PostalCode,
    Probate,
    Property,
    Publication,
    QualityOfData,
    Reference,
    Relationship,
    Religion,
    Repository,
    Residence,
    Restriction,
    Retirement,
    RecordFileNumber,
    RecordIdNumber,
    Role,
    Romanised,
    Sex,
    SealingChild,
    SealingSpouse,
    Source,
    SurnamePrefix,
    SocialSecurityNumber,
    State,
    Status,
    Submitter,
    Submission,
    Surname,
    Temple,
    Text,
    Time,
    Title,
    Trailer,
    Type,
    Version,
    Wife,
    Will,
    Web,
}

impl FromStr for GedcomLineTag {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "ABBR" => Ok(Self::Abbreviation),
            "ADDR" => Ok(Self::Address),
            "ADR1" => Ok(Self::Address1),
            "ADR2" => Ok(Self::Address2),
            "ADOP" => Ok(Self::Adoption),
            "AFN" => Ok(Self::AncestralFileNumber),
            "AGE" => Ok(Self::Age),
            "AGNC" => Ok(Self::Agency),
            "ALIA" => Ok(Self::Alias),
            "ANCE" => Ok(Self::Ancestors),
            "ANCI" => Ok(Self::AncestorInterest),
            "ANUL" => Ok(Self::Annulment),
            "ASSO" => Ok(Self::Associates),
            "AUTH" => Ok(Self::Author),
            "BAPL" => Ok(Self::BaptismLds),
            "BAPM" => Ok(Self::Baptism),
            "BARM" => Ok(Self::BarMitzvah),
            "BASM" => Ok(Self::BasMitzvah),
            "BIRT" => Ok(Self::Birth),
            "BLES" => Ok(Self::Blessing),
            "BURI" => Ok(Self::Burial),
            "CALN" => Ok(Self::CallNumber),
            "CAST" => Ok(Self::Caste),
            "CAUS" => Ok(Self::Cause),
            "CENS" => Ok(Self::Census),
            "CHAN" => Ok(Self::Change),
            "CHAR" => Ok(Self::Character),
            "CHIL" => Ok(Self::Child),
            "CHR" => Ok(Self::Christening),
            "CHRA" => Ok(Self::AdultChristening),
            "CITY" => Ok(Self::City),
            "CONC" => Ok(Self::Concatenation),
            "CONF" => Ok(Self::Confirmation),
            "CONL" => Ok(Self::ConfirmationLds),
            "CONT" => Ok(Self::Continued),
            "COPR" => Ok(Self::Copyright),
            "CORP" => Ok(Self::Corporate),
            "CREM" => Ok(Self::Cremation),
            "CTRY" => Ok(Self::Country),
            "DATA" => Ok(Self::Data),
            "DATE" => Ok(Self::Date),
            "DEAT" => Ok(Self::Death),
            "DESC" => Ok(Self::Descendants),
            "DESI" => Ok(Self::DescendantInterest),
            "DEST" => Ok(Self::Destination),
            "DIV" => Ok(Self::Divorce),
            "DIVF" => Ok(Self::DivorceFiled),
            "DSCR" => Ok(Self::PhysicalDescription),
            "EDUC" => Ok(Self::Education),
            "EMAI" => Ok(Self::Email),
            "EMIG" => Ok(Self::Emigration),
            "ENDL" => Ok(Self::Endowment),
            "ENGA" => Ok(Self::Engagement),
            "EVEN" => Ok(Self::Event),
            "FACT" => Ok(Self::Fact),
            "FAM" => Ok(Self::Family),
            "FAMC" => Ok(Self::FamilyChild),
            "FAMF" => Ok(Self::FamilyFile),
            "FAMS" => Ok(Self::FamilySpouse),
            "FAX" => Ok(Self::Facsimile),
            "FCOM" => Ok(Self::FirstCommunion),
            "FILE" => Ok(Self::File),
            "FORM" => Ok(Self::Format),
            "FONE" => Ok(Self::Phonetic),
            "GEDC" => Ok(Self::Gedcom),
            "GIVN" => Ok(Self::GivenName),
            "GRAD" => Ok(Self::Graduation),
            "HEAD" => Ok(Self::Header),
            "HUSB" => Ok(Self::Husband),
            "IDNO" => Ok(Self::IdentityNumber),
            "IMMI" => Ok(Self::Immigration),
            "INDI" => Ok(Self::Individual),
            "LANG" => Ok(Self::Language),
            "LATI" => Ok(Self::Latitude),
            "LONG" => Ok(Self::Longitude),
            "MAP" => Ok(Self::Map),
            "MARB" => Ok(Self::MarriageBanns),
            "MARC" => Ok(Self::MarriageContract),
            "MARL" => Ok(Self::MarriageLicense),
            "MARR" => Ok(Self::Marriage),
            "MARS" => Ok(Self::MarriageSettlement),
            "MEDI" => Ok(Self::Media),
            "NAME" => Ok(Self::Name),
            "NATI" => Ok(Self::Nationality),
            "NATU" => Ok(Self::Naturalisation),
            "NCHI" => Ok(Self::ChildrenCount),
            "NICK" => Ok(Self::Nickname),
            "NMR" => Ok(Self::MarriageCount),
            "NOTE" => Ok(Self::Note),
            "NPFX" => Ok(Self::NamePrefix),
            "NSFX" => Ok(Self::NameSuffix),
            "OBJE" => Ok(Self::Object),
            "OCCU" => Ok(Self::Occupation),
            "ORDI" => Ok(Self::Ordinance),
            "ORDN" => Ok(Self::Ordination),
            "PAGE" => Ok(Self::Page),
            "PEDI" => Ok(Self::Pedigree),
            "PHON" => Ok(Self::Phone),
            "PLAC" => Ok(Self::Place),
            "POST" => Ok(Self::PostalCode),
            "PROB" => Ok(Self::Probate),
            "PROP" => Ok(Self::Property),
            "PUBL" => Ok(Self::Publication),
            "QUAY" => Ok(Self::QualityOfData),
            "REFN" => Ok(Self::Reference),
            "RELA" => Ok(Self::Relationship),
            "RELI" => Ok(Self::Religion),
            "REPO" => Ok(Self::Repository),
            "RESI" => Ok(Self::Residence),
            "RESN" => Ok(Self::Restriction),
            "RETI" => Ok(Self::Retirement),
            "RFN" => Ok(Self::RecordFileNumber),
            "RIN" => Ok(Self::RecordIdNumber),
            "ROLE" => Ok(Self::Role),
            "ROMN" => Ok(Self::Romanised),
            "SEX" => Ok(Self::Sex),
            "SLGC" => Ok(Self::SealingChild),
            "SLGS" => Ok(Self::SealingSpouse),
            "SOUR" => Ok(Self::Source),
            "SPFX" => Ok(Self::SurnamePrefix),
            "SSN" => Ok(Self::SocialSecurityNumber),
            "STAE" => Ok(Self::State),
            "STAT" => Ok(Self::Status),
            "SUBM" => Ok(Self::Submitter),
            "SUBN" => Ok(Self::Submission),
            "SURN" => Ok(Self::Surname),
            "TEMP" => Ok(Self::Temple),
            "TEXT" => Ok(Self::Text),
            "TIME" => Ok(Self::Time),
            "TITL" => Ok(Self::Title),
            "TRLR" => Ok(Self::Trailer),
            "TYPE" => Ok(Self::Type),
            "VERS" => Ok(Self::Version),
            "WIFE" => Ok(Self::Wife),
            "WILL" => Ok(Self::Will),
            "WWW" => Ok(Self::Web),
            value => {
                if VALID_CUSTOM_TAG.is_match(value) {
                    Ok(Self::Custom(String::from(value)))
                } else {
                    Err("Custom tag value must start with _ and contain only A-Z, a-z, 0-9 and _")
                }
            }
        }
    }
}
