// Summitfest Closing Ceremony
// status: planned
// requires: engine: no StartingIntensity
// requires: intensity
// requires: operation Intensify
// Starting intensity 3
// Covercast — Whenever you cast another instant or sorcery spell, if five or more mana was spent to cast it, this card intensifies.
// Add X {U} and X {R}, where X is this spell's intensity.
// Draw a card.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && ((q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)) && !(Some(asker.source) == Some(id))) && subject.object().is_some_and(|candidate: ObjectId| (Some(candidate).map_or(0, |o: ObjectId| { let spent = q.obj(o).spent; [Mana::W, Mana::U, Mana::B, Mana::R, Mana::G, Mana::C].into_iter().map(|mana| i64::from(spent.get(mana))).sum::<i64>() })) >= (5i64)) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller); if !{ let q: &dyn Query = g; item.subject.object().is_some_and(|candidate: ObjectId| (Some(candidate).map_or(0, |o: ObjectId| { let spent = q.obj(o).spent; [Mana::W, Mana::U, Mana::B, Mana::R, Mana::G, Mana::C].into_iter().map(|mana| i64::from(spent.get(mana))).sum::<i64>() })) >= (5i64)) } { return Ok(()); } let asker = &Asker::new(item.object, item.controller);
planned!("operation-intensify" => cantrip_mechanics::intensify::intensify(g, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, |q: &dyn Query, asker: &Asker, id: ObjectId| Some(asker.source) == Some(id)));
g.end_sentence();
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mana_players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_2_0 = { let q: &dyn Query = g; (Some(asker.source).map_or(0i64, |o: ObjectId| planned_value!("intensity", 0i64 => i64::from(q.state().obj(o).intensity)))) as u32 };
let mana_2_1 = { let q: &dyn Query = g; (Some(asker.source).map_or(0i64, |o: ObjectId| planned_value!("intensity", 0i64 => i64::from(q.state().obj(o).intensity)))) as u32 };
for player in mana_players_2 {
g.add_mana(player, Mana::U, mana_2_0);g.add_mana(player, Mana::R, mana_2_1);}
g.end_sentence();
let amount_3 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_3 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_3 { g.draw(player, amount_3); }
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-starting-intensity" => enters: cantrip_mechanics::intensity::starting::<3>);

pub static CARD: CardDef = CardDef { name: "Summitfest Closing Ceremony", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 1, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_2), abilities: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_1_event), ability_1, "Covercast — Whenever you cast another instant or sorcery spell, if five or more mana was spent to cast it, this card intensifies.").targets(&[]).functions_in(ZoneKinds::of(&[ZoneKind::Hand, ZoneKind::Library, ZoneKind::Graveyard, ZoneKind::Exile, ZoneKind::Stack]))], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
