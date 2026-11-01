// Tomik, Wielder of Law
// status: ready
// Affinity for planeswalkers (This spell costs {1} less to cast for each planeswalker you control.)
// Flying, vigilance
// Whenever an opponent attacks with creatures, if two or more of those creatures are attacking you and/or planeswalkers you control, that opponent loses 3 life and you draw a card.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_affinity(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { if costed != Costed::Spell(source) { return CostDelta::NONE; } let asker = &Asker::new(source, player); let count = q.permanents_of(player).into_iter().filter(|&id| q.chars(id).is(CardType::Planeswalker)).count(); CostDelta::less(count.min(255) as u8) }
fn ability_3_if(q: &dyn Query, asker: &Asker) -> bool { (((q.state().battlefield().iter().copied().filter(|&id| q.state().attackers.iter().any(|a| a.id == id && { let defender = &a.target; match *defender { Target::Player(p) => Some(asker.controller) == Some(p), Target::Object(o) => q.chars(o).is(CardType::Planeswalker) && Some(asker.controller) == Some(q.controller(o)), _ => false } })).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| (q.state().attackers.iter().any(|a| a.id == id) || q.was_attacking(id))).collect::<Vec<ObjectId>>()).len() as i64) >= (2i64) }
#[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defender, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(q.controller(id).0), u32::from(defending.0)]; if q.chars(id).is(CardType::Creature) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(id)) && true { return Some(Fired { subject, data }); } }
 None }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (3i64).max(0) as i32 }; let players_0 = { let q: &dyn Query = g; Some(PlayerId(item.data[0] as u8)).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.lose_life(player, amount_0); }
g.end_sentence();
let amount_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.draw(player, amount_1); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Tomik, Wielder of Law", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Advisor]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Vigilance]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_3_event), ability_3, "Whenever an opponent attacks with creatures, if two or more of those creatures are attacking you and/or planeswalkers you control, that opponent loses 3 life and you draw a card.").targets(&[]).condition(ability_3_if).once_per_batch()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, own_cost_changes: &[ability_0_affinity], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
