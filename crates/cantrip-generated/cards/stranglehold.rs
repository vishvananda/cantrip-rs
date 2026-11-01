// Stranglehold
// status: planned
// requires: prohibited action SearchLibraries
// requires: replacement of BeginExtraTurn
// Your opponents can't search libraries.
// If an opponent would begin an extra turn, that player skips that turn instead.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player != asker.controller }
planned_item!("prohibited-action-search-libraries" => statics: player_rule(ability_0_0_applies, PlayerRule::Cant(Prohibition::SearchLibraries)));
planned_item!("replacement-of-begin-extra-turn" => callbacks: #[allow(unused_variables)] fn ability_1_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::ExtraTurn(p) = proposal { let subject = Target::Player(p.player); let data = [u32::MAX; 2]; if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p.player) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_1_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::ExtraTurn(seen) = &seen { let p = seen; let subject = Target::Player(p.player); let data = [u32::MAX; 2]; if let Proposal::ExtraTurn(p) = proposal { p.replaced = true; } } }
);
planned_item!("replacement-of-begin-extra-turn" => replacements: replacement(ReplacementKind::ExtraTurn, ability_1_applies, ability_1_apply, "If an opponent would begin an extra turn, that player skips that turn instead."));

pub static CARD: CardDef = CardDef { name: "Stranglehold", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
