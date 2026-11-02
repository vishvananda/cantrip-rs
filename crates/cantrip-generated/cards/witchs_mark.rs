// Witch's Mark
// status: ready
// You may discard a card. If you do, draw two cards.
// Create a Wicked Role token attached to up to one target creature you control. (If you control another Role on it, put that one into the graveyard. Enchanted creature gets +1/+1. When this token is put into a graveyard, each opponent loses 1 life.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0).between(0, 1)];
pub mod token_5 {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_enchant(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = ((1i64) as i16, (1i64) as i16); chars.power = chars.power.map(|v| v.saturating_add(p)); chars.toughness = chars.toughness.map(|v| v.saturating_add(t)); }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *old; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Graveyard) && from.is(ZoneKind::Battlefield) && (Some(asker.source) == Some(id) && true) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as i32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
for player in players_0 { g.lose_life(player, amount_0); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Wicked", token_words: &[], oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Aura,Subtype::Role]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_2_event), ability_2, "Whenever this goes there from the battlefield, Lose 1 life.").targets(&[])], statics: &[static_ability(Layer::PtModify, cantrip_mechanics::attach::attached_object, ability_1_apply_0, "Static: Pump.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: Some(target(ability_0_enchant)), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let payer_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let (paid_0, after_0, before_0, picked_0): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if true {
let amount_2 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_2: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().collect(); let mut cards_2 = Vec::new(); for player in players_2 { cards_2.extend({ let hand = g.state().zone(Zone::hand(player)).to_vec(); cantrip_mechanics::choose::cards(g, player, asker.source, Purpose::Discard, hand, amount_2, true)? }); } let discarded_2 = g.discard_together(&cards_2);
let paid_1 = !discarded_2.clone().is_empty();
(paid_1, { let q: &dyn Query = g; (discarded_2).clone() }, { let q: &dyn Query = g; (discarded_2.clone()).clone() }, Vec::new()) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
let amount_3 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let players_3 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_3 { g.draw(player, amount_3); }
g.end_sentence();
}
g.end_sentence();
let players_4 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_4 = Vec::new();
let count_4_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_4_0 = g.card_id(&token_5::CARD);
for player in players_4 {
tokens_4.extend(std::iter::repeat((def_4_0,player)).take(count_4_0 as usize));
} let words_4 = Words::NONE;
let host_4 = { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.object()) }; let mut created_4: Vec<ObjectId> = Vec::new(); if let Some(host) = host_4 { for &(def, player) in &tokens_4 { created_4.extend(g.create_token_attached(def, player, host)?); } }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Witch's Mark", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_5::CARD];
