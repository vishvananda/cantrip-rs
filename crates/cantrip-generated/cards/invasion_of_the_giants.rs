// Invasion of the Giants
// status: planned
// requires: operation ChangeSpell
// (As this Saga enters and after your draw step, add a lore counter. Sacrifice after III.)
// I — Scry 2.
// II — Draw a card. Then you may reveal a Giant card from your hand. When you do, this Saga deals 2 damage to target opponent or planeswalker.
// III — The next Giant spell you cast this turn costs {2} less to cast.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (2i64).max(0) as usize }; let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::scry(g, player_0, asker.source, amount_0)?;
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn reflexive_5_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { match t { Target::Player(p) => (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p), Target::Object(id) => q.is_on_battlefield(id) && { let c = q.chars(id); c.is(CardType::Creature) || c.is(CardType::Planeswalker) || c.is(CardType::Battle) } && (q.chars(id).is(CardType::Planeswalker)), _ => false } }
static REFLEXIVE_5_TARGETS: &[cantrip_api::TargetSpec] = &[target(reflexive_5_target_0)];
fn reflexive_5(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let source_6 = { let q: &dyn Query = g; Some(asker.source) }; let amount_6 = { let q: &dyn Query = g; (2i64).max(0) as u16 }; let recipients_6 = { let q: &dyn Query = g; t_0.clone() }; if let Some(source) = source_6 { let batch: Vec<(ObjectId, Target, u16)> = recipients_6.iter().map(|t| (source, *t, amount_6)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
fn ability_0_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let amount_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.draw(player, amount_1); }
g.end_sentence();
{
let selected_2_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_2_candidates = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Giant))).collect::<Vec<ObjectId>>() }; let selected_2_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_2 = { cantrip_mechanics::choose::select(g, selected_2_player, asker.source, Purpose::Reveal, selected_2_candidates, cantrip_mechanics::select::Selection::of(selected_2_count, &[]))? };
if !selected_2.is_empty() { 
let objects_3 = { let q: &dyn Query = g; selected_2.clone() }; let mut owners_3: Vec<PlayerId> = objects_3.iter().map(|o| g.state().obj(*o).owner).collect(); owners_3.dedup(); for owner in owners_3 { let cards: Vec<ObjectId> = objects_3.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
if !objects_3.clone().is_empty() && true { g.reflexive_trigger(item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
hoisted_7.extend(selected_2.clone());
hoisted_8.extend(objects_3.clone());
}
}
g.end_sentence();
 Ok(()) }
fn ability_0_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("operation-change-spell" => cantrip_mechanics::spell_changes::next_spell(g, asker, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, |q: &dyn Query, asker: &Asker, id: ObjectId| q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Giant)), NextSpell { this_turn: true, from_hand: false }, &[SpellChange::Cost(CostDelta::less((2i64) as u8))]));
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Invasion of the Giants", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 1, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Saga]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_0_0, "I — Scry 2.").targets(&[]),chapter(2, ability_0_1, "II — Draw a card. Then you may reveal a Giant card from your hand. When you do, this Saga deals 2 damage to target opponent or planeswalker.").targets(&[]),chapter(3, ability_0_2, "III — The next Giant spell you cast this turn costs {2} less to cast.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(reflexive_5, "").targets(REFLEXIVE_5_TARGETS)], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
