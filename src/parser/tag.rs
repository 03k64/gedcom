use super::{primitive::parse_alphanum, util::vec_to_string};
use crate::models::gedcom::GedcomLineTag;
use lazy_static::lazy_static;
use nom::{
    error::{make_error, ErrorKind},
    multi::many1,
    Err, IResult,
};
use regex::Regex;

lazy_static! {
    static ref VALID_TAG: Regex = Regex::new(
        r#"(ABBR|ADDR|ADR1|ADR2|ADOP|AFN|AGE|AGNC|ALIA|ANCE|ANCI|ANUL|ASSO|AUTH|BAPL|BAPM|BARM|BASM|BIRT|BLES|BURI|CALN|CAST|CAUS|CENS|CHAN|CHAR|CHIL|CHR|CHRA|CITY|CONC|CONF|CONL|CONT|COPR|CORP|CREM|CTRY|DATA|DATE|DEAT|DESC|DESI|DEST|DIV|DIVF|DSCR|EDUC|EMAI|EMIG|ENDL|ENGA|EVEN|FACT|FAM|FAMC|FAMF|FAMS|FAX|FCOM|FILE|FORM|FONE|GEDC|GIVN|GRAD|HEAD|HUSB|IDNO|IMMI|INDI|LANG|LATI|LONG|MAP|MARB|MARC|MARL|MARR|MARS|MEDI|NAME|NATI|NATU|NCHI|NICK|NMR|NOTE|NPFX|NSFX|OBJE|OCCU|ORDI|ORDN|PAGE|PEDI|PHON|PLAC|POST|PROB|PROP|PUBL|QUAY|REFN|RELA|RELI|REPO|RESI|RESN|RETI|RFN|RIN|ROLE|ROMN|SEX|SLGC|SLGS|SOUR|SPFX|SSN|STAE|STAT|SUBM|SUBN|SURN|TEMP|TEXT|TIME|TITL|TRLR|TYPE|VERS|WIFE|WILL|WWW|_[A-Za-z0-9_]+)"#
    ).unwrap();
}

