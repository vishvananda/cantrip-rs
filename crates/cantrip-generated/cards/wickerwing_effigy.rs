// Wickerwing Effigy
// status: planned
// requires: engine: no effect adds colors
// requires: operation ChangeSpell
// Defender
// You may look at the top card of your library any time.
// You may cast creature spells from the top of your library.
// Whenever you cast a creature spell from your library, it becomes a black Bird in addition to its other colors and types, has flying, and has base power and toughness 1/1.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_0_rule(q: &dyn Query, asker: &Asker, spell: ObjectId) -> bool { q.chars(spell).is(CardType::Creature) }
#[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && (q.chars(id).is(CardType::Creature) && cast_at(q, event, id).is_some_and(|cast| cast.from == ZoneKind::Library && q.state().obj(cast.spell).owner == cast.player)) { return Some(Fired { subject, data }); } }
 None }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("operation-change-spell" => if let Some(spell) = { let q: &dyn Query = g; item.subject.object() } { let changes = { let q: &dyn Query = g; [SpellChange::Effect(planned_value!("engine: no effect adds colors", EffectKind::Pump(0, 0) => EffectKind::AddColors(ColorSet(4)))), SpellChange::Effect(EffectKind::AddSubtype(q.words(asker.source).subtype(Subtype::Bird))), SpellChange::Effect(EffectKind::AddKeywords(Keywords::of(&[Keyword::Flying]))), SpellChange::Effect(EffectKind::SetPt((1i64) as i16, (1i64) as i16))] }; cantrip_mechanics::spell_changes::change(g, asker, spell, &changes); });
g.end_sentence();
 Ok(()) }
fn cast_of<'a>(q: &'a dyn Query, id: ObjectId) -> Option<&'a Cast> { let records = q.history().all(); let spell = records.iter().rev().find_map(|r| match r.event { Event::ZoneChange { old, new, reason: ZoneReason::Resolve, .. } if new == id => Some(old), _ => None }).unwrap_or(id); records.iter().rev().find_map(|r| match &r.event { Event::Cast(cast) if cast.spell == spell => Some(cast), _ => None }) }
#[allow(dead_code)] fn cast_at<'a>(q: &'a dyn Query, event: &Event, id: ObjectId) -> Option<&'a Cast> { match event { Event::ZoneChange { old, new, reason: ZoneReason::Resolve, .. } if *new == id => cast_of(q, *old), _ => cast_of(q, id) } }

pub static CARD: CardDef = CardDef { name: "Wickerwing Effigy", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Scarecrow]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Defender]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_3_event), ability_3, "Whenever you cast a creature spell from your library, it becomes a black Bird in addition to its other colors and types, has flying, and has base power and toughness 1/1.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, play_rules: &[PlayRule::LookAtLibraryTop,PlayRule::LibraryTopSpells(ability_2_0_rule, None)], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
