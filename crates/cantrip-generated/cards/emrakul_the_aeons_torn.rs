// Emrakul, the Aeons Torn
// status: planned
// requires: engine: no protection from this quality
// This spell can't be countered.
// When you cast this spell, take an extra turn after this one.
// Flying, protection from spells that are one or more colors, annihilator 6
// When Emrakul is put into a graveyard from anywhere, its owner shuffles their graveyard into their library.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
{ let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let times = 1u32; g.add_extra_turns(player, times); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_5_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [new.0, u32::MAX]; if to.is(ZoneKind::Graveyard) && (Some(asker.source) == Some(id) && true && !(q.state().obj(id).token)) { return Some(Fired { subject, data }); } }
 None }
fn ability_5(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_1 = { let q: &dyn Query = g; { let mut all: Vec<ObjectId> = Vec::new(); for id in ((((Some(ObjectId(item.data[0])).filter(|o| *o != ObjectId::NONE).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Graveyard).collect::<Vec<ObjectId>>()).get(0).copied().map(|o: ObjectId| q.state().obj(o).owner).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } all } }; let moved_1 = g.put_into_library(&objects_1, LibraryPlace::Top);
for player in { let q: &dyn Query = g; ((Some(ObjectId(item.data[0])).filter(|o| *o != ObjectId::NONE).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Graveyard).collect::<Vec<ObjectId>>()).get(0).copied().map(|o: ObjectId| q.state().obj(o).owner).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-protection-from-this-quality" => statics: cantrip_mechanics::protection::from_matching(ObjectKind::Any, |q, asker, source| !(q.chars(source).colors.is_empty()), "Protection from a quality"));

pub static CARD: CardDef = CardDef { name: "Emrakul, the Aeons Torn", token_words: &[], oracle: true, cost: ManaCost { generic: 15, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Eldrazi]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(15), toughness: Some(15), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::CantBeCountered,Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_1_event), ability_1, "When you cast this spell, take an extra turn after this one.").targets(&[]).functions_in(ZoneKinds::of(&[ZoneKind::Stack])),cantrip_mechanics::triggers::annihilator::<6>("Annihilator"),triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_5_event), ability_5, "When Emrakul is put into a graveyard from anywhere, its owner shuffles their graveyard into their library.").targets(&[]).functions_in(ZoneKinds::of(&[ZoneKind::Graveyard]))], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
