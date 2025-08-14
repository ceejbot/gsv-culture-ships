//! In case you ever needed Iain M. Banks's Culture ship names as a service. Names sourced from the
//! pleasingly extensive [Wikipedia article](https://en.wikipedia.org/wiki/List_of_spacecraft_in_the_Culture_series)
//! listing them, with a handful of additions.
//!
//! # Examples
//!
//! ```
//! let ship = gsv_culture_ships::random();
//! println!("Out-of-context problem observed by {}.", ship);
//! ```

const SHIPS: [&str; 172] = [
    "(D)GOU Limiting Factor",
    "(D)ROU Zealot",
    "(ex-)GCU Smile Tolerantly",
    "Beastly To The Animals",
    "Clipper Screw Loose",
    "Cruise Ship Just Passing Through",
    "Fixed Grin",
    "FP/(D)GOU Eight Rounds Rapid",
    "FP/(D)GOU No One Knows What The Dead Think",
    "FP/(D)LOU Hylozoist",
    "FP/(D)ROU Refreshingly Unconcerned With The Vulgar Exigencies Of Veracity",
    "FP/(D)ROU The Usual But Etymologically Unsatisfactory",
    "FP/(D)ROU Value Judgement",
    "GCU A Series Of Unlikely Explanations",
    "GCU A Ship With A View",
    "GCU Ablation",
    "GCU Arbitrary",
    "GCU Armchair Traveller",
    "GCU Arrested Development",
    "GCU Big Sexy Beast",
    "GCU Bodhisattva, OAQS",
    "GCU Boo!",
    "GCU Cantankerous",
    "GCU Credibility Problem",
    "GCU Different Tan",
    "GCU Displacement Activity",
    "GCU Dramatic Exit",
    "GCU Due to the Intimate Nature of the Hoagie Room",
    "GCU Excuses And Accusations",
    "GCU Experiencing A Significant Gravitas Shortfall",
    "GCU Fate Amenable To Change",
    "GCU Flexible Demeanour",
    "GCU Funny, It Worked Last Time...",
    "GCU God Told Me To Do It",
    "GCU Grey Area",
    "GCU Halation Effect",
    "GCU Happy Idiot Talk",
    "GCU Helpless In The Face Of Your Beauty",
    "GCU Heresiarch",
    "GCU I Thought He Was With You",
    "GCU It'll Be Over By Christmas",
    "GCU It's Character Forming",
    "GCU It's My Party And I'll Sing If I Want To",
    "GCU Jaundiced Outlook",
    "GCU Just Another Victim Of The Ambient Morality",
    "GCU Just One More Thing…",
    "GCU Just Read The Instructions",
    "GCU Just Testing",
    "GCU Lightly Seared On The Reality Grill",
    "GCU Minority Report",
    "GCU Nervous Energy",
    "GCU Never Talk To Strangers",
    "GCU Not Wanted On Voyage",
    "GCU Of Course I Still Love You",
    "GCU Only Slightly Bent",
    "GCU Perfidy",
    "GCU Problem Child",
    "GCU Prosthetic Conscience",
    "GCU Pure Big Mad Boat Man",
    "GCU Qualifier",
    "GCU Reasonable Excuse",
    "GCU Recent Convert",
    "GCU Sacrificial Victim",
    "GCU Space Monster",
    "GCU Stranger Here Myself",
    "GCU Sweet and Full of Grace",
    "GCU Synchronize Your Dogmas",
    "GCU Tactical Grace",
    "GCU Thank You And Goodnight",
    "GCU The Precise Nature Of The Catastrophe",
    "GCU Transient Atmospheric Phenomenon",
    "GCU Ultimate Ship The Second",
    "GCU Unacceptable Behaviour",
    "GCU Undesirable Alien",
    "GCU Unwitting Accomplice",
    "GCU Very Little Gravitas Indeed",
    "GCU Warm, Considering",
    "GCU Well I Was In The Neighbourhood",
    "GCU You Naughty Monsters",
    "GCU You Would If You Really Loved Me",
    "GCU You'll Thank Me Later",
    "GCV Steely Glint",
    "GCV Subtle Shift In Emphasis",
    "GOU Headcrash",
    "GOU Questionable Ethics",
    "GOU Xenocrat",
    "GOU/PS Falling Outside The Normal Moral Constraints",
    "GSV A Fine Disregard For Awkward Facts",
    "GSV Anticipation Of A New Lover's Arrival, The",
    "GSV Bad For Business",
    "GSV Bora Horza Gobuchul",
    "GSV Cargo Cult",
    "GSV Congenital Optimist",
    "GSV Contents May Differ",
    "GSV Death And Gravity",
    "GSV Determinist",
    "GSV Dressed Up To Party",
    "GSV Empiricist",
    "GSV Eschatologist",
    "GSV Ethics Gradient",
    "GSV Experiencing A Significant Gravitas Shortfall",
    "GSV Honest Mistake",
    "GSV Irregular Apocalypse",
    "GSV Just The Washing Instruction Chip In Life's Rich Tapestry",
    "GSV Kakistocrat",
    "GSV Lasting Damage",
    "GSV Lasting Damage I",
    "GSV Lasting Damage II",
    "GSV Limivorous",
    "GSV Little Rascal",
    "GSV No Fixed Abode",
    "GSV No More Mr Nice Guy",
    "GSV Pelagian",
    "GSV Quietly Confident,",
    "GSV Sanctioned Parts List",
    "GSV Seed Drill",
    "GSV Sense Amid Madness, Wit Amidst Folly",
    "GSV Size Isn't Everything",
    "GSV So Much For Subtlety",
    "GSV Teething Problems",
    "GSV The Ends Of Invention",
    "GSV Total Internal Reflection",
    "GSV Unfortunate Conflict Of Evidence",
    "GSV Uninvited Guest",
    "GSV Unreliable Witness",
    "GSV Use Psychology",
    "GSV What Are The Civilian Applications?",
    "GSV What Is The Answer And Why?",
    "GSV Wisdom Like Silence",
    "GSV Yawning Angel",
    "GSV Youthful Indiscretion",
    "GSV Zero Gravitas",
    "Hidden Income",
    "I Blame My Mother",
    "I Blame Your Mother",
    "LCU Anything Legal Considered",
    "LCU Beats Working",
    "Liveware Problem",
    "LOU Attitude Adjuster",
    "LOU Caconym",
    "LOU Gunboat Diplomat",
    "LOU New Toy",
    "LSV Misophist",
    "LSV Profit Margin",
    "LSV Serious Callers Only",
    "LSV Xenoglossicist",
    "LSV You Call This Clean?",
    "MSV Don't Try This At Home",
    "MSV Not Invented Here",
    "MSV Passing By And Thought I'd Drop In",
    "MSV Pressure Drop",
    "Now We Try It My Way",
    "OU/e Mistake Not…",
    "ROU Frank Exchange Of Views",
    "ROU Heavy Messing",
    "ROU Killing Time",
    "ROU Learned Response",
    "ROU Nuisance Value",
    "ROU Revisionist",
    "ROU Trade Surplus",
    "Scar Glamour",
    "Shoot Them Later",
    "Superlifter Charitable View",
    "Superlifter Kiss My Ass",
    "Superlifter Prime Mover",
    "Superlifter Zoologist",
    "VFP/(D)LOU Rapid Random Response Unit",
    "VFP/(D)ROU Outstanding Contribution To The Historical Process",
    "VFP/(D)ROU Resistance Is Character-Forming",
    "VFP/(D)ROU Sausage-making Is An Outcome Of Efficient Butchery",
    "VFP/(D)ROU Xenophobe",
    "VFP/(D)ROU You'll Clean That Up Before You Leave",
];

