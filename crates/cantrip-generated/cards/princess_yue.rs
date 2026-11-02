// Princess Yue
// status: planned
// requires: enters tapped as something else
// When Princess Yue dies, if she was a nonland creature, return this card to the battlefield tapped under your control. She's a land named Moon. She gains "{T}: Add {C}." (She's still legendary.)
// {T}: Scry 2.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *old; let subject = Target::Object(id); let data = [new.0, u32::MAX]; if to.is(ZoneKind::Graveyard) && from.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) && subject.object().is_some_and(|candidate: ObjectId| (!(q.chars(candidate).is(CardType::Land)) && q.chars(candidate).is(CardType::Creature))) { return Some(Fired { subject, data }); } }
 None }
fn granted_1_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_2_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_2 {
g.add_mana(player, Mana::C, mana_2_0);}
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller); if !{ let q: &dyn Query = g; item.subject.object().is_some_and(|candidate: ObjectId| (!(q.chars(candidate).is(CardType::Land)) && q.chars(candidate).is(CardType::Creature))) } { return Ok(()); } let asker = &Asker::new(item.object, item.controller);
let objects_0: Vec<ObjectId> = { let q: &dyn Query = g; ((Some(ObjectId(item.data[0])).filter(|o| *o != ObjectId::NONE).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Graveyard).collect::<Vec<ObjectId>>()).get(0).copied().into_iter().collect::<Vec<ObjectId>>() }; let moved_0: Vec<ObjectId> = planned_value!("enters tapped as something else", Vec::<ObjectId>::new() => cantrip_mechanics::enter::in_state(g, asker, &objects_0, Destination::Battlefield({ let q: &dyn Query = g; Some(asker.controller) }), Arrival { tapped: true, counters: &[], states: &[ArrivalState::Changes(&[EffectKind::SetTypes(TypeSet::of(&[CardType::Land])), EffectKind::SetName("Moon"), EffectKind::AddAbility(AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0))])] }, false)?);
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_3 = { let q: &dyn Query = g; (2i64).max(0) as usize }; let player_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::scry(g, player_3, asker.source, amount_3)?;
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Princess Yue", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Noble,Subtype::Ally]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When Princess Yue dies, if she was a nonland creature, return this card to the battlefield tapped under your control. She's a land named Moon. She gains \"{T}: Add {C}.\"").targets(&[]),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_1, "{T}: Scry 2.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), Adds { pool: ManaPool([0, 0, 0, 0, 0, 1]), ..add_by(granted_1_mana) }, "{T}: Add {C}.")], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