pub fn parse_tag(input: &str) -> IResult<&str, GedcomLineTag> {
    match many1(parse_alphanum)(input).map(vec_to_string) {
        Err(e) => Err(e),
        Ok((input, output)) => match output.parse::<GedcomLineTag>() {
            Ok(tag) => Ok((input, tag)),
            Err(_) => Err(Err::Failure(make_error(input, ErrorKind::ManyMN))),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::parse_tag;
    use crate::models::gedcom::GedcomLineTag;
    use nom::{error::ErrorKind, Err};

    #[test]
    fn test_parse_tag_custom_tag_valid() {
        let input = "_VALID";
        let expected = Ok(("", GedcomLineTag::Custom(String::from("_VALID"))));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_custom_tag_invalid() {
        let input = "INVALID";
        let expected = Err(Err::Failure(("", ErrorKind::ManyMN)));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_abbr() {
        let input = "ABBR";
        let expected = Ok(("", GedcomLineTag::Abbreviation));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_addr() {
        let input = "ADDR";
        let expected = Ok(("", GedcomLineTag::Address));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_adr1() {
        let input = "ADR1";
        let expected = Ok(("", GedcomLineTag::Address1));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_adr2() {
        let input = "ADR2";
        let expected = Ok(("", GedcomLineTag::Address2));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_adop() {
        let input = "ADOP";
        let expected = Ok(("", GedcomLineTag::Adoption));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_afn() {
        let input = "AFN";
        let expected = Ok(("", GedcomLineTag::AncestralFileNumber));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_age() {
        let input = "AGE";
        let expected = Ok(("", GedcomLineTag::Age));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_agnc() {
        let input = "AGNC";
        let expected = Ok(("", GedcomLineTag::Agency));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_alia() {
        let input = "ALIA";
        let expected = Ok(("", GedcomLineTag::Alias));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_ance() {
        let input = "ANCE";
        let expected = Ok(("", GedcomLineTag::Ancestors));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_anci() {
        let input = "ANCI";
        let expected = Ok(("", GedcomLineTag::AncestorInterest));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_anul() {
        let input = "ANUL";
        let expected = Ok(("", GedcomLineTag::Annulment));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_asso() {
        let input = "ASSO";
        let expected = Ok(("", GedcomLineTag::Associates));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_auth() {
        let input = "AUTH";
        let expected = Ok(("", GedcomLineTag::Author));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_bapl() {
        let input = "BAPL";
        let expected = Ok(("", GedcomLineTag::BaptismLds));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_bapm() {
        let input = "BAPM";
        let expected = Ok(("", GedcomLineTag::Baptism));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_barm() {
        let input = "BARM";
        let expected = Ok(("", GedcomLineTag::BarMitzvah));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_basm() {
        let input = "BASM";
        let expected = Ok(("", GedcomLineTag::BasMitzvah));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_birt() {
        let input = "BIRT";
        let expected = Ok(("", GedcomLineTag::Birth));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_bles() {
        let input = "BLES";
        let expected = Ok(("", GedcomLineTag::Blessing));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_buri() {
        let input = "BURI";
        let expected = Ok(("", GedcomLineTag::Burial));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_caln() {
        let input = "CALN";
        let expected = Ok(("", GedcomLineTag::CallNumber));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_cast() {
        let input = "CAST";
        let expected = Ok(("", GedcomLineTag::Caste));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_caus() {
        let input = "CAUS";
        let expected = Ok(("", GedcomLineTag::Cause));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_cens() {
        let input = "CENS";
        let expected = Ok(("", GedcomLineTag::Census));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_chan() {
        let input = "CHAN";
        let expected = Ok(("", GedcomLineTag::Change));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_char() {
        let input = "CHAR";
        let expected = Ok(("", GedcomLineTag::Character));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_chil() {
        let input = "CHIL";
        let expected = Ok(("", GedcomLineTag::Child));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_chr() {
        let input = "CHR";
        let expected = Ok(("", GedcomLineTag::Christening));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_chra() {
        let input = "CHRA";
        let expected = Ok(("", GedcomLineTag::AdultChristening));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_city() {
        let input = "CITY";
        let expected = Ok(("", GedcomLineTag::City));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_conc() {
        let input = "CONC";
        let expected = Ok(("", GedcomLineTag::Concatenation));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_conf() {
        let input = "CONF";
        let expected = Ok(("", GedcomLineTag::Confirmation));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_conl() {
        let input = "CONL";
        let expected = Ok(("", GedcomLineTag::ConfirmationLds));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_cont() {
        let input = "CONT";
        let expected = Ok(("", GedcomLineTag::Continued));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_copr() {
        let input = "COPR";
        let expected = Ok(("", GedcomLineTag::Copyright));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_corp() {
        let input = "CORP";
        let expected = Ok(("", GedcomLineTag::Corporate));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_crem() {
        let input = "CREM";
        let expected = Ok(("", GedcomLineTag::Cremation));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_ctry() {
        let input = "CTRY";
        let expected = Ok(("", GedcomLineTag::Country));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_data() {
        let input = "DATA";
        let expected = Ok(("", GedcomLineTag::Data));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_date() {
        let input = "DATE";
        let expected = Ok(("", GedcomLineTag::Date));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_deat() {
        let input = "DEAT";
        let expected = Ok(("", GedcomLineTag::Death));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_desc() {
        let input = "DESC";
        let expected = Ok(("", GedcomLineTag::Descendants));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_desi() {
        let input = "DESI";
        let expected = Ok(("", GedcomLineTag::DescendantInterest));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_dest() {
        let input = "DEST";
        let expected = Ok(("", GedcomLineTag::Destination));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_div() {
        let input = "DIV";
        let expected = Ok(("", GedcomLineTag::Divorce));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_divf() {
        let input = "DIVF";
        let expected = Ok(("", GedcomLineTag::DivorceFiled));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_dscr() {
        let input = "DSCR";
        let expected = Ok(("", GedcomLineTag::PhysicalDescription));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_educ() {
        let input = "EDUC";
        let expected = Ok(("", GedcomLineTag::Education));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_emai() {
        let input = "EMAI";
        let expected = Ok(("", GedcomLineTag::Email));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_emig() {
        let input = "EMIG";
        let expected = Ok(("", GedcomLineTag::Emigration));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_endl() {
        let input = "ENDL";
        let expected = Ok(("", GedcomLineTag::Endowment));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_enga() {
        let input = "ENGA";
        let expected = Ok(("", GedcomLineTag::Engagement));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_even() {
        let input = "EVEN";
        let expected = Ok(("", GedcomLineTag::Event));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_fact() {
        let input = "FACT";
        let expected = Ok(("", GedcomLineTag::Fact));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_fam() {
        let input = "FAM";
        let expected = Ok(("", GedcomLineTag::Family));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_famc() {
        let input = "FAMC";
        let expected = Ok(("", GedcomLineTag::FamilyChild));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_famf() {
        let input = "FAMF";
        let expected = Ok(("", GedcomLineTag::FamilyFile));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_fams() {
        let input = "FAMS";
        let expected = Ok(("", GedcomLineTag::FamilySpouse));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_fax() {
        let input = "FAX";
        let expected = Ok(("", GedcomLineTag::Facsimile));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_fcom() {
        let input = "FCOM";
        let expected = Ok(("", GedcomLineTag::FirstCommunion));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_file() {
        let input = "FILE";
        let expected = Ok(("", GedcomLineTag::File));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_form() {
        let input = "FORM";
        let expected = Ok(("", GedcomLineTag::Format));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_fone() {
        let input = "FONE";
        let expected = Ok(("", GedcomLineTag::Phonetic));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_gedc() {
        let input = "GEDC";
        let expected = Ok(("", GedcomLineTag::Gedcom));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_givn() {
        let input = "GIVN";
        let expected = Ok(("", GedcomLineTag::GivenName));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_grad() {
        let input = "GRAD";
        let expected = Ok(("", GedcomLineTag::Graduation));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_head() {
        let input = "HEAD";
        let expected = Ok(("", GedcomLineTag::Header));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_husb() {
        let input = "HUSB";
        let expected = Ok(("", GedcomLineTag::Husband));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_idno() {
        let input = "IDNO";
        let expected = Ok(("", GedcomLineTag::IdentityNumber));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_immi() {
        let input = "IMMI";
        let expected = Ok(("", GedcomLineTag::Immigration));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_indi() {
        let input = "INDI";
        let expected = Ok(("", GedcomLineTag::Individual));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_lang() {
        let input = "LANG";
        let expected = Ok(("", GedcomLineTag::Language));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_lati() {
        let input = "LATI";
        let expected = Ok(("", GedcomLineTag::Latitude));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_long() {
        let input = "LONG";
        let expected = Ok(("", GedcomLineTag::Longitude));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_map() {
        let input = "MAP";
        let expected = Ok(("", GedcomLineTag::Map));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_marb() {
        let input = "MARB";
        let expected = Ok(("", GedcomLineTag::MarriageBanns));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_marc() {
        let input = "MARC";
        let expected = Ok(("", GedcomLineTag::MarriageContract));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_marl() {
        let input = "MARL";
        let expected = Ok(("", GedcomLineTag::MarriageLicense));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_marr() {
        let input = "MARR";
        let expected = Ok(("", GedcomLineTag::Marriage));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_mars() {
        let input = "MARS";
        let expected = Ok(("", GedcomLineTag::MarriageSettlement));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_medi() {
        let input = "MEDI";
        let expected = Ok(("", GedcomLineTag::Media));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_name() {
        let input = "NAME";
        let expected = Ok(("", GedcomLineTag::Name));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_nati() {
        let input = "NATI";
        let expected = Ok(("", GedcomLineTag::Nationality));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_natu() {
        let input = "NATU";
        let expected = Ok(("", GedcomLineTag::Naturalisation));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_nchi() {
        let input = "NCHI";
        let expected = Ok(("", GedcomLineTag::ChildrenCount));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_nick() {
        let input = "NICK";
        let expected = Ok(("", GedcomLineTag::Nickname));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_nmr() {
        let input = "NMR";
        let expected = Ok(("", GedcomLineTag::MarriageCount));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_note() {
        let input = "NOTE";
        let expected = Ok(("", GedcomLineTag::Note));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_npfx() {
        let input = "NPFX";
        let expected = Ok(("", GedcomLineTag::NamePrefix));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_nsfx() {
        let input = "NSFX";
        let expected = Ok(("", GedcomLineTag::NameSuffix));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_obje() {
        let input = "OBJE";
        let expected = Ok(("", GedcomLineTag::Object));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_occu() {
        let input = "OCCU";
        let expected = Ok(("", GedcomLineTag::Occupation));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_ordi() {
        let input = "ORDI";
        let expected = Ok(("", GedcomLineTag::Ordinance));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_ordn() {
        let input = "ORDN";
        let expected = Ok(("", GedcomLineTag::Ordination));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_page() {
        let input = "PAGE";
        let expected = Ok(("", GedcomLineTag::Page));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_pedi() {
        let input = "PEDI";
        let expected = Ok(("", GedcomLineTag::Pedigree));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_phon() {
        let input = "PHON";
        let expected = Ok(("", GedcomLineTag::Phone));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_plac() {
        let input = "PLAC";
        let expected = Ok(("", GedcomLineTag::Place));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_post() {
        let input = "POST";
        let expected = Ok(("", GedcomLineTag::PostalCode));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_prob() {
        let input = "PROB";
        let expected = Ok(("", GedcomLineTag::Probate));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_prop() {
        let input = "PROP";
        let expected = Ok(("", GedcomLineTag::Property));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_publ() {
        let input = "PUBL";
        let expected = Ok(("", GedcomLineTag::Publication));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_quay() {
        let input = "QUAY";
        let expected = Ok(("", GedcomLineTag::QualityOfData));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_refn() {
        let input = "REFN";
        let expected = Ok(("", GedcomLineTag::Reference));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_rela() {
        let input = "RELA";
        let expected = Ok(("", GedcomLineTag::Relationship));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_reli() {
        let input = "RELI";
        let expected = Ok(("", GedcomLineTag::Religion));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_repo() {
        let input = "REPO";
        let expected = Ok(("", GedcomLineTag::Repository));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_resi() {
        let input = "RESI";
        let expected = Ok(("", GedcomLineTag::Residence));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_resn() {
        let input = "RESN";
        let expected = Ok(("", GedcomLineTag::Restriction));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_reti() {
        let input = "RETI";
        let expected = Ok(("", GedcomLineTag::Retirement));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_rfn() {
        let input = "RFN";
        let expected = Ok(("", GedcomLineTag::RecordFileNumber));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_rin() {
        let input = "RIN";
        let expected = Ok(("", GedcomLineTag::RecordIdNumber));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_role() {
        let input = "ROLE";
        let expected = Ok(("", GedcomLineTag::Role));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_romn() {
        let input = "ROMN";
        let expected = Ok(("", GedcomLineTag::Romanised));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_sex() {
        let input = "SEX";
        let expected = Ok(("", GedcomLineTag::Sex));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_slgc() {
        let input = "SLGC";
        let expected = Ok(("", GedcomLineTag::SealingChild));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_slgs() {
        let input = "SLGS";
        let expected = Ok(("", GedcomLineTag::SealingSpouse));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_sour() {
        let input = "SOUR";
        let expected = Ok(("", GedcomLineTag::Source));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_spfx() {
        let input = "SPFX";
        let expected = Ok(("", GedcomLineTag::SurnamePrefix));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_ssn() {
        let input = "SSN";
        let expected = Ok(("", GedcomLineTag::SocialSecurityNumber));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_stae() {
        let input = "STAE";
        let expected = Ok(("", GedcomLineTag::State));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_stat() {
        let input = "STAT";
        let expected = Ok(("", GedcomLineTag::Status));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_subm() {
        let input = "SUBM";
        let expected = Ok(("", GedcomLineTag::Submitter));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_subn() {
        let input = "SUBN";
        let expected = Ok(("", GedcomLineTag::Submission));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_surn() {
        let input = "SURN";
        let expected = Ok(("", GedcomLineTag::Surname));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_temp() {
        let input = "TEMP";
        let expected = Ok(("", GedcomLineTag::Temple));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_text() {
        let input = "TEXT";
        let expected = Ok(("", GedcomLineTag::Text));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_time() {
        let input = "TIME";
        let expected = Ok(("", GedcomLineTag::Time));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_titl() {
        let input = "TITL";
        let expected = Ok(("", GedcomLineTag::Title));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_trlr() {
        let input = "TRLR";
        let expected = Ok(("", GedcomLineTag::Trailer));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_type() {
        let input = "TYPE";
        let expected = Ok(("", GedcomLineTag::Type));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_vers() {
        let input = "VERS";
        let expected = Ok(("", GedcomLineTag::Version));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_wife() {
        let input = "WIFE";
        let expected = Ok(("", GedcomLineTag::Wife));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_will() {
        let input = "WILL";
        let expected = Ok(("", GedcomLineTag::Will));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_www() {
        let input = "WWW";
        let expected = Ok(("", GedcomLineTag::Web));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }
}
