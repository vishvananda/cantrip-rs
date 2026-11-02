// Pale Moon
// status: ready
// Until end of turn, if a player taps a nonbasic land for mana, it produces colorless mana instead of any other type.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn made_1_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, proposal: &Proposal) -> bool { if let Proposal::ManaProduction(m) = proposal { let id = m.permanent; let amount = m.total(); let subject = Target::Object(id); let data = [amount, u32::MAX]; if (!(q.chars(id).supertypes.basic) && q.chars(id).is(CardType::Land)) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn made_1_apply(q: &dyn Query, asker: &Asker, affected: ObjectId, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::ManaProduction(seen) = &seen { let m = seen; let id = m.permanent; let amount = m.total(); let subject = Target::Object(id); let data = [amount, u32::MAX]; if let Proposal::ManaProduction(m) = proposal { let total = m.total(); m.mana = ManaPool::default(); m.mana.0[Mana::C as usize] = total; } } }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
{  let to = Some(Target::None); let about = to.and_then(|to: Target| to.object()).or(Some(item.object)); if let (Some(to), Some(about)) = (to, about) { let _ = (to, about); g.add_player_effect(item.controller, item.object, EffectKind::ReplacementRule { card: item.ability.card, index: 0 }, Duration::EndOfTurn); } }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Pale Moon", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_replacements: &[granted_replacement(ReplacementKind::ManaProduction, made_1_applies, made_1_apply, "Until end of turn, if a player taps a nonbasic land for mana, it produces colorless mana instead of any other type.").text("Until end of turn, if a player taps a nonbasic land for mana, it produces colorless mana instead of any other type.")], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
