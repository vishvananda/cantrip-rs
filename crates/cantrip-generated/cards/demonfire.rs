// Demonfire
// status: planned
// requires: operation DealDamage
// Demonfire deals X damage to any target. If a creature dealt damage this way would die this turn, exile it instead.
// Hellbent — If you have no cards in hand, this spell can't be countered and the damage can't be prevented.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_apply(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); if ((((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).len() as i64) == (0i64) { chars.keywords = chars.keywords.with(Keyword::CantBeCountered); } }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[cantrip_mechanics::targets::ANY];
#[allow(unused_variables)] fn made_6_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, proposal: &Proposal) -> bool { if let Proposal::ZoneChange(z) = proposal { let id = z.object; let subject = Target::Object(id); let data = [u32::MAX; 2]; if z.to.is(ZoneKind::Graveyard) && z.from.is(ZoneKind::Battlefield) && (q.chars(id).is(CardType::Creature) && cantrip_mechanics::history::permanents_dealt_damage_by(q.history(), asker.source, 0).contains(&id)) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn made_6_apply(q: &dyn Query, asker: &Asker, affected: ObjectId, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::ZoneChange(seen) = &seen { let z = seen; let id = z.object; let subject = Target::Object(id); let data = [u32::MAX; 2]; if let Proposal::ZoneChange(z) = proposal { z.to = Zone::exile(q.state().obj(z.object).owner); } } }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
if { let q: &dyn Query = g; ((((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).len() as i64) == (0i64) } {
planned!("operation-deal-damage" => if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { cantrip_mechanics::damage::unpreventable(g, source, &{ let q: &dyn Query = g; t_0.clone() }, { let q: &dyn Query = g; (i64::from(item.x)).max(0) as u16 }); });
g.end_sentence();
} else {
let mut done_1: i64 = 0; let mut dealt_1: Vec<ObjectId> = Vec::new(); let source_1 = { let q: &dyn Query = g; Some(asker.source) }; let amount_1 = { let q: &dyn Query = g; (i64::from(item.x)).max(0) as u16 }; let recipients_1 = { let q: &dyn Query = g; t_0.clone() }; if let Some(source) = source_1 { let batch: Vec<(ObjectId, Target, u16)> = recipients_1.iter().map(|t| (source, *t, amount_1)).collect(); let dealt = g.damage_together(&batch); done_1 += dealt.iter().map(|&n| i64::from(n)).sum::<i64>(); dealt_1.extend(batch.iter().zip(&dealt).filter(|(_, n)| **n > 0).filter_map(|(d, _)| d.1.object())); }
g.end_sentence();
hoisted_2.extend(dealt_1.clone());
}
let mut joined_3: Vec<ObjectId> = Vec::new();
joined_3.extend(planned_value!("depends on an instruction with a gap", Vec::<ObjectId>::new() => result_0));
joined_3.extend(hoisted_2.clone());
g.end_sentence();
{  let to = Some(Target::None); let about = to.and_then(|to: Target| to.object()).or(Some(item.object)); if let (Some(to), Some(about)) = (to, about) { let _ = (to, about); g.add_player_effect(item.controller, item.object, EffectKind::ReplacementRule { card: item.ability.card, index: 0 }, Duration::EndOfTurn); } }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Demonfire", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 1, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_1_TARGETS, ability_1), abilities: &[], statics: &[characteristic_defining(Layer::Ability, ability_0_apply, "This spell can't be countered.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_replacements: &[granted_replacement(ReplacementKind::ZoneChange, made_6_applies, made_6_apply, "Hellbent — If you have no cards in hand, this spell can't be countered and the damage can't be prevented.").text("Hellbent — If you have no cards in hand, this spell can't be countered and the damage can't be prevented.")], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