/// Return all ship names as a vector of strings. Will allocate.
///
/// # Examples
///
/// ```
/// let all_ships = gsv_culture_ships::ships();
/// assert_eq!(all_ships.len(), 172);
/// ```
#[must_use]
pub fn ships() -> Vec<String> {
    SHIPS.iter().map(|xs| xs.to_string()).collect()
}

/// Return all ship names as a slice of &str. Will not allocate.
///
/// # Examples
///
/// ```
/// let ships = gsv_culture_ships::ships_as_slice();
/// assert!(ships.contains(&"GSV Zero Gravitas"));
/// ```
#[must_use]
pub const fn ships_as_slice() -> &'static [&'static str] {
    SHIPS.as_slice()
}

/// Return a randomly-selected ship name.
///
/// # Examples
///
/// ```
/// let ship = gsv_culture_ships::random();
/// assert!(!ship.is_empty());
/// ```
#[must_use]
pub fn random() -> String {
    random_str().to_string()
}

/// Return a randomly-selected ship name, as a static &str.
///
/// ```
/// let ship = gsv_culture_ships::random_str();
/// assert!(!ship.is_empty());
/// ```
#[must_use]
pub fn random_str() -> &'static str {
    let index = fastrand::usize(..SHIPS.len());
    SHIPS[index]
}

/// Return multiple randomly-selected unique ship names.
///
/// Returns up to `count` ships. If `count` exceeds the total number of ships,
/// returns all available ships.
///
/// ```
/// let ships = gsv_culture_ships::random_n(3);
/// assert!(ships.len() <= 3);
/// ```
#[must_use]
pub fn random_n(count: usize) -> Vec<String> {
    let count = count.min(SHIPS.len());
    let mut indices = (0..SHIPS.len()).collect::<Vec<_>>();
    fastrand::shuffle(&mut indices);
    indices.into_iter().take(count).map(|i| SHIPS[i].to_string()).collect()
}

/// Returns the total number of available ship names.
///
/// ```
/// let count = gsv_culture_ships::count();
/// assert_eq!(count, 172);
/// ```
#[must_use]
pub const fn count() -> usize {
    SHIPS.len()
}

#[cfg(test)]
mod tests {

    #[test]
    fn random_ship() {
        let ship = super::random();
        assert!(!ship.is_empty());
    }

    #[test]
    fn all_ships() {
        let list = super::ships();
        assert_eq!(list.len(), super::SHIPS.len());
    }

    #[test]
    fn random_ship_borrowed() {
        let ship: &'static str = super::random_str();
        assert!(!ship.is_empty());
    }

    #[test]
    fn all_ships_borrowed() {
        let list: &'static [&str] = super::ships_as_slice();
        assert_eq!(list.len(), super::SHIPS.len());
    }

    #[test]
    fn random_multiple() {
        let ships = super::random_n(5);
        assert!(ships.len() <= 5);
        // Check uniqueness
        let unique: std::collections::HashSet<_> = ships.iter().collect();
        assert_eq!(unique.len(), ships.len());
    }

    #[test]
    fn random_n_exceeds_total() {
        let ships = super::random_n(1000);
        assert_eq!(ships.len(), super::SHIPS.len());
    }

    #[test]
    fn count_ships() {
        assert_eq!(super::count(), 172);
        assert_eq!(super::count(), super::SHIPS.len());
    }
}
