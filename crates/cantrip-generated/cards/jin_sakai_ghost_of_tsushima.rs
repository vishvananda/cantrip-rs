// Jin Sakai, Ghost of Tsushima
// status: ready
// Whenever Jin Sakai deals combat damage to a player, draw a card.
// Whenever a creature you control attacks a player, if no other creatures are attacking that player, choose one —
// • Standoff — It gains double strike until end of turn.
// • Ghost — It can't be blocked this turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(asker.source) == Some(source) && matches!(*target, Target::Player(p) if true) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.draw(player, amount_0); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defender, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(q.controller(id).0), u32::from(defending.0)]; if (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))) && match *defender { Target::Player(p) => true, _ => false } && (((q.state().battlefield().iter().copied().filter(|&id| !(subject.object() == Some(id))).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().attackers.iter().any(|a| a.id == id && { let defender = &a.target; match *defender { Target::Player(p) => Some(PlayerId(data[1] as u8)) == Some(p), _ => false } })).collect::<Vec<ObjectId>>()).len() as i64) == (0i64) { return Some(Fired { subject, data }); } }
 None }
fn ability_1_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller); if !{ let q: &dyn Query = g; (((q.state().battlefield().iter().copied().filter(|&id| !(item.subject.object() == Some(id))).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().attackers.iter().any(|a| a.id == id && { let defender = &a.target; match *defender { Target::Player(p) => Some(PlayerId(item.data[1] as u8)) == Some(p), _ => false } })).collect::<Vec<ObjectId>>()).len() as i64) == (0i64) } { return Ok(()); } let asker = &Asker::new(item.object, item.controller);
let objects_1 = { let q: &dyn Query = g; (item.subject.object().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_1 = { let q: &dyn Query = g; Some(asker.source) }; let duration_1 = Duration::EndOfTurn;
let kind_1_0_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::DoubleStrike])) };
if let Some(source) = source_1 { cantrip_api::instruction(g, |g| { for &object in &objects_1 { g.add_effect(source, object, kind_1_0_0, duration_1);  } }); }
g.end_sentence();
 Ok(()) }
fn ability_1_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller); if !{ let q: &dyn Query = g; (((q.state().battlefield().iter().copied().filter(|&id| !(item.subject.object() == Some(id))).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().attackers.iter().any(|a| a.id == id && { let defender = &a.target; match *defender { Target::Player(p) => Some(PlayerId(item.data[1] as u8)) == Some(p), _ => false } })).collect::<Vec<ObjectId>>()).len() as i64) == (0i64) } { return Ok(()); } let asker = &Asker::new(item.object, item.controller);
let objects_2 = { let q: &dyn Query = g; (item.subject.object().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_2 = { let q: &dyn Query = g; Some(asker.source) }; let duration_2 = Duration::EndOfTurn;
let kind_2_0_0 = { let q: &dyn Query = g; EffectKind::CantBeBlocked };
if let Some(source) = source_2 { cantrip_api::instruction(g, |g| { for &object in &objects_2 { g.add_effect(source, object, kind_2_0_0, duration_2);  } }); }
g.end_sentence();
 Ok(()) }
static ABILITY_1_MODES: ModalDef = modes(&[mode(ability_1_mode_0, "Standoff — It gains double strike until end of turn.").targets(&[]),mode(ability_1_mode_1, "Ghost — It can't be blocked this turn.").targets(&[])]).choose(1, 1);

pub static CARD: CardDef = CardDef { name: "Jin Sakai, Ghost of Tsushima", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 1, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Samurai]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_0_event), ability_0, "Whenever Jin Sakai deals combat damage to a player, draw a card.").targets(&[]),modal_triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), &ABILITY_1_MODES, "Whenever a creature you control attacks a player, if no other creatures are attacking that player, choose one — Standoff — It gains double strike until end of turn; or Ghost — It can't be blocked this turn.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
