// Melek, Izzet Paragon
// status: planned
// requires: a library's top card open to view
// Play with the top card of your library revealed.
// You may cast instant and sorcery spells from the top of your library.
// Whenever you cast an instant or sorcery spell from your library, copy it. You may choose new targets for the copy.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("a-library-s-top-card-open-to-view" => #[allow(unused_variables)] fn library_top_open_0(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller });
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, spell: ObjectId) -> bool { (q.chars(spell).is(CardType::Instant) || q.chars(spell).is(CardType::Sorcery)) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && ((q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)) && cast_at(q, event, id).is_some_and(|cast| cast.from == ZoneKind::Library && q.state().obj(cast.spell).owner == cast.player)) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let copier_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let spell_0: Option<ObjectId> = ({ let q: &dyn Query = g; (item.subject.object().into_iter().collect::<Vec<ObjectId>>()) }).first().copied();
if let Some(spell) = spell_0 { g.copy_spell(spell, copier_0, true)?; }
g.end_sentence();
 Ok(()) }
fn cast_of<'a>(q: &'a dyn Query, id: ObjectId) -> Option<&'a Cast> { let records = q.history().all(); let spell = records.iter().rev().find_map(|r| match r.event { Event::ZoneChange { old, new, reason: ZoneReason::Resolve, .. } if new == id => Some(old), _ => None }).unwrap_or(id); records.iter().rev().find_map(|r| match &r.event { Event::Cast(cast) if cast.spell == spell => Some(cast), _ => None }) }
#[allow(dead_code)] fn cast_at<'a>(q: &'a dyn Query, event: &Event, id: ObjectId) -> Option<&'a Cast> { match event { Event::ZoneChange { old, new, reason: ZoneReason::Resolve, .. } if *new == id => cast_of(q, *old), _ => cast_of(q, id) } }
planned_item!("a-library-s-top-card-open-to-view" => play_rules: PlayRule::LibraryTopRevealed(library_top_open_0));

pub static CARD: CardDef = CardDef { name: "Melek, Izzet Paragon", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 1, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Weird,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_2_event), ability_2, "Whenever you cast an instant or sorcery spell from your library, copy it. You may choose new targets for the copy.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, play_rules: &[PlayRule::LibraryTopSpells(ability_1_0_rule, None)], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
