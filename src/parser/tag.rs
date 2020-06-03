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
        r#"(?i:ABBR|ADDR|ADR1|ADR2|ADOP|AFN|AGE|AGNC|ALIA|ANCE|ANCI|ANUL|ASSO|AUTH|BAPL|BAPM|BARM|BASM|BIRT|BLES|BURI|CALN|CAST|CAUS|CENS|CHAN|CHAR|CHIL|CHR|CHRA|CITY|CONC|CONF|CONL|CONT|COPR|CORP|CREM|CTRY|DATA|DATE|DEAT|DESC|DESI|DEST|DIV|DIVF|DSCR|EDUC|EMAI|EMIG|ENDL|ENGA|EVEN|FACT|FAM|FAMC|FAMF|FAMS|FAX|FCOM|FILE|FORM|FONE|GEDC|GIVN|GRAD|HEAD|HUSB|IDNO|IMMI|INDI|LANG|LATI|LONG|MAP|MARB|MARC|MARL|MARR|MARS|MEDI|NAME|NATI|NATU|NCHI|NICK|NMR|NOTE|NPFX|NSFX|OBJE|OCCU|ORDI|ORDN|PAGE|PEDI|PHON|PLAC|POST|PROB|PROP|PUBL|QUAY|REFN|RELA|RELI|REPO|RESI|RESN|RETI|RFN|RIN|ROLE|ROMN|SEX|SLGC|SLGS|SOUR|SPFX|SSN|STAE|STAT|SUBM|SUBN|SURN|TEMP|TEXT|TIME|TITL|TRLR|TYPE|VERS|WIFE|WILL|WWW|_[A-Za-z0-9_]+)"#
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

    macro_rules! tag_test {
        ($test_name:ident, $input:literal, $expected:expr) => {
            #[test]
            fn $test_name() {
                let expected = Ok(("", $expected));
                let actual = parse_tag($input);
                assert_eq!(actual, expected);
            }
        };
    }

    #[test]
    fn test_parse_tag_custom_tag_invalid() {
        let input = "INVALID";
        let expected = Err(Err::Failure(("", ErrorKind::ManyMN)));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_custom_lowercase_tag_invalid() {
        let input = "invalid";
        let expected = Err(Err::Failure(("", ErrorKind::ManyMN)));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_tag_custom_mixedcase_tag_invalid() {
        let input = "InVaLiD";
        let expected = Err(Err::Failure(("", ErrorKind::ManyMN)));
        let actual = parse_tag(input);
        assert_eq!(actual, expected);
    }

    tag_test!(
        test_parse_tag_custom_tag_valid,
        "_VALID",
        GedcomLineTag::Custom(String::from("_VALID"))
    );

    tag_test!(
        test_parse_tag_custom_lowercase_tag_valid,
        "_valid",
        GedcomLineTag::Custom(String::from("_valid"))
    );

    tag_test!(
        test_parse_tag_custom_mixedcase_tag_valid,
        "_VaLiD",
        GedcomLineTag::Custom(String::from("_VaLiD"))
    );

    tag_test!(test_parse_tag_abbr, "ABBR", GedcomLineTag::Abbreviation);
    tag_test!(test_parse_tag_abbr_lc, "abbr", GedcomLineTag::Abbreviation);
    tag_test!(test_parse_tag_addr, "ADDR", GedcomLineTag::Address);
    tag_test!(test_parse_tag_addr_lc, "addr", GedcomLineTag::Address);
    tag_test!(test_parse_tag_adr1, "ADR1", GedcomLineTag::Address1);
    tag_test!(test_parse_tag_adr1_lc, "adr1", GedcomLineTag::Address1);
    tag_test!(test_parse_tag_adr2, "ADR2", GedcomLineTag::Address2);
    tag_test!(test_parse_tag_adr2_lc, "adr2", GedcomLineTag::Address2);
    tag_test!(test_parse_tag_adop, "ADOP", GedcomLineTag::Adoption);
    tag_test!(test_parse_tag_adop_lc, "adop", GedcomLineTag::Adoption);
    tag_test!(
        test_parse_tag_afn,
        "AFN",
        GedcomLineTag::AncestralFileNumber
    );
    tag_test!(
        test_parse_tag_afn_lc,
        "afn",
        GedcomLineTag::AncestralFileNumber
    );
    tag_test!(test_parse_tag_age, "AGE", GedcomLineTag::Age);
    tag_test!(test_parse_tag_age_lc, "age", GedcomLineTag::Age);
    tag_test!(test_parse_tag_agnc, "AGNC", GedcomLineTag::Agency);
    tag_test!(test_parse_tag_agnc_lc, "agnc", GedcomLineTag::Agency);
    tag_test!(test_parse_tag_alia, "ALIA", GedcomLineTag::Alias);
    tag_test!(test_parse_tag_alia_lc, "alia", GedcomLineTag::Alias);
    tag_test!(test_parse_tag_ance, "ANCE", GedcomLineTag::Ancestors);
    tag_test!(test_parse_tag_ance_lc, "ance", GedcomLineTag::Ancestors);
    tag_test!(test_parse_tag_anci, "ANCI", GedcomLineTag::AncestorInterest);
    tag_test!(
        test_parse_tag_anci_lc,
        "anci",
        GedcomLineTag::AncestorInterest
    );
    tag_test!(test_parse_tag_anul, "ANUL", GedcomLineTag::Annulment);
    tag_test!(test_parse_tag_anul_lc, "anul", GedcomLineTag::Annulment);
    tag_test!(test_parse_tag_asso, "ASSO", GedcomLineTag::Associates);
    tag_test!(test_parse_tag_asso_lc, "asso", GedcomLineTag::Associates);
    tag_test!(test_parse_tag_auth, "AUTH", GedcomLineTag::Author);
    tag_test!(test_parse_tag_auth_lc, "auth", GedcomLineTag::Author);
    tag_test!(test_parse_tag_bapl, "BAPL", GedcomLineTag::BaptismLds);
    tag_test!(test_parse_tag_bapl_lc, "bapl", GedcomLineTag::BaptismLds);
    tag_test!(test_parse_tag_bapm, "BAPM", GedcomLineTag::Baptism);
    tag_test!(test_parse_tag_bapm_lc, "bapm", GedcomLineTag::Baptism);
    tag_test!(test_parse_tag_barm, "BARM", GedcomLineTag::BarMitzvah);
    tag_test!(test_parse_tag_barm_lc, "barm", GedcomLineTag::BarMitzvah);
    tag_test!(test_parse_tag_basm, "BASM", GedcomLineTag::BasMitzvah);
    tag_test!(test_parse_tag_basm_lc, "basm", GedcomLineTag::BasMitzvah);
    tag_test!(test_parse_tag_birt, "BIRT", GedcomLineTag::Birth);
    tag_test!(test_parse_tag_birt_lc, "birt", GedcomLineTag::Birth);
    tag_test!(test_parse_tag_bles, "BLES", GedcomLineTag::Blessing);
    tag_test!(test_parse_tag_bles_lc, "bles", GedcomLineTag::Blessing);
    tag_test!(test_parse_tag_buri, "BURI", GedcomLineTag::Burial);
    tag_test!(test_parse_tag_buri_lc, "buri", GedcomLineTag::Burial);
    tag_test!(test_parse_tag_caln, "CALN", GedcomLineTag::CallNumber);
    tag_test!(test_parse_tag_caln_lc, "caln", GedcomLineTag::CallNumber);
    tag_test!(test_parse_tag_cast, "CAST", GedcomLineTag::Caste);
    tag_test!(test_parse_tag_cast_lc, "cast", GedcomLineTag::Caste);
    tag_test!(test_parse_tag_caus, "CAUS", GedcomLineTag::Cause);
    tag_test!(test_parse_tag_caus_lc, "caus", GedcomLineTag::Cause);
    tag_test!(test_parse_tag_cens, "CENS", GedcomLineTag::Census);
    tag_test!(test_parse_tag_cens_lc, "cens", GedcomLineTag::Census);
    tag_test!(test_parse_tag_chan, "CHAN", GedcomLineTag::Change);
    tag_test!(test_parse_tag_chan_lc, "chan", GedcomLineTag::Change);
    tag_test!(test_parse_tag_char, "CHAR", GedcomLineTag::Character);
    tag_test!(test_parse_tag_char_lc, "char", GedcomLineTag::Character);
    tag_test!(test_parse_tag_chil, "CHIL", GedcomLineTag::Child);
    tag_test!(test_parse_tag_chil_lc, "chil", GedcomLineTag::Child);
    tag_test!(test_parse_tag_chr, "CHR", GedcomLineTag::Christening);
    tag_test!(test_parse_tag_chr_lc, "chr", GedcomLineTag::Christening);
    tag_test!(test_parse_tag_chra, "CHRA", GedcomLineTag::AdultChristening);
    tag_test!(
        test_parse_tag_chra_lc,
        "chra",
        GedcomLineTag::AdultChristening
    );
    tag_test!(test_parse_tag_city, "CITY", GedcomLineTag::City);
    tag_test!(test_parse_tag_city_lc, "city", GedcomLineTag::City);
    tag_test!(test_parse_tag_conc, "CONC", GedcomLineTag::Concatenation);
    tag_test!(test_parse_tag_conc_lc, "conc", GedcomLineTag::Concatenation);
    tag_test!(test_parse_tag_conf, "CONF", GedcomLineTag::Confirmation);
    tag_test!(test_parse_tag_conf_lc, "conf", GedcomLineTag::Confirmation);
    tag_test!(test_parse_tag_conl, "CONL", GedcomLineTag::ConfirmationLds);
    tag_test!(
        test_parse_tag_conl_lc,
        "conl",
        GedcomLineTag::ConfirmationLds
    );
    tag_test!(test_parse_tag_cont, "CONT", GedcomLineTag::Continued);
    tag_test!(test_parse_tag_cont_lc, "cont", GedcomLineTag::Continued);
    tag_test!(test_parse_tag_copr, "COPR", GedcomLineTag::Copyright);
    tag_test!(test_parse_tag_copr_lc, "copr", GedcomLineTag::Copyright);
    tag_test!(test_parse_tag_corp, "CORP", GedcomLineTag::Corporate);
    tag_test!(test_parse_tag_corp_lc, "corp", GedcomLineTag::Corporate);
    tag_test!(test_parse_tag_crem, "CREM", GedcomLineTag::Cremation);
    tag_test!(test_parse_tag_crem_lc, "crem", GedcomLineTag::Cremation);
    tag_test!(test_parse_tag_ctry, "CTRY", GedcomLineTag::Country);
    tag_test!(test_parse_tag_ctry_lc, "ctry", GedcomLineTag::Country);
    tag_test!(test_parse_tag_data, "DATA", GedcomLineTag::Data);
    tag_test!(test_parse_tag_data_lc, "data", GedcomLineTag::Data);
    tag_test!(test_parse_tag_date, "DATE", GedcomLineTag::Date);
    tag_test!(test_parse_tag_date_lc, "date", GedcomLineTag::Date);
    tag_test!(test_parse_tag_deat, "DEAT", GedcomLineTag::Death);
    tag_test!(test_parse_tag_deat_lc, "deat", GedcomLineTag::Death);
    tag_test!(test_parse_tag_desc, "DESC", GedcomLineTag::Descendants);
    tag_test!(test_parse_tag_desc_lc, "desc", GedcomLineTag::Descendants);
    tag_test!(
        test_parse_tag_desi,
        "DESI",
        GedcomLineTag::DescendantInterest
    );
    tag_test!(
        test_parse_tag_desi_lc,
        "desi",
        GedcomLineTag::DescendantInterest
    );
    tag_test!(test_parse_tag_dest, "DEST", GedcomLineTag::Destination);
    tag_test!(test_parse_tag_dest_lc, "dest", GedcomLineTag::Destination);
    tag_test!(test_parse_tag_div, "DIV", GedcomLineTag::Divorce);
    tag_test!(test_parse_tag_div_lc, "div", GedcomLineTag::Divorce);
    tag_test!(test_parse_tag_divf, "DIVF", GedcomLineTag::DivorceFiled);
    tag_test!(test_parse_tag_divf_lc, "divf", GedcomLineTag::DivorceFiled);
    tag_test!(
        test_parse_tag_dscr,
        "DSCR",
        GedcomLineTag::PhysicalDescription
    );
    tag_test!(
        test_parse_tag_dscr_lc,
        "dscr",
        GedcomLineTag::PhysicalDescription
    );
    tag_test!(test_parse_tag_educ, "EDUC", GedcomLineTag::Education);
    tag_test!(test_parse_tag_educ_lc, "educ", GedcomLineTag::Education);
    tag_test!(test_parse_tag_emai, "EMAI", GedcomLineTag::Email);
    tag_test!(test_parse_tag_emai_lc, "emai", GedcomLineTag::Email);
    tag_test!(test_parse_tag_emig, "EMIG", GedcomLineTag::Emigration);
    tag_test!(test_parse_tag_emig_lc, "emig", GedcomLineTag::Emigration);
    tag_test!(test_parse_tag_endl, "ENDL", GedcomLineTag::Endowment);
    tag_test!(test_parse_tag_endl_lc, "endl", GedcomLineTag::Endowment);
    tag_test!(test_parse_tag_enga, "ENGA", GedcomLineTag::Engagement);
    tag_test!(test_parse_tag_enga_lc, "enga", GedcomLineTag::Engagement);
    tag_test!(test_parse_tag_even, "EVEN", GedcomLineTag::Event);
    tag_test!(test_parse_tag_even_lc, "even", GedcomLineTag::Event);
    tag_test!(test_parse_tag_fact, "FACT", GedcomLineTag::Fact);
    tag_test!(test_parse_tag_fact_lc, "fact", GedcomLineTag::Fact);
    tag_test!(test_parse_tag_fam, "FAM", GedcomLineTag::Family);
    tag_test!(test_parse_tag_fam_lc, "fam", GedcomLineTag::Family);
    tag_test!(test_parse_tag_famc, "FAMC", GedcomLineTag::FamilyChild);
    tag_test!(test_parse_tag_famc_lc, "famc", GedcomLineTag::FamilyChild);
    tag_test!(test_parse_tag_famf, "FAMF", GedcomLineTag::FamilyFile);
    tag_test!(test_parse_tag_famf_lc, "famf", GedcomLineTag::FamilyFile);
    tag_test!(test_parse_tag_fams, "FAMS", GedcomLineTag::FamilySpouse);
    tag_test!(test_parse_tag_fams_lc, "fams", GedcomLineTag::FamilySpouse);
    tag_test!(test_parse_tag_fax, "FAX", GedcomLineTag::Facsimile);
    tag_test!(test_parse_tag_fax_lc, "fax", GedcomLineTag::Facsimile);
    tag_test!(test_parse_tag_fcom, "FCOM", GedcomLineTag::FirstCommunion);
    tag_test!(
        test_parse_tag_fcom_lc,
        "fcom",
        GedcomLineTag::FirstCommunion
    );
    tag_test!(test_parse_tag_file, "FILE", GedcomLineTag::File);
    tag_test!(test_parse_tag_file_lc, "file", GedcomLineTag::File);
    tag_test!(test_parse_tag_form, "FORM", GedcomLineTag::Format);
    tag_test!(test_parse_tag_form_lc, "form", GedcomLineTag::Format);
    tag_test!(test_parse_tag_fone, "FONE", GedcomLineTag::Phonetic);
    tag_test!(test_parse_tag_fone_lc, "fone", GedcomLineTag::Phonetic);
    tag_test!(test_parse_tag_gedc, "GEDC", GedcomLineTag::Gedcom);
    tag_test!(test_parse_tag_gedc_lc, "gedc", GedcomLineTag::Gedcom);
    tag_test!(test_parse_tag_givn, "GIVN", GedcomLineTag::GivenName);
    tag_test!(test_parse_tag_givn_lc, "givn", GedcomLineTag::GivenName);
    tag_test!(test_parse_tag_grad, "GRAD", GedcomLineTag::Graduation);
    tag_test!(test_parse_tag_grad_lc, "grad", GedcomLineTag::Graduation);
    tag_test!(test_parse_tag_head, "HEAD", GedcomLineTag::Header);
    tag_test!(test_parse_tag_head_lc, "head", GedcomLineTag::Header);
    tag_test!(test_parse_tag_husb, "HUSB", GedcomLineTag::Husband);
    tag_test!(test_parse_tag_husb_lc, "husb", GedcomLineTag::Husband);
    tag_test!(test_parse_tag_idno, "IDNO", GedcomLineTag::IdentityNumber);
    tag_test!(
        test_parse_tag_idno_lc,
        "idno",
        GedcomLineTag::IdentityNumber
    );
    tag_test!(test_parse_tag_immi, "IMMI", GedcomLineTag::Immigration);
    tag_test!(test_parse_tag_immi_lc, "immi", GedcomLineTag::Immigration);
    tag_test!(test_parse_tag_indi, "INDI", GedcomLineTag::Individual);
    tag_test!(test_parse_tag_indi_lc, "indi", GedcomLineTag::Individual);
    tag_test!(test_parse_tag_lang, "LANG", GedcomLineTag::Language);
    tag_test!(test_parse_tag_lang_lc, "lang", GedcomLineTag::Language);
    tag_test!(test_parse_tag_lati, "LATI", GedcomLineTag::Latitude);
    tag_test!(test_parse_tag_lati_lc, "lati", GedcomLineTag::Latitude);
    tag_test!(test_parse_tag_long, "LONG", GedcomLineTag::Longitude);
    tag_test!(test_parse_tag_long_lc, "long", GedcomLineTag::Longitude);
    tag_test!(test_parse_tag_map, "MAP", GedcomLineTag::Map);
    tag_test!(test_parse_tag_map_lc, "map", GedcomLineTag::Map);
    tag_test!(test_parse_tag_marb, "MARB", GedcomLineTag::MarriageBanns);
    tag_test!(test_parse_tag_marb_lc, "marb", GedcomLineTag::MarriageBanns);
    tag_test!(test_parse_tag_marc, "MARC", GedcomLineTag::MarriageContract);
    tag_test!(
        test_parse_tag_marc_lc,
        "marc",
        GedcomLineTag::MarriageContract
    );
    tag_test!(test_parse_tag_marl, "MARL", GedcomLineTag::MarriageLicense);
    tag_test!(
        test_parse_tag_marl_lc,
        "marl",
        GedcomLineTag::MarriageLicense
    );
    tag_test!(test_parse_tag_marr, "MARR", GedcomLineTag::Marriage);
    tag_test!(test_parse_tag_marr_lc, "marr", GedcomLineTag::Marriage);
    tag_test!(
        test_parse_tag_mars,
        "MARS",
        GedcomLineTag::MarriageSettlement
    );
    tag_test!(
        test_parse_tag_mars_lc,
        "mars",
        GedcomLineTag::MarriageSettlement
    );
    tag_test!(test_parse_tag_medi, "MEDI", GedcomLineTag::Media);
    tag_test!(test_parse_tag_medi_lc, "medi", GedcomLineTag::Media);
    tag_test!(test_parse_tag_name, "NAME", GedcomLineTag::Name);
    tag_test!(test_parse_tag_name_lc, "name", GedcomLineTag::Name);
    tag_test!(test_parse_tag_nati, "NATI", GedcomLineTag::Nationality);
    tag_test!(test_parse_tag_nati_lc, "nati", GedcomLineTag::Nationality);
    tag_test!(test_parse_tag_natu, "NATU", GedcomLineTag::Naturalisation);
    tag_test!(
        test_parse_tag_natu_lc,
        "natu",
        GedcomLineTag::Naturalisation
    );
    tag_test!(test_parse_tag_nchi, "NCHI", GedcomLineTag::ChildrenCount);
    tag_test!(test_parse_tag_nchi_lc, "nchi", GedcomLineTag::ChildrenCount);
    tag_test!(test_parse_tag_nick, "NICK", GedcomLineTag::Nickname);
    tag_test!(test_parse_tag_nick_lc, "nick", GedcomLineTag::Nickname);
    tag_test!(test_parse_tag_nmr, "NMR", GedcomLineTag::MarriageCount);
    tag_test!(test_parse_tag_nmr_lc, "nmr", GedcomLineTag::MarriageCount);
    tag_test!(test_parse_tag_note, "NOTE", GedcomLineTag::Note);
    tag_test!(test_parse_tag_note_lc, "note", GedcomLineTag::Note);
    tag_test!(test_parse_tag_npfx, "NPFX", GedcomLineTag::NamePrefix);
    tag_test!(test_parse_tag_npfx_lc, "npfx", GedcomLineTag::NamePrefix);
    tag_test!(test_parse_tag_nsfx, "NSFX", GedcomLineTag::NameSuffix);
    tag_test!(test_parse_tag_nsfx_lc, "nsfx", GedcomLineTag::NameSuffix);
    tag_test!(test_parse_tag_obje, "OBJE", GedcomLineTag::Object);
    tag_test!(test_parse_tag_obje_lc, "obje", GedcomLineTag::Object);
    tag_test!(test_parse_tag_occu, "OCCU", GedcomLineTag::Occupation);
    tag_test!(test_parse_tag_occu_lc, "occu", GedcomLineTag::Occupation);
    tag_test!(test_parse_tag_ordi, "ORDI", GedcomLineTag::Ordinance);
    tag_test!(test_parse_tag_ordi_lc, "ordi", GedcomLineTag::Ordinance);
    tag_test!(test_parse_tag_ordn, "ORDN", GedcomLineTag::Ordination);
    tag_test!(test_parse_tag_ordn_lc, "ordn", GedcomLineTag::Ordination);
    tag_test!(test_parse_tag_page, "PAGE", GedcomLineTag::Page);
    tag_test!(test_parse_tag_page_lc, "page", GedcomLineTag::Page);
    tag_test!(test_parse_tag_pedi, "PEDI", GedcomLineTag::Pedigree);
    tag_test!(test_parse_tag_pedi_lc, "pedi", GedcomLineTag::Pedigree);
    tag_test!(test_parse_tag_phon, "PHON", GedcomLineTag::Phone);
    tag_test!(test_parse_tag_phon_lc, "phon", GedcomLineTag::Phone);
    tag_test!(test_parse_tag_plac, "PLAC", GedcomLineTag::Place);
    tag_test!(test_parse_tag_plac_lc, "plac", GedcomLineTag::Place);
    tag_test!(test_parse_tag_post, "POST", GedcomLineTag::PostalCode);
    tag_test!(test_parse_tag_post_lc, "post", GedcomLineTag::PostalCode);
    tag_test!(test_parse_tag_prob, "PROB", GedcomLineTag::Probate);
    tag_test!(test_parse_tag_prob_lc, "prob", GedcomLineTag::Probate);
    tag_test!(test_parse_tag_prop, "PROP", GedcomLineTag::Property);
    tag_test!(test_parse_tag_prop_lc, "prop", GedcomLineTag::Property);
    tag_test!(test_parse_tag_publ, "PUBL", GedcomLineTag::Publication);
    tag_test!(test_parse_tag_publ_lc, "publ", GedcomLineTag::Publication);
    tag_test!(test_parse_tag_quay, "QUAY", GedcomLineTag::QualityOfData);
    tag_test!(test_parse_tag_quay_lc, "quay", GedcomLineTag::QualityOfData);
    tag_test!(test_parse_tag_refn, "REFN", GedcomLineTag::Reference);
    tag_test!(test_parse_tag_refn_lc, "refn", GedcomLineTag::Reference);
    tag_test!(test_parse_tag_rela, "RELA", GedcomLineTag::Relationship);
    tag_test!(test_parse_tag_rela_lc, "rela", GedcomLineTag::Relationship);
    tag_test!(test_parse_tag_reli, "RELI", GedcomLineTag::Religion);
    tag_test!(test_parse_tag_reli_lc, "reli", GedcomLineTag::Religion);
    tag_test!(test_parse_tag_repo, "REPO", GedcomLineTag::Repository);
    tag_test!(test_parse_tag_repo_lc, "repo", GedcomLineTag::Repository);
    tag_test!(test_parse_tag_resi, "RESI", GedcomLineTag::Residence);
    tag_test!(test_parse_tag_resi_lc, "resi", GedcomLineTag::Residence);
    tag_test!(test_parse_tag_resn, "RESN", GedcomLineTag::Restriction);
    tag_test!(test_parse_tag_resn_lc, "resn", GedcomLineTag::Restriction);
    tag_test!(test_parse_tag_reti, "RETI", GedcomLineTag::Retirement);
    tag_test!(test_parse_tag_reti_lc, "reti", GedcomLineTag::Retirement);
    tag_test!(test_parse_tag_rfn, "RFN", GedcomLineTag::RecordFileNumber);
    tag_test!(
        test_parse_tag_rfn_lc,
        "rfn",
        GedcomLineTag::RecordFileNumber
    );
    tag_test!(test_parse_tag_rin, "RIN", GedcomLineTag::RecordIdNumber);
    tag_test!(test_parse_tag_rin_lc, "rin", GedcomLineTag::RecordIdNumber);
    tag_test!(test_parse_tag_role, "ROLE", GedcomLineTag::Role);
    tag_test!(test_parse_tag_role_lc, "role", GedcomLineTag::Role);
    tag_test!(test_parse_tag_romn, "ROMN", GedcomLineTag::Romanised);
    tag_test!(test_parse_tag_romn_lc, "romn", GedcomLineTag::Romanised);
    tag_test!(test_parse_tag_sex, "SEX", GedcomLineTag::Sex);
    tag_test!(test_parse_tag_sex_lc, "sex", GedcomLineTag::Sex);
    tag_test!(test_parse_tag_slgc, "SLGC", GedcomLineTag::SealingChild);
    tag_test!(test_parse_tag_slgc_lc, "slgc", GedcomLineTag::SealingChild);
    tag_test!(test_parse_tag_slgs, "SLGS", GedcomLineTag::SealingSpouse);
    tag_test!(test_parse_tag_slgs_lc, "slgs", GedcomLineTag::SealingSpouse);
    tag_test!(test_parse_tag_sour, "SOUR", GedcomLineTag::Source);
    tag_test!(test_parse_tag_sour_lc, "sour", GedcomLineTag::Source);
    tag_test!(test_parse_tag_spfx, "SPFX", GedcomLineTag::SurnamePrefix);
    tag_test!(test_parse_tag_spfx_lc, "spfx", GedcomLineTag::SurnamePrefix);
    tag_test!(
        test_parse_tag_ssn,
        "SSN",
        GedcomLineTag::SocialSecurityNumber
    );
    tag_test!(
        test_parse_tag_ssn_lc,
        "ssn",
        GedcomLineTag::SocialSecurityNumber
    );
    tag_test!(test_parse_tag_stae, "STAE", GedcomLineTag::State);
    tag_test!(test_parse_tag_stae_lc, "stae", GedcomLineTag::State);
    tag_test!(test_parse_tag_stat, "STAT", GedcomLineTag::Status);
    tag_test!(test_parse_tag_stat_lc, "stat", GedcomLineTag::Status);
    tag_test!(test_parse_tag_subm, "SUBM", GedcomLineTag::Submitter);
    tag_test!(test_parse_tag_subm_lc, "subm", GedcomLineTag::Submitter);
    tag_test!(test_parse_tag_subn, "SUBN", GedcomLineTag::Submission);
    tag_test!(test_parse_tag_subn_lc, "subn", GedcomLineTag::Submission);
    tag_test!(test_parse_tag_surn, "SURN", GedcomLineTag::Surname);
    tag_test!(test_parse_tag_surn_lc, "surn", GedcomLineTag::Surname);
    tag_test!(test_parse_tag_temp, "TEMP", GedcomLineTag::Temple);
    tag_test!(test_parse_tag_temp_lc, "temp", GedcomLineTag::Temple);
    tag_test!(test_parse_tag_text, "TEXT", GedcomLineTag::Text);
    tag_test!(test_parse_tag_text_lc, "text", GedcomLineTag::Text);
    tag_test!(test_parse_tag_time, "TIME", GedcomLineTag::Time);
    tag_test!(test_parse_tag_time_lc, "time", GedcomLineTag::Time);
    tag_test!(test_parse_tag_titl, "TITL", GedcomLineTag::Title);
    tag_test!(test_parse_tag_titl_lc, "titl", GedcomLineTag::Title);
    tag_test!(test_parse_tag_trlr, "TRLR", GedcomLineTag::Trailer);
    tag_test!(test_parse_tag_trlr_lc, "trlr", GedcomLineTag::Trailer);
    tag_test!(test_parse_tag_type, "TYPE", GedcomLineTag::Type);
    tag_test!(test_parse_tag_type_lc, "type", GedcomLineTag::Type);
    tag_test!(test_parse_tag_vers, "VERS", GedcomLineTag::Version);
    tag_test!(test_parse_tag_vers_lc, "vers", GedcomLineTag::Version);
    tag_test!(test_parse_tag_wife, "WIFE", GedcomLineTag::Wife);
    tag_test!(test_parse_tag_wife_lc, "wife", GedcomLineTag::Wife);
    tag_test!(test_parse_tag_will, "WILL", GedcomLineTag::Will);
    tag_test!(test_parse_tag_will_lc, "will", GedcomLineTag::Will);
    tag_test!(test_parse_tag_www, "WWW", GedcomLineTag::Web);
    tag_test!(test_parse_tag_www_lc, "www", GedcomLineTag::Web);
}
