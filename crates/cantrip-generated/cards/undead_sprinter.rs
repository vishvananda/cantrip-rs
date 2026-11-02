// Undead Sprinter
// status: planned
// requires: cast the card itself on these terms
// Trample, haste
// You may cast this card from your graveyard if a non-Zombie creature died this turn. If you do, this creature enters with a +1/+1 counter on it.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_2_if(q: &dyn Query, asker: &Asker) -> bool { (q.history().this_turn().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *old; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Graveyard) && from.is(ZoneKind::Battlefield) && (!(q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Zombie))) && q.chars(id).is(CardType::Creature)) { return Some(Fired { subject, data }); } }  None }).count() as i64) >= (1i64) }
planned_item!("cast-the-card-itself-on-these-terms" => play_terms: &[PlayTerm::Rider(&[&[EnterFlag::EntersWithACounter(CounterType::PTCounter(1, 1))]])]);
planned_item!("cast-the-card-itself-on-these-terms" => own_casts: OwnCast { zone: ZoneKind::Graveyard, condition: Some(ability_2_if), grant: PlayGrant { terms: Some(0), ..PlayGrant::CAST } });

pub static CARD: CardDef = CardDef { name: "Undead Sprinter", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Zombie]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample,Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
