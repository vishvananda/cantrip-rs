// Price of Glory
// status: planned
// requires: engine: only a mana ability triggers on tapping for mana
// Whenever a player taps a land for mana, if it's not that player's turn, destroy that land.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller); if !{ let q: &dyn Query = g; ({ let other = Some(PlayerId(item.data[0] as u8)); q.state().apnap().filter(|p| Some(*p) != other).collect::<Vec<PlayerId>>() }).into_iter().any(|bound_0: PlayerId| Some(bound_0) == Some(q.state().turn.active)) } { return Ok(()); } let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; item.subject.object().into_iter().collect::<Vec<ObjectId>>() }; let destroyed_0 = g.destroy_all(&objects_0);
g.end_sentence();
 Ok(()) }
planned_item!("engine-only-a-mana-ability-triggers-on-tapping-for-mana" => matcher: #[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::TappedForMana { permanent, player, produced } = event { let id = *permanent; let subject = Target::Object(id); let data = [u32::from(player.0), Mana::ALL.iter().filter(|&&m| produced.get(m) > 0).fold(0u32, |types, &m| types | 1 << m as u32)]; if true && q.chars(id).is(CardType::Land) && ({ let other = Some(PlayerId(data[0] as u8)); q.state().apnap().filter(|p| Some(*p) != other).collect::<Vec<PlayerId>>() }).into_iter().any(|bound_0: PlayerId| Some(bound_0) == Some(q.state().turn.active)) { return Some(Fired { subject, data }); } }
 None });

pub static CARD: CardDef = CardDef { name: "Price of Glory", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("engine: only a mana ability triggers on tapping for mana", EventMask(0) => EventMask(EventMask::TAPPED.0)), ability_0_event), ability_0, "Whenever a player taps a land for mana, if it's not that player's turn, destroy that land.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
