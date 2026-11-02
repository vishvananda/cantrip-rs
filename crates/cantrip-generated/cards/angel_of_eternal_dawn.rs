// Angel of Eternal Dawn
// status: planned
// requires: ability Unique
// Flying
// When this creature enters, it becomes day.
// It can't become night.
// Your opponents can't cast spells with mana value greater than the number of turns they have begun.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
g.become_day()?;
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_3_0_rule(q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId) -> bool { !(player != asker.controller && (i64::from(q.mana_value(spell))) > (Some(player).map_or(0i64, |p: PlayerId| q.history().all().iter().filter(|r| matches!(r.event, Event::TurnBegan { player, .. } if player == p)).count() as i64))) }
planned_item!("ability-unique" => statics: game_rule(GameRule::NeverNight));

pub static CARD: CardDef = CardDef { name: "Angel of Eternal Dawn", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Angel]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When this creature enters, it becomes day.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, spell_rules: &[ability_3_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
