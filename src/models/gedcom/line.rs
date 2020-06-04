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
        match value.to_uppercase().as_str() {
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
            _ => {
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

    macro_rules! tag_test {
        ($test_name:ident, $input:literal, $expected:expr) => {
            #[test]
            fn $test_name() {
                let actual = GedcomLineTag::from_str($input);
                assert!(actual.is_ok());
                let actual = actual.unwrap();
                assert_eq!(actual, $expected);
            }
        };
    }

    #[test]
    fn test_from_str_custom_tag_invalid() {
        let input = "INVALID";
        let expected =
            Err("Custom tag value must start with _ and contain only A-Z, a-z, 0-9 and _");
        let actual = GedcomLineTag::from_str(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_from_str_custom_lowercase_tag_invalid() {
        let input = "invalid";
        let expected =
            Err("Custom tag value must start with _ and contain only A-Z, a-z, 0-9 and _");
        let actual = GedcomLineTag::from_str(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_from_str_custom_mixedcase_tag_invalid() {
        let input = "InVaLiD";
        let expected =
            Err("Custom tag value must start with _ and contain only A-Z, a-z, 0-9 and _");
        let actual = GedcomLineTag::from_str(input);
        assert_eq!(actual, expected);
    }

    tag_test!(
        test_from_str_custom_tag_valid,
        "_VALID",
        GedcomLineTag::Custom(String::from("_VALID"))
    );

    tag_test!(
        test_from_str_custom_lowercase_tag_valid,
        "_valid",
        GedcomLineTag::Custom(String::from("_valid"))
    );

    tag_test!(
        test_from_str_custom_mixedcase_tag_valid,
        "_VaLiD",
        GedcomLineTag::Custom(String::from("_VaLiD"))
    );

    tag_test!(test_from_str_abbr, "ABBR", GedcomLineTag::Abbreviation);
    tag_test!(test_from_str_abbr_lc, "abbr", GedcomLineTag::Abbreviation);
    tag_test!(test_from_str_addr, "ADDR", GedcomLineTag::Address);
    tag_test!(test_from_str_addr_lc, "addr", GedcomLineTag::Address);
    tag_test!(test_from_str_adr1, "ADR1", GedcomLineTag::Address1);
    tag_test!(test_from_str_adr1_lc, "adr1", GedcomLineTag::Address1);
    tag_test!(test_from_str_adr2, "ADR2", GedcomLineTag::Address2);
    tag_test!(test_from_str_adr2_lc, "adr2", GedcomLineTag::Address2);
    tag_test!(test_from_str_adop, "ADOP", GedcomLineTag::Adoption);
    tag_test!(test_from_str_adop_lc, "adop", GedcomLineTag::Adoption);
    tag_test!(test_from_str_afn, "AFN", GedcomLineTag::AncestralFileNumber);
    tag_test!(
        test_from_str_afn_lc,
        "afn",
        GedcomLineTag::AncestralFileNumber
    );
    tag_test!(test_from_str_age, "AGE", GedcomLineTag::Age);
    tag_test!(test_from_str_age_lc, "age", GedcomLineTag::Age);
    tag_test!(test_from_str_agnc, "AGNC", GedcomLineTag::Agency);
    tag_test!(test_from_str_agnc_lc, "agnc", GedcomLineTag::Agency);
    tag_test!(test_from_str_alia, "ALIA", GedcomLineTag::Alias);
    tag_test!(test_from_str_alia_lc, "alia", GedcomLineTag::Alias);
    tag_test!(test_from_str_ance, "ANCE", GedcomLineTag::Ancestors);
    tag_test!(test_from_str_ance_lc, "ance", GedcomLineTag::Ancestors);
    tag_test!(test_from_str_anci, "ANCI", GedcomLineTag::AncestorInterest);
    tag_test!(
        test_from_str_anci_lc,
        "anci",
        GedcomLineTag::AncestorInterest
    );
    tag_test!(test_from_str_anul, "ANUL", GedcomLineTag::Annulment);
    tag_test!(test_from_str_anul_lc, "anul", GedcomLineTag::Annulment);
    tag_test!(test_from_str_asso, "ASSO", GedcomLineTag::Associates);
    tag_test!(test_from_str_asso_lc, "asso", GedcomLineTag::Associates);
    tag_test!(test_from_str_auth, "AUTH", GedcomLineTag::Author);
    tag_test!(test_from_str_auth_lc, "auth", GedcomLineTag::Author);
    tag_test!(test_from_str_bapl, "BAPL", GedcomLineTag::BaptismLds);
    tag_test!(test_from_str_bapl_lc, "bapl", GedcomLineTag::BaptismLds);
    tag_test!(test_from_str_bapm, "BAPM", GedcomLineTag::Baptism);
    tag_test!(test_from_str_bapm_lc, "bapm", GedcomLineTag::Baptism);
    tag_test!(test_from_str_barm, "BARM", GedcomLineTag::BarMitzvah);
    tag_test!(test_from_str_barm_lc, "barm", GedcomLineTag::BarMitzvah);
    tag_test!(test_from_str_basm, "BASM", GedcomLineTag::BasMitzvah);
    tag_test!(test_from_str_basm_lc, "basm", GedcomLineTag::BasMitzvah);
    tag_test!(test_from_str_birt, "BIRT", GedcomLineTag::Birth);
    tag_test!(test_from_str_birt_lc, "birt", GedcomLineTag::Birth);
    tag_test!(test_from_str_bles, "BLES", GedcomLineTag::Blessing);
    tag_test!(test_from_str_bles_lc, "bles", GedcomLineTag::Blessing);
    tag_test!(test_from_str_buri, "BURI", GedcomLineTag::Burial);
    tag_test!(test_from_str_buri_lc, "buri", GedcomLineTag::Burial);
    tag_test!(test_from_str_caln, "CALN", GedcomLineTag::CallNumber);
    tag_test!(test_from_str_caln_lc, "caln", GedcomLineTag::CallNumber);
    tag_test!(test_from_str_cast, "CAST", GedcomLineTag::Caste);
    tag_test!(test_from_str_cast_lc, "cast", GedcomLineTag::Caste);
    tag_test!(test_from_str_caus, "CAUS", GedcomLineTag::Cause);
    tag_test!(test_from_str_caus_lc, "caus", GedcomLineTag::Cause);
    tag_test!(test_from_str_cens, "CENS", GedcomLineTag::Census);
    tag_test!(test_from_str_cens_lc, "cens", GedcomLineTag::Census);
    tag_test!(test_from_str_chan, "CHAN", GedcomLineTag::Change);
    tag_test!(test_from_str_chan_lc, "chan", GedcomLineTag::Change);
    tag_test!(test_from_str_char, "CHAR", GedcomLineTag::Character);
    tag_test!(test_from_str_char_lc, "char", GedcomLineTag::Character);
    tag_test!(test_from_str_chil, "CHIL", GedcomLineTag::Child);
    tag_test!(test_from_str_chil_lc, "chil", GedcomLineTag::Child);
    tag_test!(test_from_str_chr, "CHR", GedcomLineTag::Christening);
    tag_test!(test_from_str_chr_lc, "chr", GedcomLineTag::Christening);
    tag_test!(test_from_str_chra, "CHRA", GedcomLineTag::AdultChristening);
    tag_test!(
        test_from_str_chra_lc,
        "chra",
        GedcomLineTag::AdultChristening
    );
    tag_test!(test_from_str_city, "CITY", GedcomLineTag::City);
    tag_test!(test_from_str_city_lc, "city", GedcomLineTag::City);
    tag_test!(test_from_str_conc, "CONC", GedcomLineTag::Concatenation);
    tag_test!(test_from_str_conc_lc, "conc", GedcomLineTag::Concatenation);
    tag_test!(test_from_str_conf, "CONF", GedcomLineTag::Confirmation);
    tag_test!(test_from_str_conf_lc, "conf", GedcomLineTag::Confirmation);
    tag_test!(test_from_str_conl, "CONL", GedcomLineTag::ConfirmationLds);
    tag_test!(
        test_from_str_conl_lc,
        "conl",
        GedcomLineTag::ConfirmationLds
    );
    tag_test!(test_from_str_cont, "CONT", GedcomLineTag::Continued);
    tag_test!(test_from_str_cont_lc, "cont", GedcomLineTag::Continued);
    tag_test!(test_from_str_copr, "COPR", GedcomLineTag::Copyright);
    tag_test!(test_from_str_copr_lc, "copr", GedcomLineTag::Copyright);
    tag_test!(test_from_str_corp, "CORP", GedcomLineTag::Corporate);
    tag_test!(test_from_str_corp_lc, "corp", GedcomLineTag::Corporate);
    tag_test!(test_from_str_crem, "CREM", GedcomLineTag::Cremation);
    tag_test!(test_from_str_crem_lc, "crem", GedcomLineTag::Cremation);
    tag_test!(test_from_str_ctry, "CTRY", GedcomLineTag::Country);
    tag_test!(test_from_str_ctry_lc, "ctry", GedcomLineTag::Country);
    tag_test!(test_from_str_data, "DATA", GedcomLineTag::Data);
    tag_test!(test_from_str_data_lc, "data", GedcomLineTag::Data);
    tag_test!(test_from_str_date, "DATE", GedcomLineTag::Date);
    tag_test!(test_from_str_date_lc, "date", GedcomLineTag::Date);
    tag_test!(test_from_str_deat, "DEAT", GedcomLineTag::Death);
    tag_test!(test_from_str_deat_lc, "deat", GedcomLineTag::Death);
    tag_test!(test_from_str_desc, "DESC", GedcomLineTag::Descendants);
    tag_test!(test_from_str_desc_lc, "desc", GedcomLineTag::Descendants);
    tag_test!(
        test_from_str_desi,
        "DESI",
        GedcomLineTag::DescendantInterest
    );
    tag_test!(
        test_from_str_desi_lc,
        "desi",
        GedcomLineTag::DescendantInterest
    );
    tag_test!(test_from_str_dest, "DEST", GedcomLineTag::Destination);
    tag_test!(test_from_str_dest_lc, "dest", GedcomLineTag::Destination);
    tag_test!(test_from_str_div, "DIV", GedcomLineTag::Divorce);
    tag_test!(test_from_str_div_lc, "div", GedcomLineTag::Divorce);
    tag_test!(test_from_str_divf, "DIVF", GedcomLineTag::DivorceFiled);
    tag_test!(test_from_str_divf_lc, "divf", GedcomLineTag::DivorceFiled);
    tag_test!(
        test_from_str_dscr,
        "DSCR",
        GedcomLineTag::PhysicalDescription
    );
    tag_test!(
        test_from_str_dscr_lc,
        "dscr",
        GedcomLineTag::PhysicalDescription
    );
    tag_test!(test_from_str_educ, "EDUC", GedcomLineTag::Education);
    tag_test!(test_from_str_educ_lc, "educ", GedcomLineTag::Education);
    tag_test!(test_from_str_emai, "EMAI", GedcomLineTag::Email);
    tag_test!(test_from_str_emai_lc, "emai", GedcomLineTag::Email);
    tag_test!(test_from_str_emig, "EMIG", GedcomLineTag::Emigration);
    tag_test!(test_from_str_emig_lc, "emig", GedcomLineTag::Emigration);
    tag_test!(test_from_str_endl, "ENDL", GedcomLineTag::Endowment);
    tag_test!(test_from_str_endl_lc, "endl", GedcomLineTag::Endowment);
    tag_test!(test_from_str_enga, "ENGA", GedcomLineTag::Engagement);
    tag_test!(test_from_str_enga_lc, "enga", GedcomLineTag::Engagement);
    tag_test!(test_from_str_even, "EVEN", GedcomLineTag::Event);
    tag_test!(test_from_str_even_lc, "even", GedcomLineTag::Event);
    tag_test!(test_from_str_fact, "FACT", GedcomLineTag::Fact);
    tag_test!(test_from_str_fact_lc, "fact", GedcomLineTag::Fact);
    tag_test!(test_from_str_fam, "FAM", GedcomLineTag::Family);
    tag_test!(test_from_str_fam_lc, "fam", GedcomLineTag::Family);
    tag_test!(test_from_str_famc, "FAMC", GedcomLineTag::FamilyChild);
    tag_test!(test_from_str_famc_lc, "famc", GedcomLineTag::FamilyChild);
    tag_test!(test_from_str_famf, "FAMF", GedcomLineTag::FamilyFile);
    tag_test!(test_from_str_famf_lc, "famf", GedcomLineTag::FamilyFile);
    tag_test!(test_from_str_fams, "FAMS", GedcomLineTag::FamilySpouse);
    tag_test!(test_from_str_fams_lc, "fams", GedcomLineTag::FamilySpouse);
    tag_test!(test_from_str_fax, "FAX", GedcomLineTag::Facsimile);
    tag_test!(test_from_str_fax_lc, "fax", GedcomLineTag::Facsimile);
    tag_test!(test_from_str_fcom, "FCOM", GedcomLineTag::FirstCommunion);
    tag_test!(test_from_str_fcom_lc, "fcom", GedcomLineTag::FirstCommunion);
    tag_test!(test_from_str_file, "FILE", GedcomLineTag::File);
    tag_test!(test_from_str_file_lc, "file", GedcomLineTag::File);
    tag_test!(test_from_str_form, "FORM", GedcomLineTag::Format);
    tag_test!(test_from_str_form_lc, "form", GedcomLineTag::Format);
    tag_test!(test_from_str_fone, "FONE", GedcomLineTag::Phonetic);
    tag_test!(test_from_str_fone_lc, "fone", GedcomLineTag::Phonetic);
    tag_test!(test_from_str_gedc, "GEDC", GedcomLineTag::Gedcom);
    tag_test!(test_from_str_gedc_lc, "gedc", GedcomLineTag::Gedcom);
    tag_test!(test_from_str_givn, "GIVN", GedcomLineTag::GivenName);
    tag_test!(test_from_str_givn_lc, "givn", GedcomLineTag::GivenName);
    tag_test!(test_from_str_grad, "GRAD", GedcomLineTag::Graduation);
    tag_test!(test_from_str_grad_lc, "grad", GedcomLineTag::Graduation);
    tag_test!(test_from_str_head, "HEAD", GedcomLineTag::Header);
    tag_test!(test_from_str_head_lc, "head", GedcomLineTag::Header);
    tag_test!(test_from_str_husb, "HUSB", GedcomLineTag::Husband);
    tag_test!(test_from_str_husb_lc, "husb", GedcomLineTag::Husband);
    tag_test!(test_from_str_idno, "IDNO", GedcomLineTag::IdentityNumber);
    tag_test!(test_from_str_idno_lc, "idno", GedcomLineTag::IdentityNumber);
    tag_test!(test_from_str_immi, "IMMI", GedcomLineTag::Immigration);
    tag_test!(test_from_str_immi_lc, "immi", GedcomLineTag::Immigration);
    tag_test!(test_from_str_indi, "INDI", GedcomLineTag::Individual);
    tag_test!(test_from_str_indi_lc, "indi", GedcomLineTag::Individual);
    tag_test!(test_from_str_lang, "LANG", GedcomLineTag::Language);
    tag_test!(test_from_str_lang_lc, "lang", GedcomLineTag::Language);
    tag_test!(test_from_str_lati, "LATI", GedcomLineTag::Latitude);
    tag_test!(test_from_str_lati_lc, "lati", GedcomLineTag::Latitude);
    tag_test!(test_from_str_long, "LONG", GedcomLineTag::Longitude);
    tag_test!(test_from_str_long_lc, "long", GedcomLineTag::Longitude);
    tag_test!(test_from_str_map, "MAP", GedcomLineTag::Map);
    tag_test!(test_from_str_map_lc, "map", GedcomLineTag::Map);
    tag_test!(test_from_str_marb, "MARB", GedcomLineTag::MarriageBanns);
    tag_test!(test_from_str_marb_lc, "marb", GedcomLineTag::MarriageBanns);
    tag_test!(test_from_str_marc, "MARC", GedcomLineTag::MarriageContract);
    tag_test!(
        test_from_str_marc_lc,
        "marc",
        GedcomLineTag::MarriageContract
    );
    tag_test!(test_from_str_marl, "MARL", GedcomLineTag::MarriageLicense);
    tag_test!(
        test_from_str_marl_lc,
        "marl",
        GedcomLineTag::MarriageLicense
    );
    tag_test!(test_from_str_marr, "MARR", GedcomLineTag::Marriage);
    tag_test!(test_from_str_marr_lc, "marr", GedcomLineTag::Marriage);
    tag_test!(
        test_from_str_mars,
        "MARS",
        GedcomLineTag::MarriageSettlement
    );
    tag_test!(
        test_from_str_mars_lc,
        "mars",
        GedcomLineTag::MarriageSettlement
    );
    tag_test!(test_from_str_medi, "MEDI", GedcomLineTag::Media);
    tag_test!(test_from_str_medi_lc, "medi", GedcomLineTag::Media);
    tag_test!(test_from_str_name, "NAME", GedcomLineTag::Name);
    tag_test!(test_from_str_name_lc, "name", GedcomLineTag::Name);
    tag_test!(test_from_str_nati, "NATI", GedcomLineTag::Nationality);
    tag_test!(test_from_str_nati_lc, "nati", GedcomLineTag::Nationality);
    tag_test!(test_from_str_natu, "NATU", GedcomLineTag::Naturalisation);
    tag_test!(test_from_str_natu_lc, "natu", GedcomLineTag::Naturalisation);
    tag_test!(test_from_str_nchi, "NCHI", GedcomLineTag::ChildrenCount);
    tag_test!(test_from_str_nchi_lc, "nchi", GedcomLineTag::ChildrenCount);
    tag_test!(test_from_str_nick, "NICK", GedcomLineTag::Nickname);
    tag_test!(test_from_str_nick_lc, "nick", GedcomLineTag::Nickname);
    tag_test!(test_from_str_nmr, "NMR", GedcomLineTag::MarriageCount);
    tag_test!(test_from_str_nmr_lc, "nmr", GedcomLineTag::MarriageCount);
    tag_test!(test_from_str_note, "NOTE", GedcomLineTag::Note);
    tag_test!(test_from_str_note_lc, "note", GedcomLineTag::Note);
    tag_test!(test_from_str_npfx, "NPFX", GedcomLineTag::NamePrefix);
    tag_test!(test_from_str_npfx_lc, "npfx", GedcomLineTag::NamePrefix);
    tag_test!(test_from_str_nsfx, "NSFX", GedcomLineTag::NameSuffix);
    tag_test!(test_from_str_nsfx_lc, "nsfx", GedcomLineTag::NameSuffix);
    tag_test!(test_from_str_obje, "OBJE", GedcomLineTag::Object);
    tag_test!(test_from_str_obje_lc, "obje", GedcomLineTag::Object);
    tag_test!(test_from_str_occu, "OCCU", GedcomLineTag::Occupation);
    tag_test!(test_from_str_occu_lc, "occu", GedcomLineTag::Occupation);
    tag_test!(test_from_str_ordi, "ORDI", GedcomLineTag::Ordinance);
    tag_test!(test_from_str_ordi_lc, "ordi", GedcomLineTag::Ordinance);
    tag_test!(test_from_str_ordn, "ORDN", GedcomLineTag::Ordination);
    tag_test!(test_from_str_ordn_lc, "ordn", GedcomLineTag::Ordination);
    tag_test!(test_from_str_page, "PAGE", GedcomLineTag::Page);
    tag_test!(test_from_str_page_lc, "page", GedcomLineTag::Page);
    tag_test!(test_from_str_pedi, "PEDI", GedcomLineTag::Pedigree);
    tag_test!(test_from_str_pedi_lc, "pedi", GedcomLineTag::Pedigree);
    tag_test!(test_from_str_phon, "PHON", GedcomLineTag::Phone);
    tag_test!(test_from_str_phon_lc, "phon", GedcomLineTag::Phone);
    tag_test!(test_from_str_plac, "PLAC", GedcomLineTag::Place);
    tag_test!(test_from_str_plac_lc, "plac", GedcomLineTag::Place);
    tag_test!(test_from_str_post, "POST", GedcomLineTag::PostalCode);
    tag_test!(test_from_str_post_lc, "post", GedcomLineTag::PostalCode);
    tag_test!(test_from_str_prob, "PROB", GedcomLineTag::Probate);
    tag_test!(test_from_str_prob_lc, "prob", GedcomLineTag::Probate);
    tag_test!(test_from_str_prop, "PROP", GedcomLineTag::Property);
    tag_test!(test_from_str_prop_lc, "prop", GedcomLineTag::Property);
    tag_test!(test_from_str_publ, "PUBL", GedcomLineTag::Publication);
    tag_test!(test_from_str_publ_lc, "publ", GedcomLineTag::Publication);
    tag_test!(test_from_str_quay, "QUAY", GedcomLineTag::QualityOfData);
    tag_test!(test_from_str_quay_lc, "quay", GedcomLineTag::QualityOfData);
    tag_test!(test_from_str_refn, "REFN", GedcomLineTag::Reference);
    tag_test!(test_from_str_refn_lc, "refn", GedcomLineTag::Reference);
    tag_test!(test_from_str_rela, "RELA", GedcomLineTag::Relationship);
    tag_test!(test_from_str_rela_lc, "rela", GedcomLineTag::Relationship);
    tag_test!(test_from_str_reli, "RELI", GedcomLineTag::Religion);
    tag_test!(test_from_str_reli_lc, "reli", GedcomLineTag::Religion);
    tag_test!(test_from_str_repo, "REPO", GedcomLineTag::Repository);
    tag_test!(test_from_str_repo_lc, "repo", GedcomLineTag::Repository);
    tag_test!(test_from_str_resi, "RESI", GedcomLineTag::Residence);
    tag_test!(test_from_str_resi_lc, "resi", GedcomLineTag::Residence);
    tag_test!(test_from_str_resn, "RESN", GedcomLineTag::Restriction);
    tag_test!(test_from_str_resn_lc, "resn", GedcomLineTag::Restriction);
    tag_test!(test_from_str_reti, "RETI", GedcomLineTag::Retirement);
    tag_test!(test_from_str_reti_lc, "reti", GedcomLineTag::Retirement);
    tag_test!(test_from_str_rfn, "RFN", GedcomLineTag::RecordFileNumber);
    tag_test!(test_from_str_rfn_lc, "rfn", GedcomLineTag::RecordFileNumber);
    tag_test!(test_from_str_rin, "RIN", GedcomLineTag::RecordIdNumber);
    tag_test!(test_from_str_rin_lc, "rin", GedcomLineTag::RecordIdNumber);
    tag_test!(test_from_str_role, "ROLE", GedcomLineTag::Role);
    tag_test!(test_from_str_role_lc, "role", GedcomLineTag::Role);
    tag_test!(test_from_str_romn, "ROMN", GedcomLineTag::Romanised);
    tag_test!(test_from_str_romn_lc, "romn", GedcomLineTag::Romanised);
    tag_test!(test_from_str_sex, "SEX", GedcomLineTag::Sex);
    tag_test!(test_from_str_sex_lc, "sex", GedcomLineTag::Sex);
    tag_test!(test_from_str_slgc, "SLGC", GedcomLineTag::SealingChild);
    tag_test!(test_from_str_slgc_lc, "slgc", GedcomLineTag::SealingChild);
    tag_test!(test_from_str_slgs, "SLGS", GedcomLineTag::SealingSpouse);
    tag_test!(test_from_str_slgs_lc, "slgs", GedcomLineTag::SealingSpouse);
    tag_test!(test_from_str_sour, "SOUR", GedcomLineTag::Source);
    tag_test!(test_from_str_sour_lc, "sour", GedcomLineTag::Source);
    tag_test!(test_from_str_spfx, "SPFX", GedcomLineTag::SurnamePrefix);
    tag_test!(test_from_str_spfx_lc, "spfx", GedcomLineTag::SurnamePrefix);
    tag_test!(
        test_from_str_ssn,
        "SSN",
        GedcomLineTag::SocialSecurityNumber
    );
    tag_test!(
        test_from_str_ssn_lc,
        "ssn",
        GedcomLineTag::SocialSecurityNumber
    );
    tag_test!(test_from_str_stae, "STAE", GedcomLineTag::State);
    tag_test!(test_from_str_stae_lc, "stae", GedcomLineTag::State);
    tag_test!(test_from_str_stat, "STAT", GedcomLineTag::Status);
    tag_test!(test_from_str_stat_lc, "stat", GedcomLineTag::Status);
    tag_test!(test_from_str_subm, "SUBM", GedcomLineTag::Submitter);
    tag_test!(test_from_str_subm_lc, "subm", GedcomLineTag::Submitter);
    tag_test!(test_from_str_subn, "SUBN", GedcomLineTag::Submission);
    tag_test!(test_from_str_subn_lc, "subn", GedcomLineTag::Submission);
    tag_test!(test_from_str_surn, "SURN", GedcomLineTag::Surname);
    tag_test!(test_from_str_surn_lc, "surn", GedcomLineTag::Surname);
    tag_test!(test_from_str_temp, "TEMP", GedcomLineTag::Temple);
    tag_test!(test_from_str_temp_lc, "temp", GedcomLineTag::Temple);
    tag_test!(test_from_str_text, "TEXT", GedcomLineTag::Text);
    tag_test!(test_from_str_text_lc, "text", GedcomLineTag::Text);
    tag_test!(test_from_str_time, "TIME", GedcomLineTag::Time);
    tag_test!(test_from_str_time_lc, "time", GedcomLineTag::Time);
    tag_test!(test_from_str_titl, "TITL", GedcomLineTag::Title);
    tag_test!(test_from_str_titl_lc, "titl", GedcomLineTag::Title);
    tag_test!(test_from_str_trlr, "TRLR", GedcomLineTag::Trailer);
    tag_test!(test_from_str_trlr_lc, "trlr", GedcomLineTag::Trailer);
    tag_test!(test_from_str_type, "TYPE", GedcomLineTag::Type);
    tag_test!(test_from_str_type_lc, "type", GedcomLineTag::Type);
    tag_test!(test_from_str_vers, "VERS", GedcomLineTag::Version);
    tag_test!(test_from_str_vers_lc, "vers", GedcomLineTag::Version);
    tag_test!(test_from_str_wife, "WIFE", GedcomLineTag::Wife);
    tag_test!(test_from_str_wife_lc, "wife", GedcomLineTag::Wife);
    tag_test!(test_from_str_will, "WILL", GedcomLineTag::Will);
    tag_test!(test_from_str_will_lc, "will", GedcomLineTag::Will);
    tag_test!(test_from_str_www, "WWW", GedcomLineTag::Web);
    tag_test!(test_from_str_www_lc, "www", GedcomLineTag::Web);
}
