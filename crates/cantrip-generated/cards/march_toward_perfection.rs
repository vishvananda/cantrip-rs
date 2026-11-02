// March Toward Perfection
// status: planned
// requires: operation ChangeSpell
// requires: operation DraftFromSpellbook
// You get a one-time boon with "When you cast a Phyrexian creature spell, that creature enters with an additional +1/+1 counter and deathtouch counter on it."
// Draft a card from March Toward Perfection's spellbook.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn watch_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Phyrexian)) && q.chars(id).is(CardType::Creature)) { return Some(Fired { subject, data }); } }
 None }
fn watch_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("operation-change-spell" => if let Some(spell) = { let q: &dyn Query = g; item.subject.object() } { let changes = { let q: &dyn Query = g; [SpellChange::EntersWithCounters(CounterKind::PlusOne, (1i64).max(0) as i16), SpellChange::EntersWithCounters(CounterKind::Deathtouch, (1i64).max(0) as i16)] }; cantrip_mechanics::spell_changes::change(g, asker, spell, &changes); });
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if let Some(source) = Some(item.object) { g.delayed_trigger(DelayedWhen::UntilEvent, source, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
let times_3: u32 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut drafted_3: Vec<ObjectId> = Vec::new(); for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { drafted_3.extend(planned_value!("operation DraftFromSpellbook", Vec::<ObjectId>::new() => cantrip_mechanics::conjure::draft_from_spellbook(g, player, asker.source, "March Toward Perfection", times_3)?)); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "March Toward Perfection", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), watch_1_event), watch_1, "You get a one-time boon with \"When you cast a Phyrexian creature spell, that creature enters with an additional +1/+1 counter and deathtouch counter on it.\"").targets(&[])], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
