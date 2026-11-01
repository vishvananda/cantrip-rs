// Library of Leng
// status: planned
// requires: replacement destination OwnersLibraryTop
// requires: replacement event Discard
// You have no maximum hand size.
// If an effect causes you to discard a card, discard it, but you may put it on top of your library instead of into your graveyard.
#[allow(unused_imports)] use cantrip_api::*;
static INSTEAD_0: Question = Question::new("Instead", "", "If you don't, the event goes on as it was.", "Do this instead", "Don't");
#[allow(unused_variables)] fn ability_1_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::ZoneChange(z) = proposal { let id = z.object; let owner = q.state().obj(id).owner; let subject = Target::Object(id); let data = [u32::from(owner.0), u32::MAX]; if z.reason == ZoneReason::Discard && Some(asker.controller) == Some(owner) && planned_value!("replacement event Discard", true => q.proposal_cause().is_some()) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_1_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::ZoneChange(seen) = &seen { let z = seen; let id = z.object; let owner = q.state().obj(id).owner; let subject = Target::Object(id); let data = [u32::from(owner.0), u32::MAX]; if let Proposal::ZoneChange(z) = proposal { z.to = Zone::library(q.state().obj(z.object).owner); planned_value!("replacement destination OwnersLibraryTop", () => z.library_end = Some(LibraryEnd::Top)); } } }

pub static CARD: CardDef = CardDef { name: "Library of Leng", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[replacement(ReplacementKind::ZoneChange, ability_1_applies, ability_1_apply, "If an effect causes you to discard a card, discard it, but you may put it on top of your library instead of into your graveyard.").optional(&INSTEAD_0)], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, play_rules: &[PlayRule::MaximumHandSize(None)], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
