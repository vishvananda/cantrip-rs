// Breaking Wave
// status: ready
// You may cast this spell as though it had flash if you pay {2} more to cast it. (You may cast it any time you could cast an instant.)
// Simultaneously untap all tapped creatures and tap all untapped creatures.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_of_flash(q: &dyn Query, asker: &Asker, targets: &[Target]) -> Cost { if q.state().obj(asker.source).marks.untimely_by_own_flash { Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })) } else { Cost::NONE } }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let turned_0_0: Vec<ObjectId> = ({ let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (!(q.state().obj(id).tapped) && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).into_iter().filter(|&o| g.state().obj(o).tapped != true).collect();
let turned_0_1: Vec<ObjectId> = ({ let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.state().obj(id).tapped && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).into_iter().filter(|&o| g.state().obj(o).tapped != false).collect();
cantrip_api::instruction(g, |g| { g.tap_together(&turned_0_0); for &object in &turned_0_1 { g.untap(object); } });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Breaking Wave", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_1), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, cast_as_though_flash: true, cost_of_targets: Some(ability_0_of_flash), untimely_cleanup_rider: true, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
