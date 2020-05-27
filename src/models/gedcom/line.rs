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

#[cfg(test)]
mod tests {
    use super::GedcomLineTag;
    use std::str::FromStr;

    #[test]
    fn test_gedcom_line_tag_from_str_abbr() {
        let input = "ABBR";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Abbreviation;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_addr() {
        let input = "ADDR";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Address;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_adr1() {
        let input = "ADR1";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Address1;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_adr2() {
        let input = "ADR2";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Address2;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_adop() {
        let input = "ADOP";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Adoption;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_afn() {
        let input = "AFN";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::AncestralFileNumber;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_age() {
        let input = "AGE";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Age;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_agnc() {
        let input = "AGNC";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Agency;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_alia() {
        let input = "ALIA";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Alias;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_ance() {
        let input = "ANCE";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Ancestors;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_anci() {
        let input = "ANCI";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::AncestorInterest;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_anul() {
        let input = "ANUL";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Annulment;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_asso() {
        let input = "ASSO";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Associates;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_auth() {
        let input = "AUTH";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Author;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_bapl() {
        let input = "BAPL";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::BaptismLds;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_bapm() {
        let input = "BAPM";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Baptism;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_barm() {
        let input = "BARM";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::BarMitzvah;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_basm() {
        let input = "BASM";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::BasMitzvah;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_birth() {
        let input = "BIRT";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Birth;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_bles() {
        let input = "BLES";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Blessing;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_buri() {
        let input = "BURI";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Burial;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_caln() {
        let input = "CALN";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::CallNumber;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_cast() {
        let input = "CAST";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Caste;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_caus() {
        let input = "CAUS";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Cause;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_cens() {
        let input = "CENS";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Census;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_chan() {
        let input = "CHAN";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Change;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_char() {
        let input = "CHAR";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Character;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_chil() {
        let input = "CHIL";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Child;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_chr() {
        let input = "CHR";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Christening;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_chra() {
        let input = "CHRA";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::AdultChristening;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_city() {
        let input = "CITY";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::City;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_conc() {
        let input = "CONC";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Concatenation;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_conf() {
        let input = "CONF";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Confirmation;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_conl() {
        let input = "CONL";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::ConfirmationLds;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_cont() {
        let input = "CONT";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Continued;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_copr() {
        let input = "COPR";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Copyright;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_corp() {
        let input = "CORP";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Corporate;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_crem() {
        let input = "CREM";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Cremation;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_ctry() {
        let input = "CTRY";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Country;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_data() {
        let input = "DATA";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Data;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_date() {
        let input = "DATE";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Date;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_deat() {
        let input = "DEAT";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Death;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_desc() {
        let input = "DESC";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Descendants;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_desi() {
        let input = "DESI";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::DescendantInterest;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_dest() {
        let input = "DEST";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Destination;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_div() {
        let input = "DIV";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Divorce;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_divf() {
        let input = "DIVF";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::DivorceFiled;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_dscr() {
        let input = "DSCR";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::PhysicalDescription;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_educ() {
        let input = "EDUC";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Education;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_emai() {
        let input = "EMAI";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Email;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_emig() {
        let input = "EMIG";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Emigration;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_endl() {
        let input = "ENDL";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Endowment;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_enga() {
        let input = "ENGA";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Engagement;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_even() {
        let input = "EVEN";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Event;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_fact() {
        let input = "FACT";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Fact;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_fam() {
        let input = "FAM";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Family;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_famc() {
        let input = "FAMC";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::FamilyChild;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_famf() {
        let input = "FAMF";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::FamilyFile;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_fams() {
        let input = "FAMS";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::FamilySpouse;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_fax() {
        let input = "FAX";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Facsimile;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_fcom() {
        let input = "FCOM";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::FirstCommunion;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_file() {
        let input = "FILE";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::File;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_form() {
        let input = "FORM";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Format;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_fone() {
        let input = "FONE";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Phonetic;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_gedc() {
        let input = "GEDC";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Gedcom;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_givn() {
        let input = "GIVN";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::GivenName;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_grad() {
        let input = "GRAD";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Graduation;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_head() {
        let input = "HEAD";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Header;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_husb() {
        let input = "HUSB";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Husband;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_idno() {
        let input = "IDNO";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::IdentityNumber;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_immi() {
        let input = "IMMI";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Immigration;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_indi() {
        let input = "INDI";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Individual;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_lang() {
        let input = "LANG";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Language;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_lati() {
        let input = "LATI";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Latitude;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_long() {
        let input = "LONG";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Longitude;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_map() {
        let input = "MAP";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Map;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_marb() {
        let input = "MARB";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::MarriageBanns;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_marc() {
        let input = "MARC";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::MarriageContract;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_marl() {
        let input = "MARL";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::MarriageLicense;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_marr() {
        let input = "MARR";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Marriage;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_mars() {
        let input = "MARS";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::MarriageSettlement;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_medi() {
        let input = "MEDI";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Media;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_name() {
        let input = "NAME";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Name;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_nati() {
        let input = "NATI";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Nationality;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_natu() {
        let input = "NATU";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Naturalisation;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_nchi() {
        let input = "NCHI";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::ChildrenCount;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_nick() {
        let input = "NICK";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Nickname;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_nmr() {
        let input = "NMR";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::MarriageCount;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_note() {
        let input = "NOTE";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Note;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_npfx() {
        let input = "NPFX";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::NamePrefix;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_nsfx() {
        let input = "NSFX";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::NameSuffix;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_obje() {
        let input = "OBJE";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Object;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_occu() {
        let input = "OCCU";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Occupation;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_ordi() {
        let input = "ORDI";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Ordinance;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_ordn() {
        let input = "ORDN";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Ordination;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_page() {
        let input = "PAGE";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Page;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_pedi() {
        let input = "PEDI";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Pedigree;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_phon() {
        let input = "PHON";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Phone;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_plac() {
        let input = "PLAC";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Place;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_post() {
        let input = "POST";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::PostalCode;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_prob() {
        let input = "PROB";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Probate;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_prop() {
        let input = "PROP";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Property;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_publ() {
        let input = "PUBL";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Publication;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_quay() {
        let input = "QUAY";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::QualityOfData;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_refn() {
        let input = "REFN";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Reference;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_rela() {
        let input = "RELA";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Relationship;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_reli() {
        let input = "RELI";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Religion;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_repo() {
        let input = "REPO";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Repository;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_resi() {
        let input = "RESI";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Residence;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_resn() {
        let input = "RESN";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Restriction;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_reti() {
        let input = "RETI";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Retirement;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_rfn() {
        let input = "RFN";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::RecordFileNumber;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_rin() {
        let input = "RIN";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::RecordIdNumber;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_role() {
        let input = "ROLE";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Role;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_romn() {
        let input = "ROMN";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Romanised;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_sex() {
        let input = "SEX";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Sex;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_slgc() {
        let input = "SLGC";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::SealingChild;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_slgs() {
        let input = "SLGS";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::SealingSpouse;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_sour() {
        let input = "SOUR";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Source;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_spfx() {
        let input = "SPFX";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::SurnamePrefix;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_ssn() {
        let input = "SSN";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::SocialSecurityNumber;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_stae() {
        let input = "STAE";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::State;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_stat() {
        let input = "STAT";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Status;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_subm() {
        let input = "SUBM";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Submitter;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_subn() {
        let input = "SUBN";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Submission;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_surn() {
        let input = "SURN";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Surname;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_temp() {
        let input = "TEMP";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Temple;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_text() {
        let input = "TEXT";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Text;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_time() {
        let input = "TIME";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Time;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_titl() {
        let input = "TITL";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Title;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_trlr() {
        let input = "TRLR";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Trailer;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_type() {
        let input = "TYPE";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Type;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_vers() {
        let input = "VERS";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Version;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_wife() {
        let input = "WIFE";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Wife;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_will() {
        let input = "WILL";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Will;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_str_www() {
        let input = "WWW";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Web;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_valid_custom() {
        let input = "_CUSTOM";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let expected = GedcomLineTag::Custom(String::from("_CUSTOM"));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gedcom_line_tag_from_invalid_custom() {
        let input = "INVALID";

        let actual = GedcomLineTag::from_str(input);
        assert!(actual.is_err());

        let expected =
            Err("Custom tag value must start with _ and contain only A-Z, a-z, 0-9 and _");
        assert_eq!(actual, expected);
    }
}
