// Advanced Floral Invocations
// status: planned
// requires: engine: no perpetual effects
// requires: static play permission with an effect on what is played
// (Gain the next level as a sorcery to add its ability.)
// Landfall — Whenever a land you control enters, mill two cards.
// {B/G}: Level 2
// Whenever a land you control enters, creature cards in your graveyard perpetually get +1/+1.
// {4}{B/G}{B/G}: Level 3
// You may play lands and cast creature spells from your graveyard. If you cast a spell this way, that creature enters with a finality counter on it.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && (q.chars(id).is(CardType::Land) && Some(asker.controller) == Some(q.controller(id))) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let mills_0: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_0)).collect(); let milled_0 = g.mill_together(&mills_0);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_level_2_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && (q.chars(id).is(CardType::Land) && Some(asker.controller) == Some(q.controller(id))) && q.state().obj(asker.source).class_level >= 2 { return Some(Fired { subject, data }); } }
 None }
fn ability_1_level_2_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("engine-no-perpetual-effects" => let objects_2: Vec<ObjectId> = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() };
for &object in &objects_2 { let kind_2_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::Pump((1i64) as i16, (1i64) as i16)) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_2_0); });
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_level_3_0_0_0_cost(q: &dyn Query, asker: &Asker, card: ObjectId, player: PlayerId) -> Option<Cost> { (player == asker.controller && (q.state().obj(asker.source).class_level >= 3) && (q.chars(card).is(CardType::Land) && (q.state().obj(card).became.is_none() && q.state().obj(card).zone.kind() == ZoneKind::Graveyard && Some(asker.controller) == Some(q.state().obj(card).owner))) && q.chars(card).is(CardType::Land)).then(|| Cost::mana(q.card(q.chars(card).card).cost)) }
#[allow(unused_variables)] fn ability_1_level_3_0_0_1_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { (q.state().obj(asker.source).class_level >= 3) && player == asker.controller }
planned_item!("static-play-permission-with-an-effect-on-what-is-played" => play_terms: &[PlayTerm::Rider(&[&[SpellEffect::EntersWithACounter(CounterType::FinalityCounter)]])]);
planned_item!("static-play-permission-with-an-effect-on-what-is-played" => statics: player_rule(ability_1_level_3_0_0_1_applies, PlayerRule::Play { zone: ZoneKind::Graveyard, cards: CardsOfRule::Asked(|q: &dyn Query, asker: &Asker, player: PlayerId, card: ObjectId| ((((Some(player).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(asker.controller) == Some(q.state().obj(id).owner))).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>()).contains(&card)), grant: PlayGrant { terms: Some(0), ..PlayGrant::CAST } }));

pub static CARD: CardDef = CardDef { name: "Advanced Floral Invocations", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [Some((Mana::B, Mana::G)),None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Class]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "Landfall — Whenever a land you control enters, mill two cards.").targets(&[]),cantrip_mechanics::class::level_two("{B/G}", "{B/G}: Level 2"),triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_level_2_0_event), ability_1_level_2_0, "You may play lands and cast creature spells from your graveyard. If you cast a spell this way, that creature enters with a finality counter on it.").targets(&[]),cantrip_mechanics::class::level_three("{4}{B/G}{B/G}", "{4}{B/G}{B/G}: Level 3")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_ways: &[GrantedWay { text: "You may play lands and cast creature spells from your graveyard. If you cast a spell this way, that creature enters with a finality counter on it.", permission: true, lands: true, cost: ability_1_level_3_0_0_0_cost, ..GrantedWay::CAST }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
