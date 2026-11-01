// Aven Mindcensor
// status: planned
// requires: replacement of SearchLibrary
// Flash
// Flying
// If an opponent would search a library, that player searches the top four cards of that library instead.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("replacement-of-search-library" => callbacks: #[allow(unused_variables)] fn ability_2_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::SearchLibrary(p) = proposal { let subject = Target::Player(p.player); let data = [u32::MAX; 2]; if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p.player) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_2_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::SearchLibrary(seen) = &seen { let p = seen; let subject = Target::Player(p.player); let data = [u32::MAX; 2]; if let Proposal::SearchLibrary(p) = proposal { p.replaced = true;p.top = Some((4i64).max(0) as u32); } } }
);
planned_item!("replacement-of-search-library" => replacements: replacement(ReplacementKind::SearchLibrary, ability_2_applies, ability_2_apply, "If an opponent would search a library, that player searches the top four cards of that library instead."));

pub static CARD: CardDef = CardDef { name: "Aven Mindcensor", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Bird,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flash,Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
