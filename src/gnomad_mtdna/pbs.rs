//! Data structures for (de-)serialization as generated by `prost-build`.

use noodles_vcf::record::info::field;
use std::str::FromStr;

include!(concat!(env!("OUT_DIR"), "/annonars.gnomad_mtdna.pbs.rs"));

/// Options struct that allows to specify which details fields are to be extracted from
/// gnomAD-mtDNA VCF records.
///
/// The only field that has `true` as its default is `vep`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DetailsOptions {
    /// Enable extraction of `Vep` records.
    pub vep: bool,
    /// Enable creation of `QualityInfo`.
    pub quality: bool,
    /// Enable creation of `HeteroplasmyInfo`.
    pub heteroplasmy: bool,
    /// Enable creation of `FilterHistograms`.
    pub filter_hists: bool,
    /// Enable creation of `PopulationInfo`.
    pub pop_details: bool,
    /// Enable creation of `HaplogroupInfo`.
    pub haplogroups_details: bool,
    /// Enable creation of `AgeInfo`.
    pub age_hists: bool,
    /// Enable creation of `DepthInfo`.
    pub depth_details: bool,
}

impl Default for DetailsOptions {
    fn default() -> Self {
        Self {
            vep: true,
            quality: false,
            heteroplasmy: false,
            filter_hists: false,
            pop_details: false,
            haplogroups_details: false,
            age_hists: false,
            depth_details: false,
        }
    }
}

impl DetailsOptions {
    /// Create a new `DetailsOptions` with all fields enabled.
    pub fn with_all_enabled() -> Self {
        Self {
            vep: true,
            quality: true,
            heteroplasmy: true,
            filter_hists: true,
            pop_details: true,
            haplogroups_details: true,
            age_hists: true,
            depth_details: true,
        }
    }
}

impl From<(String, f32)> for Prediction {
    fn from((prediction, score): (String, f32)) -> Self {
        Self { prediction, score }
    }
}

impl FromStr for Vep {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s.split('|').collect::<Vec<_>>();

