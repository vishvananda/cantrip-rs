// Brazen Boarding
// status: planned
// requires: numeric expression ExcessDamage
// requires: operation Conjure
// Brazen Boarding deals 4 damage to target creature or planeswalker. If excess damage was dealt this way, conjure a card with mana value equal to that excess damage from Brazen Boarding's spellbook onto the battlefield. If that excess damage is 4 or greater, instead conjure a card named Admiral Beckett Brass onto the battlefield.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Planeswalker)))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let mut done_0: i64 = 0; let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let amount_0 = { let q: &dyn Query = g; (4i64).max(0) as u16 }; let recipients_0 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() }; if let Some(source) = source_0 { let batch: Vec<(ObjectId, Target, u16)> = recipients_0.iter().map(|t| (source, *t, amount_0)).collect(); done_0 += g.damage_together(&batch).iter().map(|&n| i64::from(n)).sum::<i64>(); }
g.end_sentence();
if { let q: &dyn Query = g; (planned_value!("numeric expression ExcessDamage", 0i64 => i64::from(result.excess))) > (0i64) } {
if { let q: &dyn Query = g; (planned_value!("numeric expression ExcessDamage", 0i64 => i64::from(result.excess))) >= (4i64) } {
let count_1: u32 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut conjured_1: Vec<ObjectId> = Vec::new(); for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { conjured_1.extend(planned_value!("operation Conjure", Vec::<ObjectId>::new() => cantrip_mechanics::conjure::conjure(g, player, player, asker.source, Conjured::Named("Admiral Beckett Brass"), count_1, ConjureTo::Battlefield { tapped: false })?)); }
g.end_sentence();
hoisted_2.extend(conjured_1.clone());
hoisted_3.extend(conjured_1.clone());
} else {
let count_4: u32 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut conjured_4: Vec<ObjectId> = Vec::new(); for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { conjured_4.extend(planned_value!("operation Conjure", Vec::<ObjectId>::new() => cantrip_mechanics::conjure::conjure(g, player, player, asker.source, Conjured::FromSpellbookMatching("Brazen Boarding", CardsInOracle::ManaValueIs("Comparison(Equal, ExcessDamage { result: ResultId(0), to: Any })")), count_4, ConjureTo::Battlefield { tapped: false })?)); }
g.end_sentence();
hoisted_5.extend(conjured_4.clone());
hoisted_6.extend(conjured_4.clone());
}
g.end_sentence();
hoisted_7.extend(hoisted_2.clone());
hoisted_8.extend(hoisted_5.clone());
hoisted_9.extend(hoisted_3.clone());
hoisted_10.extend(hoisted_6.clone());
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Brazen Boarding", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
