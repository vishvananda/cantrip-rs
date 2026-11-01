// Nightveil Specter
// status: ready
// Flying
// Whenever this creature deals combat damage to a player, that player exiles the top card of their library.
// You may play lands and cast spells from among cards exiled with this creature.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(asker.source) == Some(source) && matches!(*target, Target::Player(p) if true) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (((Some(PlayerId(item.data[1] as u8)).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let moved_0 = { let by = g.linked(item); g.exile_with(&objects_0, by, false) };
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_0_allows(q: &dyn Query, asker: &Asker, card: ObjectId, player: PlayerId) -> bool { player == asker.controller && q.exiled_with(q.linked_to(asker)).contains(&card) && q.chars(card).is(CardType::Land) }
#[allow(unused_variables)] fn ability_2_1_allows(q: &dyn Query, asker: &Asker, card: ObjectId, player: PlayerId) -> bool { player == asker.controller && q.exiled_with(q.linked_to(asker)).contains(&card) }

pub static CARD: CardDef = CardDef { name: "Nightveil Specter", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [Some((Mana::U, Mana::B)),Some((Mana::U, Mana::B)),Some((Mana::U, Mana::B)),None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Specter]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_1_event), ability_1, "Whenever this creature deals combat damage to a player, that player exiles the top card of their library.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, exile_casts: &[ExileCast { text: "Play from exile", allows: ability_2_0_allows, additional: Cost::NONE, any_type: false, lands: true, life_for_mana_value: false },ExileCast { text: "Cast from exile", allows: ability_2_1_allows, additional: Cost::NONE, any_type: false, lands: false, life_for_mana_value: false }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
