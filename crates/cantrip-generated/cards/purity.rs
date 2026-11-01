// Purity
// status: planned
// requires: prevention with an effect besides
// Flying
// If noncombat damage would be dealt to you, prevent that damage. You gain life equal to the damage prevented this way.
// When Purity is put into a graveyard from anywhere, shuffle it into its owner's library.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_1_then_steps(g: &mut dyn Game, asker: &Asker, subject: Target, data: [u32; 2], done_prevented: u16) -> Result<(), Suspend> { let amount_0 = { let q: &dyn Query = g; ((done_prevented as i64)).clamp(0, 2147483647) as i32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.gain_life(player, amount_0); }
 Ok(()) }
#[allow(unused_variables, dead_code)] fn ability_1_then(g: &mut dyn Game, asker: &Asker, e: &cantrip_api::DamageEvent, prevented: u16) { let q: &dyn Query = g; let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; let _ = ability_1_then_steps(g, asker, subject, data, prevented); }
#[allow(unused_variables)] fn ability_1_applies(q: &dyn Query, asker: &Asker, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if !combat && true && true && matches!(*target, Target::Player(p) if Some(asker.controller) == Some(p)) { return true; } }
 false }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [new.0, u32::MAX]; if to.is(ZoneKind::Graveyard) && (Some(asker.source) == Some(id) && true && !(q.state().obj(id).token)) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_1 = { let q: &dyn Query = g; Some(ObjectId(item.data[0])).filter(|o| *o != ObjectId::NONE).into_iter().collect::<Vec<ObjectId>>() }; let moved_1 = g.put_into_library(&objects_1, LibraryPlace::ShuffledIn);
g.end_sentence();
 Ok(()) }
planned_item!("prevention-with-an-effect-besides" => preventions: PreventionDef { hint: "If noncombat damage would be dealt to you, prevent that damage. You gain life equal to the damage prevented this way.", applies: ability_1_applies, amount: None, then: Some(ability_1_then) });

pub static CARD: CardDef = CardDef { name: "Purity", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [3, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elemental,Subtype::Incarnation]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(6), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_2_event), ability_2, "When Purity is put into a graveyard from anywhere, shuffle it into its owner's library.").targets(&[]).functions_in(ZoneKinds::of(&[ZoneKind::Graveyard]))], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
