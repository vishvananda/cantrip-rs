// Repeated Reverberation
// status: ready
// When you next cast an instant spell, cast a sorcery spell, or activate a loyalty ability this turn, copy that spell or ability twice. You may choose new targets for the copies.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn delayed_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && ((q.state().stack.iter().find(|i| i.object == id).is_some_and(|i| i.kind == StackKind::Spell) && q.chars(id).is(CardType::Instant)) || (q.state().stack.iter().find(|i| i.object == id).is_some_and(|i| i.kind == StackKind::Spell) && q.chars(id).is(CardType::Sorcery))) { return Some(Fired { subject, data }); } }
if let Event::AbilityActivated { source, ability, player, .. } = event { let id = *source; let def = q.activated_ability(*ability); let subject = Target::Player(*player); let serial = match cantrip_mechanics::triggers::activated_ability_on_stack(q, event) { Some(Target::Ability(serial)) => serial, _ => 0 }; let data = [id.0, serial]; if def.is_some() && Some(asker.controller) == Some(*player) && def.is_some_and(|d| d.is_loyalty_ability()) { return Some(Fired { subject, data }); } }
 None }
fn delayed_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let runs_2: u32 = { let q: &dyn Query = g; (2i64).max(0) as u32 };
let copier_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; for _ in 0..runs_2 { match item.subject { Target::Object(spell) => { g.copy_spell(spell, copier_2, true)?; } _ => { let t = Target::Ability(item.data[1]); g.copy_ability(t, copier_2, true)?; } } }
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if let Some(source) = Some(item.object) { g.delayed_trigger(DelayedWhen::UntilEvent, source, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Repeated Reverberation", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0 | EventMask::ABILITY_ACTIVATED.0), delayed_1_event), delayed_1, "Whenever a spell is cast, ").targets(&[])], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
