const ADJ_LIST: &'static [&'static str] = &["adj-f", "adj-i", "adj-ix", "adj-ku", "adj-na", "adj-nari", "adj-no", "adj-pn", "adj-shiku", "adj-t", "aux-adj"];

const ADV_LIST: &'static [&'static str] = &["adv-to", "adv"];

const AUX_V_LIST: &'static [&'static str] = &["aux-v"];

const CONJ_LIST: &'static [&'static str] = &["conj"];

const INT_LIST: &'static [&'static str] = &["int"];

const N_LIST: &'static [&'static str] = &["n-adv", "n-pr", "n-pref", "n-suf", "n-t", "n"];

const PRT_LIST: &'static [&'static str] = &["prt"];

const PREF_LIST: &'static [&'static str] = &["pref"];

const SUF_LIST: &'static [&'static str] = &["suf"];

const V_LIST: &'static [&'static str] = &["v1-s", "v1", "v2a-s", "v2b-k", "v2d-s", "v2g-k", "v2g-s", "v2h-k", "v2h-s", "v2k-k", "v2k-s", "v2m-s", "v2n-s", "v2r-k", "v2r-s", "v2s-s", "v2t-k", "v2t-s", "v2w-s", "v2y-k", "v2y-s", "v2z-s", "v4b", "v4g", "v4h", "v4k", "v4m", "v4r", "v4s", "v4t", "v5aru", "v5b", "v5g", "v5k-s", "v5k", "v5m", "v5n", "v5r-i", "v5r", "v5s", "v5t", "v5u-s", "v5u", "vi", "vk", "vn", "vr", "vs-c", "vs-i", "vs-s", "vs", "vt", "vz"];

fn conversion_map(string: &str) -> Option<&[&str]> {
    match string {
            "形容詞"    => Some(ADJ_LIST),      // adjective
            "副詞"      => Some(ADV_LIST),      // adverb 
            "助動詞"    => Some(AUX_V_LIST),    // auxiliary verb
            "接続詞"    => Some(CONJ_LIST),     // conjunction
            "感動詞"    => Some(INT_LIST),      // interjection
            "名詞"      => Some(N_LIST),        // noun
            "助詞"      => Some(PRT_LIST),      // particle
            "接頭辞"    => Some(PREF_LIST),     // prefix
            "接尾辞"    => Some(SUF_LIST),      // suffix
            "動詞"      => Some(V_LIST),        // verb
            _           => None
    }
}

pub fn convert_pos_list(pos_list: &Vec<String>) -> Vec<String> {
    let mut converted_list: Vec<String> = Vec::new();
    for pos in pos_list.iter() {
        let list = conversion_map(pos);

        if let Some(list) = list {
            for item in list {
                converted_list.push(item.to_string());
            }
        }
    }

    converted_list
}