        Ok(Vep {
            allele: values[0].to_string(),
            consequence: values[1].to_string(),
            impact: values[2].to_string(),
            symbol: values[3].to_string(),
            gene: values[4].to_string(),
            feature_type: values[5].to_string(),
            feature: values[6].to_string(),
            feature_biotype: values[7].to_string(),
            exon: (!values[8].is_empty()).then(|| values[8].to_string()),
            intron: (!values[9].is_empty()).then(|| values[9].to_string()),
            hgvsc: (!values[10].is_empty()).then(|| values[10].to_string()),
            hgvsp: (!values[11].is_empty()).then(|| values[11].to_string()),
            cdna_position: (!values[12].is_empty()).then(|| values[12].to_string()),
            cds_position: (!values[13].is_empty()).then(|| values[13].to_string()),
            protein_position: (!values[14].is_empty()).then(|| values[14].to_string()),
            amino_acids: (!values[15].is_empty()).then(|| values[15].to_string()),
            codons: (!values[16].is_empty()).then(|| values[16].to_string()),
            dbsnp_id: (!values[17].is_empty()).then(|| values[17].to_string()),
            distance: (!values[18].is_empty()).then(|| values[18].to_string()),
            strand: (!values[19].is_empty()).then(|| values[19].to_string()),
            variant_class: (!values[20].is_empty()).then(|| values[20].to_string()),
            minimised: (!values[21].is_empty()).then(|| values[21].to_string()),
            symbol_source: (!values[22].is_empty()).then(|| values[22].to_string()),
            hgnc_id: (!values[23].is_empty()).then(|| values[23].to_string()),
            canonical: (!values[24].is_empty()).then(|| values[24] == "YES"),
            tsl: (!values[25].is_empty())
                .then(|| values[25].parse())
                .transpose()?,
            appris: (!values[26].is_empty()).then(|| values[26].to_string()),
            ccds: (!values[27].is_empty()).then(|| values[27].to_string()),
            ensp: (!values[28].is_empty()).then(|| values[28].to_string()),
            swissprot: (!values[29].is_empty()).then(|| values[29].to_string()),
            trembl: (!values[30].is_empty()).then(|| values[30].to_string()),
            uniparc: (!values[31].is_empty()).then(|| values[31].to_string()),
            gene_pheno: (!values[32].is_empty()).then(|| values[32].to_string()),
            sift: (!values[33].is_empty())
                .then(|| -> Result<(String, f32), anyhow::Error> {
                    let tokens = values[33].split('(').collect::<Vec<_>>();
                    let mut tmp = tokens[1].chars();
                    tmp.next_back();
                    let score = tmp.as_str();
                    Ok((tokens[0].to_string(), score.parse::<f32>()?))
                })
                .transpose()?
                .map(|val| val.into()),
            polyphen: (!values[34].is_empty())
                .then(|| -> Result<(String, f32), anyhow::Error> {
                    let tokens = values[34].split('(').collect::<Vec<_>>();
                    let mut tmp = tokens[1].chars();
                    tmp.next_back();
                    let score = tmp.as_str();
                    Ok((tokens[0].to_string(), score.parse::<f32>()?))
                })
                .transpose()?
                .map(|val| val.into()),
            domains: (!values[35].is_empty())
                .then(|| {
                    let pairs = values[35].split('&').collect::<Vec<_>>();
                    pairs
                        .iter()
                        .map(|p| {
                            let tmp = p.split(':').collect::<Vec<_>>();
                            Domain {
                                source: tmp[0].to_string(),
                                id: tmp[1].to_string(),
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default(),
            hgvs_offset: (!values[36].is_empty()).then(|| values[36].to_string()),
            motif_name: (!values[37].is_empty()).then(|| values[37].to_string()),
            motif_pos: (!values[38].is_empty()).then(|| values[38].to_string()),
            high_inf_pos: (!values[39].is_empty()).then(|| values[39].to_string()),
            motif_score_change: (!values[40].is_empty()).then(|| values[40].to_string()),
            lof: (!values[41].is_empty()).then(|| values[41].to_string()),
            lof_filter: (!values[42].is_empty()).then(|| values[42].to_string()),
            lof_flags: (!values[43].is_empty()).then(|| values[43].to_string()),
            lof_info: (!values[44].is_empty()).then(|| values[44].to_string()),
        })
    }
}

impl Record {
    /// Creates a new `Record` from a VCF record and allele number.
    pub fn from_vcf_allele(
        record: &noodles_vcf::record::Record,
        allele_no: usize,
        options: &DetailsOptions,
    ) -> Result<Self, anyhow::Error> {
        assert!(allele_no == 0, "only allele 0 is supported");

        // Extract mandatory fields.
        let chrom = record.chromosome().to_string();
        let pos: usize = record.position().into();
        let pos = pos as i32;
        let ref_allele = record.reference_bases().to_string();
        let alt_allele = record
            .alternate_bases()
            .get(allele_no)
            .ok_or_else(|| anyhow::anyhow!("no such allele: {}", allele_no))?
            .to_string();
        let variant_collapsed = Self::get_string(record, "variant_collapsed")?;
        let excluded_ac = Self::get_i32(record, "excluded_AC")?;
        let an = Self::get_i32(record, "AN")?;
        let ac_hom = Self::get_i32(record, "AC_hom")?;
        let ac_het = Self::get_i32(record, "AC_het")?;
        let af_hom = Self::get_f32(record, "AF_hom")?;
        let af_het = Self::get_f32(record, "AF_het")?;
        let filters = Self::get_filters(record)?;
        let mitotip_score = Self::get_f32(record, "mitotip_score").ok();
        let mitotip_trna_prediction = Self::get_string(record, "mitotip_trna_prediction").ok();
        let pon_mt_trna_prediction = Self::get_string(record, "pon_mt_trna_prediction").ok();
        let pon_ml_probability_of_pathogenicity =
            Self::get_string(record, "pon_ml_probability_of_pathogenicity").ok();

        // Extract optional fields.
        let vep = options
            .vep
            .then(|| Self::extract_vep(record))
            .transpose()?
            .unwrap_or_default();
        let quality_info = options
            .quality
            .then(|| Self::extract_quality(record))
            .transpose()?;
        let heteroplasmy_info = options
            .heteroplasmy
            .then(|| Self::extract_heteroplasmy(record))
            .transpose()?;
        let filter_histograms = options
            .filter_hists
            .then(|| Self::extract_filter_histograms(record))
            .transpose()?;
        let population_info = options
            .pop_details
            .then(|| Self::extract_population(record))
            .transpose()?;
        let haplogroup_info = options
            .haplogroups_details
            .then(|| Self::extract_haplogroup(record))
            .transpose()?;
        let age_info = options
            .age_hists
            .then(|| Self::extract_age(record))
            .transpose()?;
        let depth_info = options
            .depth_details
            .then(|| Self::extract_depth(record))
            .transpose()?;

        Ok(Record {
            chrom,
            pos,
            ref_allele,
            alt_allele,
            variant_collapsed,
            excluded_ac,
            an,
            ac_hom,
            ac_het,
            af_hom,
            af_het,
            filters,
            mitotip_score,
            mitotip_trna_prediction,
            pon_mt_trna_prediction,
            pon_ml_probability_of_pathogenicity,
            vep,
            quality_info,
            heteroplasmy_info,
            filter_histograms,
            population_info,
            haplogroup_info,
            age_info,
            depth_info,
        })
    }

    /// Extract the "vep" field.
    fn extract_vep(record: &noodles_vcf::Record) -> Result<Vec<Vep>, anyhow::Error> {
        if let Some(Some(field::Value::Array(field::value::Array::String(v)))) =
            record.info().get(&field::Key::from_str("vep")?)
        {
            v.iter()
                .flat_map(|v| v.as_ref().map(|s| Vep::from_str(s)))
                .collect::<Result<Vec<_>, _>>()
        } else {
            anyhow::bail!("missing INFO/vep in gnomAD-mtDNA record")
        }
    }

    /// Extract the heteroplasmy-related fields from the VCF record.
    fn extract_heteroplasmy(
        record: &noodles_vcf::record::Record,
    ) -> Result<HeteroplasmyInfo, anyhow::Error> {
        Ok(HeteroplasmyInfo {
            heteroplasmy_below_min_het_threshold_hist: Self::get_vec::<i32>(
                record,
                "heteroplasmy_below_min_het_threshold_hist",
            )?,
            hl_hist: Self::get_vec::<i32>(record, "hl_hist")?,
            common_low_heteroplasmy: Self::get_flag(record, "common_low_heteroplasmy")?,
            max_hl: Self::get_f32(record, "max_hl")?,
        })
    }

    /// Extract the filter histogram related fields form the VCF record.
    fn extract_filter_histograms(
        record: &noodles_vcf::record::Record,
    ) -> Result<FilterHistograms, anyhow::Error> {
        Ok(FilterHistograms {
            base_qual_hist: Self::get_vec::<i32>(record, "base_qual_hist").unwrap_or_default(),
            position_hist: Self::get_vec::<i32>(record, "position_hist").unwrap_or_default(),
            strand_bias_hist: Self::get_vec::<i32>(record, "strand_bias_hist").unwrap_or_default(),
            weak_evidence_hist: Self::get_vec::<i32>(record, "weak_evidence_hist")
                .unwrap_or_default(),
            contamination_hist: Self::get_vec::<i32>(record, "contamination_hist")
                .unwrap_or_default(),
        })
    }

    /// Extract the population related fields from the VCF record.
    fn extract_population(
        record: &noodles_vcf::record::Record,
    ) -> Result<PopulationInfo, anyhow::Error> {
        Ok(PopulationInfo {
            pop_an: Self::get_vec::<i32>(record, "pop_AN")?,
            pop_ac_het: Self::get_vec::<i32>(record, "pop_AC_het").unwrap_or_default(),
            pop_ac_hom: Self::get_vec::<i32>(record, "pop_AC_hom").unwrap_or_default(),
            pop_af_hom: Self::get_vec::<f32>(record, "pop_AF_hom").unwrap_or_default(),
            pop_af_het: Self::get_vec::<f32>(record, "pop_AF_het").unwrap_or_default(),
            pop_hl_hist: Self::get_vec_vec::<i32>(record, "pop_hl_hist").unwrap_or_default(),
        })
    }

    /// Extract the haplogroup related fields from the VCF record.
    fn extract_haplogroup(
        record: &noodles_vcf::record::Record,
    ) -> Result<HaplogroupInfo, anyhow::Error> {
        Ok(HaplogroupInfo {
            hap_defining_variant: Self::get_flag(record, "hap_defining_variant")?,
            hap_an: Self::get_vec::<i32>(record, "hap_AN").unwrap_or_default(),
            hap_ac_het: Self::get_vec::<i32>(record, "hap_AC_het").unwrap_or_default(),
            hap_ac_hom: Self::get_vec::<i32>(record, "hap_AC_hom").unwrap_or_default(),
            hap_af_het: Self::get_vec::<f32>(record, "hap_AF_het").unwrap_or_default(),
            hap_af_hom: Self::get_vec::<f32>(record, "hap_AF_hom").unwrap_or_default(),
            hap_hl_hist: Self::get_vec_vec::<i32>(record, "hap_hl_hist").unwrap_or_default(),
            hap_faf_hom: Self::get_vec::<f32>(record, "hap_faf_hom").unwrap_or_default(),
            hapmax_af_hom: Self::get_string(record, "hapmax_AF_hom").ok(),
            hapmax_af_het: Self::get_string(record, "hapmax_AF_het").ok(),
            faf_hapmax_hom: Self::get_f32(record, "faf_hapmax_hom").ok(),
        })
    }

    /// Extract the age related fields from the VCF record.
    fn extract_age(record: &noodles_vcf::record::Record) -> Result<AgeInfo, anyhow::Error> {
        Ok(AgeInfo {
            age_hist_hom_bin_freq: Self::get_vec::<i32>(record, "age_hist_hom_bin_freq")
                .unwrap_or_default(),
            age_hist_hom_n_smaller: Self::get_i32(record, "age_hist_hom_n_smaller").ok(),
            age_hist_hom_n_larger: Self::get_i32(record, "age_hist_hom_n_larger").ok(),
            age_hist_het_bin_freq: Self::get_vec::<i32>(record, "age_hist_het_bin_freq")
                .unwrap_or_default(),
            age_hist_het_n_smaller: Self::get_i32(record, "age_hist_het_n_smaller").ok(),
            age_hist_het_n_larger: Self::get_i32(record, "age_hist_het_n_larger").ok(),
        })
    }

    /// Extract the depth related fields from the VCF record.
    fn extract_depth(record: &noodles_vcf::record::Record) -> Result<DepthInfo, anyhow::Error> {
        Ok(DepthInfo {
            dp_hist_all_n_larger: Self::get_i32(record, "dp_hist_all_n_larger").ok(),
            dp_hist_alt_n_larger: Self::get_i32(record, "dp_hist_alt_n_larger").ok(),
            dp_hist_all_bin_freq: Self::get_vec::<i32>(record, "dp_hist_all_bin_freq")
                .unwrap_or_default(),
            dp_hist_alt_bin_freq: Self::get_vec::<i32>(record, "dp_hist_alt_bin_freq")
                .unwrap_or_default(),
        })
    }

    /// Extract the quality-related fields from the VCF record.
    fn extract_quality(record: &noodles_vcf::record::Record) -> Result<QualityInfo, anyhow::Error> {
        Ok(QualityInfo {
            dp_mean: Self::get_f32(record, "dp_mean").ok(),
            mq_mean: Self::get_f32(record, "mq_mean").ok(),
            tlod_mean: Self::get_f32(record, "tlod_mean").ok(),
        })
    }

    /// Extract a `String` field from a record.
    fn get_string(record: &noodles_vcf::Record, name: &str) -> Result<String, anyhow::Error> {
        if let Some(Some(field::Value::String(v))) = record.info().get(&field::Key::from_str(name)?)
        {
            Ok(v.to_string())
        } else {
            anyhow::bail!("missing INFO/{} in gnomAD-mtDNA record", name)
        }
    }

    /// Extract a flag field from a record.
    fn get_flag(record: &noodles_vcf::Record, name: &str) -> Result<bool, anyhow::Error> {
        Ok(matches!(
            record.info().get(&field::Key::from_str(name)?),
            Some(Some(field::Value::Flag))
        ))
    }

    /// Extract an `i32` field from a record.
    fn get_i32(record: &noodles_vcf::Record, name: &str) -> Result<i32, anyhow::Error> {
        if let Some(Some(field::Value::Integer(v))) =
            record.info().get(&field::Key::from_str(name)?)
        {
            Ok(*v)
        } else {
            anyhow::bail!("missing INFO/{} in gnomAD-mtDNA record", name)
        }
    }

    /// Extract an `f32` field from a record.
    fn get_f32(record: &noodles_vcf::Record, name: &str) -> Result<f32, anyhow::Error> {
        if let Some(Some(field::Value::Float(v))) = record.info().get(&field::Key::from_str(name)?)
        {
            Ok(*v)
        } else {
            anyhow::bail!("missing INFO/{} in gnomAD-mtDNA record", name)
        }
    }

    /// Extract an `Vec<FromStr>` field from a record encoded as a pipe symbol separated string.
    fn get_vec<T>(record: &noodles_vcf::Record, name: &str) -> Result<Vec<T>, anyhow::Error>
    where
        T: FromStr,
    {
        if let Some(Some(field::Value::String(v))) = record.info().get(&field::Key::from_str(name)?)
        {
            v.split('|')
                .map(|s| s.parse())
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| anyhow::anyhow!("failed to parse INFO/{} as Vec<_>", name))
        } else {
            anyhow::bail!("missing INFO/{} in gnomAD-mtDNA record", name)
        }
    }

    /// Extract an `Vec<Vec<FromStr>>` field from a record encoded as a list of pipe symbol
    /// separated string.
    fn get_vec_vec<T>(record: &noodles_vcf::Record, name: &str) -> Result<Vec<T>, anyhow::Error>
    where
        T: FromStr,
    {
        if let Some(Some(field::Value::Array(field::value::Array::String(value)))) =
            record.info().get(&field::Key::from_str(name)?)
        {
            Ok(value
                .iter()
                .map(|s| {
                    s.as_ref()
                        .ok_or(anyhow::anyhow!("missing value in INFO/hap_hl_hist"))
                        .map(|s| {
                            s.split('|')
                                .map(|s| s.parse())
                                .collect::<Result<Vec<T>, _>>()
                        })
                })
                .map(|r| match r {
                    Ok(Ok(v)) => Ok(v),
                    _ => anyhow::bail!("missing value in INFO/hap_hl_hist"),
                })
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| {
                    anyhow::anyhow!("failed to parse INFO/{} as Vec<Vec<_>>: {}", name, e)
                })?
                .into_iter()
                .flatten()
                .collect())
        } else {
            anyhow::bail!("missing INFO/{} in gnomAD-mtDNA record", name)
        }
    }

    /// Extract the filters fields.
    fn get_filters(record: &noodles_vcf::Record) -> Result<Vec<i32>, anyhow::Error> {
        Ok(
            if let Some(Some(field::Value::Array(field::value::Array::String(value)))) =
                record.info().get(&field::Key::from_str("filters")?)
            {
                value
                    .iter()
                    .map(|v| match v.as_ref().map(|s| s.as_str()) {
                        Some("artifact_prone_site") => Ok(Filter::ArtifactProneSite as i32),
                        Some("indel_stack") => Ok(Filter::IndelStack as i32),
                        Some("npg") => Ok(Filter::NoPassGenotype as i32),
                        Some(val) => anyhow::bail!("invalid filter value {}", val),
                        None => anyhow::bail!("missing filter value"),
                    })
                    .collect::<Result<Vec<_>, _>>()?
            } else {
                Vec::new()
            },
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vep_from_string() {
        let s = "\
        G|missense_variant|MODERATE|MT-ND5|ENSG00000198786|Transcript|ENST00000361567|\
        protein_coding|1/1||ENST00000361567.2:c.208A>G|ENSP00000354813.2:p.Thr70Ala|208|\
        208|70|T/A|Aca/Gca|1||1|SNV||HGNC|HGNC:7461|YES||P1||ENSP00000354813||||1|\
        deleterious_low_confidence(0.020)|benign(0.033)|ENSP_mappings:5xtc&ENSP_mappings:5xtd&\
        ENSP_mappings:5xth&ENSP_mappings:5xti&ENSP_mappings:5xti&Pfam:PF00662&PANTHER:PTHR42829&\
        PANTHER:PTHR42829&TIGRFAM:TIGR01974|||||||||";
        let vep = Vep::from_str(s).unwrap();

        insta::assert_yaml_snapshot!(vep);
    }

    #[test]
    fn test_record_from_vcf_allele() -> Result<(), anyhow::Error> {
        let path_vcf = "tests/gnomad-mtdna/example/gnomad-mtdna.vcf";
        let mut reader_vcf =
            noodles_util::variant::reader::Builder::default().build_from_path(path_vcf)?;
        let header = reader_vcf.read_header()?;

        let mut records = Vec::new();
        for row in reader_vcf.records(&header) {
            let vcf_record = row?;
            let record =
                Record::from_vcf_allele(&vcf_record, 0, &DetailsOptions::with_all_enabled())?;
            records.push(record);
        }

        insta::assert_yaml_snapshot!(records);

        Ok(())
    }
}
