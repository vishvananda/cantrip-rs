// Glacial Dragonhunt
// status: ready
// Draw a card, then you may discard a card. When you discard a nonland card this way, Glacial Dragonhunt deals 3 damage to target creature.
// Harmonize {4}{U}{R} (You may cast this card from your graveyard for its harmonize cost. You may tap a creature you control to reduce that cost by {X}, where X is its power. Then exile this spell.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn reflexive_3_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static REFLEXIVE_3_TARGETS: &[cantrip_api::TargetSpec] = &[target(reflexive_3_target_0)];
fn reflexive_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let source_4 = { let q: &dyn Query = g; Some(asker.source) }; let amount_4 = { let q: &dyn Query = g; (3i64).max(0) as u16 }; let recipients_4 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() }; if let Some(source) = source_4 { let batch: Vec<(ObjectId, Target, u16)> = recipients_4.iter().map(|t| (source, *t, amount_4)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.draw(player, amount_0); }
g.end_sentence();
{
let amount_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_1: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().collect(); let mut cards_1 = Vec::new(); for player in players_1 { cards_1.extend({ let hand = g.state().zone(Zone::hand(player)).to_vec(); cantrip_mechanics::choose::cards(g, player, asker.source, Purpose::Discard, hand, amount_1, true)? }); } let discarded_1 = g.discard_together(&cards_1);
if !cards_1.is_empty() { 
if !discarded_1.clone().is_empty() && { let q: &dyn Query = g; (((discarded_1.clone()).into_iter().filter(|&id| !(q.chars(id).is(CardType::Land))).collect::<Vec<ObjectId>>()).len() as i64) > (0i64) } { g.reflexive_trigger(item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
hoisted_5.extend(discarded_1.clone());
hoisted_6.extend({ let q: &dyn Query = g; discarded_1.clone() });
}
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Glacial Dragonhunt", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 1, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(reflexive_3, "When you discard a nonland card this way, Glacial Dragonhunt deals 3 damage to target creature.").targets(REFLEXIVE_3_TARGETS)], enters: None, modal: None, enchant: None, ways: &[cantrip_mechanics::harmonize::way(ManaCost { generic: 4, x: 0, symbols: [0, 1, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, "Harmonize {4}{U}{R}")], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
