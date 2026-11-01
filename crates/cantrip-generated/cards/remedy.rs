// Remedy
// status: planned
// requires: prevention limited by DamageToEach
// requires: target reference outside a resolving ability
// Prevent the next 5 damage that would be dealt this turn to any number of targets, divided as you choose.
#[allow(unused_imports)] use cantrip_api::*;
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[cantrip_mechanics::targets::ANY.between(1, 5)];
#[allow(unused_variables)] fn made_1_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if true && true && (planned_value!("target reference outside a resolving ability", Vec::<Target>::new() => q.effect_about(asker, 0))).contains(target) { return true; } }
 false }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
planned!("target-reference-outside-a-resolving-ability" => { let shield = { let q: &dyn Query = g; (i64::from(q.state().share(item, 0))).clamp(0, 65535) as u16 }; let to = Some(Target::None); let about = to.and_then(|to: Target| to.object()).or(Some(item.object)); let mut carried: Vec<Target> = Vec::new(); carried.extend({ let q: &dyn Query = g; t_0.clone() }); carried.push(Target::None);  if let (Some(to), Some(about)) = (to, about) { g.add_effect_about(&carried, item.object, about, EffectKind::Prevention { card: item.ability.card, index: 0, to, shield: planned_value!("prevention limited by DamageToEach", Shield::All => Shield::NextToEach(shield)) }, Duration::EndOfTurn); } });
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Remedy", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_preventions: &[GrantedPreventionDef { hint: "", damage: "damage", of_it: None, applies: made_1_applies }], divide: Some(Divide { total: 5, of_x: None }), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
