// Senu, Keen-Eyed Protector
// status: ready
// Flying, vigilance
// {T}, Exile Senu: You gain 2 life and scry 2.
// When a legendary creature you control attacks and isn't blocked, if this card is exiled, put it onto the battlefield attacking.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (2i64).max(0) as i32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.gain_life(player, amount_0); }
g.end_sentence();
let amount_1 = { let q: &dyn Query = g; (2i64).max(0) as usize }; let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::scry(g, player_1, asker.source, amount_1)?;
g.end_sentence();
 Ok(()) }
fn ability_3_if(q: &dyn Query, asker: &Asker) -> bool { Some(asker.source).is_some_and(|candidate: ObjectId| (q.state().obj(candidate).became.is_none() && q.state().obj(candidate).zone.kind() == ZoneKind::Exile)) }
#[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Unblocked { attacker, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::MAX, u32::MAX]; if (q.chars(id).supertypes.legendary && q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))) { return Some(Fired { subject, data }); } }
 None }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_2 = ({ let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Exile).collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_2 = { let mut moved: Vec<ObjectId> = Vec::new(); for &o in &objects_2 { let controller = player_2; let defender = cantrip_mechanics::combat::choose_attack_target(g, controller, asker.source)?.unwrap_or(Target::None); let entered = g.put_onto_battlefield_with_counters(o, controller, false, &[])?; let came = g.is_on_battlefield(entered); if came { g.enter_attacking(&[entered], defender); } if came { moved.push(entered); } } moved };
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Senu, Keen-Eyed Protector", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Bird,Subtype::Scout]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Vigilance]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }).and(EXILE_THIS), ability_2, "{T}, Exile Senu: You gain 2 life and scry 2.").targets(&[]),triggered(when_asked(EventMask(EventMask::UNBLOCKED.0), ability_3_event), ability_3, "When a legendary creature you control attacks and isn't blocked, if this card is exiled, put it onto the battlefield attacking.").targets(&[]).condition(ability_3_if).functions_in(ZoneKinds::of(&[ZoneKind::Exile]))], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
