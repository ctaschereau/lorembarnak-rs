#[macro_use] extern crate lazy_static;

use rand::Rng;
use regex::Regex;
use case::CaseExt;

// TODO : Should I define a new type for this?
// TODO : Should I have a static lifetime for this???
type SwearVariants = Vec<&'static str>;

fn get_all_swears() -> Vec<SwearVariants> {
    vec![
        vec!["tabarnak", "tabarnouche", "tabarouette", "taboire", "tabarslaque", "tabarnane"],
        vec!["câlisse", "câlique", "câline", "câline de bine", "câliboire", "caltor"],
        vec!["crisse", "christie", "crime", "bout d’crisse"],
        vec!["ostie", "astie", "estique", "ostifie", "esprit"],
        vec!["ciboire", "saint-ciboire"],
        vec!["torrieux", "torvisse"],
        vec!["cimonaque", "saint-cimonaque"],
        vec!["baptême", "batince", "batèche"],
        vec!["bâtard"],
        vec!["calvaire", "calvince", "calvinouche"],
        vec!["mosus"],
        vec!["maudit", "mautadit", "maudine", "mautadine"],
        vec!["sacrament", "sacréfice", "saint-sacrament"],
        vec!["viarge", "sainte-viarge", "bout d’viarge"],
        vec!["ciarge", "saint-ciarge", "bout d’ciarge"],
        vec!["cibouleau"],
        vec!["cibole", "cibolac"],
        vec!["enfant d’chienne"],
        vec!["verrat"],
        vec!["marde", "maudite marde", "mangeux d’marde"],
        vec!["boswell"],
        vec!["sacristi", "sapristi"],
        vec!["Jésus de plâtre", "Jésus Marie Joseph", "p’tit Jésus", "doux Jésus"],
        vec!["crucifix"],
        vec!["patente à gosse", "cochonnerie", "cossin"],
        vec!["viande à chien"],
        vec!["cul", "saintes fesses"],
        vec!["purée"],
        vec!["étole"],
        vec!["charogne", "charrue"],
        vec!["gériboire", "géritole"],
        vec!["colon"],
    ]
}

/// Generates a chain of Québécois obscenities.
/// ```
/// # use lorembarnak_rs;
/// println!("{}", lorembarnak_rs::get_text(Some(5)));
/// ```
pub fn get_text(nb_requested_option: Option<i16>) -> String {
    let nb_requested = nb_requested_option.unwrap_or_else(|| random_i16(4) + 6);

    let mut remaining = get_all_swears();
    let mut result = String::new();
    let mut previous_swear = "";
    let mut previous_index: Option<i16> = None;

    for i in 0 .. nb_requested {
        let mut current_index: Option<i16> = None;

        // If we've run out of remaining swears or only the previous family remains, reinitialize remaining.
        if remaining.is_empty() || (remaining.len() as i16 == 1 && previous_index.is_some()) {
            remaining = get_all_swears();
        }

        // Choose a random swear family that isn't the previous one.
        while current_index.is_none() || current_index == previous_index || remaining[current_index.unwrap() as usize].contains(&previous_swear) {
            current_index = Some(random_i16(remaining.len() as i16));
        }
        let family: &mut SwearVariants = remaining.get_mut(current_index.unwrap() as usize).unwrap();
        previous_index = current_index;

        // Choose a random swear, and delete the family if empty.
        let current: &str = family.remove(random_i16(family.len() as i16) as usize);
        previous_swear = current;
        if family.is_empty() {
            remaining.remove(current_index.unwrap() as usize);
            previous_index = None;
        }

        // Capitalize the fist swear, add an article prefix to others.
        if i == 0 {
            result.push_str( current.to_capitalized().as_str());
        } else {
            result.push_str(with_article(current).as_str());
        }

        // Add a period after the last swear, a space after others.
        if i == nb_requested - 1 {
            result.push_str(".");
        } else {
            result.push_str(" ")
        }
    }

    return result;
}

fn with_article(s : &str) -> String {
    lazy_static! {
        static ref STARTS_WITH_PREFIX: Regex = Regex::new(r"^(de\s|d’)").unwrap();
        static ref STARTS_WITH_VOWEL: Regex = Regex::new(r"^[aeiouhyAEIOUHYÀ-ÖØ-öø-ÿ]").unwrap();
    }

    let prefix: &str;

    if STARTS_WITH_PREFIX.is_match(s) {
        // If it already starts with "de" or "d’", don't add another.
        prefix = "";
    } else if STARTS_WITH_VOWEL.is_match(s) {
        // If it starts with a vowel, prepend with "d'"
        prefix = "d’";
    } else {
        // Otherwise prepend with "de"
        prefix = "de ";
    }

    format!("{}{}", prefix, s)
}

fn random_i16(max: i16) -> i16 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_article_works() {
        assert_eq!(with_article("tabarnak"), "de tabarnak");
        assert_eq!(with_article("d’ostie"), "d’ostie");
        assert_eq!(with_article("ostie"), "d’ostie");
    }

    #[test]
    fn random_works() {
        assert!(random_i16(100) >= 0);
        assert!(random_i16(100) < 100);
    }
}
