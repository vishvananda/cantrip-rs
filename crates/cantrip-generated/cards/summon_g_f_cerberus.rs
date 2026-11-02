// Summon: G.F. Cerberus
// status: ready
// (As this Saga enters and after your draw step, add a lore counter. Sacrifice after III.)
// I — Surveil 1. (Look at the top card of your library. You may put it into your graveyard.)
// II — Double — When you next cast an instant or sorcery spell this turn, copy it. You may choose new targets for the copy.
// III — Triple — When you next cast an instant or sorcery spell this turn, copy it twice. You may choose new targets for the copies.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::surveil(g, player_0, asker.source, amount_0)?;
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn watch_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && (q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)) { return Some(Fired { subject, data }); } }
 None }
fn watch_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let copier_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let spell_3: Option<ObjectId> = ({ let q: &dyn Query = g; (item.subject.object().into_iter().collect::<Vec<ObjectId>>()) }).first().copied();
if let Some(spell) = spell_3 { g.copy_spell(spell, copier_3, true)?; }
g.end_sentence();
 Ok(()) }
fn ability_0_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if let Some(source) = Some(item.object) { g.delayed_trigger(DelayedWhen::NextThisTurn, source, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn watch_5_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && (q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)) { return Some(Fired { subject, data }); } }
 None }
fn watch_5(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let runs_6: u32 = { let q: &dyn Query = g; (2i64).max(0) as u32 };
let copier_6 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let spell_6: Option<ObjectId> = ({ let q: &dyn Query = g; (item.subject.object().into_iter().collect::<Vec<ObjectId>>()) }).first().copied();
if let Some(spell) = spell_6 { for _ in 0..runs_6 { g.copy_spell(spell, copier_6, true)?; } }
g.end_sentence();
 Ok(()) }
fn ability_0_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if let Some(source) = Some(item.object) { g.delayed_trigger(DelayedWhen::NextThisTurn, source, item.controller, AbilityRef::delayed(item.ability.card, 1), Target::None); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Summon: G.F. Cerberus", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Saga,Subtype::Dog]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_0_0, "I — Surveil 1.").targets(&[]),chapter(2, ability_0_1, "II — Double — When you next cast an instant or sorcery spell this turn, copy it. You may choose new targets for the copy.").targets(&[]),chapter(3, ability_0_2, "III — Triple — When you next cast an instant or sorcery spell this turn, copy it twice. You may choose new targets for the copies.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), watch_2_event), watch_2, "Double — When you next cast an instant or sorcery spell this turn, copy it. You may choose new targets for the copy.").targets(&[]),triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), watch_5_event), watch_5, "Triple — When you next cast an instant or sorcery spell this turn, copy it twice. You may choose new targets for the copies.").targets(&[])], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
