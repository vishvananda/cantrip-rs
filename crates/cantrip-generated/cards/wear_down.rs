// Wear Down
// status: planned
// requires: a gift that changes how many a spell targets
// Gift a card (You may promise an opponent a gift as you cast this spell. If you do, they draw a card before its other effects.)
// Destroy target artifact or enchantment. If the gift was promised, instead destroy two target artifacts and/or enchantments.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_promised_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Enchantment)))) }
static ABILITY_0_PROMISED_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_promised_target_0).between(2, 2)];
#[allow(unused_variables)] fn ability_0_unpromised_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Enchantment)))) }
static ABILITY_0_UNPROMISED_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_unpromised_target_0)];
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { if cantrip_mechanics::gift::promised(q, asker.source) { (ability_0_promised_target_0)(q, asker, t) } else { (ability_0_unpromised_target_0)(q, asker, t) } }
fn ability_0_paid(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>() }; let destroyed_0 = g.destroy_all(&objects_0);
g.end_sentence();
 Ok(()) }
fn ability_0_unpaid(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_1 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let destroyed_1 = g.destroy_all(&objects_1);
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { if item.gift.is_some() { ability_0_paid(g, item) } else { ability_0_unpaid(g, item) } }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("a-gift-that-changes-how-many-a-spell-targets" => spell: spell(&[TargetSpec { min: 1, max: 1, filter: ability_0_target_0, label: "", if_kicked: None, together: None }.if_promised(2, 2)], ability_0));

pub static CARD: CardDef = CardDef { name: "Wear Down", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, gift: Some(cantrip_mechanics::gift::CARD), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
